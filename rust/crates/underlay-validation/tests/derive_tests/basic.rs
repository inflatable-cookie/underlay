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
}
