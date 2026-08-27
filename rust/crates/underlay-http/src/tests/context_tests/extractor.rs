use super::*;
use axum::{
    extract::FromRequestParts,
    http::{HeaderValue, Request, StatusCode},
    response::IntoResponse,
};

#[tokio::test]
async fn unauthenticated_context_extractor_returns_canonical_error_envelope() {
    let request = Request::new(());
    let (mut parts, _) = request.into_parts();
    let error = AuthenticatedContext::from_request_parts(&mut parts, &())
        .await
        .expect_err("request without a user ID should be rejected");

    assert_context_error_response(
        error.into_response(),
        StatusCode::UNAUTHORIZED,
        "auth.unauthorized",
        "Authentication required",
    )
    .await;
}

#[tokio::test]
async fn missing_context_error_returns_canonical_error_envelope() {
    assert_context_error_response(
        ContextError::MissingField("request context").into_response(),
        StatusCode::BAD_REQUEST,
        "request.context_missing",
        "Missing required context",
    )
    .await;
}

#[test]
fn test_extract_request_id_from_header() {
    let mut headers = HeaderMap::new();
    headers.insert(
        headers::X_REQUEST_ID,
        HeaderValue::from_static("test-request-id"),
    );

    let request_id = extract_request_id(&headers);
    assert_eq!(request_id, "test-request-id");
}

#[test]
fn test_extract_request_id_generates_uuid() {
    let headers = HeaderMap::new();
    let request_id = extract_request_id(&headers);

    // Should be a valid UUID
    assert!(Uuid::parse_str(&request_id).is_ok());
}
