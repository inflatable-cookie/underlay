use super::*;
use axum::{body::to_bytes, extract::FromRequest};

fn response_json(response: Response) -> serde_json::Value {
    let body = tokio::runtime::Runtime::new()
        .expect("runtime")
        .block_on(async move {
            to_bytes(response.into_body(), usize::MAX)
                .await
                .expect("body")
        });

    serde_json::from_slice(&body).expect("response should be json")
}

#[test]
fn test_validation_error_response() {
    let mut error = ValidationError::with_message("Validation failed");
    error.add_field("email", "Invalid email");

    let rejection = ValidatedJsonRejection::ValidationError(error);
    let response = rejection.into_response();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let json = response_json(response);
    let error = json["error"].as_object().expect("error object");

    assert_eq!(
        error
            .get("fieldErrors")
            .and_then(|value| value["email"].as_str()),
        Some("Invalid email")
    );
    assert!(error.get("field_errors").is_none());
}

#[test]
fn json_rejection_uses_canonical_error_envelope() {
    let request = Request::builder()
        .header("content-type", "application/json")
        .body(axum::body::Body::from("{"))
        .expect("request");

    let rejection = tokio::runtime::Runtime::new()
        .expect("runtime")
        .block_on(async move { Json::<serde_json::Value>::from_request(request, &()).await })
        .expect_err("malformed json should reject");

    let response = ValidatedJsonRejection::JsonRejection(rejection).into_response();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let json = response_json(response);
    let error = json["error"].as_object().expect("error object");

    assert_eq!(
        error.get("code").and_then(serde_json::Value::as_str),
        Some("json.invalid")
    );
    assert!(error
        .get("message")
        .and_then(serde_json::Value::as_str)
        .is_some());
    assert!(error.get("fieldErrors").is_none());
    assert!(error.get("field_errors").is_none());
}

#[test]
fn validation_error_without_fields_omits_field_errors() {
    let rejection =
        ValidatedJsonRejection::ValidationError(ValidationError::with_message("Validation failed"));
    let response = rejection.into_response();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let json = response_json(response);
    let error = json["error"].as_object().expect("error object");

    assert_eq!(
        error.get("code").and_then(serde_json::Value::as_str),
        Some("validation.failed")
    );
    assert!(error.get("fieldErrors").is_none());
    assert!(error.get("field_errors").is_none());
}
