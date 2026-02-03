//! Nightfire content validation integration for HTTP handlers.
//!
//! Provides helpers to convert Nightfire validation errors to HTTP error responses.
//!
//! # Example
//!
//! ```rust,ignore
//! use underlay_http::{nightfire_validation_to_app_error, error_response};
//! use nightfire::{validate_nightfire_value_by_schema, NightfireValue};
//! use axum::http::StatusCode;
//!
//! async fn create_content(body: NightfireValue) -> impl IntoResponse {
//!     if let Err(validation_err) = validate_nightfire_value_by_schema(&body) {
//!         let err = nightfire_validation_to_app_error(
//!             validation_err,
//!             "content.invalid",
//!             "body",
//!             "Content body failed schema validation.",
//!         );
//!         return error_response(StatusCode::BAD_REQUEST, err).into_response();
//!     }
//!     // ... rest of handler
//! }
//! ```

use std::collections::HashMap;

use underlay_core::AppError;
use underlay_nightfire::NightfireValidationError;

/// Convert a Nightfire validation error to an AppError with field errors.
///
/// This provides consistent error formatting for Nightfire content validation
/// failures across all HTTP handlers.
///
/// # Arguments
///
/// * `validation_err` - The validation error from Nightfire
/// * `error_code` - Error code for the AppError (e.g., "content.summary_invalid")
/// * `field_name` - Field name for field_errors (e.g., "body")
/// * `message` - Human-readable message for the AppError
///
/// # Example
///
/// ```rust,ignore
/// if let Err(validation_err) = validate_nightfire_value_by_schema(&nf_body) {
///     let err = nightfire_validation_to_app_error(
///         validation_err,
///         "content.summary_invalid",
///         "body",
///         "Summary body failed schema validation.",
///     );
///     return error_response(StatusCode::BAD_REQUEST, err).into_response();
/// }
/// ```
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
mod tests {
    use super::*;
    use underlay_nightfire::StrategyCardinality;

    #[test]
    fn converts_cardinality_mismatch() {
        let err = NightfireValidationError::CardinalityMismatch {
            schema: "test:schema@1".to_string(),
            expected: StrategyCardinality::Single,
            actual_blocks: 3,
            is_single: false,
        };

        let app_err = nightfire_validation_to_app_error(
            err,
            "content.invalid",
            "body",
            "Content validation failed.",
        );

        assert_eq!(app_err.code, "content.invalid");
        assert_eq!(app_err.message, "Content validation failed.");
        assert!(app_err.field_errors.is_some());
        let field_errors = app_err.field_errors.unwrap();
        assert!(field_errors.contains_key("body"));
    }

    #[test]
    fn converts_disallowed_block_type() {
        let err = NightfireValidationError::DisallowedBlockType {
            schema: "test:schema@1".to_string(),
            block_type: "forbidden.block".to_string(),
        };

        let app_err = nightfire_validation_to_app_error(
            err,
            "content.invalid",
            "body",
            "Content validation failed.",
        );

        let field_errors = app_err.field_errors.unwrap();
        assert!(field_errors
            .get("body")
            .unwrap()
            .contains("forbidden.block"));
    }

    #[test]
    fn converts_unknown_block_type() {
        let err = NightfireValidationError::UnknownBlockType {
            schema: "test:schema@1".to_string(),
            block_type: "mystery.block".to_string(),
        };

        let app_err = nightfire_validation_to_app_error(
            err,
            "content.invalid",
            "body",
            "Content validation failed.",
        );

        let field_errors = app_err.field_errors.unwrap();
        assert!(field_errors.get("body").unwrap().contains("mystery.block"));
    }

    #[test]
    fn converts_unknown_strategy() {
        let err = NightfireValidationError::UnknownStrategy {
            schema: "unknown:schema@1".to_string(),
        };

        let app_err = nightfire_validation_to_app_error(
            err,
            "content.invalid",
            "body",
            "Content validation failed.",
        );

        let field_errors = app_err.field_errors.unwrap();
        assert!(field_errors
            .get("body")
            .unwrap()
            .contains("unknown:schema@1"));
    }
}
