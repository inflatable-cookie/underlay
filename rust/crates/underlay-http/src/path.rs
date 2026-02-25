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
#[allow(clippy::result_large_err)]
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
#[allow(clippy::result_large_err)]
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
#[path = "tests/path_tests.rs"]
mod tests;
