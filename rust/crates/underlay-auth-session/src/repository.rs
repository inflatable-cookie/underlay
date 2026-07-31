use async_trait::async_trait;
use underlay_auth::AuthResult;
use underlay_core::Uuid;

use crate::types::SessionRecord;

/// Account status as enforced on refresh. Unknown statuses must map to a
/// non-active variant (fail closed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountStatus {
    Active,
    Suspended,
    Deleted,
}

/// Account state re-checked on every refresh.
#[derive(Debug, Clone)]
pub struct AccountState {
    pub status: AccountStatus,
    /// Roles issued into the next access token, resolved fresh from the
    /// account store (not from the session snapshot).
    pub roles: Vec<String>,
}

/// User/account lookup used by the refresh path.
#[async_trait]
pub trait AccountProvider: Send + Sync {
    /// Return the account's current status and roles, or `None` if the
    /// account no longer exists.
    async fn account_state(&self, user_id: Uuid) -> AuthResult<Option<AccountState>>;
}

/// Persistence for session records. Implementations map their own schema
/// (table/column names, JSON shapes) onto [`SessionRecord`].
#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn get_session(&self, session_id: Uuid) -> AuthResult<Option<SessionRecord>>;

    async fn insert_session(&self, session: &SessionRecord) -> AuthResult<()>;

    /// Atomically rotate to `session`'s new token state, but only while the
    /// stored refresh lineage still matches `expected_refresh_token_id` and
    /// `expected_refresh_token_version`. Returns `true` if this call won the
    /// rotation; `false` if a concurrent refresh rotated first (a benign
    /// double-submit race, not reuse).
    async fn rotate_session_if_current(
        &self,
        session: &SessionRecord,
        expected_refresh_token_id: Uuid,
        expected_refresh_token_version: i32,
    ) -> AuthResult<bool>;

    async fn revoke_session(&self, session_id: Uuid, reason: &str) -> AuthResult<()>;

    async fn list_sessions_for_user(&self, user_id: Uuid) -> AuthResult<Vec<SessionRecord>>;

    /// Revoke all active sessions for a user. Returns rows affected.
    async fn revoke_all_sessions_for_user(&self, user_id: Uuid, reason: &str) -> AuthResult<u64>;
}
