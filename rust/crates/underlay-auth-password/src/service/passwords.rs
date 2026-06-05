use underlay_auth::hashing::{PasswordHasherExt, PasswordVerifierExt};
use underlay_auth::Credential;
use underlay_core::Uuid;
use underlay_ratelimit::RateLimitBackend;

use crate::errors::{PasswordAuthError, PasswordAuthResult};

use super::{PasswordAuthRepository, PasswordAuthService};

impl<R, H, V, L> PasswordAuthService<R, H, V, L>
where
    R: PasswordAuthRepository,
    H: PasswordHasherExt,
    V: PasswordVerifierExt,
    L: RateLimitBackend,
{
    /// Create or replace a user's password credential.
    pub async fn set_password(
        &self,
        user_id: Uuid,
        password: &str,
    ) -> PasswordAuthResult<Credential> {
        self.validate_new_password(password).await?;

        let new_hash = self.hasher.hash_password(password.as_bytes())?;

        if let Some(existing) = self.repository.find_password_credential(user_id).await? {
            if self
                .verifier
                .verify_password(password.as_bytes(), &existing.secret_encrypted)?
            {
                return Err(PasswordAuthError::PasswordSameAsCurrent);
            }

            self.repository
                .update_password_credential(existing.id, &new_hash)
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

    /// Change a user's password.
    pub async fn change_password(
        &self,
        user_id: Uuid,
        current_password: &str,
        new_password: &str,
    ) -> PasswordAuthResult<()> {
        self.validate_new_password(new_password).await?;

        let credential = match self.repository.find_password_credential(user_id).await? {
            Some(c) => c,
            None => return Err(PasswordAuthError::CredentialNotFound),
        };

        let current_hash = &credential.secret_encrypted;
        let is_valid = self
            .verifier
            .verify_password(current_password.as_bytes(), current_hash)?;

        if !is_valid {
            return Err(PasswordAuthError::WrongPassword);
        }

        if self
            .verifier
            .verify_password(new_password.as_bytes(), current_hash)?
        {
            return Err(PasswordAuthError::PasswordSameAsCurrent);
        }

        let new_hash = self.hasher.hash_password(new_password.as_bytes())?;

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

        let new_hash = self.hasher.hash_password(new_password.as_bytes())?;

        self.repository
            .update_password_credential(credential.id, &new_hash)
            .await?;

        Ok(())
    }
}
