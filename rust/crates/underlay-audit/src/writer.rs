//! Audit log writer functions.

use crate::entry::{AuditEntry, AuditLogRow};
use crate::DbPool;
use tracing::{info, instrument};

/// Append an audit log entry to the specified table.
///
/// The table must have the expected schema (see crate documentation).
/// The table name should be fully qualified (e.g., "infra.audit_log").
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `table` - Fully qualified table name (e.g., "infra.audit_log")
/// * `entry` - The audit entry to log
///
/// # Example
///
/// ```rust,ignore
/// use underlay_audit::{AuditAction, AuditEntry, append_audit_log};
///
/// append_audit_log(
///     &pool,
///     "infra.audit_log",
///     AuditEntry::new(
///         Some(user_id),
///         AuditAction::Create,
///         "pathway",
///         pathway_id,
///     ).with_details(serde_json::json!({ "title": title })),
/// ).await?;
/// ```
#[instrument(skip(pool, entry), fields(action = %entry.action, resource_type = %entry.resource_type))]
pub async fn append_audit_log(
    pool: &DbPool,
    table: &str,
    entry: AuditEntry,
) -> Result<AuditLogRow, sqlx::Error> {
    // Validate table name to prevent SQL injection
    // Only allow alphanumeric, underscore, and dot (for schema.table)
    if !table
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
    {
        return Err(sqlx::Error::Protocol(
            "Invalid table name: must contain only alphanumeric, underscore, or dot".to_string(),
        ));
    }

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

    let row = sqlx::query_as::<_, AuditLogRow>(&query)
        .bind(entry.user_id)
        .bind(entry.action.as_str())
        .bind(&entry.resource_type)
        .bind(entry.resource_id)
        .bind(&entry.details)
        .bind(&entry.correlation_id)
        .bind(&entry.ip_address)
        .fetch_one(pool)
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

/// Append an audit log entry without waiting for the result.
///
/// This is useful for fire-and-forget audit logging where you don't want
/// to block the main request flow. Errors are logged but not returned.
///
/// # Arguments
///
/// * `pool` - Database connection pool (cloned for async task)
/// * `table` - Fully qualified table name
/// * `entry` - The audit entry to log
pub fn append_audit_log_async(pool: DbPool, table: &'static str, entry: AuditEntry) {
    tokio::spawn(async move {
        if let Err(e) = append_audit_log(&pool, table, entry).await {
            tracing::error!(error = %e, "Failed to write audit log entry");
        }
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn validate_table_name() {
        // Valid table names
        assert!("infra.audit_log"
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '.'));
        assert!("audit_log"
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '.'));

        // Invalid table names
        assert!(!"audit; DROP TABLE users"
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '.'));
        assert!(!"audit-log"
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '.'));
    }
}
