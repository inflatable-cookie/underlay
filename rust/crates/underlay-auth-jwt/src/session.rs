//! Session management built on top of JWT issuance.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use underlay_core::Uuid;

use crate::claims::AccessTokenClaims;
use crate::error::{JwtError, JwtResult};
use crate::fingerprint::token_fingerprint;
use crate::service::JwtService;

/// Session state stored in application persistence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub id: Uuid,
    pub user_id: Uuid,
    pub roles: Vec<String>,

    pub is_active: bool,

    pub access_token_fingerprint: String,
    pub refresh_token_fingerprint: String,

    pub refresh_token_id: Uuid,
    pub refresh_token_version: u32,
}

#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn get_session(&self, session_id: &Uuid) -> JwtResult<Option<SessionState>>;
    async fn create_session(&self, session: &SessionState) -> JwtResult<()>;
    async fn update_session(&self, session: &SessionState) -> JwtResult<()>;
    async fn rotate_session_if_current(
        &self,
        session: &SessionState,
        expected_refresh_token_fingerprint: &str,
        expected_refresh_token_id: Uuid,
        expected_refresh_token_version: u32,
    ) -> JwtResult<bool>;
    async fn delete_session(&self, session_id: &Uuid) -> JwtResult<()>;
    async fn revoke_all_user_sessions(&self, user_id: &Uuid) -> JwtResult<u64>;
    async fn get_user_sessions(&self, user_id: &Uuid) -> JwtResult<Vec<SessionState>>;
}

#[derive(Debug, Clone)]
pub struct SessionTokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone)]
pub struct SessionManager<S: SessionStore> {
    jwt: JwtService,
    store: Arc<S>,
}

impl<S: SessionStore> SessionManager<S> {
    pub fn new(jwt: JwtService, store: Arc<S>) -> Self {
        Self { jwt, store }
    }

    pub async fn create_session(
        &self,
        user_id: Uuid,
        roles: Vec<String>,
    ) -> JwtResult<SessionTokens> {
        let session_id = Uuid::new_v7();
        let (access_token, _access_claims) =
            self.jwt
                .issue_access_token(user_id, session_id, roles.clone())?;

        let (refresh_token, refresh_claims) =
            self.jwt.issue_refresh_token(user_id, session_id, None, 1)?;

        let session = SessionState {
            id: session_id,
            user_id,
            roles,
            is_active: true,
            access_token_fingerprint: token_fingerprint(&access_token),
            refresh_token_fingerprint: token_fingerprint(&refresh_token),
            refresh_token_id: refresh_claims.common.token_id,
            refresh_token_version: refresh_claims.version,
        };

        self.store.create_session(&session).await?;

        Ok(SessionTokens {
            access_token,
            refresh_token,
        })
    }

    pub async fn verify_access_token(&self, token: &str) -> JwtResult<AccessTokenClaims> {
        let claims = self.jwt.verify_access_token(token)?;

        let session = self
            .store
            .get_session(&claims.session_id)
            .await?
            .ok_or(JwtError::SessionRevoked)?;

        if !session.is_active {
            return Err(JwtError::SessionRevoked);
        }

        if session.access_token_fingerprint != token_fingerprint(token) {
            return Err(JwtError::TokenFingerprintMismatch);
        }

        Ok(claims)
    }

    pub async fn refresh_session(&self, refresh_token: &str) -> JwtResult<SessionTokens> {
        let claims = self.jwt.verify_refresh_token(refresh_token)?;

        let mut session = self
            .store
            .get_session(&claims.session_id)
            .await?
            .ok_or(JwtError::SessionRevoked)?;

        if !session.is_active {
            return Err(JwtError::SessionRevoked);
        }

        if session.refresh_token_fingerprint != token_fingerprint(refresh_token) {
            return Err(JwtError::TokenFingerprintMismatch);
        }

        if session.refresh_token_id != claims.common.token_id {
            return Err(JwtError::RefreshReplayDetected);
        }

        if session.refresh_token_version != claims.version {
            return Err(JwtError::RefreshReplayDetected);
        }

        let (new_access_token, _access_claims) =
            self.jwt
                .issue_access_token(session.user_id, session.id, session.roles.clone())?;

        let (new_refresh_token, new_refresh_claims) = self.jwt.issue_refresh_token(
            session.user_id,
            session.id,
            Some(claims.common.token_id),
            claims.version + 1,
        )?;

        session.access_token_fingerprint = token_fingerprint(&new_access_token);
        session.refresh_token_fingerprint = token_fingerprint(&new_refresh_token);
        session.refresh_token_id = new_refresh_claims.common.token_id;
        session.refresh_token_version = new_refresh_claims.version;

        let rotated = self
            .store
            .rotate_session_if_current(
                &session,
                &token_fingerprint(refresh_token),
                claims.common.token_id,
                claims.version,
            )
            .await?;
        if !rotated {
            return Err(JwtError::RefreshReplayDetected);
        }

        Ok(SessionTokens {
            access_token: new_access_token,
            refresh_token: new_refresh_token,
        })
    }

    pub async fn revoke_session(&self, session_id: &Uuid) -> JwtResult<()> {
        self.store.delete_session(session_id).await
    }

    pub async fn revoke_all_user_sessions(&self, user_id: &Uuid) -> JwtResult<u64> {
        self.store.revoke_all_user_sessions(user_id).await
    }

    pub async fn get_user_sessions(&self, user_id: &Uuid) -> JwtResult<Vec<SessionState>> {
        self.store.get_user_sessions(user_id).await
    }
}
