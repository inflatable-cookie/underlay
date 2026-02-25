//! Nightfire content validation integration.
//!
//! Converts Nightfire validation errors to `AppError` responses
//! with field-level error details.
//!
//! # Example
//!
//! ```rust,ignore
//! use underlay_validation::nightfire_validation_to_app_error;
//!
//! if let Err(validation_err) = validate_nightfire_value_by_schema(&body) {
//!     let err = nightfire_validation_to_app_error(
//!         validation_err,
//!         "content.invalid",
//!         "body",
//!         "Content body failed schema validation.",
//!     );
//!     return Err(err);
//! }
//! ```

use std::collections::HashMap;

use underlay_core::AppError;
use underlay_nightfire::NightfireValidationError;

/// Convert a Nightfire validation error to an AppError with field errors.
///
/// Provides consistent error formatting for Nightfire content validation
/// failures across all HTTP handlers.
///
/// # Arguments
///
/// * `validation_err` - The validation error from Nightfire
/// * `error_code` - Error code for the AppError (e.g., "content.summary_invalid")
/// * `field_name` - Field name for field_errors (e.g., "body")
/// * `message` - Human-readable message for the AppError
pub fn nightfire_validation_to_app_error(
    validation_err: NightfireValidationError,
    error_code: &'static str,
    field_name: &str,
    message: impl Into<String>,
) -> AppError {
    let mut field_errors = HashMap::new();

    let detail = match &validation_err {
        NightfireValidationError::CardinalityMismatch {
            expected,
            actual_blocks,
            ..
        } => {
            format!("Expected {:?} block(s), got {}.", expected, actual_blocks)
        }
        NightfireValidationError::DisallowedBlockType { block_type, .. } => {
            let msg = format!("Block type \"{}\" is not allowed.", block_type);
            field_errors.insert(field_name.to_string(), msg.clone());
            msg
        }
        NightfireValidationError::UnknownBlockType { block_type, .. } => {
            let msg = format!("Unknown block type \"{}\".", block_type);
            field_errors.insert(field_name.to_string(), msg.clone());
            msg
        }
        NightfireValidationError::UnknownStrategy { schema } => {
            format!("Unknown schema identifier \"{}\".", schema)
        }
    };

    // Ensure field_errors always has an entry for the field
    if !field_errors.contains_key(field_name) {
        field_errors.insert(field_name.to_string(), detail);
    }

    AppError {
        code: error_code,
        message: message.into(),
        field_errors: Some(field_errors),
    }
}

#[cfg(test)]
#[path = "tests/nightfire_compat_tests.rs"]
mod tests;
