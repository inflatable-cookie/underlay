use super::*;
use axum::body::to_bytes;

#[test]
fn test_validation_error_response() {
    let mut error = ValidationError::with_message("Validation failed");
    error.add_field("email", "Invalid email");

    let rejection = ValidatedJsonRejection::ValidationError(error);
    let response = rejection.into_response();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = tokio::runtime::Runtime::new()
        .expect("runtime")
        .block_on(async move {
            to_bytes(response.into_body(), usize::MAX)
                .await
                .expect("body")
        });
    let json: serde_json::Value = serde_json::from_slice(&body).expect("response should be json");
    let error = json["error"].as_object().expect("error object");

    assert_eq!(
        error
            .get("fieldErrors")
            .and_then(|value| value["email"].as_str()),
        Some("Invalid email")
    );
    assert!(error.get("field_errors").is_none());
}
