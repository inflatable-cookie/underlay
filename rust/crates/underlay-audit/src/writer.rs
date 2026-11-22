//! Audit log writer functions.

use crate::entry::{AuditEntry, AuditLogRow};
use crate::error::AuditResult;
use crate::tables::AuditTable;
use crate::DbPool;
use tokio::sync::Semaphore;
use tracing::{info, instrument};

/// Maximum number of concurrent fire-and-forget audit log writes spawned by
/// [`append_audit_log_to_table_async`].
pub const MAX_CONCURRENT_ASYNC_AUDIT_WRITES: usize = 64;

static AUDIT_WRITE_SEMAPHORE: Semaphore = Semaphore::const_new(MAX_CONCURRENT_ASYNC_AUDIT_WRITES);

/// Append an audit log entry to a typed table location.
///
/// The table must have the expected schema (see crate documentation).
///
/// # Arguments
///
/// * `executor` - Database connection pool, connection, or active transaction
/// * `table` - Typed table location
/// * `entry` - The audit entry to log
///
/// # Example
///
/// ```rust,ignore
/// use underlay_audit::{append_audit_log_to_table, AuditAction, AuditEntry, AuditTable};
///
/// let audit_table = AuditTable::parse("platform.audit_log")?;
///
/// append_audit_log_to_table(
///     &pool,
///     &audit_table,
///     AuditEntry::new(
///         Some(user_id),
///         AuditAction::Create,
///         "pathway",
///         pathway_id,
///     ).with_details(serde_json::json!({ "title": title })),
/// ).await?;
/// ```
#[instrument(skip(executor, entry), fields(action = %entry.action, resource_type = %entry.resource_type))]
pub async fn append_audit_log_to_table<'e, E>(
    executor: E,
    table: &AuditTable,
    entry: AuditEntry,
) -> AuditResult<AuditLogRow>
where
    E: sqlx::PgExecutor<'e>,
{
    let table = table.quoted();

    let query = format!(
        r#"
        INSERT INTO {} (
            user_id,
            action,
            resource_type,
            resource_id,
            details,
            correlation_id,
            ip_address
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING
            id,
            occurred_at,
            user_id,
            action,
            resource_type,
            resource_id,
            details,
            correlation_id,
            ip_address
        "#,
        table
    );

    let row = sqlx::query_as::<_, AuditLogRow>(sqlx::AssertSqlSafe(query))
        .bind(entry.user_id)
        .bind(entry.action.as_str())
        .bind(&entry.resource_type)
        .bind(entry.resource_id)
        .bind(&entry.details)
        .bind(&entry.correlation_id)
        .bind(&entry.ip_address)
        .fetch_one(executor)
        .await?;

    info!(
        audit_id = %row.id,
        user_id = ?entry.user_id,
        action = %entry.action,
        resource_type = %entry.resource_type,
        resource_id = %entry.resource_id,
        "Audit log entry created"
    );

    Ok(row)
}

/// Append an audit log entry to a typed table location without waiting.
///
/// Concurrency is bounded by [`MAX_CONCURRENT_ASYNC_AUDIT_WRITES`]; when all
/// slots are busy, excess entries are dropped with a warning rather than
/// spawning unbounded background tasks.
pub fn append_audit_log_to_table_async(pool: DbPool, table: AuditTable, entry: AuditEntry) {
    let Ok(permit) = AUDIT_WRITE_SEMAPHORE.try_acquire() else {
        tracing::warn!(
            action = %entry.action,
            resource_type = %entry.resource_type,
            resource_id = %entry.resource_id,
            "audit log writer saturated; dropping audit log entry"
        );
        return;
    };

    tokio::spawn(async move {
        let _permit = permit;
        if let Err(e) = append_audit_log_to_table(&pool, &table, entry).await {
            tracing::error!(error = %e, "Failed to write audit log entry");
        }
    });
}

#[cfg(test)]
#[path = "tests/writer_tests.rs"]
mod tests;
