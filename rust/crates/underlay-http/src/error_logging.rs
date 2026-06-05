#[cfg(feature = "error-logging")]
use axum::{body::Body, http::Request, middleware::Next, response::Response};
#[cfg(feature = "error-logging")]
use sqlx::{Postgres, QueryBuilder};
#[cfg(feature = "error-logging")]
use underlay_db::DbPool;

/// Header name for passing error context to the logging middleware.
#[cfg(feature = "error-logging")]
pub const ERROR_CONTEXT_HEADER: &str = "x-error-context";

/// Database row returned from error_log queries.
#[cfg(feature = "error-logging")]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ErrorLogRow {
    pub id: uuid::Uuid,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub endpoint: String,
    pub method: String,
    pub status_code: i32,
    pub error_code: String,
    pub message: String,
    pub correlation_id: String,
    pub context: serde_json::Value,
}

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
#[cfg(feature = "error-logging")]
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

/// Filters for querying error log entries.
#[cfg(feature = "error-logging")]
#[derive(Debug, Clone)]
pub struct ErrorLogFilters {
    pub since: Option<chrono::DateTime<chrono::Utc>>,
    pub until: Option<chrono::DateTime<chrono::Utc>>,
    pub status_class: Option<ErrorLogStatusClass>,
    pub status_code: Option<i32>,
    pub error_code: Option<String>,
    pub endpoint: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

#[cfg(feature = "error-logging")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorLogStatusClass {
    Client,
    Server,
}

#[cfg(feature = "error-logging")]
impl Default for ErrorLogFilters {
    fn default() -> Self {
        Self {
            since: None,
            until: None,
            status_class: None,
            status_code: None,
            error_code: None,
            endpoint: None,
            limit: 100,
            offset: 0,
        }
    }
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
#[cfg(feature = "error-logging")]
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
#[cfg(feature = "error-logging")]
#[derive(Clone)]
pub struct DbErrorLogSink {
    pool: DbPool,
}

#[cfg(feature = "error-logging")]
impl DbErrorLogSink {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[cfg(feature = "error-logging")]
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

/// Get a single error log entry by ID.
#[cfg(feature = "error-logging")]
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
#[cfg(feature = "error-logging")]
pub async fn count_error_logs(
    pool: &DbPool,
    filters: &ErrorLogFilters,
) -> Result<i64, sqlx::Error> {
    let mut query =
        QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM platform.error_log WHERE 1=1");

    push_error_log_filters(&mut query, filters);

    query.build_query_scalar::<i64>().fetch_one(pool).await
}

#[cfg(feature = "error-logging")]
fn push_error_log_filters<'a>(
    query: &mut QueryBuilder<'a, Postgres>,
    filters: &'a ErrorLogFilters,
) {
    if let Some(since) = filters.since {
        query.push(" AND occurred_at >= ").push_bind(since);
    }
    if let Some(until) = filters.until {
        query.push(" AND occurred_at <= ").push_bind(until);
    }
    if let Some(status_code) = filters.status_code {
        query.push(" AND status_code = ").push_bind(status_code);
    } else if let Some(status_class) = filters.status_class {
        match status_class {
            ErrorLogStatusClass::Client => {
                query.push(" AND status_code >= 400 AND status_code < 500");
            }
            ErrorLogStatusClass::Server => {
                query.push(" AND status_code >= 500 AND status_code < 600");
            }
        }
    }
    if let Some(error_code) = filters.error_code.as_deref() {
        query.push(" AND error_code = ").push_bind(error_code);
    }
    if let Some(endpoint) = filters.endpoint.as_deref() {
        query.push(" AND endpoint = ").push_bind(endpoint);
    }
}

/// Configuration for the error logging middleware.
#[cfg(feature = "error-logging")]
#[derive(Clone)]
pub struct ErrorLoggingConfig {
    pool: DbPool,
    /// Optional source identifier for the app (e.g., "acme-api", "farmyard-api").
    pub source: Option<String>,
    /// Whether to log 4xx client errors (default: true).
    pub log_client_errors: bool,
    /// Whether to log 5xx server errors (default: true).
    pub log_server_errors: bool,
}

#[cfg(feature = "error-logging")]
impl ErrorLoggingConfig {
    /// Create a new configuration with the given database pool.
    pub fn new(pool: DbPool) -> Self {
        Self {
            pool,
            source: None,
            log_client_errors: true,
            log_server_errors: true,
        }
    }

    /// Set the source identifier for log entries.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Set whether to log 4xx client errors.
    pub fn with_client_errors(mut self, enabled: bool) -> Self {
        self.log_client_errors = enabled;
        self
    }

