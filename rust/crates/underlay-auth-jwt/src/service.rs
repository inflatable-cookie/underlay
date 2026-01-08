//! JWT issuance + session management.

use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use underlay_core::Uuid;

use crate::claims::{AccessTokenClaims, CommonClaims, RefreshTokenClaims, TokenUse};
use crate::config::JwtConfig;
use crate::error::{JwtError, JwtResult};
use crate::fingerprint::token_fingerprint;

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    config: JwtConfig,
}

impl std::fmt::Debug for JwtService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtService")
            .field("encoding_key", &"[REDACTED]")
            .field("decoding_key", &"[REDACTED]")
            .field("config", &self.config)
            .finish()
    }
}

impl JwtService {
    pub fn new(config: JwtConfig) -> JwtResult<Self> {
        let private_key_der = STANDARD
            .decode(&config.private_key_b64)
            .map_err(|_| JwtError::Key("Invalid base64 private key".to_string()))?;
        let encoding_key = EncodingKey::from_ed_der(&private_key_der);

        let public_key_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(&config.public_key_b64)
            .or_else(|_| base64::engine::general_purpose::STANDARD.decode(&config.public_key_b64))
            .map_err(|e| JwtError::Key(format!("Base64 error: {}", e)))?;
        let public_key_b64url = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(public_key_bytes);

        let decoding_key = DecodingKey::from_ed_components(&public_key_b64url)
            .map_err(|e| JwtError::Key(e.to_string()))?;

        let service = Self {
            encoding_key,
            decoding_key,
            config,
        };

        // Fail fast if the keypair is incompatible (startup-time cost only).
        service.validate_keypair_compatibility()?;

        Ok(service)
    }

    pub fn config(&self) -> &JwtConfig {
        &self.config
    }

    fn validate_keypair_compatibility(&self) -> JwtResult<()> {
        // Issue and verify a short-lived token using the currently configured
        // issuer/audience. This is startup-only and prevents silent key mismatch.
        let session_id = Uuid::new_v7();
        let user_id = Uuid::new_v7();

        let (token, _claims) = self.issue_access_token(user_id, session_id, vec![])?;

        // If this fails, either the public key doesn't match the private key,
        // or the validation config is inconsistent.
        self.verify_access_token(&token).map(|_| ())
    }

    pub fn issue_access_token(
        &self,
        user_id: Uuid,
        session_id: Uuid,
        roles: Vec<String>,
    ) -> JwtResult<(String, AccessTokenClaims)> {
        let now = Utc::now().timestamp() as u64;
        let exp = (Utc::now() + self.config.access_token_lifetime())
            .timestamp() as u64;

        let claims = AccessTokenClaims {
            common: CommonClaims {
                issuer: self.config.issuer.clone(),
                subject: user_id,
                audience: self.config.audience.clone(),
                issued_at: now,
                expires_at: exp,
                not_before: Some(now),
                token_id: Uuid::new_v7(),
            },
            token_use: TokenUse::Access,
            session_id,
            roles,
        };

        let mut header = Header::new(Algorithm::EdDSA);
        header.typ = Some("JWT".to_string());

        let token = encode(&header, &claims, &self.encoding_key)
            .map_err(|e| JwtError::Internal(e.to_string()))?;

        Ok((token, claims))
    }

    pub fn issue_refresh_token(
        &self,
        user_id: Uuid,
        session_id: Uuid,
        previous_token_id: Option<Uuid>,
        version: u32,
    ) -> JwtResult<(String, RefreshTokenClaims)> {
        let now = Utc::now().timestamp() as u64;
        let exp = (Utc::now() + self.config.refresh_token_lifetime())
            .timestamp() as u64;

        let claims = RefreshTokenClaims {
            common: CommonClaims {
                issuer: self.config.issuer.clone(),
                subject: user_id,
                audience: self.config.audience.clone(),
                issued_at: now,
                expires_at: exp,
                not_before: Some(now),
                token_id: Uuid::new_v7(),
            },
            token_use: TokenUse::Refresh,
            session_id,
            previous_token_id,
            version,
        };

        let mut header = Header::new(Algorithm::EdDSA);
        header.typ = Some("JWT".to_string());

        let token = encode(&header, &claims, &self.encoding_key)
            .map_err(|e| JwtError::Internal(e.to_string()))?;

        Ok((token, claims))
    }

    pub fn verify_access_token(&self, token: &str) -> JwtResult<AccessTokenClaims> {
        let claims: AccessTokenClaims = self.decode_and_validate(token)?;
        if claims.token_use != TokenUse::Access {
            return Err(JwtError::UnsupportedTokenType);
        }
        Ok(claims)
    }

    pub fn verify_refresh_token(&self, token: &str) -> JwtResult<RefreshTokenClaims> {
        let claims: RefreshTokenClaims = self.decode_and_validate(token)?;
        if claims.token_use != TokenUse::Refresh {
            return Err(JwtError::UnsupportedTokenType);
        }
        Ok(claims)
    }

