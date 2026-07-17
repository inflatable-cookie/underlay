use std::collections::HashMap;
use underlay_core::{AppError, ErrorCode};

/// Result type for auth operations.
pub type AuthResult<T> = Result<T, AuthError>;

/// Generate a `From<ProviderError> for AuthError` impl declaratively.
///
/// Each arm maps provider error variants to `AuthError` variants.
/// The match body is written as raw `pat => expr` arms, with `AuthError`
/// available unqualified in scope.
///
/// # Example
///
/// ```rust,ignore
/// use underlay_auth::impl_auth_error_from;
///
/// impl_auth_error_from!(JwtError, err, {
///     JwtError::Expired => AuthError::SessionExpired,
///     JwtError::InvalidToken | JwtError::UnsupportedTokenType => AuthError::TokenInvalid,
///     JwtError::Config(msg) | JwtError::Key(msg) | JwtError::Internal(msg) => {
///         AuthError::Internal(msg)
///     }
/// });
/// ```
#[macro_export]
macro_rules! impl_auth_error_from {
    ($error_type:ty, $err:ident, { $( $arms:tt )* }) => {
        impl From<$error_type> for $crate::AuthError {
            fn from($err: $error_type) -> Self {
                #[allow(unused_imports)]
                use $crate::AuthError;
                match $err {
                    $( $arms )*
                }
            }
        }
    };
}

/// Comprehensive authentication errors for Underlay-based apps.
///
/// Error codes follow the pattern `auth.<category>.<specific>` for easy mapping
/// to HTTP status codes and client-side handling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthError {
    // User not found / registration errors
    /// User does not exist with the given identifier.
    UserNotFound,
    /// Email address is already registered.
    EmailAlreadyExists,
    /// Registration is disabled or rate-limited.
    RegistrationDisabled,
    /// User account is suspended.
    AccountSuspended,
    /// User account is deleted.
    AccountDeleted,

    // Password errors
    /// Authentication failed (wrong username/email or password).
    WrongCredentials,
    /// Password is incorrect.
    WrongPassword,
    /// Password does not meet strength requirements.
    PasswordTooWeak,
    /// Password has been found in known breach databases.
    PasswordCompromised,
    /// Password change requires current password.
    PasswordChangeRequiresCurrent,
    /// New password cannot be the same as current password.
    PasswordSameAsCurrent,

    // 2FA errors
    /// 2FA is required but not provided.
    TwoFactorRequired,
    /// 2FA code is invalid or expired.
    TwoFactorInvalid,
    /// 2FA setup has not been completed.
    TwoFactorNotSetUp,
    /// Backup code is invalid or already used.
    BackupCodeInvalid,
    /// Backup codes are exhausted.
    BackupCodesExhausted,

    // Session errors
    /// Session has expired.
    SessionExpired,
    /// Session has been revoked.
    SessionRevoked,
    /// Token is invalid (malformed, wrong signature).
    TokenInvalid,
    /// Token is malformed.
    TokenMalformed,
    /// Token is not yet valid (nbf claim).
    TokenNotYetValid,
    /// Token fingerprint does not match (possible token theft).
    TokenFingerprintMismatch,

    // Backward-compatible alias for TokenInvalid (used by extractors)
    /// @deprecated Use TokenInvalid instead
    InvalidToken,

    // PassKey/WebAuthn errors
    /// PassKey registration failed.
    PassKeyRegistrationFailed,
    /// PassKey authentication failed.
    PassKeyAuthenticationFailed,
    /// PassKey credential not found.
    PassKeyCredentialNotFound,
    /// PassKey counter regression detected (possible token theft).
    PassKeyCounterRegression,

    // OAuth errors
    /// OAuth provider error.
    OAuthError(String),
    /// OAuth connection already exists for this provider.
    OAuthAlreadyConnected,
    /// OAuth connection not found for user.
    OAuthNotConnected,
    /// OAuth token refresh failed.
    OAuthTokenRefreshFailed,
    /// OAuth access denied by user.
    OAuthAccessDenied,

    // Rate limiting
    /// Too many attempts, rate limited.
    RateLimited {
        /// Seconds until the rate limit resets.
        retry_after_seconds: u64,
    },

    // Generic errors
    /// Operation requires authentication.
    Unauthorized,
    /// Authenticated but lacks permission.
    Forbidden,
    /// Invalid or malformed request.
    BadRequest(String),
    /// Internal server error.
    Internal(String),
}

