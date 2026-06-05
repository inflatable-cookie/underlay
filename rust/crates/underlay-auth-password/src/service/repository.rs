use async_trait::async_trait;
use underlay_auth::{Credential, User};
use underlay_core::Uuid;

use crate::errors::PasswordAuthResult;

/// Repository for user and credential operations.
#[async_trait]
pub trait PasswordAuthRepository: Send + Sync {
    /// Find a user by email.
    async fn find_user_by_email(&self, email: &str) -> PasswordAuthResult<Option<User>>;

    /// Find a user by ID.
    async fn find_user_by_id(&self, user_id: Uuid) -> PasswordAuthResult<Option<User>>;

    /// Find a password credential for a user.
    async fn find_password_credential(
        &self,
        user_id: Uuid,
    ) -> PasswordAuthResult<Option<Credential>>;

    /// Create a password credential for a user.
    async fn create_password_credential(
        &self,
        user_id: Uuid,
        password_hash: &str,
    ) -> PasswordAuthResult<Credential>;

    /// Update a password credential.
    async fn update_password_credential(
        &self,
        credential_id: Uuid,
        password_hash: &str,
    ) -> PasswordAuthResult<()>;

    /// Delete a password credential.
    async fn delete_password_credential(&self, credential_id: Uuid) -> PasswordAuthResult<()>;

    /// Record a failed login attempt.
    ///
    /// Implementations should increment failure counters and, when the threshold is reached,
    /// apply a lockout and return the remaining lockout seconds.
    async fn record_failed_login(
        &self,
        user_id: Uuid,
        max_failed_attempts: u32,
        lockout_duration_seconds: u64,
    ) -> PasswordAuthResult<FailedLoginAttempt>;

    /// Reset failed login attempts.
    async fn reset_failed_logins(&self, user_id: Uuid) -> PasswordAuthResult<()>;

    /// Get failed login count for a user.
    async fn get_failed_login_count(&self, user_id: Uuid) -> PasswordAuthResult<u32>;

    /// Returns remaining lockout seconds, or `None` if not locked out.
    async fn get_lockout_remaining_seconds(&self, user_id: Uuid)
        -> PasswordAuthResult<Option<u64>>;
}

/// Result of recording a failed login attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FailedLoginAttempt {
    pub count: u32,
    pub lockout_remaining_seconds: Option<u64>,
}
