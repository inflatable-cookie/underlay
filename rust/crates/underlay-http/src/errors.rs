use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use underlay_core::{AppError, ErrorEnvelope};
use underlay_observability::RequestId;

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

pub fn error_response(status: StatusCode, err: AppError) -> Response {
    let envelope: ErrorEnvelope = err.clone().into_envelope();

    // Useful to surface codes to non-JSON-aware layers.
    let mut res = (status, Json(envelope)).into_response();
    res.headers_mut().insert(
        axum::http::HeaderName::from_static("x-error-code"),
        axum::http::HeaderValue::from_str(err.code)
            .unwrap_or_else(|_| axum::http::HeaderValue::from_static("invalid")),
    );

    res
}
