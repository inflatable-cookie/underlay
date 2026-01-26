use thiserror::Error;

/// Errors that can occur during email operations.
#[derive(Debug, Error)]
pub enum EmailError {
    /// The email address is invalid.
    #[error("invalid email address: {0}")]
    InvalidAddress(String),

    /// The email is missing required content (subject, body, etc.).
    #[error("missing required field: {0}")]
    MissingField(&'static str),

    /// Failed to connect to the email server.
    #[error("connection failed: {0}")]
    ConnectionFailed(String),

    /// Authentication with the email server failed.
    #[error("authentication failed: {0}")]
    AuthenticationFailed(String),

    /// The email was rejected by the server.
    #[error("email rejected: {0}")]
    Rejected(String),

    /// Rate limit exceeded.
    #[error("rate limit exceeded: {0}")]
    RateLimited(String),

    /// Template rendering failed.
    #[error("template error: {0}")]
    TemplateError(String),

    /// Configuration error.
    #[error("configuration error: {0}")]
    ConfigError(String),

    /// Database error (for dev capture adapter).
    #[error("database error: {0}")]
    DatabaseError(String),

    /// Generic transport error.
    #[error("transport error: {0}")]
    TransportError(String),

    /// The adapter is not available or not configured.
    #[error("adapter not available: {0}")]
    AdapterUnavailable(String),
}

/// Result type for email operations.
pub type EmailResult<T> = Result<T, EmailError>;
