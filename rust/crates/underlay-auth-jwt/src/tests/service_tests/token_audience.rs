use underlay_core::Uuid;

use super::support::*;

#[test]
fn token_with_configured_audience_passes() {
    let (config, _) = JwtConfig::generate().unwrap();
    let config = config.with_audience("my-app");

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

    let config_with_aud = config_no_audience.with_audience("my-app");
    let jwt_with_aud = JwtService::new(config_with_aud).unwrap();

    let user_id = Uuid::new_v7();
    let session_id = Uuid::new_v7();

    let (token, _) = jwt_no_aud
        .issue_access_token(user_id, session_id, vec![])
        .unwrap();

    let result = jwt_with_aud.verify_access_token(&token);
    assert!(matches!(result, Err(JwtError::InvalidToken)));
}
