use crate::{AppError, Uuid};

#[test]
fn uuid_round_trips_from_string() {
    let id = Uuid::new_v7();
    let raw = id.to_string();

    let parsed = Uuid::parse_str(&raw).expect("uuid should parse");
    assert_eq!(parsed, id);
    assert_eq!(parsed.into_inner(), id.into_inner());
}

#[test]
fn app_error_into_envelope_preserves_code_and_message() {
    let err = AppError::new("test.code", "Something went wrong");
    let envelope = err.into_envelope();

    assert_eq!(envelope.error.code, "test.code");
    assert_eq!(envelope.error.message, "Something went wrong");
    assert!(envelope.error.field_errors.is_none());
}

#[test]
fn app_error_field_error_round_trips_into_envelope() {
    let err = AppError::with_field_error(
        "test.invalid",
        "Validation failed",
        "email",
        "Must be a valid email",
    );

    let envelope = err.into_envelope();
    let field_errors = envelope
        .error
        .field_errors
        .expect("field errors should exist");

    assert_eq!(
        field_errors.get("email"),
        Some(&"Must be a valid email".to_string())
    );
}
