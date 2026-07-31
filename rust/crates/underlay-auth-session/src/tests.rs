use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use async_trait::async_trait;
use underlay_auth::{AuthError, AuthResult};
use underlay_auth_jwt::{JwtConfig, JwtService};
use underlay_core::Uuid;

use crate::repository::{AccountProvider, AccountState, AccountStatus, SessionRepository};
use crate::service::SessionService;
use crate::types::{SessionFingerprint, SessionRecord};

struct InMemoryRepo {
    sessions: Mutex<HashMap<Uuid, SessionRecord>>,
    revoke_calls: Mutex<Vec<(Uuid, String)>>,
}

impl InMemoryRepo {
    fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            revoke_calls: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl SessionRepository for InMemoryRepo {
    async fn get_session(&self, session_id: Uuid) -> AuthResult<Option<SessionRecord>> {
        Ok(self.sessions.lock().unwrap().get(&session_id).cloned())
    }

    async fn insert_session(&self, session: &SessionRecord) -> AuthResult<()> {
        self.sessions
            .lock()
            .unwrap()
            .insert(session.id, session.clone());
        Ok(())
    }

    async fn rotate_session_if_current(
        &self,
        session: &SessionRecord,
        expected_refresh_token_id: Uuid,
        expected_refresh_token_version: i32,
    ) -> AuthResult<bool> {
        let mut sessions = self.sessions.lock().unwrap();
        let Some(current) = sessions.get(&session.id) else {
            return Ok(false);
        };
        if current.refresh_token_id != expected_refresh_token_id
            || current.refresh_token_version != expected_refresh_token_version
        {
            return Ok(false);
        }
        sessions.insert(session.id, session.clone());
        Ok(true)
    }

    async fn revoke_session(&self, session_id: Uuid, reason: &str) -> AuthResult<()> {
        self.revoke_calls
            .lock()
            .unwrap()
            .push((session_id, reason.to_string()));
        if let Some(session) = self.sessions.lock().unwrap().get_mut(&session_id) {
            session.is_active = false;
        }
        Ok(())
    }

    async fn list_sessions_for_user(&self, user_id: Uuid) -> AuthResult<Vec<SessionRecord>> {
        Ok(self
            .sessions
            .lock()
            .unwrap()
            .values()
            .filter(|s| s.user_id == user_id)
            .cloned()
            .collect())
    }

    async fn revoke_all_sessions_for_user(&self, user_id: Uuid, reason: &str) -> AuthResult<u64> {
        let mut count = 0;
        for session in self.sessions.lock().unwrap().values_mut() {
            if session.user_id == user_id && session.is_active {
                session.is_active = false;
                count += 1;
                self.revoke_calls
                    .lock()
                    .unwrap()
                    .push((session.id, reason.to_string()));
            }
        }
        Ok(count)
    }
}

struct StaticAccounts {
    status: AccountStatus,
    roles: Vec<String>,
}

#[async_trait]
impl AccountProvider for StaticAccounts {
    async fn account_state(&self, _user_id: Uuid) -> AuthResult<Option<AccountState>> {
        Ok(Some(AccountState {
            status: self.status,
            roles: self.roles.clone(),
        }))
    }
}

fn service_with(
    status: AccountStatus,
    roles: &[&str],
) -> (SessionService<InMemoryRepo, StaticAccounts>, JwtService) {
    let (config, _) = JwtConfig::generate().unwrap();
    let jwt = JwtService::new(config).unwrap();
    let accounts = StaticAccounts {
        status,
        roles: roles.iter().map(|s| s.to_string()).collect(),
    };
    (
        SessionService::new(jwt.clone(), InMemoryRepo::new(), accounts),
        jwt,
    )
}

#[tokio::test]
async fn refresh_rotates_and_applies_fresh_roles() {
    let (service, _jwt) = service_with(AccountStatus::Active, &["admin"]);
    let user_id = Uuid::new_v7();
    let (tokens, session) = service
        .create_session(user_id, vec!["user".to_string()], None)
        .await
        .unwrap();

    let outcome = service.refresh(&tokens.refresh_token, None).await.unwrap();

    assert_eq!(outcome.session.id, session.id);
    assert_eq!(outcome.roles, vec!["admin".to_string()]);
    assert_eq!(outcome.session.roles, vec!["admin".to_string()]);
    assert_ne!(outcome.tokens.refresh_token, tokens.refresh_token);
    assert!(outcome.session.refresh_token_version > 1);
}

#[tokio::test]
async fn refresh_rejects_suspended_account() {
    let (service, _jwt) = service_with(AccountStatus::Suspended, &["user"]);
    let user_id = Uuid::new_v7();
    let (tokens, _) = service
        .create_session(user_id, vec!["user".to_string()], None)
        .await
        .unwrap();

    let err = service
        .refresh(&tokens.refresh_token, None)
        .await
        .unwrap_err();
    assert!(matches!(err, AuthError::AccountSuspended));
}

#[tokio::test]
async fn refresh_rejects_deleted_account() {
    let (service, _jwt) = service_with(AccountStatus::Deleted, &["user"]);
    let user_id = Uuid::new_v7();
    let (tokens, _) = service
        .create_session(user_id, vec!["user".to_string()], None)
        .await
        .unwrap();

    let err = service
        .refresh(&tokens.refresh_token, None)
        .await
        .unwrap_err();
    assert!(matches!(err, AuthError::AccountDeleted));
}

#[tokio::test]
async fn replayed_refresh_token_revokes_family() {
    let (service, _jwt) = service_with(AccountStatus::Active, &["user"]);
    let user_id = Uuid::new_v7();
    let (tokens, session) = service
        .create_session(user_id, vec!["user".to_string()], None)
        .await
        .unwrap();

    // First refresh wins and rotates.
    service.refresh(&tokens.refresh_token, None).await.unwrap();

    // Replaying the original token is reuse: rejected and family revoked.
    let err = service
        .refresh(&tokens.refresh_token, None)
        .await
        .unwrap_err();
    assert!(
        matches!(
            err,
            AuthError::TokenInvalid | AuthError::TokenFingerprintMismatch
        ),
        "expected reuse detection, got {err:?}"
    );

    // The session must now be revoked entirely.
    let repo_sessions = service.list_sessions_for_user(user_id).await.unwrap();
    assert!(
        repo_sessions.iter().all(|s| !s.is_active),
        "family not revoked after reuse"
    );
    let _ = session;
}

#[tokio::test]
async fn absolute_timeout_revokes_session() {
    let (mut service, _jwt) = service_with(AccountStatus::Active, &["user"]);
    service = service.with_config(
        crate::SessionServiceConfig::default()
            .with_absolute_session_timeout(Duration::from_secs(0)),
    );
    let user_id = Uuid::new_v7();
    let (tokens, _) = service
        .create_session(user_id, vec!["user".to_string()], None)
        .await
        .unwrap();

    let err = service
        .refresh(&tokens.refresh_token, None)
        .await
        .unwrap_err();
    assert!(matches!(err, AuthError::SessionExpired));
}

#[tokio::test]
async fn strict_fingerprint_mode_rejects_mismatch() {
    let (mut service, _jwt) = service_with(AccountStatus::Active, &["user"]);
    service = service
        .with_config(crate::SessionServiceConfig::default().with_refresh_fingerprint_strict(true));
    let user_id = Uuid::new_v7();
    let (tokens, _) = service
        .create_session(
            user_id,
            vec!["user".to_string()],
            Some(SessionFingerprint::new(
                Some("10.0.0.1".to_string()),
                Some("agent-a".to_string()),
            )),
        )
        .await
        .unwrap();

    let err = service
        .refresh(
            &tokens.refresh_token,
            Some(SessionFingerprint::new(
                Some("10.0.0.2".to_string()),
                Some("agent-a".to_string()),
            )),
        )
        .await
        .unwrap_err();
    assert!(matches!(err, AuthError::TokenFingerprintMismatch));
}

#[tokio::test]
async fn advisory_fingerprint_mode_allows_mismatch() {
    let (service, _jwt) = service_with(AccountStatus::Active, &["user"]);
    let user_id = Uuid::new_v7();
    let (tokens, _) = service
        .create_session(
            user_id,
            vec!["user".to_string()],
            Some(SessionFingerprint::new(Some("10.0.0.1".to_string()), None)),
        )
        .await
        .unwrap();

    let outcome = service
        .refresh(
            &tokens.refresh_token,
            Some(SessionFingerprint::new(Some("10.0.0.2".to_string()), None)),
        )
        .await;
    assert!(outcome.is_ok());
}

#[tokio::test]
async fn access_principal_requires_current_fingerprint() {
    let (service, _jwt) = service_with(AccountStatus::Active, &["user"]);
    let user_id = Uuid::new_v7();
    let (tokens, _) = service
        .create_session(user_id, vec!["user".to_string()], None)
        .await
        .unwrap();

    let (subject, roles) = service
        .verify_access_principal(&tokens.access_token)
        .await
        .unwrap();
    assert_eq!(subject, user_id);
    assert_eq!(roles, vec!["user".to_string()]);

    // After rotation the old access token's fingerprint is stale.
    service.refresh(&tokens.refresh_token, None).await.unwrap();
    let err = service
        .verify_access_principal(&tokens.access_token)
        .await
        .unwrap_err();
    assert!(matches!(err, AuthError::TokenFingerprintMismatch));
}

#[tokio::test]
async fn revoke_session_for_user_enforces_ownership() {
    let (service, _jwt) = service_with(AccountStatus::Active, &["user"]);
    let user_id = Uuid::new_v7();
    let other_id = Uuid::new_v7();
    let (_, session) = service
        .create_session(user_id, vec!["user".to_string()], None)
        .await
        .unwrap();

    let err = service
        .revoke_session_for_user(other_id, session.id, "test")
        .await
        .unwrap_err();
    assert!(matches!(err, AuthError::Forbidden));

    service
        .revoke_session_for_user(user_id, session.id, "test")
        .await
        .unwrap();
    assert!(service
        .list_sessions_for_user(user_id)
        .await
        .unwrap()
        .iter()
        .all(|s| !s.is_active));
}
