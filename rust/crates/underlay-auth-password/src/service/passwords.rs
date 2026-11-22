use underlay_auth::hashing::{PasswordHasherExt, PasswordVerifierExt};
use underlay_auth::Credential;
use underlay_core::Uuid;
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
    /// Create or replace a user's password credential.
    pub async fn set_password(
        &self,
        user_id: Uuid,
        password: &str,
    ) -> PasswordAuthResult<Credential> {
        self.validate_new_password(password).await?;

        let existing_id =
            if let Some(existing) = self.repository.find_password_credential(user_id).await? {
                if self
                    .verify_password(password, &existing.secret_encrypted)
                    .await?
                {
                    return Err(PasswordAuthError::PasswordSameAsCurrent);
                }
                Some(existing.id)
            } else {
                None
            };

        let new_hash = self.hash_password(password).await?;
        if let Some(existing_id) = existing_id {
            self.repository
                .update_password_credential(existing_id, &new_hash)
                .await?;

            let updated = self
                .repository
                .find_password_credential(user_id)
                .await?
                .ok_or(PasswordAuthError::Internal(
                    "Password credential missing after update".to_string(),
                ))?;

            Ok(updated)
        } else {
            self.repository
                .create_password_credential(user_id, &new_hash)
                .await
        }
    }

    /// Change a user's password after checking the per-user attempt limit and current password.
    pub async fn change_password(
        &self,
        user_id: Uuid,
        current_password: &str,
        new_password: &str,
    ) -> PasswordAuthResult<()> {
        self.check_rate_limit(
            &format!("change-password:{user_id}"),
            self.config.rate_limit_max_attempts(),
        )
        .await?;

        if current_password.len() > MAX_PASSWORD_BYTES {
            return Err(PasswordAuthError::WrongPassword);
        }

        let credential = match self.repository.find_password_credential(user_id).await? {
            Some(c) => c,
            None => return Err(PasswordAuthError::CredentialNotFound),
        };

        let current_hash = credential.secret_encrypted;
        let is_valid = self
            .verify_password(current_password, &current_hash)
            .await?;

        if !is_valid {
            return Err(PasswordAuthError::WrongPassword);
        }

        self.validate_new_password(new_password).await?;

        if self.verify_password(new_password, &current_hash).await? {
            return Err(PasswordAuthError::PasswordSameAsCurrent);
        }

        let new_hash = self.hash_password(new_password).await?;

        self.repository
            .update_password_credential(credential.id, &new_hash)
            .await?;

        Ok(())
    }

    /// Reset a user's password (admin/internal use).
    pub async fn reset_password(
        &self,
        user_id: Uuid,
        new_password: &str,
    ) -> PasswordAuthResult<()> {
        self.validate_new_password(new_password).await?;

        let credential = match self.repository.find_password_credential(user_id).await? {
            Some(c) => c,
            None => return Err(PasswordAuthError::CredentialNotFound),
        };

        let new_hash = self.hash_password(new_password).await?;

        self.repository
            .update_password_credential(credential.id, &new_hash)
            .await?;

        self.repository.reset_failed_logins(user_id).await?;

        Ok(())
    }
}
