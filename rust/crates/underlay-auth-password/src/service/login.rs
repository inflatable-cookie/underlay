use underlay_auth::hashing::{PasswordHasherExt, PasswordVerifierExt};
use underlay_auth::{AuthError, CredentialMetadata, User, UserStatus};
use underlay_ratelimit::RateLimitBackend;

use crate::errors::{PasswordAuthError, PasswordAuthResult};

use super::core::MAX_PASSWORD_BYTES;
use super::{PasswordAuthRepository, PasswordAuthService};

impl<R, H, V, L> PasswordAuthService<R, H, V, L>
where
    R: PasswordAuthRepository,
    H: PasswordHasherExt + 'static,
    V: PasswordVerifierExt + 'static,
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
        // Email-only limit first: an attacker rotating (spoofed or real) IPs
        // gets a fresh `email:ip` counter per request, so a per-email backstop
        // is what actually bounds per-account brute force.
        self.check_rate_limit(
            &format!("login:{normalized_email}"),
            self.config.rate_limit_email_max_attempts(),
        )
        .await?;

        if let Some(ip) = ip {
            self.check_rate_limit(
                &format!("login:{normalized_email}:{ip}"),
                self.config.rate_limit_max_attempts(),
            )
            .await?;
        }

        if password.len() > MAX_PASSWORD_BYTES {
            return Err(PasswordAuthError::WrongPassword);
        }

        let user = match self
            .repository
            .find_user_by_email(&normalized_email)
            .await?
        {
            Some(u) => u,
            None => {
                // Equalize timing: run one KDF pass so an unknown email costs
                // the same as a real verify. Without this, the fast miss path
                // is an account-existence oracle.
                self.dummy_verify().await;
                return Err(PasswordAuthError::CredentialNotFound);
            }
        };

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
            None => {
                self.dummy_verify().await;
                return Err(PasswordAuthError::CredentialNotFound);
            }
        };

        let password_hash = match credential.metadata {
            CredentialMetadata::Password { .. } => credential.secret_encrypted,
            _ => {
                return Err(PasswordAuthError::Internal(
                    "Invalid credential type".to_string(),
                ));
            }
        };

        let is_valid = self.verify_password(password, &password_hash).await?;

        if is_valid {
            // Check account status only AFTER verifying credentials so
            // unauthenticated callers cannot enumerate suspended/deleted
            // accounts via error codes or fast-path timing.
            if user.status == UserStatus::Suspended {
                return Err(AuthError::AccountSuspended.into());
            }
            if user.status == UserStatus::Deleted {
                return Err(AuthError::AccountDeleted.into());
            }
            if self.repository.supports_password_rehash()
                && self.hasher.needs_rehash(&password_hash)
            {
                let new_hash = self.hash_password(password).await?;
                self.repository
                    .update_password_credential_if_current(credential.id, &password_hash, &new_hash)
                    .await?;
            }
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

    /// Burn one KDF computation to equalize the timing of account-miss paths
    /// against a real password verify. The result is intentionally discarded.
    async fn dummy_verify(&self) {
        let _ = self.hash_password("underlay-login-timing-equalizer").await;
    }
}
