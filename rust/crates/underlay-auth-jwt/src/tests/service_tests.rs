use super::*;
use crate::config::JwtConfig;
use crate::session::{SessionManager, SessionState, SessionStore};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use async_trait::async_trait;
use underlay_core::Uuid;

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
        self.sessions
            .lock()
            .await
            .insert(session.id, session.clone());
        Ok(())
    }

    async fn update_session(&self, session: &SessionState) -> JwtResult<()> {
        self.sessions
            .lock()
            .await
            .insert(session.id, session.clone());
        Ok(())
    }

    async fn rotate_session_if_current(
        &self,
        session: &SessionState,
        expected_refresh_token_fingerprint: &str,
        expected_refresh_token_id: Uuid,
        expected_refresh_token_version: u32,
    ) -> JwtResult<bool> {
        let mut sessions = self.sessions.lock().await;
        let Some(current) = sessions.get_mut(&session.id) else {
            return Ok(false);
        };

        if !current.is_active
            || current.refresh_token_fingerprint != expected_refresh_token_fingerprint
            || current.refresh_token_id != expected_refresh_token_id
            || current.refresh_token_version != expected_refresh_token_version
        {
            return Ok(false);
        }

        *current = session.clone();
        Ok(true)
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

    let claims = manager
        .verify_access_token(&tokens.access_token)
        .await
        .unwrap();
    assert_eq!(claims.common.subject, user_id);

    let refreshed = manager
        .refresh_session(&tokens.refresh_token)
        .await
        .unwrap();
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

#[tokio::test]
async fn rotate_session_if_current_rejects_stale_refresh_state() {
    let store = MemoryStore::default();
    let session = SessionState {
        id: Uuid::new_v7(),
        user_id: Uuid::new_v7(),
        roles: vec!["user".to_string()],
        is_active: true,
        access_token_fingerprint: "access-1".to_string(),
        refresh_token_fingerprint: "refresh-1".to_string(),
        refresh_token_id: Uuid::new_v7(),
        refresh_token_version: 1,
    };
    store.create_session(&session).await.unwrap();

    let mut rotated = session.clone();
    rotated.access_token_fingerprint = "access-2".to_string();
    rotated.refresh_token_fingerprint = "refresh-2".to_string();
    rotated.refresh_token_id = Uuid::new_v7();
    rotated.refresh_token_version = 2;

    assert!(store
        .rotate_session_if_current(
            &rotated,
            &session.refresh_token_fingerprint,
            session.refresh_token_id,
            session.refresh_token_version,
        )
        .await
        .unwrap());

    let mut stale_rotation = rotated.clone();
    stale_rotation.access_token_fingerprint = "access-3".to_string();
    stale_rotation.refresh_token_fingerprint = "refresh-3".to_string();
    stale_rotation.refresh_token_id = Uuid::new_v7();
    stale_rotation.refresh_token_version = 3;

    assert!(!store
        .rotate_session_if_current(
            &stale_rotation,
            &session.refresh_token_fingerprint,
            session.refresh_token_id,
            session.refresh_token_version,
        )
        .await
        .unwrap());
}

mod key_generation {
    use super::*;
    use crate::{keys::URL_SAFE_NO_PAD, KeyPair};

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
    use super::*;
    use crate::keys::URL_SAFE_NO_PAD;

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
    fn mismatched_public_key_fails_service_startup() {
        let (config1, _) = JwtConfig::generate().unwrap();
        let (config2, _) = JwtConfig::generate().unwrap();

        let mismatched = JwtConfig {
            private_key_b64: config1.private_key_b64,
            public_key_b64: config2.public_key_b64,
            ..JwtConfig::default()
        };

        let result = JwtService::new(mismatched);
        assert!(
            matches!(result, Err(JwtError::InvalidToken)),
            "got: {result:?}"
        );
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

        let (token, claims) = jwt
            .issue_access_token(user_id, session_id, roles.clone())
            .unwrap();

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

        let (token, _claims) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();
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

        let (token, _claims) = jwt
            .issue_refresh_token(user_id, session_id, None, 1)
            .unwrap();
        let verified = jwt.verify_refresh_token(&token).unwrap();

        assert_eq!(verified.common.subject, user_id);
        assert_eq!(verified.session_id, session_id);
        assert_eq!(verified.token_use, TokenUse::Refresh);
        assert_eq!(verified.version, 1);
    }

    #[test]
    fn expired_token_returns_expired_error() {
        let (mut config, _) = JwtConfig::generate().unwrap();
        config.leeway_seconds = 0;
        let jwt = JwtService::new(config).unwrap();

        let now = chrono::Utc::now().timestamp() as u64;
        let user_id = Uuid::new_v7();
        let session_id = Uuid::new_v7();

        let claims = AccessTokenClaims {
            common: CommonClaims {
                issuer: jwt.config.issuer.clone(),
                subject: user_id,
                audience: jwt.config.audience.clone(),
                issued_at: now.saturating_sub(120),
                expires_at: now.saturating_sub(60),
                not_before: Some(now.saturating_sub(120)),
                token_id: Uuid::new_v7(),
            },
            token_use: TokenUse::Access,
            session_id,
            roles: vec![],
        };

        let mut header = Header::new(Algorithm::EdDSA);
        header.typ = Some("JWT".to_string());

        let token = encode(&header, &claims, &jwt.encoding_key).unwrap();
        let result = jwt.verify_access_token(&token);

        assert!(matches!(result, Err(JwtError::Expired)), "got: {result:?}");
    }

    #[test]
    fn token_not_yet_valid_returns_not_yet_valid_error() {
        let (mut config, _) = JwtConfig::generate().unwrap();
        config.leeway_seconds = 0;
        let jwt = JwtService::new(config).unwrap();

        let now = chrono::Utc::now().timestamp() as u64;
        let user_id = Uuid::new_v7();
        let session_id = Uuid::new_v7();

        let claims = AccessTokenClaims {
            common: CommonClaims {
                issuer: jwt.config.issuer.clone(),
                subject: user_id,
                audience: jwt.config.audience.clone(),
                issued_at: now,
                expires_at: now + 600,
                not_before: Some(now + 300),
                token_id: Uuid::new_v7(),
            },
            token_use: TokenUse::Access,
            session_id,
            roles: vec![],
        };

        let mut header = Header::new(Algorithm::EdDSA);
        header.typ = Some("JWT".to_string());

        let token = encode(&header, &claims, &jwt.encoding_key).unwrap();
        let result = jwt.verify_access_token(&token);

        assert!(
            matches!(result, Err(JwtError::NotYetValid)),
            "got: {result:?}"
        );
    }

    #[test]
    fn leeway_allows_slightly_expired_tokens() {
        let (mut config, _) = JwtConfig::generate().unwrap();
        config.leeway_seconds = 30;
        let jwt = JwtService::new(config).unwrap();

        let now = chrono::Utc::now().timestamp() as u64;
        let user_id = Uuid::new_v7();
        let session_id = Uuid::new_v7();

        let claims = AccessTokenClaims {
            common: CommonClaims {
                issuer: jwt.config.issuer.clone(),
                subject: user_id,
                audience: jwt.config.audience.clone(),
                issued_at: now.saturating_sub(120),
                expires_at: now.saturating_sub(10),
                not_before: Some(now.saturating_sub(120)),
                token_id: Uuid::new_v7(),
            },
            token_use: TokenUse::Access,
            session_id,
            roles: vec![],
        };

        let mut header = Header::new(Algorithm::EdDSA);
        header.typ = Some("JWT".to_string());

        let token = encode(&header, &claims, &jwt.encoding_key).unwrap();
        let result = jwt.verify_access_token(&token);

        assert!(result.is_ok(), "got: {result:?}");
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

        let (token, _) = jwt1
            .issue_access_token(user_id, session_id, vec![])
            .unwrap();

        let result = jwt2.verify_access_token(&token);
        assert!(matches!(result, Err(JwtError::InvalidToken)));
    }

    #[test]
    fn access_token_with_refresh_use_fails() {
        let (config, _) = JwtConfig::generate().unwrap();
        let jwt = JwtService::new(config).unwrap();

        let user_id = Uuid::new_v7();
        let session_id = Uuid::new_v7();

        let (refresh_token, _) = jwt
            .issue_refresh_token(user_id, session_id, None, 1)
            .unwrap();

        let result = jwt.verify_access_token(&refresh_token);

        assert!(
            matches!(result, Err(JwtError::UnsupportedTokenType)),
            "Expected UnsupportedTokenType but got: {:?}",
            result
        );
    }

    #[test]
    fn refresh_token_with_access_use_fails() {
        let (config, _) = JwtConfig::generate().unwrap();
        let jwt = JwtService::new(config).unwrap();

        let user_id = Uuid::new_v7();
        let session_id = Uuid::new_v7();

        let (access_token, _) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();

        let result = jwt.verify_refresh_token(&access_token);

        assert!(
            matches!(result, Err(JwtError::UnsupportedTokenType)),
            "Expected UnsupportedTokenType but got: {:?}",
            result
        );
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

        let (token, _) = jwt_no_aud
            .issue_access_token(user_id, session_id, vec![])
            .unwrap();

        let result = jwt_with_aud.verify_access_token(&token);
        assert!(matches!(result, Err(JwtError::InvalidToken)));
    }
}

mod token_fingerprint_tests {
    use super::*;
    use crate::fingerprint::token_fingerprint;
    use crate::keys::URL_SAFE_NO_PAD;

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
        assert_eq!(JwtError::Config("test".to_string()).code(), "auth.internal");
        assert_eq!(JwtError::Key("test".to_string()).code(), "auth.internal");
        assert_eq!(JwtError::Expired.code(), "auth.session_expired");
        assert_eq!(JwtError::NotYetValid.code(), "auth.token_not_yet_valid");
        assert_eq!(JwtError::InvalidToken.code(), "auth.token_invalid");
        assert_eq!(JwtError::MalformedToken.code(), "auth.token_malformed");
        assert_eq!(JwtError::SessionRevoked.code(), "auth.session_revoked");
        assert_eq!(
            JwtError::TokenFingerprintMismatch.code(),
            "auth.token_fingerprint_mismatch"
        );
        assert_eq!(JwtError::RefreshReplayDetected.code(), "auth.token_invalid");
        assert_eq!(JwtError::UnsupportedTokenType.code(), "auth.token_invalid");
        assert_eq!(
            JwtError::Internal("test".to_string()).code(),
            "auth.internal"
        );
    }

    #[test]
    fn jwt_error_converts_to_auth_error() {
        use underlay_auth::AuthError;

        assert_eq!(AuthError::SessionExpired, JwtError::Expired.into());
        assert_eq!(AuthError::TokenNotYetValid, JwtError::NotYetValid.into());
        assert_eq!(AuthError::TokenInvalid, JwtError::InvalidToken.into());
        assert_eq!(AuthError::TokenMalformed, JwtError::MalformedToken.into());
        assert_eq!(AuthError::SessionRevoked, JwtError::SessionRevoked.into());
        assert_eq!(
            AuthError::TokenFingerprintMismatch,
            JwtError::TokenFingerprintMismatch.into()
        );
        assert_eq!(
            AuthError::TokenInvalid,
            JwtError::RefreshReplayDetected.into()
        );
        assert_eq!(
            AuthError::TokenInvalid,
            JwtError::UnsupportedTokenType.into()
        );
        assert!(matches!(
            JwtError::Config("error".into()).into(),
            AuthError::Internal(_)
        ));
        assert!(matches!(
            JwtError::Key("error".into()).into(),
            AuthError::Internal(_)
        ));
        assert!(matches!(
            JwtError::Internal("error".into()).into(),
            AuthError::Internal(_)
        ));
    }
}
