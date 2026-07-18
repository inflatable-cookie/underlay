use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use underlay_core::{AppError, ErrorEnvelope};
use underlay_observability::RequestId;

const ERROR_CODE_HEADER: &str = "x-error-code";

/// Internal error detail carried in response extensions for the error-logging
/// middleware. Never serialized onto the wire: internal messages and context
/// (which can include raw DB/IO cause strings) must not reach clients. The
/// sanitized error envelope is the only client-facing error surface.
#[derive(Debug, Clone)]
pub struct ErrorDetail {
    pub message: String,
    pub context: serde_json::Value,
}

pub trait ErrorLogSink: Send + Sync {
    fn record(&self, ctx: ErrorLogContext);
}

#[derive(Debug, Clone)]
pub struct ErrorLogContext {
    pub request_id: Option<RequestId>,
    pub status: StatusCode,
    pub code: String,
    pub message: String,
}

/// Canonical API error type for handlers.
///
/// `ApiError` is the preferred path for returning errors from HTTP handlers.
/// It emits the standard Underlay error envelope and logging headers used by
/// error logging middleware.
#[derive(Debug, Clone)]
pub struct ApiError {
    pub status: StatusCode,
    pub err: AppError,
    pub context: serde_json::Value,
}

/// Canonical result type for HTTP handlers.
pub type ApiResult<T> = Result<T, ApiError>;

impl ApiError {
    pub fn new(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            err: AppError::new(code, message),
            context: serde_json::json!({}),
        }
    }

    pub fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, code, message)
    }

    pub fn not_found(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, code, message)
    }

    pub fn conflict(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, code, message)
    }

    pub fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, code, message)
    }

    pub fn with_field_errors(
        mut self,
        field_errors: std::collections::HashMap<String, String>,
    ) -> Self {
        self.err.field_errors = Some(field_errors);
        self
    }

    pub fn with_context(mut self, context: serde_json::Value) -> Self {
        self.context = context;
        self
    }

    pub fn with_cause<E: std::fmt::Display>(mut self, cause: &E) -> Self {
        let cause_value = serde_json::Value::String(cause.to_string());
        match &mut self.context {
            serde_json::Value::Object(map) => {
                map.insert("cause".to_string(), cause_value);
            }
            _ => {
                self.context = serde_json::json!({
                    "cause": cause_value,
                    "context": self.context
                });
            }
        }
        self
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let message = self.err.message.clone();
        let mut res = error_response(self.status, self.err);

        res.extensions_mut().insert(ErrorDetail {
            message,
            context: self.context,
        });

        res
    }
}

pub fn error_response(status: StatusCode, err: AppError) -> Response {
    let envelope: ErrorEnvelope = err.clone().into_envelope();

    // Useful to surface codes to non-JSON-aware layers.
    let mut res = (status, Json(envelope)).into_response();
    res.headers_mut().insert(
        axum::http::HeaderName::from_static(ERROR_CODE_HEADER),
        axum::http::HeaderValue::from_str(err.code)
            .unwrap_or_else(|_| axum::http::HeaderValue::from_static("invalid")),
    );

    res
}
