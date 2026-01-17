//! Error types for rate limiting.

use thiserror::Error;

/// Errors that can occur during rate limiting operations.
#[derive(Debug, Error)]
pub enum RateLimitError {
    /// Backend storage error.
    #[error("backend error: {0}")]
    Backend(String),

    /// Configuration error.
    #[error("configuration error: {0}")]
    Config(String),
}

/// Result type for rate limiting operations.
pub type Result<T> = std::result::Result<T, RateLimitError>;
