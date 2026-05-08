//! JWT/session errors.

use thiserror::Error;
use underlay_auth::AuthError;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum JwtError {
    #[error("JWT config error: {0}")]
    Config(String),

    #[error("JWT key error: {0}")]
    Key(String),

    #[error("Token has expired")]
    Expired,

    #[error("Token is not yet valid")]
    NotYetValid,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token is malformed")]
    MalformedToken,

    #[error("Session has been revoked")]
    SessionRevoked,

    #[error("Token fingerprint mismatch")]
    TokenFingerprintMismatch,

    #[error("Refresh token replay detected")]
    RefreshReplayDetected,

    #[error("Unsupported token type")]
    UnsupportedTokenType,

    #[error("Internal error: {0}")]
    Internal(String),
}

impl JwtError {
    pub fn code(&self) -> &'static str {
        let auth_error: AuthError = self.clone().into();
        auth_error.code()
    }
}

underlay_auth::impl_auth_error_from!(JwtError, err, {
    JwtError::Expired => AuthError::SessionExpired,
    JwtError::NotYetValid => AuthError::TokenNotYetValid,
    JwtError::InvalidToken
    | JwtError::RefreshReplayDetected
    | JwtError::UnsupportedTokenType => AuthError::TokenInvalid,
    JwtError::MalformedToken => AuthError::TokenMalformed,
    JwtError::SessionRevoked => AuthError::SessionRevoked,
    JwtError::TokenFingerprintMismatch => AuthError::TokenFingerprintMismatch,
    JwtError::Config(msg) | JwtError::Key(msg) | JwtError::Internal(msg) => {
        AuthError::Internal(msg)
    }
});

pub type JwtResult<T> = Result<T, JwtError>;
