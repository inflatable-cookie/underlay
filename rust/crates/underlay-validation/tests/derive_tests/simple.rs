use underlay_validation::Validate;

#[derive(Validate)]
struct SimpleValidators {
    #[validate(url)]
    website: String,

    #[validate(uuid)]
    id: String,

    #[validate(username)]
    handle: String,

    #[validate(slug)]
    path: String,
}

#[test]
fn test_simple_validators_valid() {
    let req = SimpleValidators {
        website: "https://example.com".to_string(),
        id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        handle: "john_doe".to_string(),
        path: "my-article".to_string(),
    };

    assert!(req.validate().is_ok());
}

#[test]
fn test_simple_validators_invalid() {
    let req = SimpleValidators {
        website: "not-a-url".to_string(),
        id: "not-a-uuid".to_string(),
        handle: "john@doe".to_string(),
        path: "My Article".to_string(),
    };

    let err = req.validate().unwrap_err();
    assert!(err.has_field("website"));
    assert!(err.has_field("id"));
    assert!(err.has_field("handle"));
    assert!(err.has_field("path"));
}

#[derive(Validate)]
struct NumericValidators {
    #[validate(positive)]
    count: i32,

    #[validate(non_negative)]
    balance: i32,
}

#[test]
fn test_numeric_validators_valid() {
    let req = NumericValidators {
        count: 5,
        balance: 0,
    };

    assert!(req.validate().is_ok());
}

#[test]
fn test_numeric_validators_invalid() {
    let req = NumericValidators {
        count: 0,
        balance: -1,
    };

    let err = req.validate().unwrap_err();
    assert!(err.has_field("count"));
    assert!(err.has_field("balance"));
}

#[derive(Validate)]
struct AlphanumericTest {
    #[validate(alphanumeric)]
    code: String,
}

#[test]
fn test_alphanumeric_valid() {
    let req = AlphanumericTest {
        code: "ABC123".to_string(),
    };

    assert!(req.validate().is_ok());
}

#[test]
fn test_alphanumeric_invalid() {
    let req = AlphanumericTest {
        code: "ABC-123".to_string(),
    };

    let err = req.validate().unwrap_err();
    assert!(err.has_field("code"));
}
