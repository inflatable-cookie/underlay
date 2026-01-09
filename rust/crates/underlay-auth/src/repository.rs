//! Repository traits for auth operations.
//!
//! Underlay defines the trait interfaces; applications implement them
//! with their specific database access patterns (sqlx, diesel, etc.).

use async_trait::async_trait;

use underlay_core::Uuid;

use crate::types::{Credential, CredentialMetadata, CredentialType, Session, User, UserStatus};
use crate::{AuthError, AuthEvent, AuthEventType};

/// Arguments for creating a new session.
#[derive(Debug, Clone)]
pub struct NewSession<'a> {
    /// The user ID for this session.
    pub user_id: Uuid,
    /// Fingerprint of the access token.
    pub access_token_fingerprint: &'a str,
    /// Fingerprint of the refresh token.
    pub refresh_token_fingerprint: &'a str,
    /// When the access token expires.
    pub access_token_expires_at: chrono::DateTime<chrono::Utc>,
    /// When the refresh token expires.
    pub refresh_token_expires_at: chrono::DateTime<chrono::Utc>,
    /// Client IP address.
    pub ip_address: Option<&'a str>,
    /// Client user agent.
    pub user_agent: Option<&'a str>,
}

/// Result type for repository operations.
pub type RepoResult<T> = Result<T, AuthError>;

/// Repository trait for user operations.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Find a user by ID.
    async fn find_by_id(&self, user_id: Uuid) -> RepoResult<Option<User>>;

    /// Find a user by email.
    async fn find_by_email(&self, email: &str) -> RepoResult<Option<User>>;

    /// Create a new user.
    async fn create(&self, email: &str, display_name: &str) -> RepoResult<User>;

    /// Update a user's status.
    async fn update_status(&self, user_id: Uuid, status: UserStatus) -> RepoResult<()>;

    /// Delete a user (soft delete).
    async fn delete(&self, user_id: Uuid) -> RepoResult<()>;

    /// Check if email is available.
    async fn email_available(&self, email: &str) -> RepoResult<bool>;
}

/// Repository trait for credential operations.
#[async_trait]
pub trait CredentialRepository: Send + Sync {
    /// Find a credential by ID.
    async fn find_by_id(&self, credential_id: Uuid) -> RepoResult<Option<Credential>>;

    /// Find all credentials for a user.
    async fn find_by_user_id(&self, user_id: Uuid) -> RepoResult<Vec<Credential>>;

    /// Find a password credential for a user.
    async fn find_password_by_user_id(&self, user_id: Uuid) -> RepoResult<Option<Credential>>;

    /// Find a TOTP credential for a user.
    async fn find_totp_by_user_id(&self, user_id: Uuid) -> RepoResult<Option<Credential>>;

    /// Find PassKey credentials for a user.
    async fn find_passkeys_by_user_id(&self, user_id: Uuid) -> RepoResult<Vec<Credential>>;

    /// Find OAuth connection for a user and provider.
    async fn find_oauth_by_user_and_provider(
        &self,
        user_id: Uuid,
        provider: CredentialType,
    ) -> RepoResult<Option<Credential>>;

    /// Create a new credential.
    async fn create(
        &self,
        user_id: Uuid,
        credential_type: CredentialType,
        secret_encrypted: &str,
        metadata: &CredentialMetadata,
    ) -> RepoResult<Credential>;

    /// Update a credential's verified status.
    async fn set_verified(&self, credential_id: Uuid, verified: bool) -> RepoResult<()>;

    /// Update last used timestamp.
    async fn update_last_used(&self, credential_id: Uuid) -> RepoResult<()>;

    /// Delete a credential.
    async fn delete(&self, credential_id: Uuid) -> RepoResult<()>;

    /// Delete all credentials for a user.
    async fn delete_all_for_user(&self, user_id: Uuid) -> RepoResult<()>;
}

