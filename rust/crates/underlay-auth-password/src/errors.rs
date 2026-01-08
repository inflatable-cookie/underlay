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
    /// Internal error.
    Internal(String),
}

impl From<PasswordAuthError> for AuthError {
    fn from(err: PasswordAuthError) -> Self {
        match err {
            PasswordAuthError::PasswordTooWeak(_) => AuthError::PasswordTooWeak,
            PasswordAuthError::PasswordCompromised => AuthError::PasswordCompromised,
            PasswordAuthError::PasswordSameAsCurrent => AuthError::PasswordSameAsCurrent,
            PasswordAuthError::WrongPassword => AuthError::WrongCredentials,
            PasswordAuthError::AccountLocked {
                retry_after_seconds,
            } => AuthError::RateLimited {
                retry_after_seconds,
            },
            PasswordAuthError::RateLimited {
                retry_after_seconds,
            } => AuthError::RateLimited {
                retry_after_seconds,
            },
            PasswordAuthError::CredentialNotFound => AuthError::WrongCredentials,
            PasswordAuthError::Internal(msg) => AuthError::Internal(msg),
        }
    }
}

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
            AuthError::AccountSuspended => {
                PasswordAuthError::Internal("Account suspended".to_string())
            }
            AuthError::AccountDeleted => PasswordAuthError::Internal("Account deleted".to_string()),
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
mod tests {
    use super::*;
    use underlay_auth::AuthError;

    #[test]
    fn error_codes_are_correct() {
        assert_eq!(
            AuthError::PasswordTooWeak,
            Into::<AuthError>::into(PasswordAuthError::PasswordTooWeak("test".to_string()))
        );
        assert_eq!(
            AuthError::PasswordCompromised,
            Into::<AuthError>::into(PasswordAuthError::PasswordCompromised)
        );
        assert_eq!(
            AuthError::WrongCredentials,
            Into::<AuthError>::into(PasswordAuthError::WrongPassword)
        );
        assert_eq!(
            AuthError::WrongCredentials,
            Into::<AuthError>::into(PasswordAuthError::CredentialNotFound)
        );
    }

    #[test]
    fn rate_limited_maps_correctly() {
        use underlay_auth::AuthError;

        let err: AuthError = PasswordAuthError::RateLimited {
            retry_after_seconds: 60,
        }
        .into();
        assert!(matches!(
            err,
            AuthError::RateLimited {
                retry_after_seconds: 60
            }
        ));
    }

    #[test]
    fn account_locked_maps_correctly() {
        use underlay_auth::AuthError;

        let err: AuthError = PasswordAuthError::AccountLocked {
            retry_after_seconds: 300,
        }
        .into();
        assert!(matches!(
            err,
            AuthError::RateLimited {
                retry_after_seconds: 300
            }
        ));
    }

    #[test]
    fn display_formatting_is_correct() {
        let tests = [
            (
                PasswordAuthError::PasswordTooWeak("try harder".to_string()),
                "Password too weak: try harder",
            ),
            (
                PasswordAuthError::PasswordCompromised,
                "Password has been found in a data breach",
            ),
            (
                PasswordAuthError::PasswordSameAsCurrent,
                "New password must be different from current password",
            ),
            (PasswordAuthError::WrongPassword, "Password is incorrect"),
            (
                PasswordAuthError::AccountLocked {
                    retry_after_seconds: 120,
                },
                "Account locked. Try again in 120 seconds",
            ),
            (
                PasswordAuthError::RateLimited {
                    retry_after_seconds: 60,
                },
                "Too many attempts. Try again in 60 seconds",
            ),
            (
                PasswordAuthError::CredentialNotFound,
                "Password credential not found",
            ),
            (
                PasswordAuthError::Internal("test error".to_string()),
                "Internal error: test error",
            ),
        ];

        for (err, expected) in tests {
            assert_eq!(err.to_string(), expected);
        }
    }

    #[test]
    fn io_error_converts_to_internal() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
        let auth_err: PasswordAuthError = io_err.into();
        assert!(matches!(auth_err, PasswordAuthError::Internal(msg) if msg.contains("test")));
    }

    #[test]
    fn string_converts_to_internal() {
        let auth_err: PasswordAuthError = "test error".to_string().into();
        assert!(matches!(auth_err, PasswordAuthError::Internal(msg) if msg == "test error"));
    }
}