impl AuthError {
    /// Get the stable error code for this error.
    pub fn code(&self) -> &'static str {
        match self {
            // User not found / registration
            AuthError::UserNotFound => "auth.user_not_found",
            AuthError::EmailAlreadyExists => "auth.email_already_exists",
            AuthError::RegistrationDisabled => "auth.registration_disabled",
            AuthError::AccountSuspended => "auth.account_suspended",
            AuthError::AccountDeleted => "auth.account_deleted",

            // Password
            AuthError::WrongCredentials => "auth.wrong_credentials",
            AuthError::WrongPassword => "auth.wrong_password",
            AuthError::PasswordTooWeak => "auth.password_weak",
            AuthError::PasswordCompromised => "auth.password_compromised",
            AuthError::PasswordChangeRequiresCurrent => "auth.password_change_requires_current",
            AuthError::PasswordSameAsCurrent => "auth.password_same_as_current",

            // 2FA
            AuthError::TwoFactorRequired => "auth.2fa_required",
            AuthError::TwoFactorInvalid => "auth.2fa_invalid",
            AuthError::TwoFactorNotSetUp => "auth.2fa_not_set_up",
            AuthError::BackupCodeInvalid => "auth.backup_code_invalid",
            AuthError::BackupCodesExhausted => "auth.backup_codes_exhausted",

            // Session
            AuthError::SessionExpired => "auth.session_expired",
            AuthError::SessionRevoked => "auth.session_revoked",
            AuthError::TokenInvalid => "auth.token_invalid",
            AuthError::TokenMalformed => "auth.token_malformed",
            AuthError::TokenNotYetValid => "auth.token_not_yet_valid",
            AuthError::TokenFingerprintMismatch => "auth.token_fingerprint_mismatch",
            // Backward-compatible alias
            AuthError::InvalidToken => "auth.token_invalid",

            // PassKey
            AuthError::PassKeyRegistrationFailed => "auth.passkey_registration_failed",
            AuthError::PassKeyAuthenticationFailed => "auth.passkey_authentication_failed",
            AuthError::PassKeyCredentialNotFound => "auth.passkey_credential_not_found",
            AuthError::PassKeyCounterRegression => "auth.passkey_counter_regression",

            // OAuth
            AuthError::OAuthError(_) => "auth.oauth_error",
            AuthError::OAuthAlreadyConnected => "auth.oauth_already_connected",
            AuthError::OAuthNotConnected => "auth.oauth_not_connected",
            AuthError::OAuthTokenRefreshFailed => "auth.oauth_token_refresh_failed",
            AuthError::OAuthAccessDenied => "auth.oauth_access_denied",

            // Rate limiting
            AuthError::RateLimited { .. } => "auth.rate_limited",

            // Generic
            AuthError::Unauthorized => "auth.unauthorized",
            AuthError::Forbidden => "auth.forbidden",
            AuthError::BadRequest(_) => "auth.bad_request",
            AuthError::Internal(_) => "auth.internal",
        }
    }

    /// Convert to AppError for HTTP response handling.
    pub fn into_app_error(self) -> AppError {
        let code = self.code();
        let message = self.message();
        AppError::new(code, message)
    }

    /// Get a user-friendly message for this error.
    pub fn message(&self) -> String {
        match self {
            AuthError::UserNotFound => "No account found with this email.".into(),
            AuthError::EmailAlreadyExists => "An account with this email already exists.".into(),
            AuthError::RegistrationDisabled => "Registration is currently disabled.".into(),
            AuthError::AccountSuspended => "Your account has been suspended.".into(),
            AuthError::AccountDeleted => "This account has been deleted.".into(),
            AuthError::WrongCredentials => "Invalid email or password.".into(),
            AuthError::WrongPassword => "Incorrect password.".into(),
            AuthError::PasswordTooWeak => "Password does not meet requirements.".into(),
            AuthError::PasswordCompromised => {
                "This password has been found in a data breach.".into()
            }
            AuthError::PasswordChangeRequiresCurrent => {
                "Please enter your current password to change it.".into()
            }
            AuthError::PasswordSameAsCurrent => {
                "New password must be different from current password.".into()
            }
            AuthError::TwoFactorRequired => "Two-factor authentication required.".into(),
            AuthError::TwoFactorInvalid => "Invalid two-factor code.".into(),
            AuthError::TwoFactorNotSetUp => "Two-factor authentication is not set up.".into(),
            AuthError::BackupCodeInvalid => "Invalid backup code.".into(),
            AuthError::BackupCodesExhausted => "No backup codes remaining.".into(),
            AuthError::SessionExpired => "Your session has expired. Please sign in again.".into(),
            AuthError::SessionRevoked => "Your session has been revoked.".into(),
            AuthError::TokenInvalid => "Invalid authentication token.".into(),
            AuthError::TokenMalformed => "Malformed authentication token.".into(),
            AuthError::TokenNotYetValid => "Authentication token is not yet valid.".into(),
            AuthError::TokenFingerprintMismatch => "Session mismatch. Please sign in again.".into(),
            AuthError::InvalidToken => "Invalid authentication token.".into(),
            AuthError::PassKeyRegistrationFailed => {
                "PassKey registration failed. Please try again.".into()
            }
            AuthError::PassKeyAuthenticationFailed => {
                "PassKey authentication failed. Please try again.".into()
            }
            AuthError::PassKeyCredentialNotFound => "PassKey not found.".into(),
            AuthError::PassKeyCounterRegression => {
                "Security issue detected. Please sign in again.".into()
            }
            AuthError::OAuthError(_) => "OAuth sign-in failed. Please try again.".into(),
            AuthError::OAuthAlreadyConnected => {
                "This account is already connected to this provider.".into()
            }
            AuthError::OAuthNotConnected => {
                "This account is not connected to this provider.".into()
            }
            AuthError::OAuthTokenRefreshFailed => {
                "Failed to refresh OAuth token. Please reconnect.".into()
            }
            AuthError::OAuthAccessDenied => "Authorization was denied. Please try again.".into(),
            AuthError::RateLimited {
                retry_after_seconds,
            } => format!(
                "Too many attempts. Please try again in {} seconds.",
                retry_after_seconds
            ),
            AuthError::Unauthorized => "Please sign in to continue.".into(),
            AuthError::Forbidden => "You do not have permission to perform this action.".into(),
            AuthError::BadRequest(details) => details.clone(),
            AuthError::Internal(_) => "An internal error occurred.".into(),
        }
    }

    /// Get additional details for the error (used in API envelope).
    pub fn details(&self) -> Option<HashMap<String, String>> {
        match self {
            AuthError::RateLimited {
                retry_after_seconds,
            } => Some(
                [(
                    "retry_after_seconds".to_string(),
                    retry_after_seconds.to_string(),
                )]
                .into_iter()
                .collect(),
            ),
            AuthError::OAuthError(_) => None,
            AuthError::BadRequest(details) => Some(
                [("details".to_string(), details.clone())]
                    .into_iter()
                    .collect(),
            ),
            AuthError::Internal(_) => None,
            _ => None,
        }
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Stable code plus the user-facing message; keeps AuthError usable
        // with `?`, `anyhow`, and `Box<dyn Error>`.
        write!(f, "{}: {}", self.code(), self.message())
    }
}

impl std::error::Error for AuthError {}

impl ErrorCode for AuthError {
    fn code(&self) -> &str {
        // Delegate to the inherent code() (returns &'static str).
        AuthError::code(self)
    }
}
