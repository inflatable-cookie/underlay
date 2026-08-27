use super::*;
use axum::{
    body::to_bytes,
    http::{header, StatusCode},
    response::Response,
};
use serde_json::{json, Value};

mod extractor;
mod model;
mod proxy_resolution;

async fn assert_context_error_response(
    response: Response,
    expected_status: StatusCode,
    expected_code: &str,
    expected_message: &str,
) {
    assert_eq!(response.status(), expected_status);
    assert_eq!(
        response.headers().get(header::CONTENT_TYPE).unwrap(),
        "application/json"
    );
    assert_eq!(
        response.headers().get("x-error-code").unwrap(),
        expected_code
    );

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let envelope: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        envelope,
        json!({
            "error": {
                "code": expected_code,
                "message": expected_message
            }
        })
    );
}
