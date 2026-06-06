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
