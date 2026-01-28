//! Helpers for parsing and validating path parameters.
//!
//! Provides utilities for parsing UUID path parameters with consistent error responses.
//!
//! # Example
//!
//! ```rust,ignore
//! use underlay_http::parse_uuid_path;
//! use axum::response::IntoResponse;
//!
//! pub async fn get_item(Path(item_id): Path<String>) -> impl IntoResponse {
//!     let uuid = parse_uuid_path(&item_id, "itemId")?;
//!     // uuid is now a valid underlay_core::Uuid
//! }
//! ```

use axum::http::StatusCode;
use axum::response::Response;

use underlay_core::{AppError, Uuid};

use crate::error_response;

/// Parse a string path parameter as a UUID.
///
/// Returns an `underlay_core::Uuid` which is the standard UUID type used across
/// Underlay-based projects. Returns a 400 Bad Request error response if the
/// string is not a valid UUID.
///
/// # Arguments
/// * `value` - The string value to parse (typically from a path parameter)
/// * `field_name` - The name of the field for error messages (e.g., "pathwayId", "moduleId")
///
/// # Example
/// ```rust,ignore
/// use underlay_http::parse_uuid_path;
///
/// pub async fn get_pathway(Path(pathway_id): Path<String>) -> impl IntoResponse {
///     let uuid = parse_uuid_path(&pathway_id, "pathwayId")?;
///     let parsed = PathwayId(uuid);
///     // ... rest of handler
/// }
/// ```
pub fn parse_uuid_path(value: &str, field_name: &str) -> Result<Uuid, Response> {
    Uuid::parse_str(value).map_err(|_| {
        let err = AppError::new(
            "validation.invalid_id",
            format!("Invalid {field_name}; expected UUID string."),
        );
        error_response(StatusCode::BAD_REQUEST, err)
    })
}

/// Parse a string path parameter as a UUID, returning a raw `uuid::Uuid`.
///
/// This is useful when working with database functions that expect `uuid::Uuid`
/// rather than the `underlay_core::Uuid` wrapper type.
///
/// # Arguments
/// * `value` - The string value to parse (typically from a path parameter)
/// * `field_name` - The name of the field for error messages (e.g., "pathwayId", "moduleId")
///
/// # Example
/// ```rust,ignore
/// use underlay_http::parse_uuid_path_raw;
///
/// pub async fn update_summary(Path(summary_id): Path<String>) -> impl IntoResponse {
///     let uuid = parse_uuid_path_raw(&summary_id, "summaryId")?;
///     // uuid is uuid::Uuid, ready for DB calls
/// }
/// ```
pub fn parse_uuid_path_raw(value: &str, field_name: &str) -> Result<uuid::Uuid, Response> {
    uuid::Uuid::parse_str(value).map_err(|_| {
        let err = AppError::new(
            "validation.invalid_id",
            format!("Invalid {field_name}; expected UUID string."),
        );
        error_response(StatusCode::BAD_REQUEST, err)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uuid_path_valid() {
        let valid_uuid = "01234567-89ab-cdef-0123-456789abcdef";
        let result = parse_uuid_path(valid_uuid, "testId");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.to_string(), valid_uuid);
    }

    #[test]
    fn test_parse_uuid_path_invalid() {
        let invalid = "not-a-uuid";
        let result = parse_uuid_path(invalid, "testId");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_uuid_path_empty() {
        let result = parse_uuid_path("", "testId");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_uuid_path_raw_valid() {
        let valid_uuid = "01234567-89ab-cdef-0123-456789abcdef";
        let result = parse_uuid_path_raw(valid_uuid, "testId");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), valid_uuid);
    }

    #[test]
    fn test_parse_uuid_path_raw_invalid() {
        let result = parse_uuid_path_raw("invalid", "testId");
        assert!(result.is_err());
    }
}
