use underlay_validation::Validate;

#[derive(Validate)]
struct DerivedRequest {
    #[validate(email)]
    email: String,

    #[validate(length(min = 8, max = 100))]
    password: String,

    #[validate(range(min = 18, max = 120))]
    age: i32,

    #[validate(required)]
    name: String,
}

#[test]
fn test_derived_valid() {
    let req = DerivedRequest {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        age: 25,
        name: "John".to_string(),
    };

    assert!(req.validate().is_ok());
}

#[test]
fn test_derived_invalid_email() {
    let req = DerivedRequest {
        email: "not-an-email".to_string(),
        password: "password123".to_string(),
        age: 25,
        name: "John".to_string(),
    };

    let err = req.validate().unwrap_err();
    assert!(err.has_field("email"));
    assert!(!err.has_field("password"));
}

#[test]
fn test_derived_multiple_errors() {
    let req = DerivedRequest {
        email: "bad".to_string(),
        password: "short".to_string(),
        age: 10,
        name: "".to_string(),
    };

    let err = req.validate().unwrap_err();
    assert!(err.has_field("email"));
    assert!(err.has_field("password"));
    assert!(err.has_field("age"));
    assert!(err.has_field("name"));
    assert_eq!(err.field_count(), 4);
}

#[derive(Validate)]
struct OptionalDerivedRequest {
    #[validate(email, length(min = 5, max = 100))]
    email: Option<String>,

    #[validate(range(min = 1, max = 10))]
    rating: Option<i32>,
}

#[test]
fn test_derived_option_skips_none_and_validates_some() {
    let none_req = OptionalDerivedRequest {
        email: None,
        rating: None,
    };
    assert!(none_req.validate().is_ok());

    let valid_req = OptionalDerivedRequest {
        email: Some("user@example.com".to_string()),
        rating: Some(5),
    };
    assert!(valid_req.validate().is_ok());

    let invalid_req = OptionalDerivedRequest {
        email: Some("bad".to_string()),
        rating: Some(99),
    };
    let err = invalid_req.validate().unwrap_err();
    assert!(err.has_field("email"));
    assert!(err.has_field("rating"));
    assert_eq!(err.field_count(), 2);

    let email_msg = &err.get_field("email").unwrap().message;
    assert!(email_msg.contains("Invalid email address"));
    assert!(email_msg.contains("Must be at least 5 characters"));
}

#[derive(Validate)]
struct RequiredOptionalRequest {
    #[validate(required)]
    name: Option<String>,

    #[validate(required, email)]
    email: Option<String>,
}

#[test]
fn test_derived_required_on_option_rejects_none_and_blank() {
    let missing = RequiredOptionalRequest {
        name: None,
        email: None,
    };
    let err = missing.validate().unwrap_err();
    assert_eq!(err.field_count(), 2);
    assert_eq!(
        err.get_field("name").and_then(|e| e.code.as_deref()),
        Some("required")
    );
    assert_eq!(
        err.get_field("email").and_then(|e| e.code.as_deref()),
        Some("required")
    );

    let blank = RequiredOptionalRequest {
        name: Some("   ".to_string()),
        email: Some("".to_string()),
    };
    let err = blank.validate().unwrap_err();
    assert_eq!(err.field_count(), 2);
    assert_eq!(
        err.get_field("name").and_then(|e| e.code.as_deref()),
        Some("required")
    );
    assert!(err
        .get_field("email")
        .unwrap()
        .message
        .contains("This field is required"));
    assert!(err
        .get_field("email")
        .unwrap()
        .message
        .contains("Invalid email address"));

    let valid = RequiredOptionalRequest {
        name: Some("Alice".to_string()),
        email: Some("alice@example.com".to_string()),
    };
    assert!(valid.validate().is_ok());
}
