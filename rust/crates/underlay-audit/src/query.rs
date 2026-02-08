//! Audit log query functions.

use crate::entry::AuditLogRow;
use crate::error::AuditResult;
use crate::DbPool;
use chrono::{DateTime, Utc};
use tracing::instrument;
use uuid::Uuid;

/// Filters for listing audit log entries.
#[derive(Debug, Clone, Default)]
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

impl AuditLogFilters {
    /// Create filters with default pagination (50 entries).
    pub fn new() -> Self {
        Self {
            limit: 50,
            offset: 0,
            ..Default::default()
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

    /// Set pagination.
    pub fn with_pagination(mut self, limit: i64, offset: i64) -> Self {
        self.limit = limit;
        self.offset = offset;
        self
    }
}

/// List audit log entries with filtering and pagination.
///
/// Results are ordered by `occurred_at` descending (most recent first).
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `table` - Fully qualified table name (e.g., "platform.audit_log")
/// * `filters` - Query filters
#[instrument(skip(pool, filters))]
pub async fn list_audit_logs(
    pool: &DbPool,
    table: &str,
    filters: AuditLogFilters,
) -> AuditResult<Vec<AuditLogRow>> {
    crate::validate_table_name(table)?;

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
        ORDER BY occurred_at DESC
        LIMIT $7 OFFSET $8
        "#,
        table
    );

    Ok(sqlx::query_as::<_, AuditLogRow>(&query)
        .bind(filters.user_id)
        .bind(filters.action)
        .bind(filters.resource_type)
        .bind(filters.resource_id)
        .bind(filters.since)
        .bind(filters.until)
        .bind(filters.limit)
        .bind(filters.offset)
        .fetch_all(pool)
        .await?)
}

/// Get a single audit log entry by ID.
#[instrument(skip(pool))]
pub async fn get_audit_log_by_id(
    pool: &DbPool,
    table: &str,
    id: Uuid,
) -> AuditResult<Option<AuditLogRow>> {
    crate::validate_table_name(table)?;

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

    Ok(sqlx::query_as::<_, AuditLogRow>(&query)
        .bind(id)
        .fetch_optional(pool)
        .await?)
}

/// Count audit log entries matching filters.
#[instrument(skip(pool, filters))]
pub async fn count_audit_logs(
    pool: &DbPool,
    table: &str,
    filters: &AuditLogFilters,
) -> AuditResult<i64> {
    crate::validate_table_name(table)?;

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

    let row: (i64,) = sqlx::query_as(&query)
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
mod tests {
    use super::*;

    #[test]
    fn filters_builder() {
        let filters = AuditLogFilters::new()
            .with_action("create")
            .with_resource_type("pathway")
            .with_pagination(100, 50);

        assert_eq!(filters.action, Some("create".to_string()));
        assert_eq!(filters.resource_type, Some("pathway".to_string()));
        assert_eq!(filters.limit, 100);
        assert_eq!(filters.offset, 50);
    }
}
