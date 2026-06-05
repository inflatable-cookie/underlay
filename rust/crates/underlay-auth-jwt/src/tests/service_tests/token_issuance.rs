use underlay_core::Uuid;

use super::support::*;

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
