//! Password authentication errors.

use underlay_auth::AuthError;

/// Result type for password auth operations.
pub type PasswordAuthResult<T> = Result<T, PasswordAuthError>;

/// Errors specific to password authentication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordAuthError {
    /// Password does not meet strength requirements.
    PasswordTooWeak(String),
    /// Password has been found in a data breach.
    PasswordCompromised,
    /// Password is the same as the current password.
    PasswordSameAsCurrent,
    /// Password is incorrect.
    WrongPassword,
    /// Account is locked due to too many failed attempts.
    AccountLocked {
        /// Seconds until the lockout expires.
        retry_after_seconds: u64,
    },
    /// Rate limit exceeded for this operation.
    RateLimited {
        /// Seconds until the rate limit resets.
        retry_after_seconds: u64,
    },
    /// Credential not found for this user.
    CredentialNotFound,
    /// Account is suspended.
    AccountSuspended,
    /// Account is deleted.
    AccountDeleted,
    /// Internal error.
    Internal(String),
}

underlay_auth::impl_auth_error_from!(PasswordAuthError, err, {
    PasswordAuthError::PasswordTooWeak(_) => AuthError::PasswordTooWeak,
    PasswordAuthError::PasswordCompromised => AuthError::PasswordCompromised,
    PasswordAuthError::PasswordSameAsCurrent => AuthError::PasswordSameAsCurrent,
    PasswordAuthError::WrongPassword | PasswordAuthError::CredentialNotFound => {
        AuthError::WrongCredentials
    }
    PasswordAuthError::AccountSuspended => AuthError::AccountSuspended,
    PasswordAuthError::AccountDeleted => AuthError::AccountDeleted,
    PasswordAuthError::AccountLocked { retry_after_seconds }
    | PasswordAuthError::RateLimited { retry_after_seconds } => {
        AuthError::RateLimited { retry_after_seconds }
    }
    PasswordAuthError::Internal(msg) => AuthError::Internal(msg),
});

impl From<std::io::Error> for PasswordAuthError {
    fn from(err: std::io::Error) -> Self {
        PasswordAuthError::Internal(err.to_string())
    }
}

impl From<String> for PasswordAuthError {
    fn from(err: String) -> Self {
        PasswordAuthError::Internal(err)
    }
}

impl From<AuthError> for PasswordAuthError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::UserNotFound => PasswordAuthError::CredentialNotFound,
            AuthError::WrongCredentials => PasswordAuthError::WrongPassword,
            AuthError::WrongPassword => PasswordAuthError::WrongPassword,
            AuthError::AccountSuspended => PasswordAuthError::AccountSuspended,
            AuthError::AccountDeleted => PasswordAuthError::AccountDeleted,
            AuthError::RateLimited {
                retry_after_seconds,
            } => PasswordAuthError::RateLimited {
                retry_after_seconds,
            },
            _ => PasswordAuthError::Internal(err.message()),
        }
    }
}

impl std::fmt::Display for PasswordAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PasswordAuthError::PasswordTooWeak(msg) => {
                write!(f, "Password too weak: {}", msg)
            }
            PasswordAuthError::PasswordCompromised => {
                write!(f, "Password has been found in a data breach")
            }
            PasswordAuthError::PasswordSameAsCurrent => {
                write!(f, "New password must be different from current password")
            }
            PasswordAuthError::WrongPassword => {
                write!(f, "Password is incorrect")
            }
            PasswordAuthError::AccountSuspended => {
                write!(f, "Account suspended")
            }
            PasswordAuthError::AccountDeleted => {
                write!(f, "Account deleted")
            }
            PasswordAuthError::AccountLocked {
                retry_after_seconds,
            } => {
                write!(
                    f,
                    "Account locked. Try again in {} seconds",
                    retry_after_seconds
                )
            }
            PasswordAuthError::RateLimited {
                retry_after_seconds,
            } => {
                write!(
                    f,
                    "Too many attempts. Try again in {} seconds",
                    retry_after_seconds
                )
            }
            PasswordAuthError::CredentialNotFound => {
                write!(f, "Password credential not found")
            }
            PasswordAuthError::Internal(msg) => {
                write!(f, "Internal error: {}", msg)
            }
        }
    }
}

impl std::error::Error for PasswordAuthError {}

#[cfg(test)]
#[path = "tests/errors_tests.rs"]
mod tests;
