//! Error types for email TOTP operations.

use thiserror::Error;

/// Errors that can occur during email TOTP operations.
#[derive(Debug, Error)]
pub enum EmailTotpError {
    /// Rate limited: too many code requests in the time window.
    #[error("rate limited: too many code requests")]
    RateLimited,

    /// The verification code has expired.
    #[error("code expired")]
    CodeExpired,

    /// The verification code is invalid.
    #[error("invalid code")]
    InvalidCode,

    /// Too many verification attempts for this code.
    #[error("too many attempts")]
    TooManyAttempts,

    /// No active code found for this user/purpose.
    #[error("no active code found")]
    NoActiveCode,

    /// Verification session not found or expired.
    #[error("verification session not found or expired")]
    SessionNotFound,

    /// Verification session already consumed.
    #[error("verification session already used")]
    SessionAlreadyUsed,

    /// Failed to send the verification email.
    #[error("failed to send email: {0}")]
    EmailSendFailed(String),

    /// Storage backend error.
    #[error("storage error: {0}")]
    Storage(String),
}

/// Result type for email TOTP operations.
pub type EmailTotpResult<T> = Result<T, EmailTotpError>;

underlay_auth::impl_auth_error_from!(EmailTotpError, err, {
    EmailTotpError::RateLimited | EmailTotpError::TooManyAttempts => {
        AuthError::RateLimited { retry_after_seconds: 300 }
    }
    EmailTotpError::CodeExpired
    | EmailTotpError::InvalidCode
    | EmailTotpError::NoActiveCode => AuthError::TwoFactorInvalid,
    EmailTotpError::SessionNotFound => {
        AuthError::BadRequest("Verification session not found or expired".to_string())
    }
    EmailTotpError::SessionAlreadyUsed => {
        AuthError::BadRequest("Verification session already used".to_string())
    }
    EmailTotpError::EmailSendFailed(msg) | EmailTotpError::Storage(msg) => {
        AuthError::Internal(msg)
    }
});
