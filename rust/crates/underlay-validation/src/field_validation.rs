//! Live field validation helpers.
//!
//! Provides types and helpers for real-time field validation endpoints that
//! return validation feedback (valid/invalid + message) rather than HTTP errors.
//!
//! This is distinct from HTTP error responses - validation endpoints always
//! return 200 OK with a validation result, even for invalid input.
//!
//! # Example
//!
//! ```rust,ignore
//! use underlay_validation::{FieldValidationResult, parse_uuid_for_validation};
//! use axum::{Json, response::IntoResponse};
//!
//! async fn validate_field(payload: Json<ValidatePayload>) -> impl IntoResponse {
//!     let module_id = match parse_uuid_for_validation(&payload.module_id, "moduleId") {
//!         Ok(id) => id,
//!         Err(result) => return Json(result),
//!     };
//!
//!     if slug_exists(&module_id, &payload.slug).await {
//!         return Json(FieldValidationResult::invalid("Slug already exists"));
//!     }
//!
//!     Json(FieldValidationResult::valid())
//! }
//! ```

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Result of a field validation check.
///
/// Used by live validation endpoints to provide feedback to the UI
/// without returning HTTP errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValidationResult {
    /// Whether the field value is valid.
    pub valid: bool,

    /// Optional message explaining the validation result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Optional suggested alternative value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

impl FieldValidationResult {
    /// Create a successful validation result.
    pub fn valid() -> Self {
        Self {
            valid: true,
            message: None,
            suggestion: None,
        }
    }

    /// Create a failed validation result with a message.
    pub fn invalid(message: impl Into<String>) -> Self {
        Self {
            valid: false,
            message: Some(message.into()),
            suggestion: None,
        }
    }

    /// Create a failed validation result with a message and suggestion.
    pub fn invalid_with_suggestion(
        message: impl Into<String>,
        suggestion: impl Into<String>,
    ) -> Self {
        Self {
            valid: false,
            message: Some(message.into()),
            suggestion: Some(suggestion.into()),
        }
    }

    /// Add a suggestion to an existing result.
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

/// Parse a UUID string for validation purposes.
///
/// Unlike HTTP path parsers which return HTTP errors, this returns a
/// `FieldValidationResult` for use in live validation endpoints.
pub fn parse_uuid_for_validation(
    value: &str,
    field_name: &str,
) -> Result<Uuid, FieldValidationResult> {
    Uuid::parse_str(value)
        .map_err(|_| FieldValidationResult::invalid(format!("Invalid {}", field_name)))
}

/// Parse an optional UUID string for validation purposes.
///
/// Returns `Ok(None)` if the value is `None`, `Ok(Some(uuid))` if valid,
/// or `Err(FieldValidationResult)` if invalid.
pub fn parse_optional_uuid_for_validation(
    value: Option<&str>,
    field_name: &str,
) -> Result<Option<Uuid>, FieldValidationResult> {
    match value {
        None => Ok(None),
        Some(v) => parse_uuid_for_validation(v, field_name).map(Some),
    }
}

#[cfg(test)]
#[path = "tests/field_validation_tests.rs"]
mod tests;
