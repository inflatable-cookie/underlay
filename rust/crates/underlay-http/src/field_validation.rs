//! Live field validation helpers for HTTP handlers.
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
//! use underlay_http::{ValidationResult, parse_uuid_for_validation};
//! use axum::{Json, response::IntoResponse};
//!
//! async fn validate_field(payload: Json<ValidatePayload>) -> impl IntoResponse {
//!     // Parse UUID with validation result (not HTTP error)
//!     let module_id = match parse_uuid_for_validation(&payload.module_id, "moduleId") {
//!         Ok(id) => id,
//!         Err(result) => return Json(result),
//!     };
//!
//!     // Check business logic
//!     if slug_exists(&module_id, &payload.slug).await {
//!         return Json(ValidationResult::invalid("Slug already exists"));
//!     }
//!
//!     Json(ValidationResult::valid())
//! }
//! ```

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Result of a field validation check.
///
/// Used by live validation endpoints to provide feedback to the UI
/// without returning HTTP errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether the field value is valid.
    pub valid: bool,

    /// Optional message explaining the validation result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Optional suggested alternative value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

impl ValidationResult {
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
/// Unlike `parse_uuid_path` which returns HTTP errors, this returns a
/// `ValidationResult` for use in live validation endpoints.
///
/// # Example
///
/// ```rust,ignore
/// let module_id = match parse_uuid_for_validation(&payload.module_id, "moduleId") {
///     Ok(id) => id,
///     Err(result) => return Json(result),
/// };
/// ```
pub fn parse_uuid_for_validation(value: &str, field_name: &str) -> Result<Uuid, ValidationResult> {
    Uuid::parse_str(value).map_err(|_| ValidationResult::invalid(format!("Invalid {}", field_name)))
}

/// Parse an optional UUID string for validation purposes.
///
/// Returns `Ok(None)` if the value is `None`, `Ok(Some(uuid))` if valid,
/// or `Err(ValidationResult)` if invalid.
pub fn parse_optional_uuid_for_validation(
    value: Option<&str>,
    field_name: &str,
) -> Result<Option<Uuid>, ValidationResult> {
    match value {
        None => Ok(None),
        Some(v) => parse_uuid_for_validation(v, field_name).map(Some),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_result_serializes_correctly() {
        let result = ValidationResult::valid();
        let json = serde_json::to_string(&result).unwrap();
        assert_eq!(json, r#"{"valid":true}"#);
    }

    #[test]
    fn invalid_result_serializes_correctly() {
        let result = ValidationResult::invalid("Slug already exists");
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains(r#""valid":false"#));
        assert!(json.contains(r#""message":"Slug already exists""#));
    }

    #[test]
    fn invalid_with_suggestion_serializes_correctly() {
        let result =
            ValidationResult::invalid_with_suggestion("Slug already exists", "try-this-slug");
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains(r#""suggestion":"try-this-slug""#));
    }

    #[test]
    fn parse_uuid_valid() {
        let uuid_str = "01933f9a-7b1e-7c9f-8f3d-1a2b3c4d5e6f";
        let result = parse_uuid_for_validation(uuid_str, "moduleId");
        assert!(result.is_ok());
    }

    #[test]
    fn parse_uuid_invalid() {
        let result = parse_uuid_for_validation("not-a-uuid", "moduleId");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(!err.valid);
        assert!(err.message.unwrap().contains("moduleId"));
    }

    #[test]
    fn parse_optional_uuid_none() {
        let result = parse_optional_uuid_for_validation(None, "excludeId");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn parse_optional_uuid_valid() {
        let uuid_str = "01933f9a-7b1e-7c9f-8f3d-1a2b3c4d5e6f";
        let result = parse_optional_uuid_for_validation(Some(uuid_str), "excludeId");
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn parse_optional_uuid_invalid() {
        let result = parse_optional_uuid_for_validation(Some("bad"), "excludeId");
        assert!(result.is_err());
    }
}
