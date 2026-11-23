use underlay_validation::Validate;
use uuid::Uuid;

#[derive(Validate)]
struct RequiredString {
    #[validate(required)]
    name: String,
}

#[derive(Validate)]
struct RequiredOptionString {
    #[validate(required)]
    name: Option<String>,
}

#[derive(Validate)]
struct RequiredOptionI32 {
    #[validate(required)]
    count: Option<i32>,
}

#[derive(Validate)]
struct RequiredOptionUuid {
    #[validate(required)]
    id: Option<Uuid>,
}

#[test]
fn required_string_rejects_blank_and_accepts_text() {
    assert!(RequiredString {
        name: "Ada".to_string()
    }
    .validate()
    .is_ok());

    let err = RequiredString {
        name: "   ".to_string(),
    }
    .validate()
    .unwrap_err();
    assert!(err.has_field("name"));
}

#[test]
fn required_option_string_rejects_none_and_blank() {
    assert!(RequiredOptionString {
        name: Some("Ada".to_string())
    }
    .validate()
    .is_ok());

    let missing = RequiredOptionString { name: None }.validate().unwrap_err();
    assert!(missing.has_field("name"));

    let blank = RequiredOptionString {
        name: Some("  ".to_string()),
    }
    .validate()
    .unwrap_err();
    assert!(blank.has_field("name"));
}

#[test]
fn required_option_i32_checks_presence_only() {
    assert!(RequiredOptionI32 { count: Some(0) }.validate().is_ok());
    assert!(RequiredOptionI32 { count: Some(-7) }.validate().is_ok());

    let err = RequiredOptionI32 { count: None }.validate().unwrap_err();
    assert!(err.has_field("count"));
}

#[test]
fn required_option_uuid_checks_presence_only() {
    assert!(RequiredOptionUuid {
        id: Some(Uuid::nil())
    }
    .validate()
    .is_ok());

    let err = RequiredOptionUuid { id: None }.validate().unwrap_err();
    assert!(err.has_field("id"));
}
