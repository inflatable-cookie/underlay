use crate::{AppError, ErrorBody, ErrorEnvelope, ListResponse, RawUuid, SingleResponse, Uuid};

#[test]
fn uuid_round_trips_from_string() {
    let id = Uuid::new_v7();
    let raw = id.to_string();

    let parsed = Uuid::parse_str(&raw).expect("uuid should parse");
    assert_eq!(parsed, id);
    assert_eq!(parsed.into_inner(), id.into_inner());

    let from_str_parsed: Uuid = raw.parse().expect("FromStr should parse");
    assert_eq!(from_str_parsed, id);

    let raw_uuid: RawUuid = id.into();
    assert_eq!(Uuid::from(raw_uuid), id);
    assert_eq!(id.as_ref(), &raw_uuid);

    let low = Uuid::from(RawUuid::from_u128(1));
    let high = Uuid::from(RawUuid::from_u128(2));
    assert!(low < high);
}

#[test]
fn core_dtos_support_equality() {
    assert_eq!(SingleResponse { data: "ok" }, SingleResponse { data: "ok" });
    assert_eq!(
        ListResponse { data: vec![1, 2] },
        ListResponse { data: vec![1, 2] }
    );
    assert_eq!(
        ErrorEnvelope {
            error: ErrorBody {
                code: "c".to_string(),
                message: "m".to_string(),
                field_errors: None,
            },
        },
        ErrorEnvelope {
            error: ErrorBody {
                code: "c".to_string(),
                message: "m".to_string(),
                field_errors: None,
            },
        }
    );
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

#[test]
fn error_envelope_serializes_field_errors_as_field_errors_camel_case_on_wire() {
    let err = AppError::with_field_error(
        "test.invalid",
        "Validation failed",
        "email",
        "Must be a valid email",
    );

    let json = serde_json::to_value(err.into_envelope()).expect("envelope should serialize");
    let error = json
        .get("error")
        .and_then(|value| value.as_object())
        .expect("error object should exist");

    assert!(error.get("fieldErrors").is_some());
    assert!(error.get("field_errors").is_none());
}
