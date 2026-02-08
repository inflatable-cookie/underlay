//! Compatibility helpers for the `validator` crate.
//!
//! Converts `validator::ValidationErrors` into `AppError` responses
//! with field-level error details.
//!
//! # Example
//!
//! ```rust,ignore
//! use validator::Validate;
//! use underlay_validation::validation_to_app_error;
//!
//! if let Err(validation_err) = payload.validate() {
//!     let err = validation_to_app_error(
//!         &validation_err,
//!         "learning.pathway_invalid",
//!         "There is a problem with one or more fields.",
//!     );
//!     return Err(err);
//! }
//! ```

use std::collections::HashMap;

use underlay_core::AppError;
use validator::ValidationErrors;

/// Convert validator errors to an AppError with field errors.
///
/// Extracts the first error message for each field and creates an `AppError`
/// suitable for returning as an API response.
///
/// # Arguments
/// * `validation_err` - The ValidationErrors from the validator crate
/// * `error_code` - Error code for the AppError (e.g., "learning.pathway_invalid")
/// * `message` - Human-readable message (e.g., "There is a problem with one or more fields.")
pub fn validation_to_app_error(
    validation_err: &ValidationErrors,
    error_code: &'static str,
    message: &str,
) -> AppError {
    let mut field_errors = HashMap::new();

    for (field, errors) in validation_err.field_errors() {
        if let Some(err) = errors.first() {
            let msg = err
                .message
                .clone()
                .unwrap_or_else(|| "Invalid value".into());
            field_errors.insert(field.to_string(), msg.to_string());
        }
    }

    AppError {
        code: error_code,
        message: message.to_string(),
        field_errors: Some(field_errors),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_to_app_error_creates_correct_structure() {
        let mut errors = validator::ValidationErrors::new();
        let mut field_error = validator::ValidationError::new("required");
        field_error.message = Some("Name is required".into());
        errors.add("name", field_error);

        let app_error =
            validation_to_app_error(&errors, "test.validation_failed", "Validation failed");

        assert_eq!(app_error.code, "test.validation_failed");
        assert_eq!(app_error.message, "Validation failed");
        assert!(app_error.field_errors.is_some());

        let field_errors = app_error.field_errors.unwrap();
        assert_eq!(
            field_errors.get("name"),
            Some(&"Name is required".to_string())
        );
    }

    #[test]
    fn test_validation_to_app_error_default_message() {
        let mut errors = validator::ValidationErrors::new();
        let field_error = validator::ValidationError::new("length");
        errors.add("email", field_error);

        let app_error =
            validation_to_app_error(&errors, "test.validation_failed", "Validation failed");

        let field_errors = app_error.field_errors.unwrap();
        assert_eq!(
            field_errors.get("email"),
            Some(&"Invalid value".to_string())
        );
    }

    #[test]
    fn test_validation_to_app_error_multiple_fields() {
        let mut errors = validator::ValidationErrors::new();

        let mut name_error = validator::ValidationError::new("required");
        name_error.message = Some("Name is required".into());
        errors.add("name", name_error);

        let mut email_error = validator::ValidationError::new("email");
        email_error.message = Some("Invalid email format".into());
        errors.add("email", email_error);

        let app_error = validation_to_app_error(
            &errors,
            "user.invalid",
            "There is a problem with one or more fields.",
        );

        let field_errors = app_error.field_errors.unwrap();
        assert_eq!(field_errors.len(), 2);
        assert_eq!(
            field_errors.get("name"),
            Some(&"Name is required".to_string())
        );
        assert_eq!(
            field_errors.get("email"),
            Some(&"Invalid email format".to_string())
        );
    }
}
