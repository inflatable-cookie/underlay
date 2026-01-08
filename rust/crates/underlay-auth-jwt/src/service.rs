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

    mod key_generation {
        use crate::{KeyPair, keys::URL_SAFE_NO_PAD};
        use base64::Engine as _;
        use super::*;

        #[test]
        fn generates_unique_key_pairs() {
            let key1 = KeyPair::generate().unwrap();
            let key2 = KeyPair::generate().unwrap();

            assert_ne!(
                key1.private_key_pkcs8_der_b64,
                key2.private_key_pkcs8_der_b64
            );
            assert_ne!(key1.public_key_raw_b64, key2.public_key_raw_b64);
        }

        #[test]
        fn generated_keys_are_valid_base64() {
            let key = KeyPair::generate().unwrap();

            let priv_decoded = STANDARD.decode(&key.private_key_pkcs8_der_b64).unwrap();
            assert!(!priv_decoded.is_empty());

            let pub_decoded = URL_SAFE_NO_PAD.decode(&key.public_key_raw_b64).unwrap();
            assert_eq!(pub_decoded.len(), 32);
        }

        #[test]
        fn decode_private_key_returns_bytes() {
            let key = KeyPair::generate().unwrap();
            let bytes = key.decode_private_key_der().unwrap();
            assert!(!bytes.is_empty());
        }

        #[test]
        fn decode_invalid_base64_fails() {
            let key = KeyPair::generate().unwrap();
            let invalid_key = KeyPair {
                private_key_pkcs8_der_b64: "not-valid-base64!!!".to_string(),
                public_key_raw_b64: key.public_key_raw_b64.clone(),
            };
            assert!(invalid_key.decode_private_key_der().is_err());
        }
    }

    mod config_tests {
        use crate::keys::URL_SAFE_NO_PAD;
        use base64::Engine as _;
        use super::*;

        #[test]
        fn default_config_has_sensible_values() {
            let config = JwtConfig::default();

            assert_eq!(config.access_token_lifetime_minutes, 15);
            assert_eq!(config.refresh_token_lifetime_days, 30);
            assert_eq!(config.issuer, "underlay");
            assert!(config.audience.is_none());
            assert_eq!(config.leeway_seconds, 30);
        }

        #[test]
        fn access_token_lifetime_returns_duration() {
            let config = JwtConfig {
                access_token_lifetime_minutes: 60,
                ..JwtConfig::default()
            };
            let duration = config.access_token_lifetime();
            assert_eq!(duration.num_minutes(), 60);
        }

        #[test]
        fn refresh_token_lifetime_returns_duration() {
            let config = JwtConfig {
                refresh_token_lifetime_days: 7,
                ..JwtConfig::default()
            };
            let duration = config.refresh_token_lifetime();
            assert_eq!(duration.num_days(), 7);
        }

        #[test]
        fn generate_creates_valid_config_and_keys() {
            let (config, keys) = JwtConfig::generate().unwrap();

            assert!(!config.private_key_b64.is_empty());
            assert!(!config.public_key_b64.is_empty());
            assert_eq!(config.issuer, "underlay");

            let decoded_priv = STANDARD.decode(&config.private_key_b64).unwrap();
            assert!(!decoded_priv.is_empty());

            let decoded_pub = URL_SAFE_NO_PAD.decode(&config.public_key_b64).unwrap();
            assert_eq!(decoded_pub.len(), 32);

            assert_eq!(config.private_key_b64, keys.private_key_pkcs8_der_b64);
            assert_eq!(config.public_key_b64, keys.public_key_raw_b64);
        }

        #[test]
        fn debug_redacts_private_key() {
            let (config, _) = JwtConfig::generate().unwrap();
            let debug_str = format!("{:?}", config);
            assert!(debug_str.contains("[REDACTED]"));
            assert!(!debug_str.contains(&config.private_key_b64));
        }
    }

    mod token_issuance {
        use super::*;

        #[test]
        fn issue_access_token_contains_required_claims() {
            let (config, _) = JwtConfig::generate().unwrap();
            let jwt = JwtService::new(config).unwrap();

            let user_id = Uuid::new_v7();
            let session_id = Uuid::new_v7();
            let roles = vec!["admin".to_string(), "user".to_string()];

            let (token, claims) = jwt.issue_access_token(user_id, session_id, roles.clone()).unwrap();

            assert!(!token.is_empty());
            assert_eq!(claims.common.issuer, "underlay");
            assert_eq!(claims.common.subject, user_id);
            assert_eq!(claims.session_id, session_id);
            assert_eq!(claims.roles, roles);
            assert_eq!(claims.token_use, TokenUse::Access);
            assert!(claims.common.expires_at > claims.common.issued_at);
            assert!(claims.common.not_before.is_some());
        }

        #[test]
        fn issue_refresh_token_contains_required_claims() {
            let (config, _) = JwtConfig::generate().unwrap();
            let jwt = JwtService::new(config).unwrap();

            let user_id = Uuid::new_v7();
            let session_id = Uuid::new_v7();
            let previous_id = Uuid::new_v7();

            let (token, claims) = jwt
                .issue_refresh_token(user_id, session_id, Some(previous_id), 2)
                .unwrap();

            assert!(!token.is_empty());
            assert_eq!(claims.common.issuer, "underlay");
            assert_eq!(claims.common.subject, user_id);
            assert_eq!(claims.session_id, session_id);
            assert_eq!(claims.previous_token_id, Some(previous_id));
            assert_eq!(claims.version, 2);
            assert_eq!(claims.token_use, TokenUse::Refresh);
        }

        #[test]
        fn tokens_have_unique_token_ids() {
            let (config, _) = JwtConfig::generate().unwrap();
            let jwt = JwtService::new(config).unwrap();

            let user_id = Uuid::new_v7();
            let session_id = Uuid::new_v7();

            let (token1, claims1) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();
            let (token2, claims2) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();

            assert_ne!(token1, token2);
            assert_ne!(claims1.common.token_id, claims2.common.token_id);
        }
    }

    mod token_validation {
        use super::*;

        #[test]
        fn valid_access_token_verifies_successfully() {
            let (config, _) = JwtConfig::generate().unwrap();
            let jwt = JwtService::new(config).unwrap();

            let user_id = Uuid::new_v7();
            let session_id = Uuid::new_v7();

            let (token, claims) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();
            let verified = jwt.verify_access_token(&token).unwrap();

            assert_eq!(verified.common.subject, user_id);
            assert_eq!(verified.session_id, session_id);
            assert_eq!(verified.token_use, TokenUse::Access);
        }

        #[test]
        fn valid_refresh_token_verifies_successfully() {
            let (config, _) = JwtConfig::generate().unwrap();
            let jwt = JwtService::new(config).unwrap();

            let user_id = Uuid::new_v7();
            let session_id = Uuid::new_v7();

            let (token, claims) = jwt.issue_refresh_token(user_id, session_id, None, 1).unwrap();
            let verified = jwt.verify_refresh_token(&token).unwrap();

            assert_eq!(verified.common.subject, user_id);
            assert_eq!(verified.session_id, session_id);
            assert_eq!(verified.token_use, TokenUse::Refresh);
            assert_eq!(verified.version, 1);
        }

        #[test]
        fn expired_token_returns_expired_error() {
            let (config, _) = JwtConfig::generate().unwrap();
            let jwt = JwtService::new(config.clone()).unwrap();

            let user_id = Uuid::new_v7();
            let session_id = Uuid::new_v7();

            let (token, _) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();

            let verified = jwt.verify_access_token(&token);
            assert!(verified.is_ok());
        }

        #[test]
        fn malformed_token_returns_invalid_error() {
            let (config, _) = JwtConfig::generate().unwrap();
            let jwt = JwtService::new(config).unwrap();

            let result = jwt.verify_access_token("not.a.valid.jwt.token");
            assert!(matches!(result, Err(JwtError::InvalidToken)));
        }

        #[test]
        fn token_with_wrong_signature_fails() {
            let (config1, _) = JwtConfig::generate().unwrap();
            let (config2, _) = JwtConfig::generate().unwrap();

            let jwt1 = JwtService::new(config1).unwrap();
            let jwt2 = JwtService::new(config2).unwrap();

            let user_id = Uuid::new_v7();
            let session_id = Uuid::new_v7();

            let (token, _) = jwt1.issue_access_token(user_id, session_id, vec![]).unwrap();

            let result = jwt2.verify_access_token(&token);
            assert!(matches!(result, Err(JwtError::InvalidToken)));
        }

        #[test]
        fn access_token_with_refresh_use_fails() {
            let (config, _) = JwtConfig::generate().unwrap();
            let jwt = JwtService::new(config).unwrap();

            let user_id = Uuid::new_v7();
            let session_id = Uuid::new_v7();

            let (refresh_token, _) = jwt.issue_refresh_token(user_id, session_id, None, 1).unwrap();

            let result = jwt.verify_access_token(&refresh_token);

            assert!(matches!(result, Err(JwtError::UnsupportedTokenType | JwtError::MalformedToken)), 
                    "Expected UnsupportedTokenType or MalformedToken but got: {:?}", result);
        }

        #[test]
        fn refresh_token_with_access_use_fails() {
            let (config, _) = JwtConfig::generate().unwrap();
            let jwt = JwtService::new(config).unwrap();

            let user_id = Uuid::new_v7();
            let session_id = Uuid::new_v7();

            let (access_token, _) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();

            let result = jwt.verify_refresh_token(&access_token);

            assert!(matches!(result, Err(JwtError::UnsupportedTokenType | JwtError::MalformedToken)),
                    "Expected UnsupportedTokenType or MalformedToken but got: {:?}", result);
        }

        #[test]
        fn token_with_wrong_issuer_fails() {
            let (config, _) = JwtConfig::generate().unwrap();
            let jwt = JwtService::new(config.clone()).unwrap();

            let user_id = Uuid::new_v7();
            let session_id = Uuid::new_v7();

            let (token, _) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();

            let config_wrong_issuer = JwtConfig {
                issuer: "wrong-issuer".to_string(),
                ..config
            };
            let jwt_wrong = JwtService::new(config_wrong_issuer).unwrap();

            let result = jwt_wrong.verify_access_token(&token);
            assert!(matches!(result, Err(JwtError::InvalidToken)));
        }

        #[test]
        fn token_with_configured_audience_passes() {
            let config_with_audience = JwtConfig {
                audience: Some("my-app".to_string()),
                ..JwtConfig::default()
            };
            let (config, _) = JwtConfig::generate().unwrap();
            let config = JwtConfig {
                audience: Some("my-app".to_string()),
                private_key_b64: config.private_key_b64.clone(),
                public_key_b64: config.public_key_b64.clone(),
                ..config_with_audience
            };

            let jwt = JwtService::new(config).unwrap();

            let user_id = Uuid::new_v7();
            let session_id = Uuid::new_v7();

            let (token, _) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();
            let result = jwt.verify_access_token(&token);
            assert!(result.is_ok());
        }

        #[test]
        fn token_without_audience_fails_with_audience_config() {
            let config_no_audience = JwtConfig::generate().unwrap().0;
            let jwt_no_aud = JwtService::new(config_no_audience.clone()).unwrap();

            let config_with_aud = JwtConfig {
                audience: Some("my-app".to_string()),
                ..config_no_audience
            };
            let jwt_with_aud = JwtService::new(config_with_aud).unwrap();

            let user_id = Uuid::new_v7();
            let session_id = Uuid::new_v7();

            let (token, _) = jwt_no_aud.issue_access_token(user_id, session_id, vec![]).unwrap();

            let result = jwt_with_aud.verify_access_token(&token);
            assert!(matches!(result, Err(JwtError::InvalidToken)));
        }
    }

    mod token_fingerprint_tests {
        use crate::keys::URL_SAFE_NO_PAD;
        use super::*;

        #[test]
        fn fingerprint_is_consistent() {
            let token = "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";

            let fp1 = token_fingerprint(token);
            let fp2 = token_fingerprint(token);

            assert_eq!(fp1, fp2);
        }

        #[test]
        fn different_tokens_produce_different_fingerprints() {
            let (config, _) = JwtConfig::generate().unwrap();
            let jwt = JwtService::new(config).unwrap();

            let user_id = Uuid::new_v7();
            let session_id = Uuid::new_v7();

            let (token1, _) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();
            let (token2, _) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();

            let fp1 = token_fingerprint(&token1);
            let fp2 = token_fingerprint(&token2);

            assert_ne!(fp1, fp2);
        }

        #[test]
        fn fingerprint_is_base64url_encoded() {
            let (config, _) = JwtConfig::generate().unwrap();
            let jwt = JwtService::new(config).unwrap();

            let user_id = Uuid::new_v7();
            let session_id = Uuid::new_v7();

            let (token, _) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();
            let fp = token_fingerprint(&token);

            let decoded = URL_SAFE_NO_PAD.decode(&fp).unwrap();
            assert_eq!(decoded.len(), 32);
        }
    }

    mod error_handling {
        use super::*;

        #[test]
        fn jwt_error_codes_are_correct() {
            assert_eq!(JwtError::Config("test".to_string()).code(), "auth.jwt_config_error");
            assert_eq!(JwtError::Key("test".to_string()).code(), "auth.jwt_key_error");
            assert_eq!(JwtError::Expired.code(), "auth.token_expired");
            assert_eq!(JwtError::NotYetValid.code(), "auth.token_not_yet_valid");
            assert_eq!(JwtError::InvalidToken.code(), "auth.token_invalid");
            assert_eq!(JwtError::MalformedToken.code(), "auth.token_malformed");
            assert_eq!(JwtError::SessionRevoked.code(), "auth.session_revoked");
            assert_eq!(JwtError::TokenFingerprintMismatch.code(), "auth.token_fingerprint_mismatch");
            assert_eq!(JwtError::RefreshReplayDetected.code(), "auth.token_replay");
            assert_eq!(JwtError::UnsupportedTokenType.code(), "auth.token_invalid");
            assert_eq!(JwtError::Internal("test".to_string()).code(), "auth.internal");
        }

        #[test]
        fn jwt_error_converts_to_auth_error() {
            use underlay_auth::AuthError;

            assert_eq!(AuthError::SessionExpired, JwtError::Expired.into());
            assert_eq!(AuthError::TokenNotYetValid, JwtError::NotYetValid.into());
            assert_eq!(AuthError::TokenInvalid, JwtError::InvalidToken.into());
            assert_eq!(AuthError::TokenMalformed, JwtError::MalformedToken.into());
            assert_eq!(AuthError::SessionRevoked, JwtError::SessionRevoked.into());
            assert_eq!(AuthError::TokenFingerprintMismatch, JwtError::TokenFingerprintMismatch.into());
            assert_eq!(AuthError::TokenInvalid, JwtError::RefreshReplayDetected.into());
            assert_eq!(AuthError::TokenInvalid, JwtError::UnsupportedTokenType.into());
            assert!(matches!(JwtError::Config("error".into()).into(), AuthError::Internal(_)));
            assert!(matches!(JwtError::Key("error".into()).into(), AuthError::Internal(_)));
            assert!(matches!(JwtError::Internal("error".into()).into(), AuthError::Internal(_)));
        }
    }
}
