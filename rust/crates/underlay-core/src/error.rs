use std::collections::HashMap;

use crate::dto::{ErrorBody, ErrorEnvelope};

/// Shared accessor for an error's stable, machine-readable code.
///
/// Codes follow the `<domain>.<category>.<specific>` convention workspace-wide
/// (e.g. `auth.token.invalid`, `media.upload.too_large`). The code is stable
/// wire/API vocabulary and must not change once shipped; the human-facing
/// message can. Every public Underlay error type should implement both
/// `std::error::Error` (so it composes with `?`, `anyhow`, and
/// `Box<dyn Error>`) and this trait.
pub trait ErrorCode {
    /// The stable error code for this error.
    fn code(&self) -> &str;
}

/// Core application error type used across Underlay-based projects.
///
/// This is intentionally minimal; domain crates are expected to define
/// more specific constructors and stable error codes.
#[derive(Debug, Clone)]
pub struct AppError {
    pub code: &'static str,
    pub message: String,
    /// Optional field-specific errors, keyed by form field name.
    pub field_errors: Option<HashMap<String, String>>,
}

impl AppError {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        AppError {
            code,
            message: message.into(),
            field_errors: None,
        }
    }

    pub fn with_field_error(
        code: &'static str,
        message: impl Into<String>,
        field: impl Into<String>,
        field_message: impl Into<String>,
    ) -> Self {
        let mut map = HashMap::new();
        map.insert(field.into(), field_message.into());

        AppError {
            code,
            message: message.into(),
            field_errors: Some(map),
        }
    }

    pub fn into_envelope(self) -> ErrorEnvelope {
        ErrorEnvelope::from(self)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for AppError {}

impl ErrorCode for AppError {
    fn code(&self) -> &str {
        self.code
    }
}

impl From<AppError> for ErrorBody {
    fn from(err: AppError) -> Self {
        ErrorBody {
            code: err.code.to_string(),
            message: err.message,
            field_errors: err.field_errors,
        }
    }
}

impl From<AppError> for ErrorEnvelope {
    fn from(err: AppError) -> Self {
        ErrorEnvelope { error: err.into() }
    }
}

pub type AppResult<T> = Result<T, AppError>;
