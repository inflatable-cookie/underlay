use underlay_core::Uuid;

use super::support::*;

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
