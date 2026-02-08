//! Declarative validation for Underlay.
//!
//! This crate provides a `Validate` trait and common validators for request validation.
//! It integrates with Axum to return proper HTTP error responses with field-level errors.
//!
//! # Example (Manual Implementation)
//!
//! ```rust
//! use underlay_validation::{Validate, ValidationError, validators};
//!
//! struct CreateUserRequest {
//!     email: String,
//!     password: String,
//!     age: i32,
//! }
//!
//! impl Validate for CreateUserRequest {
//!     fn validate(&self) -> Result<(), ValidationError> {
//!         let mut errors = ValidationError::new();
//!
//!         if let Err(e) = validators::email(&self.email) {
//!             errors.add_field("email", e);
//!         }
//!
//!         if let Err(e) = validators::length(&self.password, Some(8), Some(100)) {
//!             errors.add_field("password", e);
//!         }
//!
//!         if let Err(e) = validators::range(self.age, Some(18), Some(120)) {
//!             errors.add_field("age", e);
//!         }
//!
//!         errors.into_result()
//!     }
//! }
//! ```
//!
//! # Example (Derive Macro)
//!
//! With the `derive` feature (enabled by default):
//!
//! ```rust,ignore
//! use underlay_validation::Validate;
//!
//! #[derive(Validate)]
//! struct CreateUserRequest {
//!     #[validate(email)]
//!     email: String,
//!
//!     #[validate(length(min = 8, max = 100))]
//!     password: String,
//!
//!     #[validate(range(min = 18, max = 120))]
//!     age: i32,
//! }
//! ```

mod error;
pub mod validators;

pub use error::{FieldError, ValidationError, ValidationResult};

/// Trait for types that can be validated.
///
/// Implement this trait to add validation to your request types.
/// The `validate` method should check all fields and return a
/// `ValidationError` containing all field errors.
///
/// You can implement this trait manually or use the `#[derive(Validate)]` macro.
pub trait Validate {
    /// Validate the struct and return any errors.
    ///
    /// Returns `Ok(())` if validation passes, or `Err(ValidationError)`
    /// containing all field errors if validation fails.
    fn validate(&self) -> ValidationResult<()>;

    /// Validate and return self if valid.
    ///
    /// This is a convenience method for chaining.
    fn validated(self) -> ValidationResult<Self>
    where
        Self: Sized,
    {
        self.validate()?;
        Ok(self)
    }
}

// Re-export the derive macro when the feature is enabled.
// This allows `#[derive(Validate)]` to work after importing `use underlay_validation::Validate;`.
#[cfg(feature = "derive")]
pub use underlay_validation_derive::Validate;

/// Extension trait for validating Options.
pub trait ValidateOption<T> {
    /// Validate the inner value if Some, skip if None.
    fn validate_if_some(&self) -> ValidationResult<()>;
}

impl<T: Validate> ValidateOption<T> for Option<T> {
    fn validate_if_some(&self) -> ValidationResult<()> {
        match self {
            Some(value) => value.validate(),
            None => Ok(()),
        }
    }
}

#[cfg(feature = "axum")]
mod axum_integration;

#[cfg(feature = "axum")]
pub use axum_integration::ValidatedJson;

#[cfg(feature = "validator-compat")]
mod validator_compat;

#[cfg(feature = "validator-compat")]
pub use validator_compat::validation_to_app_error;

#[cfg(feature = "nightfire")]
mod nightfire_compat;

#[cfg(feature = "nightfire")]
pub use nightfire_compat::nightfire_validation_to_app_error;

#[cfg(feature = "field-validation")]
pub mod field_validation;

#[cfg(feature = "field-validation")]
pub use field_validation::{
    parse_optional_uuid_for_validation, parse_uuid_for_validation, FieldValidationResult,
};

pub mod slugify;

pub use slugify::{
    is_reserved_slug, is_valid_slug_format, validate_slug, SlugValidationError, RESERVED_SLUGS,
};

#[cfg(feature = "slugify")]
pub use slugify::slugify;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestRequest {
        email: String,
        password: String,
        age: i32,
    }

    impl Validate for TestRequest {
        fn validate(&self) -> ValidationResult<()> {
            let mut errors = ValidationError::new();

            if let Err(e) = validators::email(&self.email) {
                errors.add_field("email", e);
            }

            if let Err(e) = validators::length(&self.password, Some(8), Some(100)) {
                errors.add_field("password", e);
            }

            if let Err(e) = validators::range(self.age, Some(18), Some(120)) {
                errors.add_field("age", e);
            }

            errors.into_result()
        }
    }

    #[test]
    fn test_valid_request() {
        let req = TestRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            age: 25,
        };

        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let req = TestRequest {
            email: "not-an-email".to_string(),
            password: "password123".to_string(),
            age: 25,
        };

        let err = req.validate().unwrap_err();
        assert!(err.has_field("email"));
        assert!(!err.has_field("password"));
    }

    #[test]
    fn test_multiple_errors() {
        let req = TestRequest {
            email: "bad".to_string(),
            password: "short".to_string(),
            age: 10,
        };

        let err = req.validate().unwrap_err();
        assert!(err.has_field("email"));
        assert!(err.has_field("password"));
        assert!(err.has_field("age"));
        assert_eq!(err.field_count(), 3);
    }

    #[test]
    fn test_validated_method() {
        let req = TestRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            age: 25,
        };

        let result = req.validated();
        assert!(result.is_ok());
    }
}
