use super::append_error_log;
use underlay_db::DbPool;

/// Database implementation of the `ErrorLogSink` trait.
///
/// This allows the HTTP error response system to automatically log errors
/// to the database in a non-blocking manner.
///
/// # Example
///
/// ```rust,ignore
/// use underlay_http::error_logging::DbErrorLogSink;
/// use underlay_http::ErrorLogSink;
///
/// let sink = DbErrorLogSink::new(pool.clone());
///
/// // Use in error responses
/// sink.record(ErrorLogContext {
///     request_id: Some(request_id),
///     status: StatusCode::INTERNAL_SERVER_ERROR,
///     code: "database_error".to_string(),
///     message: "Failed to query database".to_string(),
/// });
/// ```
#[derive(Clone)]
pub struct DbErrorLogSink {
    pool: DbPool,
}

impl DbErrorLogSink {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl crate::ErrorLogSink for DbErrorLogSink {
    fn record(&self, ctx: crate::ErrorLogContext) {
        let pool = self.pool.clone();
        let status = ctx.status.as_u16() as i32;
        let correlation_id = ctx
            .request_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        tokio::spawn(async move {
            let _ = append_error_log(
                &pool,
                "", // endpoint - will be filled by middleware
                "", // method - will be filled by middleware
                status,
                &ctx.code,
                &ctx.message,
                &correlation_id,
                serde_json::json!({}),
            )
            .await;
        });
    }
}
