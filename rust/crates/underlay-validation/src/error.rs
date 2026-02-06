//! Validation error types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Result type alias for validation operations.
pub type ValidationResult<T> = Result<T, ValidationError>;

/// A field-level validation error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldError {
    /// The error message.
    pub message: String,
    /// Optional error code for i18n.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl FieldError {
    /// Create a new field error with just a message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: None,
        }
    }

    /// Create a new field error with a message and code.
    pub fn with_code(message: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: Some(code.into()),
        }
    }
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<String> for FieldError {
    fn from(message: String) -> Self {
        Self::new(message)
    }
}

impl From<&str> for FieldError {
    fn from(message: &str) -> Self {
        Self::new(message)
    }
}

/// A validation error containing field-level errors.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationError {
    /// Overall error message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Error code for the overall error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Field-level errors.
    pub field_errors: HashMap<String, FieldError>,
}

impl ValidationError {
    /// Create a new empty validation error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a validation error with a message.
    pub fn with_message(message: impl Into<String>) -> Self {
        Self {
            message: Some(message.into()),
            code: Some("validation.failed".to_string()),
            field_errors: HashMap::new(),
        }
    }

    /// Add a field error.
    pub fn add_field(&mut self, field: impl Into<String>, error: impl Into<FieldError>) {
        self.field_errors.insert(field.into(), error.into());
    }

    /// Check if there are any field errors.
    pub fn has_errors(&self) -> bool {
        !self.field_errors.is_empty()
    }

    /// Check if a specific field has an error.
    pub fn has_field(&self, field: &str) -> bool {
        self.field_errors.contains_key(field)
    }

    /// Get the number of field errors.
    pub fn field_count(&self) -> usize {
        self.field_errors.len()
    }

    /// Get a specific field error.
    pub fn get_field(&self, field: &str) -> Option<&FieldError> {
        self.field_errors.get(field)
    }

    /// Convert to a Result, returning Ok if no errors.
    pub fn into_result(self) -> ValidationResult<()> {
        if self.has_errors() {
            Err(self.finalize())
        } else {
            Ok(())
        }
    }

    /// Finalize the error, setting the message if needed.
    pub fn finalize(mut self) -> Self {
        if self.message.is_none() && self.has_errors() {
            self.message = Some("Validation failed".to_string());
            self.code = Some("validation.failed".to_string());
        }
        self
    }

    /// Merge another validation error into this one.
    pub fn merge(&mut self, other: ValidationError) {
        for (field, error) in other.field_errors {
            self.field_errors.insert(field, error);
        }
    }

    /// Add a field error with a prefix (for nested validation).
    pub fn add_nested(&mut self, prefix: &str, field: &str, error: impl Into<FieldError>) {
        let key = if prefix.is_empty() {
            field.to_string()
        } else {
            format!("{}.{}", prefix, field)
        };
        self.field_errors.insert(key, error.into());
    }

    /// Merge another validation error with a prefix.
    pub fn merge_nested(&mut self, prefix: &str, other: ValidationError) {
        for (field, error) in other.field_errors {
            self.add_nested(prefix, &field, error);
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref msg) = self.message {
            write!(f, "{}", msg)?;
        } else {
            write!(f, "Validation failed")?;
        }

        if !self.field_errors.is_empty() {
            write!(f, ": ")?;
            let errors: Vec<_> = self
                .field_errors
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v.message))
                .collect();
            write!(f, "{}", errors.join(", "))?;
        }

        Ok(())
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_error_creation() {
        let err = FieldError::new("Invalid email");
        assert_eq!(err.message, "Invalid email");
        assert!(err.code.is_none());

        let err = FieldError::with_code("Invalid email", "email.invalid");
        assert_eq!(err.message, "Invalid email");
        assert_eq!(err.code, Some("email.invalid".to_string()));
    }

    #[test]
    fn test_validation_error_builder() {
        let mut err = ValidationError::new();
        assert!(!err.has_errors());

        err.add_field("email", "Invalid email");
        err.add_field(
            "password",
            FieldError::with_code("Too short", "password.short"),
        );

        assert!(err.has_errors());
        assert_eq!(err.field_count(), 2);
        assert!(err.has_field("email"));
        assert!(err.has_field("password"));
        assert!(!err.has_field("username"));
    }

    #[test]
    fn test_into_result() {
        let err = ValidationError::new();
        assert!(err.into_result().is_ok());

        let mut err = ValidationError::new();
        err.add_field("email", "Invalid");
        assert!(err.into_result().is_err());
    }

    #[test]
    fn test_nested_errors() {
        let mut err = ValidationError::new();
        err.add_nested("user", "email", "Invalid email");
        err.add_nested("user", "name", "Required");

        assert!(err.has_field("user.email"));
        assert!(err.has_field("user.name"));
    }

    #[test]
    fn test_merge() {
        let mut err1 = ValidationError::new();
        err1.add_field("email", "Invalid");

        let mut err2 = ValidationError::new();
        err2.add_field("password", "Too short");

        err1.merge(err2);
        assert_eq!(err1.field_count(), 2);
    }

    #[test]
    fn test_serialization() {
        let mut err = ValidationError::new();
        err.add_field(
            "email",
            FieldError::with_code("Invalid email", "email.invalid"),
        );

        let json = serde_json::to_string(&err.finalize()).unwrap();
        assert!(json.contains("field_errors"));
        assert!(json.contains("email.invalid"));
    }
}
