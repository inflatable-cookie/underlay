use super::*;

#[test]
fn test_validation_error_response() {
    let mut error = ValidationError::with_message("Validation failed");
    error.add_field("email", "Invalid email");

    let rejection = ValidatedJsonRejection::ValidationError(error);
    let response = rejection.into_response();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
