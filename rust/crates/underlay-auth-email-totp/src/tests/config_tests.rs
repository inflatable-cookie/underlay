use super::*;

#[test]
fn test_default_config() {
    let config = EmailTotpConfig::default();
    assert_eq!(config.code_expiry_minutes, 10);
    assert_eq!(config.max_codes_per_hour, 5);
    assert_eq!(config.max_attempts, 5);
    assert_eq!(config.session_expiry_minutes, 5);
    assert_eq!(config.code_length, 6);
}

#[test]
fn test_config_builder() {
    let config = EmailTotpConfig::new()
        .with_code_expiry_minutes(15)
        .with_max_codes_per_hour(10)
        .with_max_attempts(3)
        .with_session_expiry_minutes(10)
        .with_code_length(8);

    assert_eq!(config.code_expiry_minutes, 15);
    assert_eq!(config.max_codes_per_hour, 10);
    assert_eq!(config.max_attempts, 3);
    assert_eq!(config.session_expiry_minutes, 10);
    assert_eq!(config.code_length, 8);
}
