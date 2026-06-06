use underlay_core::Uuid;

use super::support::*;

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

    let config_wrong_issuer = config.with_issuer("wrong-issuer");
    let jwt_wrong = JwtService::new(config_wrong_issuer).unwrap();

    let result = jwt_wrong.verify_access_token(&token);
    assert!(matches!(result, Err(JwtError::InvalidToken)));
}
