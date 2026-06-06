use underlay_validation::{FieldError, Validate};

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
