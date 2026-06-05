use std::sync::Arc;

use crate::session::{SessionManager, SessionState, SessionStore};
use underlay_core::Uuid;

use super::support::*;

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

    assert!(matches!(
        manager.refresh_session(&tokens.refresh_token).await,
        Err(JwtError::RefreshReplayDetected) | Err(JwtError::TokenFingerprintMismatch)
    ));

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
