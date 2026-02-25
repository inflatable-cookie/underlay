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
#[path = "tests/validator_compat_tests.rs"]
mod tests;
