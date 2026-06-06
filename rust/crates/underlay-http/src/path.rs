//! Helpers for parsing and validating path parameters.

use axum::http::StatusCode;
use axum::response::Response;

use underlay_core::{AppError, Uuid};

use crate::error_response;

/// Parse a string path parameter as a UUID.
///
/// Returns a 400 Bad Request response if the value is not a valid UUID.
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
/// Use this when a caller needs `uuid::Uuid` rather than `underlay_core::Uuid`.
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
