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
/// suitable for returning as an API response. The transport envelope omits
/// `fieldErrors` entirely when no field-scoped feedback exists, so an empty
/// map is normalised to `None`.
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
    let field_errors = validation_err
        .field_errors()
        .iter()
        .filter_map(|(field, errors)| {
            let err = errors.first()?;
            let msg = err.message.as_deref().unwrap_or("Invalid value");
            Some((field.to_string(), msg.to_string()))
        })
        .collect::<HashMap<_, _>>();

    AppError {
        code: error_code,
        message: message.to_string(),
        field_errors: (!field_errors.is_empty()).then_some(field_errors),
    }
}

#[cfg(test)]
#[path = "tests/validator_compat_tests.rs"]
mod tests;