    fn decode_and_validate<T: serde::de::DeserializeOwned>(&self, token: &str) -> JwtResult<T> {
        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.leeway = self.config.leeway_seconds;
        validation.validate_nbf = true;

        // Require core JWT claims we rely on.
        if self.config.audience.is_some() {
            validation.set_required_spec_claims(&["exp", "iss", "sub", "aud"]);
        } else {
            validation.set_required_spec_claims(&["exp", "iss", "sub"]);
        }

        validation.set_issuer(&[self.config.issuer.clone()]);
        if let Some(aud) = &self.config.audience {
            validation.set_audience(&[aud.clone()]);
        }

        match decode::<T>(token, &self.decoding_key, &validation) {
            Ok(data) => Ok(data.claims),
            Err(err) => {
                use jsonwebtoken::errors::ErrorKind;
                match err.kind() {
                    ErrorKind::ExpiredSignature => Err(JwtError::Expired),
                    ErrorKind::ImmatureSignature => Err(JwtError::NotYetValid),
                    ErrorKind::InvalidToken => Err(JwtError::InvalidToken),
                    ErrorKind::InvalidSignature => Err(JwtError::InvalidToken),
                    ErrorKind::InvalidIssuer => Err(JwtError::InvalidToken),
                    ErrorKind::InvalidAudience => Err(JwtError::InvalidToken),
                    ErrorKind::Json(_) => Err(JwtError::MalformedToken),
                    _ => Err(JwtError::InvalidToken),
                }
            }
        }
    }
}

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

    pub async fn create_session(&self, user_id: Uuid, roles: Vec<String>) -> JwtResult<SessionTokens> {
        let session_id = Uuid::new_v7();
        let (access_token, _access_claims) =
            self.jwt.issue_access_token(user_id, session_id, roles.clone())?;

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

        self.store.update_session(&session).await?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::JwtConfig;
    use std::collections::HashMap;
    use tokio::sync::Mutex;

    #[derive(Debug, Default)]
    struct MemoryStore {
        sessions: Mutex<HashMap<Uuid, SessionState>>,
    }

    #[async_trait]
    impl SessionStore for MemoryStore {
        async fn get_session(&self, session_id: &Uuid) -> JwtResult<Option<SessionState>> {
            Ok(self.sessions.lock().await.get(session_id).cloned())
        }

        async fn create_session(&self, session: &SessionState) -> JwtResult<()> {
            self.sessions.lock().await.insert(session.id, session.clone());
            Ok(())
        }

        async fn update_session(&self, session: &SessionState) -> JwtResult<()> {
            self.sessions.lock().await.insert(session.id, session.clone());
            Ok(())
        }

        async fn delete_session(&self, session_id: &Uuid) -> JwtResult<()> {
            self.sessions.lock().await.remove(session_id);
            Ok(())
        }

        async fn revoke_all_user_sessions(&self, user_id: &Uuid) -> JwtResult<u64> {
            let mut sessions = self.sessions.lock().await;
            let before = sessions.len();
            sessions.retain(|_, s| &s.user_id != user_id);
            Ok((before - sessions.len()) as u64)
        }

        async fn get_user_sessions(&self, user_id: &Uuid) -> JwtResult<Vec<SessionState>> {
            Ok(self
                .sessions
                .lock()
                .await
                .values()
                .filter(|s| &s.user_id == user_id)
                .cloned()
                .collect())
        }
    }

    #[tokio::test]
    async fn issues_verifies_refreshes_and_revokes() {
        let (config, _keys) = JwtConfig::generate().unwrap();
        let jwt = JwtService::new(config).unwrap();

        let store = Arc::new(MemoryStore::default());
        let manager = SessionManager::new(jwt, store.clone());

        let user_id = Uuid::new_v7();
        let tokens = manager
            .create_session(user_id, vec!["user".to_string()])
            .await
            .unwrap();

        let claims = manager.verify_access_token(&tokens.access_token).await.unwrap();
        assert_eq!(claims.common.subject, user_id);

        let refreshed = manager.refresh_session(&tokens.refresh_token).await.unwrap();
        assert_ne!(tokens.refresh_token, refreshed.refresh_token);

        // Refresh token replay should be rejected.
        assert!(matches!(
            manager.refresh_session(&tokens.refresh_token).await,
            Err(JwtError::RefreshReplayDetected) | Err(JwtError::TokenFingerprintMismatch)
        ));

        // Revoke session and ensure access token is rejected.
        manager.revoke_session(&claims.session_id).await.unwrap();
        assert!(matches!(
            manager.verify_access_token(&refreshed.access_token).await,
            Err(JwtError::SessionRevoked)
        ));
    }
}
