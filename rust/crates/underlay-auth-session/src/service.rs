use chrono::Utc;
use underlay_auth::{AuthError, AuthResult};
use underlay_auth_jwt::{token_fingerprint, JwtService};
use underlay_core::Uuid;

use crate::config::SessionServiceConfig;
use crate::repository::{AccountProvider, AccountStatus, SessionRepository};
use crate::types::{SessionFingerprint, SessionRecord, Tokens};

/// Result of a successful refresh: the rotated session and its new tokens.
#[derive(Debug, Clone)]
pub struct RefreshOutcome {
    pub session: SessionRecord,
    pub tokens: Tokens,
    /// Roles issued into the new access token (fresh from the account
    /// provider, not the pre-rotation session snapshot).
    pub roles: Vec<String>,
}

/// Canonical session service: create, refresh, logout, revoke.
///
/// Security properties are enforced by the flow itself and cannot be
/// skipped by callers; see the crate-level docs for the list.
#[derive(Clone)]
pub struct SessionService<R, A> {
    jwt: JwtService,
    repo: R,
    accounts: A,
    config: SessionServiceConfig,
}

impl<R, A> SessionService<R, A>
where
    R: SessionRepository,
    A: AccountProvider,
{
    pub fn new(jwt: JwtService, repo: R, accounts: A) -> Self {
        Self {
            jwt,
            repo,
            accounts,
            config: SessionServiceConfig::default(),
        }
    }

    pub fn with_config(mut self, config: SessionServiceConfig) -> Self {
        self.config = config;
        self
    }

    pub fn jwt(&self) -> &JwtService {
        &self.jwt
    }

    /// Create a new session for `user_id` and return the first token pair.
    pub async fn create_session(
        &self,
        user_id: Uuid,
        roles: Vec<String>,
        fingerprint: Option<SessionFingerprint>,
    ) -> AuthResult<(Tokens, SessionRecord)> {
        let session_id = Uuid::new_v7();
        let fingerprint = fingerprint.unwrap_or_default();

        let (access_token, access_claims) = self
            .jwt
            .issue_access_token(user_id, session_id, roles.clone())
            .map_err(AuthError::from)?;
        let (refresh_token, refresh_claims) = self
            .jwt
            .issue_refresh_token(user_id, session_id, None, 1)
            .map_err(AuthError::from)?;

        let now = Utc::now();
        let record = SessionRecord {
            id: session_id,
            user_id,
            roles,
            is_active: true,
            access_token_fingerprint: token_fingerprint(&access_token),
            refresh_token_fingerprint: token_fingerprint(&refresh_token),
            refresh_token_id: refresh_claims.common.token_id,
            refresh_token_version: refresh_claims.version as i32,
            access_token_expires_at: timestamp_to_datetime(access_claims.common.expires_at),
            refresh_token_expires_at: timestamp_to_datetime(refresh_claims.common.expires_at),
            created_at: now,
            updated_at: now,
            last_used_at: now,
            ip_address: fingerprint.ip_address,
            user_agent: fingerprint.user_agent,
            revoked_reason: None,
            revoked_at: None,
        };

        self.repo.insert_session(&record).await?;

        Ok((
            Tokens {
                access_token,
                refresh_token,
            },
            record,
        ))
    }

    /// Rotate a session via its refresh token.
    pub async fn refresh(
        &self,
        refresh_token: &str,
        current_fingerprint: Option<SessionFingerprint>,
    ) -> AuthResult<RefreshOutcome> {
        let claims = self
            .jwt
            .verify_refresh_token(refresh_token)
            .map_err(AuthError::from)?;

        let mut session = self
            .repo
            .get_session(claims.session_id)
            .await?
            .ok_or(AuthError::SessionRevoked)?;

        if !session.is_active {
            return Err(AuthError::SessionRevoked);
        }

        // Reuse detection. A refresh token that verified and targets this
        // active session but is not its current token — stale fingerprint,
        // or a superseded id/version — is a replayed or stolen token.
        // Revoke the whole session family so neither the stolen token nor
        // the legitimate current one can be used again. The legitimate
        // concurrent-refresh race never reaches here: both racers read the
        // same current id/version and pass these checks; the loser is
        // caught by the atomic CAS below, not revoked.
        let expected_version =
            i32::try_from(claims.version).map_err(|_| AuthError::TokenInvalid)?;

        if session.refresh_token_fingerprint != token_fingerprint(refresh_token) {
            self.revoke_family_on_reuse(session.id).await;
            return Err(AuthError::TokenFingerprintMismatch);
        }

        if session.refresh_token_id != claims.common.token_id
            || session.refresh_token_version != expected_version
        {
            self.revoke_family_on_reuse(session.id).await;
            return Err(AuthError::TokenInvalid);
        }

        // Absolute session lifetime cap.
        let session_age = Utc::now() - session.created_at;
        if session_age
            > chrono::Duration::from_std(self.config.absolute_session_timeout)
                .unwrap_or(chrono::Duration::days(30))
        {
            tracing::info!(
                session_id = %session.id,
                user_id = %session.user_id,
                session_age_days = session_age.num_days(),
                "session exceeded absolute timeout, revoking"
            );
            self.repo
                .revoke_session(session.id, "absolute_timeout")
                .await?;
            return Err(AuthError::SessionExpired);
        }

        // Client fingerprint validation (advisory by default).
        if let Some(ref current) = current_fingerprint {
            if let Some(mismatch) = session.fingerprint().mismatch_description(current) {
                tracing::warn!(
                    session_id = %session.id,
                    user_id = %session.user_id,
                    %mismatch,
                    strict = self.config.refresh_fingerprint_strict,
                    "session fingerprint mismatch on token refresh"
                );
                if self.config.refresh_fingerprint_strict {
                    return Err(AuthError::TokenFingerprintMismatch);
                }
            }
        }

        // Account re-check on every refresh: a suspended/deleted account
        // must not keep a session alive, and role changes take effect on
        // rotation instead of waiting for re-login.
        let account = self
            .accounts
            .account_state(session.user_id)
            .await?
            .ok_or(AuthError::UserNotFound)?;
        match account.status {
            AccountStatus::Active => {}
            AccountStatus::Suspended => return Err(AuthError::AccountSuspended),
            AccountStatus::Deleted => return Err(AuthError::AccountDeleted),
        }
        session.roles = account.roles.clone();

        // Issue the new token pair.
        let (new_access_token, access_claims) = self
            .jwt
            .issue_access_token(session.user_id, session.id, account.roles.clone())
            .map_err(AuthError::from)?;
        let (new_refresh_token, refresh_claims) = self
            .jwt
            .issue_refresh_token(
                session.user_id,
                session.id,
                Some(session.refresh_token_id),
                claims.version + 1,
            )
            .map_err(AuthError::from)?;

        session.access_token_fingerprint = token_fingerprint(&new_access_token);
        session.refresh_token_fingerprint = token_fingerprint(&new_refresh_token);
        session.refresh_token_id = refresh_claims.common.token_id;
        session.refresh_token_version =
            i32::try_from(refresh_claims.version).map_err(|_| AuthError::TokenInvalid)?;
        session.access_token_expires_at = timestamp_to_datetime(access_claims.common.expires_at);
        session.refresh_token_expires_at = timestamp_to_datetime(refresh_claims.common.expires_at);
        session.updated_at = Utc::now();
        session.last_used_at = Utc::now();

        // Track fingerprint migration when provided.
        if let Some(fp) = current_fingerprint {
            if fp.ip_address.is_some() {
                session.ip_address = fp.ip_address;
            }
            if fp.user_agent.is_some() {
                session.user_agent = fp.user_agent;
            }
        }

        // Atomic compare-and-swap rotation: writes the new token state only
        // if the row still carries the id/version we read. Losing the swap
        // means a concurrent refresh rotated first — a legitimate
        // double-submit race, not reuse — so reject without revoking the
        // family; the client retries with the freshly issued token.
        let rotated = self
            .repo
            .rotate_session_if_current(&session, claims.common.token_id, expected_version)
            .await?;
        if !rotated {
            return Err(AuthError::TokenInvalid);
        }

        Ok(RefreshOutcome {
            session,
            tokens: Tokens {
                access_token: new_access_token,
                refresh_token: new_refresh_token,
            },
            roles: account.roles,
        })
    }

    /// Verify an access token against the live session (fingerprint-bound).
    pub async fn verify_access_principal(
        &self,
        access_token: &str,
    ) -> AuthResult<(Uuid, Vec<String>)> {
        let claims = self
            .jwt
            .verify_access_token(access_token)
            .map_err(AuthError::from)?;

        let session = self
            .repo
            .get_session(claims.session_id)
            .await?
            .ok_or(AuthError::SessionRevoked)?;

        if !session.is_active {
            return Err(AuthError::SessionRevoked);
        }

        if session.access_token_fingerprint != token_fingerprint(access_token) {
            return Err(AuthError::TokenFingerprintMismatch);
        }

        Ok((claims.common.subject, claims.roles))
    }

    /// Revoke the session the refresh token belongs to.
    pub async fn logout(&self, refresh_token: &str) -> AuthResult<()> {
        let claims = self
            .jwt
            .verify_refresh_token(refresh_token)
            .map_err(AuthError::from)?;
        self.repo.revoke_session(claims.session_id, "logout").await
    }

    pub async fn list_sessions_for_user(&self, user_id: Uuid) -> AuthResult<Vec<SessionRecord>> {
        self.repo.list_sessions_for_user(user_id).await
    }

    /// Revoke a specific session, only if it belongs to `user_id`.
    pub async fn revoke_session_for_user(
        &self,
        user_id: Uuid,
        session_id: Uuid,
        reason: &str,
    ) -> AuthResult<()> {
        let session = self.repo.get_session(session_id).await?;

        match session {
            Some(s) if s.user_id == user_id => {
                self.repo.revoke_session(session_id, reason).await
            }
            Some(_) => Err(AuthError::Forbidden),
            None => Err(AuthError::BadRequest("Session not found".into())),
        }
    }

    /// Revoke all active sessions for a user.
    pub async fn revoke_all_sessions_for_user(&self, user_id: Uuid, reason: &str) -> AuthResult<u64> {
        self.repo.revoke_all_sessions_for_user(user_id, reason).await
    }

    /// Revoke a whole session family after refresh-token reuse detection.
    /// Best-effort: a revoke failure is logged but does not mask the auth
    /// error returned to the caller (the reused token is rejected
    /// regardless).
    async fn revoke_family_on_reuse(&self, session_id: Uuid) {
        if let Err(e) = self
            .repo
            .revoke_session(session_id, "refresh_reuse_detected")
            .await
        {
            tracing::error!(
                %session_id,
                "failed to revoke session family after refresh reuse detection: {e:?}"
            );
        }
    }
}

fn timestamp_to_datetime(ts: u64) -> chrono::DateTime<Utc> {
    chrono::DateTime::from_timestamp(ts as i64, 0).unwrap_or_else(Utc::now)
}
