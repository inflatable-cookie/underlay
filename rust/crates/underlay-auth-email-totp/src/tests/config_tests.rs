use super::*;

#[test]
fn test_default_config() {
    let config = EmailTotpConfig::default();
    assert_eq!(config.code_expiry_minutes(), 10);
    assert_eq!(config.max_codes_per_hour(), 5);
    assert_eq!(config.max_attempts(), 5);
    assert_eq!(config.session_expiry_minutes(), 5);
    assert_eq!(config.code_length(), 6);
}

#[test]
fn test_config_builder() {
    let config = EmailTotpConfig::new()
        .with_code_expiry_minutes(15)
        .with_max_codes_per_hour(10)
        .with_max_attempts(3)
        .with_session_expiry_minutes(10)
        .with_code_length(8);

    assert_eq!(config.code_expiry_minutes(), 15);
    assert_eq!(config.max_codes_per_hour(), 10);
    assert_eq!(config.max_attempts(), 3);
    assert_eq!(config.session_expiry_minutes(), 10);
    assert_eq!(config.code_length(), 8);
}

#[test]
fn config_setters_clamp_out_of_bounds_values() {
    let low = EmailTotpConfig::new()
        .with_code_expiry_minutes(0)
        .with_max_codes_per_hour(-3)
        .with_max_attempts(0)
        .with_session_expiry_minutes(-1)
        .with_code_length(2);

    assert_eq!(low.code_expiry_minutes(), 1);
    assert_eq!(low.max_codes_per_hour(), 1);
    assert_eq!(low.max_attempts(), 1);
    assert_eq!(low.session_expiry_minutes(), 1);
    assert_eq!(low.code_length(), 6);

    let high = EmailTotpConfig::new()
        .with_code_expiry_minutes(100_000)
        .with_session_expiry_minutes(100_000)
        .with_code_length(100);

    assert_eq!(high.code_expiry_minutes(), 24 * 60);
    assert_eq!(high.session_expiry_minutes(), 24 * 60);
    assert_eq!(high.code_length(), 19);
}
