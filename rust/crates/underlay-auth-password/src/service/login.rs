use std::time::Duration;

use underlay_auth::hashing::{PasswordHasherExt, PasswordVerifierExt};
use underlay_auth::{AuthError, CredentialMetadata, User, UserStatus};
use underlay_ratelimit::{RateLimitBackend, RateLimitConfig};

use crate::errors::{PasswordAuthError, PasswordAuthResult};

use super::{PasswordAuthRepository, PasswordAuthService};

impl<R, H, V, L> PasswordAuthService<R, H, V, L>
where
    R: PasswordAuthRepository,
    H: PasswordHasherExt,
    V: PasswordVerifierExt,
    L: RateLimitBackend,
{
    /// Verify login credentials.
    pub async fn verify_login(&self, email: &str, password: &str) -> PasswordAuthResult<User> {
        self.verify_login_with_context(email, password, None).await
    }

    /// Verify login credentials with an optional context (e.g. IP address).
    pub async fn verify_login_with_context(
        &self,
        email: &str,
        password: &str,
        ip: Option<&str>,
    ) -> PasswordAuthResult<User> {
        let normalized_email = email.trim().to_lowercase();
        let rate_limit_key = match ip {
            Some(ip) => format!("login:{}:{}", normalized_email, ip),
            None => format!("login:{}", normalized_email),
        };

        let rate_limit_config = RateLimitConfig::new(
            self.config.rate_limit_max_attempts() as u64,
            Duration::from_secs(self.config.rate_limit_window_seconds()),
        );
        let rate_result = self
            .rate_limiter
            .check_and_increment(&rate_limit_key, &rate_limit_config)
            .await
            .map_err(|e| PasswordAuthError::Internal(format!("Rate limit error: {}", e)))?;

        if rate_result.is_denied() {
            return Err(PasswordAuthError::RateLimited {
                retry_after_seconds: rate_result.retry_after_secs(),
            });
        }

        let user = match self
            .repository
            .find_user_by_email(&normalized_email)
            .await?
        {
            Some(u) => u,
            None => return Err(PasswordAuthError::CredentialNotFound),
        };

        if user.status == UserStatus::Suspended {
            return Err(AuthError::AccountSuspended.into());
        }
        if user.status == UserStatus::Deleted {
            return Err(AuthError::AccountDeleted.into());
        }

        if let Some(retry_after_seconds) = self
            .repository
            .get_lockout_remaining_seconds(user.id)
            .await?
        {
            return Err(PasswordAuthError::AccountLocked {
                retry_after_seconds,
            });
        }

        let credential = match self.repository.find_password_credential(user.id).await? {
            Some(c) => c,
            None => return Err(PasswordAuthError::CredentialNotFound),
        };

        let password_hash = match &credential.metadata {
            CredentialMetadata::Password { .. } => &credential.secret_encrypted,
            _ => {
                return Err(PasswordAuthError::Internal(
                    "Invalid credential type".to_string(),
                ));
            }
        };

        let is_valid = self
            .verifier
            .verify_password(password.as_bytes(), password_hash)?;

        if is_valid {
            self.repository.reset_failed_logins(user.id).await?;
            Ok(user)
        } else {
            let attempt = self
                .repository
                .record_failed_login(
                    user.id,
                    self.config.max_failed_attempts(),
                    self.config.lockout_duration_seconds(),
                )
                .await?;

            if let Some(retry_after_seconds) = attempt.lockout_remaining_seconds {
                Err(PasswordAuthError::AccountLocked {
                    retry_after_seconds,
                })
            } else if attempt.count >= self.config.max_failed_attempts() {
                Err(PasswordAuthError::AccountLocked {
                    retry_after_seconds: self.config.lockout_duration_seconds(),
                })
            } else {
                Err(PasswordAuthError::WrongPassword)
            }
        }
    }
}