    /// Set whether to log 5xx server errors.
    pub fn with_server_errors(mut self, enabled: bool) -> Self {
        self.log_server_errors = enabled;
        self
    }
}

/// Create an error logging middleware layer.
///
/// This middleware captures 4xx and 5xx responses and logs them to the database
/// for later inspection. It runs the logging asynchronously to avoid blocking
/// the response.
///
/// # Example
///
/// ```rust,ignore
/// use underlay_http::error_logging::{error_logging_layer, ErrorLoggingConfig};
///
/// let config = ErrorLoggingConfig::new(pool.clone())
///     .with_source("my-api")
///     .with_client_errors(true);
///
/// let app = Router::new()
///     .route("/", get(handler))
///     .layer(axum::middleware::from_fn_with_state(config, error_logging_middleware));
/// ```
#[cfg(feature = "error-logging")]
pub async fn error_logging_middleware(
    axum::extract::State(config): axum::extract::State<ErrorLoggingConfig>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let query = req.uri().query().map(|q| q.to_string());
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let res = next.run(req).await;

    let status = res.status();

    // Check if we should log this response
    let should_log = (status.is_client_error() && config.log_client_errors)
        || (status.is_server_error() && config.log_server_errors);

    if !should_log {
        return res;
    }

    let status_code = status.as_u16() as i32;

    let request_id = res
        .headers()
        .get(underlay_observability::REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let error_code = res
        .headers()
        .get("x-error-code")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let message = res
        .headers()
        .get("x-error-message")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    // Extract handler-provided context when available. If a handler does not
    // emit `x-error-context`, populate a structured fallback so logs remain
    // actionable and avoid null `handler_context`.
    let handler_context = extract_handler_context(&res)
        .unwrap_or_else(|| fallback_handler_context(&method, &path, status, &error_code));

    // Build comprehensive context object
    let context = serde_json::json!({
        "source": config.source,
        "query": query,
        "user_agent": user_agent,
        "handler_context": handler_context,
    });

    let pool = config.pool.clone();
    tokio::spawn(async move {
        if let Err(err) = append_error_log(
            &pool,
            &path,
            method.as_str(),
            status_code,
            &error_code,
            &message,
            &request_id,
            context,
        )
        .await
        {
            tracing::error!(%err, "failed to append error log entry");
        }
    });

    res
}

#[cfg(feature = "error-logging")]
fn extract_handler_context(res: &Response) -> Option<serde_json::Value> {
    res.headers()
        .get(ERROR_CONTEXT_HEADER)
        .and_then(|v| v.to_str().ok())
        .and_then(|encoded| urlencoding::decode(encoded).ok())
        .and_then(|json_str| serde_json::from_str(&json_str).ok())
}

#[cfg(feature = "error-logging")]
fn fallback_handler_context(
    method: &axum::http::Method,
    path: &str,
    status: axum::http::StatusCode,
    error_code: &str,
) -> serde_json::Value {
    serde_json::json!({
        "operation": "http.response",
        "context_source": "middleware_fallback",
        "method": method.as_str(),
        "path": path,
        "status_code": status.as_u16(),
        "error_code": error_code,
    })
}

#[cfg(all(test, feature = "error-logging"))]
mod middleware_tests {
    use super::{extract_handler_context, fallback_handler_context, ERROR_CONTEXT_HEADER};
    use axum::response::Response;
    use http::{Method, StatusCode};

    #[test]
    fn extract_handler_context_decodes_json_header() {
        let raw = r#"{"operation":"test.decode"}"#;
        let encoded = urlencoding::encode(raw);
        let response = Response::builder()
            .header(ERROR_CONTEXT_HEADER, encoded.as_ref())
            .body(axum::body::Body::empty())
            .expect("response should build");

        let context = extract_handler_context(&response).expect("context should decode");
        let operation = context.get("operation").and_then(|v| v.as_str());
        assert_eq!(operation, Some("test.decode"));
    }

    #[test]
    fn extract_handler_context_returns_none_without_header() {
        let response = Response::builder()
            .body(axum::body::Body::empty())
            .expect("response should build");

        assert!(extract_handler_context(&response).is_none());
    }

    #[test]
    fn fallback_handler_context_is_structured_and_non_null() {
        let context = fallback_handler_context(
            &Method::GET,
            "/v1/example",
            StatusCode::UNAUTHORIZED,
            "auth.unauthorized",
        );

        assert_eq!(
            context.get("operation").and_then(|v| v.as_str()),
            Some("http.response")
        );
        assert_eq!(
            context.get("context_source").and_then(|v| v.as_str()),
            Some("middleware_fallback")
        );
        assert_eq!(context.get("method").and_then(|v| v.as_str()), Some("GET"));
        assert_eq!(
            context.get("path").and_then(|v| v.as_str()),
            Some("/v1/example")
        );
        assert_eq!(
            context.get("status_code").and_then(|v| v.as_u64()),
            Some(401)
        );
        assert_eq!(
            context.get("error_code").and_then(|v| v.as_str()),
            Some("auth.unauthorized")
        );
    }
}
