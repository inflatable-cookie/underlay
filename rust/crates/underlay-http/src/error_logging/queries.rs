use super::{filters::push_error_log_filters, ErrorLogFilters, ErrorLogRow};
use sqlx::{Postgres, QueryBuilder};
use underlay_db::DbPool;

/// Append a new error log entry to the database.
///
/// This function inserts an error log record and returns the created row.
/// It's designed to be called asynchronously in a non-blocking manner
/// (e.g., via `tokio::spawn`) so that error logging doesn't slow down
/// request handling.
///
/// # Example
///
/// ```rust,ignore
/// use underlay_http::error_logging::append_error_log;
///
/// tokio::spawn(async move {
///     let _ = append_error_log(
///         &pool,
///         "/v1/users",
///         "GET",
///         500,
///         "internal_error",
///         "Database connection failed",
///         "req-123-456",
///         serde_json::json!({"details": "Connection timeout"}),
///     ).await;
/// });
/// ```
#[allow(clippy::too_many_arguments)]
pub async fn append_error_log(
    pool: &DbPool,
    endpoint: &str,
    method: &str,
    status_code: i32,
    error_code: &str,
    message: &str,
    correlation_id: &str,
    context: serde_json::Value,
) -> Result<ErrorLogRow, sqlx::Error> {
    sqlx::query_as::<_, ErrorLogRow>(
        r#"
        INSERT INTO platform.error_log (
            endpoint,
            method,
            status_code,
            error_code,
            message,
            correlation_id,
            context
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING
            id,
            occurred_at,
            endpoint,
            method,
            status_code,
            error_code,
            message,
            correlation_id,
            context
        "#,
    )
    .bind(endpoint)
    .bind(method)
    .bind(status_code)
    .bind(error_code)
    .bind(message)
    .bind(correlation_id)
    .bind(context)
    .fetch_one(pool)
    .await
}

/// List error log entries with optional filters.
///
/// Returns error logs ordered by `occurred_at DESC` (most recent first).
///
/// # Example
///
/// ```rust,ignore
/// use underlay_http::error_logging::{list_error_logs, ErrorLogFilters};
///
/// let filters = ErrorLogFilters {
///     status_code: Some(500),
///     limit: 50,
///     ..Default::default()
/// };
///
/// let errors = list_error_logs(&pool, filters).await?;
/// ```
pub async fn list_error_logs(
    pool: &DbPool,
    filters: ErrorLogFilters,
) -> Result<Vec<ErrorLogRow>, sqlx::Error> {
    let mut query = QueryBuilder::<Postgres>::new(
        "SELECT id, occurred_at, endpoint, method, status_code, error_code, message, correlation_id, context
         FROM platform.error_log
         WHERE 1=1"
    );

    push_error_log_filters(&mut query, &filters);
    query
        .push(" ORDER BY occurred_at DESC LIMIT ")
        .push_bind(filters.limit)
        .push(" OFFSET ")
        .push_bind(filters.offset);

    query.build_query_as::<ErrorLogRow>().fetch_all(pool).await
}

/// Get a single error log entry by ID.
pub async fn get_error_log_by_id(
    pool: &DbPool,
    id: uuid::Uuid,
) -> Result<Option<ErrorLogRow>, sqlx::Error> {
    sqlx::query_as::<_, ErrorLogRow>(
        r#"
        SELECT id, occurred_at, endpoint, method, status_code, error_code,
               message, correlation_id, context
        FROM platform.error_log
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

/// Count error log entries matching filters (for pagination).
pub async fn count_error_logs(
    pool: &DbPool,
    filters: &ErrorLogFilters,
) -> Result<i64, sqlx::Error> {
    let mut query =
        QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM platform.error_log WHERE 1=1");

    push_error_log_filters(&mut query, filters);

    query.build_query_scalar::<i64>().fetch_one(pool).await
}
