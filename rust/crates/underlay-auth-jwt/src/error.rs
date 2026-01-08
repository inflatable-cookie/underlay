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
        match self {
            JwtError::Config(_) => "auth.jwt_config_error",
            JwtError::Key(_) => "auth.jwt_key_error",
            JwtError::Expired => "auth.token_expired",
            JwtError::NotYetValid => "auth.token_not_yet_valid",
            JwtError::InvalidToken => "auth.token_invalid",
            JwtError::MalformedToken => "auth.token_malformed",
            JwtError::SessionRevoked => "auth.session_revoked",
            JwtError::TokenFingerprintMismatch => "auth.token_fingerprint_mismatch",
            JwtError::RefreshReplayDetected => "auth.token_replay",
            JwtError::UnsupportedTokenType => "auth.token_invalid",
            JwtError::Internal(_) => "auth.internal",
        }
    }
}

impl From<JwtError> for AuthError {
    fn from(err: JwtError) -> Self {
        match err {
            JwtError::Expired => AuthError::SessionExpired,
            JwtError::NotYetValid => AuthError::TokenNotYetValid,
            JwtError::InvalidToken => AuthError::TokenInvalid,
            JwtError::MalformedToken => AuthError::TokenMalformed,
            JwtError::SessionRevoked => AuthError::SessionRevoked,
            JwtError::TokenFingerprintMismatch => AuthError::TokenFingerprintMismatch,
            JwtError::RefreshReplayDetected => AuthError::TokenInvalid,
            JwtError::UnsupportedTokenType => AuthError::TokenInvalid,
            JwtError::Config(msg) | JwtError::Key(msg) | JwtError::Internal(msg) => {
                AuthError::Internal(msg)
            }
        }
    }
}

pub type JwtResult<T> = Result<T, JwtError>;
