//! Integration tests for the derive macro.
//!
//! These tests are in an integration test file (rather than in lib.rs) because
//! the derive macro generates code with `::underlay_validation::` paths, which
//! only resolve correctly when importing the crate externally.

use underlay_validation::{FieldError, Validate};

// Test basic derive with multiple validators
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

// Test simple validators
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

// Test numeric validators
#[derive(Validate)]
struct NumericValidators {
    #[validate(positive)]
    count: i32,

    #[validate(non_negative)]
    balance: i32,
}

#[test]
fn test_numeric_validators_valid() {
    let req = NumericValidators { count: 5, balance: 0 };

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

// Test skip validator
#[derive(Validate)]
struct SkipTest {
    #[validate(email)]
    email: String,

    #[validate(skip)]
    #[allow(dead_code)]
    not_validated: String,
}

#[test]
fn test_skip_validator() {
    let req = SkipTest {
        email: "test@example.com".to_string(),
        not_validated: "anything goes here".to_string(),
    };

    assert!(req.validate().is_ok());
}

// Test custom validator
fn validate_starts_with_a(value: &str) -> Result<(), FieldError> {
    if value.starts_with('a') || value.starts_with('A') {
        Ok(())
    } else {
        Err(FieldError::new("Must start with 'a'"))
    }
}

#[derive(Validate)]
struct CustomValidatorTest {
    #[validate(custom = "validate_starts_with_a")]
    name: String,
}

#[test]
fn test_custom_validator_valid() {
    let req = CustomValidatorTest {
        name: "Alice".to_string(),
    };

    assert!(req.validate().is_ok());
}

#[test]
fn test_custom_validator_invalid() {
    let req = CustomValidatorTest {
        name: "Bob".to_string(),
    };

    let err = req.validate().unwrap_err();
    assert!(err.has_field("name"));
}

// Test pattern validator
#[derive(Validate)]
struct PatternTest {
    #[validate(pattern = r"^\d{3}-\d{4}$")]
    phone: String,
}

#[test]
fn test_pattern_valid() {
    let req = PatternTest {
        phone: "123-4567".to_string(),
    };

    assert!(req.validate().is_ok());
}

#[test]
fn test_pattern_invalid() {
    let req = PatternTest {
        phone: "1234567".to_string(),
    };

    let err = req.validate().unwrap_err();
    assert!(err.has_field("phone"));
}

// Test collection validators
#[derive(Validate)]
struct CollectionTest {
    #[validate(not_empty)]
    items: Vec<String>,

    #[validate(collection_length(min = 2, max = 5))]
    tags: Vec<String>,
}

#[test]
fn test_collection_validators_valid() {
    let req = CollectionTest {
        items: vec!["a".to_string()],
        tags: vec!["tag1".to_string(), "tag2".to_string()],
    };

    assert!(req.validate().is_ok());
}

#[test]
fn test_collection_validators_invalid() {
    let req = CollectionTest {
        items: vec![],
        tags: vec!["tag1".to_string()],
    };

    let err = req.validate().unwrap_err();
    assert!(err.has_field("items"));
    assert!(err.has_field("tags"));
}

// Test nested validation
#[derive(Validate)]
struct Address {
    #[validate(required)]
    city: String,
}

#[derive(Validate)]
struct NestedTest {
    #[validate(email)]
    email: String,

    #[validate(nested)]
    address: Address,
}

#[test]
fn test_nested_valid() {
    let req = NestedTest {
        email: "test@example.com".to_string(),
        address: Address {
            city: "London".to_string(),
        },
    };

    assert!(req.validate().is_ok());
}

#[test]
fn test_nested_invalid() {
    let req = NestedTest {
        email: "test@example.com".to_string(),
        address: Address {
            city: "".to_string(),
        },
    };

    let err = req.validate().unwrap_err();
    assert!(err.has_field("address.city"));
}

// Test alphanumeric validator
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
