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