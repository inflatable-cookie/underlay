//! Audit log query functions.

use crate::entry::AuditLogRow;
use crate::error::AuditResult;
use crate::tables::AuditTable;
use crate::DbPool;
use chrono::{DateTime, Utc};
use tracing::instrument;
use uuid::Uuid;

/// Default page size for audit log queries.
pub const DEFAULT_AUDIT_LOG_LIMIT: i64 = 50;

/// Maximum page size allowed for audit log queries.
pub const MAX_AUDIT_LOG_LIMIT: i64 = 1_000;

/// Filters for listing audit log entries.
#[derive(Debug, Clone)]
pub struct AuditLogFilters {
    /// Filter by user ID.
    pub user_id: Option<Uuid>,
    /// Filter by action type.
    pub action: Option<String>,
    /// Filter by resource type.
    pub resource_type: Option<String>,
    /// Filter by resource ID.
    pub resource_id: Option<Uuid>,
    /// Filter by entries after this time.
    pub since: Option<DateTime<Utc>>,
    /// Filter by entries before this time.
    pub until: Option<DateTime<Utc>>,
    /// Maximum number of entries to return.
    pub limit: i64,
    /// Number of entries to skip.
    pub offset: i64,
}

impl Default for AuditLogFilters {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditLogFilters {
    /// Create filters with default pagination (50 entries).
    pub fn new() -> Self {
        Self {
            user_id: None,
            action: None,
            resource_type: None,
            resource_id: None,
            since: None,
            until: None,
            limit: DEFAULT_AUDIT_LOG_LIMIT,
            offset: 0,
        }
    }

    /// Filter by user.
    pub fn with_user(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Filter by action.
    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// Filter by resource type.
    pub fn with_resource_type(mut self, resource_type: impl Into<String>) -> Self {
        self.resource_type = Some(resource_type.into());
        self
    }

    /// Filter by resource ID.
    pub fn with_resource_id(mut self, resource_id: Uuid) -> Self {
        self.resource_id = Some(resource_id);
        self
    }

    /// Filter by time range.
    pub fn with_time_range(mut self, since: DateTime<Utc>, until: DateTime<Utc>) -> Self {
        self.since = Some(since);
        self.until = Some(until);
        self
    }

    /// Set pagination, clamping `limit` to `1..=MAX_AUDIT_LOG_LIMIT` and `offset` to `>= 0`.
    pub fn with_pagination(mut self, limit: i64, offset: i64) -> Self {
        self.limit = limit.clamp(1, MAX_AUDIT_LOG_LIMIT);
        self.offset = offset.max(0);
        self
    }
}

/// List audit log entries from a typed table location.
///
/// Results are ordered by `occurred_at` descending (with `id` descending as
/// tiebreaker), newest first.
#[instrument(skip(pool, filters))]
pub async fn list_audit_logs_from_table(
    pool: &DbPool,
    table: &AuditTable,
    filters: AuditLogFilters,
) -> AuditResult<Vec<AuditLogRow>> {
    let table = table.quoted();

    // Build dynamic query with filters
    // We use a fixed parameter order for predictable binding
    let query = format!(
        r#"
        SELECT
            id,
            occurred_at,
            user_id,
            action,
            resource_type,
            resource_id,
            details,
            correlation_id,
            ip_address
        FROM {}
        WHERE
            ($1::uuid IS NULL OR user_id = $1)
            AND ($2::text IS NULL OR action = $2)
            AND ($3::text IS NULL OR resource_type = $3)
            AND ($4::uuid IS NULL OR resource_id = $4)
            AND ($5::timestamptz IS NULL OR occurred_at >= $5)
            AND ($6::timestamptz IS NULL OR occurred_at <= $6)
        ORDER BY occurred_at DESC, id DESC
        LIMIT $7 OFFSET $8
        "#,
        table
    );

    Ok(sqlx::query_as::<_, AuditLogRow>(sqlx::AssertSqlSafe(query))
        .bind(filters.user_id)
        .bind(filters.action)
        .bind(filters.resource_type)
        .bind(filters.resource_id)
        .bind(filters.since)
        .bind(filters.until)
        .bind(filters.limit.clamp(1, MAX_AUDIT_LOG_LIMIT))
        .bind(filters.offset.max(0))
        .fetch_all(pool)
        .await?)
}

/// Get a single audit log entry by ID from a typed table location.
#[instrument(skip(pool))]
pub async fn get_audit_log_by_id_from_table(
    pool: &DbPool,
    table: &AuditTable,
    id: Uuid,
) -> AuditResult<Option<AuditLogRow>> {
    let table = table.quoted();

    let query = format!(
        r#"
        SELECT
            id,
            occurred_at,
            user_id,
            action,
            resource_type,
            resource_id,
            details,
            correlation_id,
            ip_address
        FROM {}
        WHERE id = $1
        "#,
        table
    );

    Ok(sqlx::query_as::<_, AuditLogRow>(sqlx::AssertSqlSafe(query))
        .bind(id)
        .fetch_optional(pool)
        .await?)
}

/// Count audit log entries matching filters from a typed table location.
#[instrument(skip(pool, filters))]
pub async fn count_audit_logs_from_table(
    pool: &DbPool,
    table: &AuditTable,
    filters: &AuditLogFilters,
) -> AuditResult<i64> {
    let table = table.quoted();

    let query = format!(
        r#"
        SELECT COUNT(*) as count
        FROM {}
        WHERE
            ($1::uuid IS NULL OR user_id = $1)
            AND ($2::text IS NULL OR action = $2)
            AND ($3::text IS NULL OR resource_type = $3)
            AND ($4::uuid IS NULL OR resource_id = $4)
            AND ($5::timestamptz IS NULL OR occurred_at >= $5)
            AND ($6::timestamptz IS NULL OR occurred_at <= $6)
        "#,
        table
    );

    let row: (i64,) = sqlx::query_as(sqlx::AssertSqlSafe(query))
        .bind(filters.user_id)
        .bind(&filters.action)
        .bind(&filters.resource_type)
        .bind(filters.resource_id)
        .bind(filters.since)
        .bind(filters.until)
        .fetch_one(pool)
        .await?;

    Ok(row.0)
}

#[cfg(test)]
#[path = "tests/query_tests.rs"]
mod tests;
