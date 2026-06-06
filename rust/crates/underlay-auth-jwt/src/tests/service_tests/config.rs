use base64::Engine as _;

use crate::keys::URL_SAFE_NO_PAD;

use super::support::*;

#[test]
fn default_config_has_sensible_values() {
    let config = JwtConfig::default();

    assert_eq!(config.access_token_lifetime_minutes(), 15);
    assert_eq!(config.refresh_token_lifetime_days(), 30);
    assert_eq!(config.issuer(), "underlay");
    assert!(config.audience().is_none());
    assert_eq!(config.leeway_seconds(), 30);
}

#[test]
fn access_token_lifetime_returns_duration() {
    let config = JwtConfig::default().with_access_token_lifetime_minutes(60);
    let duration = config.access_token_lifetime();
    assert_eq!(duration.num_minutes(), 60);
}

#[test]
fn refresh_token_lifetime_returns_duration() {
    let config = JwtConfig::default().with_refresh_token_lifetime_days(7);
    let duration = config.refresh_token_lifetime();
    assert_eq!(duration.num_days(), 7);
}

#[test]
fn generate_creates_valid_config_and_keys() {
    let (config, keys) = JwtConfig::generate().unwrap();

    assert!(!config.private_key_b64().is_empty());
    assert!(!config.public_key_b64().is_empty());
    assert_eq!(config.issuer(), "underlay");

    let decoded_priv = STANDARD.decode(config.private_key_b64()).unwrap();
    assert!(!decoded_priv.is_empty());

    let decoded_pub = URL_SAFE_NO_PAD.decode(config.public_key_b64()).unwrap();
    assert_eq!(decoded_pub.len(), 32);

    assert_eq!(
        config.private_key_b64(),
        keys.private_key_pkcs8_der_b64.as_str()
    );
    assert_eq!(config.public_key_b64(), keys.public_key_raw_b64.as_str());
}

#[test]
fn mismatched_public_key_fails_service_startup() {
    let (config1, _) = JwtConfig::generate().unwrap();
    let (config2, _) = JwtConfig::generate().unwrap();

    let mismatched = JwtConfig::from_values(
        config1.private_key_b64().to_string(),
        config2.public_key_b64().to_string(),
        crate::JwtBehaviorDefaults::default(),
    );

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
    assert!(!debug_str.contains(config.private_key_b64()));
}
