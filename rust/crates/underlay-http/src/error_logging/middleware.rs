use super::{append_error_log, ErrorLoggingConfig};
use crate::errors::ErrorDetail;
use axum::{body::Body, http::Request, middleware::Next, response::Response};

/// Legacy header name once used to pass error context to the logging
/// middleware. Error detail now travels in response extensions
/// ([`ErrorDetail`]); this header is stripped from every response so internal
/// detail can never reach clients.
pub const ERROR_CONTEXT_HEADER: &str = "x-error-context";

const ERROR_MESSAGE_HEADER: &str = "x-error-message";

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
pub async fn error_logging_middleware(
    axum::extract::State(config): axum::extract::State<ErrorLoggingConfig>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let query = req.uri().query().map(redact_query);
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let mut res = next.run(req).await;

    // Internal error detail must never ship to clients, even if an older
    // layer or handler still writes these headers.
    res.headers_mut().remove(ERROR_MESSAGE_HEADER);
    res.headers_mut().remove(ERROR_CONTEXT_HEADER);

    let status = res.status();

    let should_log = (status.is_client_error() && config.log_client_errors())
        || (status.is_server_error() && config.log_server_errors());

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

    let detail = res.extensions().get::<ErrorDetail>();

    let message = detail.map(|d| d.message.clone()).unwrap_or_default();

    // Keep logs actionable when handlers do not attach error detail.
    let handler_context = detail
        .map(|d| d.context.clone())
        .unwrap_or_else(|| fallback_handler_context(&method, &path, status, &error_code));

    let context = serde_json::json!({
        "source": config.source(),
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

const SENSITIVE_QUERY_KEYS: &[&str] = &[
    "token",
    "access_token",
    "refresh_token",
    "id_token",
    "api_key",
    "apikey",
    "key",
    "secret",
    "client_secret",
    "password",
    "code",
    "session",
    "session_id",
    "signature",
    "sig",
    "auth",
];

/// Redact values of known-sensitive query parameters before the query string
/// is persisted to the error log.
fn redact_query(query: &str) -> String {
    query
        .split('&')
        .map(|pair| {
            let Some((key, _)) = pair.split_once('=') else {
                return pair.to_string();
            };
            if SENSITIVE_QUERY_KEYS.contains(&key.trim().to_ascii_lowercase().as_str()) {
                format!("{key}=[REDACTED]")
            } else {
                pair.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("&")
}

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

#[cfg(test)]
mod tests {
    use super::{fallback_handler_context, redact_query};
    use http::{Method, StatusCode};

    #[test]
    fn redact_query_masks_sensitive_values() {
        assert_eq!(
            redact_query("access_token=abc123&page=2"),
            "access_token=[REDACTED]&page=2"
        );
        assert_eq!(
            redact_query("PASSWORD=hunter2&Code=xyz"),
            "PASSWORD=[REDACTED]&Code=[REDACTED]"
        );
        assert_eq!(redact_query("page=2&sort=name"), "page=2&sort=name");
        assert_eq!(redact_query("flag"), "flag");
        assert_eq!(redact_query(""), "");
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
