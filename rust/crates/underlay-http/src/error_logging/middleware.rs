use super::{append_error_log, ErrorLoggingConfig};
use axum::{body::Body, http::Request, middleware::Next, response::Response};

/// Header name for passing error context to the logging middleware.
pub const ERROR_CONTEXT_HEADER: &str = "x-error-context";

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
    let query = req.uri().query().map(|q| q.to_string());
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let res = next.run(req).await;

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

    let message = res
        .headers()
        .get("x-error-message")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    // Keep logs actionable when handlers do not emit x-error-context.
    let handler_context = extract_handler_context(&res)
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

fn extract_handler_context(res: &Response) -> Option<serde_json::Value> {
    res.headers()
        .get(ERROR_CONTEXT_HEADER)
        .and_then(|v| v.to_str().ok())
        .and_then(|encoded| urlencoding::decode(encoded).ok())
        .and_then(|json_str| serde_json::from_str(&json_str).ok())
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
