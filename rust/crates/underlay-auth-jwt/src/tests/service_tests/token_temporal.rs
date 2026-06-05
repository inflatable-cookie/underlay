use jsonwebtoken::{encode, Algorithm, Header};
use underlay_core::Uuid;

use super::support::*;

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