/// Repository trait for session operations.
#[async_trait]
pub trait SessionRepository: Send + Sync {
    /// Find a session by ID.
    async fn find_by_id(&self, session_id: Uuid) -> RepoResult<Option<Session>>;

    /// Find a session by access token fingerprint.
    async fn find_by_access_fingerprint(&self, fingerprint: &str) -> RepoResult<Option<Session>>;

    /// Find a session by refresh token fingerprint.
    async fn find_by_refresh_fingerprint(&self, fingerprint: &str) -> RepoResult<Option<Session>>;

    /// Find all active sessions for a user.
    async fn find_active_by_user_id(&self, user_id: Uuid) -> RepoResult<Vec<Session>>;

    /// Create a new session.
    async fn create(&self, session: NewSession<'_>) -> RepoResult<Session>;

    /// Update last used timestamp.
    async fn update_last_used(&self, session_id: Uuid) -> RepoResult<()>;

    /// Refresh a session (rotate tokens).
    async fn refresh(
        &self,
        session_id: Uuid,
        new_access_fingerprint: &str,
        new_refresh_fingerprint: &str,
        new_access_expires_at: chrono::DateTime<chrono::Utc>,
        new_refresh_expires_at: chrono::DateTime<chrono::Utc>,
    ) -> RepoResult<Session>;

    /// Revoke a session.
    async fn revoke(&self, session_id: Uuid, reason: &str) -> RepoResult<()>;

    /// Revoke all sessions for a user.
    async fn revoke_all_for_user(
        &self,
        user_id: Uuid,
        reason: &str,
        exclude_session_id: Option<Uuid>,
    ) -> RepoResult<()>;

    /// Clean up expired sessions.
    async fn clean_expired(&self) -> RepoResult<u64>;
}

/// Repository trait for audit log operations.
#[async_trait]
pub trait AuditLogRepository: Send + Sync {
    /// Create an audit log entry.
    async fn create(&self, event: &AuthEvent) -> RepoResult<()>;

    /// Find audit log entries for a user.
    async fn find_by_user_id(&self, user_id: Uuid, limit: u64) -> RepoResult<Vec<AuthEvent>>;

    /// Find audit log entries by IP address.
    async fn find_by_ip_address(&self, ip_address: &str, limit: u64) -> RepoResult<Vec<AuthEvent>>;

    /// Find failed login attempts for a user.
    async fn find_failed_logins(
        &self,
        user_id: Uuid,
        since: chrono::DateTime<chrono::Utc>,
        limit: u64,
    ) -> RepoResult<Vec<AuthEvent>>;

    /// Count events by type for a user.
    async fn count_by_type_and_user(
        &self,
        event_type: AuthEventType,
        user_id: Uuid,
        since: chrono::DateTime<chrono::Utc>,
    ) -> RepoResult<u64>;
}

/// Repository trait for rate limiting.
#[async_trait]
pub trait RateLimitRepository: Send + Sync {
    /// Check if a key is rate limited.
    async fn is_rate_limited(
        &self,
        key: &str,
        max_count: u64,
        window_seconds: u64,
    ) -> RepoResult<(bool, u64)>;

    /// Increment a rate limit counter.
    async fn increment(&self, key: &str, window_seconds: u64) -> RepoResult<u64>;

    /// Reset a rate limit for a key.
    async fn reset(&self, key: &str) -> RepoResult<()>;
}

/// Composite repository for all auth operations.
///
/// Applications can implement this trait to provide a unified repository,
/// or implement individual traits separately.
#[async_trait]
pub trait AuthRepository:
    UserRepository
    + CredentialRepository
    + SessionRepository
    + AuditLogRepository
    + RateLimitRepository
    + Send
    + Sync
{
}

#[async_trait]
impl<
        T: UserRepository
            + CredentialRepository
            + SessionRepository
            + AuditLogRepository
            + RateLimitRepository
            + Send
            + Sync,
    > AuthRepository for T
{
}
