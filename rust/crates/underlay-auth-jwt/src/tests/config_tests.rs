use super::*;

static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn with_env_var(key: &str, value: Option<&str>) -> Option<String> {
    let previous = std::env::var(key).ok();
    match value {
        Some(value) => std::env::set_var(key, value),
        None => std::env::remove_var(key),
    }
    previous
}

fn restore_env_var(key: &str, previous: Option<String>) {
    match previous {
        Some(value) => std::env::set_var(key, value),
        None => std::env::remove_var(key),
    }
}

#[test]
fn from_env_requires_private_key() {
    let _lock = ENV_LOCK.lock().unwrap();

    let prev_priv = with_env_var("AUTH_JWT_PRIVATE_KEY", None);
    let prev_pub = with_env_var("AUTH_JWT_PUBLIC_KEY", Some("abc"));

    let result = JwtConfig::from_env();
    assert!(matches!(result, Err(JwtError::Config(_))));

    restore_env_var("AUTH_JWT_PRIVATE_KEY", prev_priv);
    restore_env_var("AUTH_JWT_PUBLIC_KEY", prev_pub);
}

#[test]
fn from_env_requires_public_key() {
    let _lock = ENV_LOCK.lock().unwrap();

    let prev_priv = with_env_var("AUTH_JWT_PRIVATE_KEY", Some("abc"));
    let prev_pub = with_env_var("AUTH_JWT_PUBLIC_KEY", None);

    let result = JwtConfig::from_env();
    assert!(matches!(result, Err(JwtError::Config(_))));

    restore_env_var("AUTH_JWT_PRIVATE_KEY", prev_priv);
    restore_env_var("AUTH_JWT_PUBLIC_KEY", prev_pub);
}

#[test]
fn from_env_parses_defaults_and_overrides() {
    let _lock = ENV_LOCK.lock().unwrap();

    let (generated, _keys) = JwtConfig::generate().unwrap();

    let prev_priv = with_env_var("AUTH_JWT_PRIVATE_KEY", Some(generated.private_key_b64()));
    let prev_pub = with_env_var("AUTH_JWT_PUBLIC_KEY", Some(generated.public_key_b64()));
    let prev_access = with_env_var("AUTH_ACCESS_TOKEN_LIFETIME_MINUTES", Some("42"));
    let prev_refresh = with_env_var("AUTH_REFRESH_TOKEN_LIFETIME_DAYS", Some("7"));
    let prev_issuer = with_env_var("AUTH_JWT_ISSUER", Some("issuer-x"));
    let prev_aud = with_env_var("AUTH_JWT_AUDIENCE", Some("aud-x"));
    let prev_leeway = with_env_var("AUTH_JWT_LEEWAY_SECONDS", Some("5"));

    let config = JwtConfig::from_env().unwrap();

    assert_eq!(config.private_key_b64(), generated.private_key_b64());
    assert_eq!(config.public_key_b64(), generated.public_key_b64());
    assert_eq!(config.access_token_lifetime_minutes(), 42);
    assert_eq!(config.refresh_token_lifetime_days(), 7);
    assert_eq!(config.issuer(), "issuer-x");
    assert_eq!(config.audience(), Some("aud-x"));
    assert_eq!(config.leeway_seconds(), 5);

    restore_env_var("AUTH_JWT_PRIVATE_KEY", prev_priv);
    restore_env_var("AUTH_JWT_PUBLIC_KEY", prev_pub);
    restore_env_var("AUTH_ACCESS_TOKEN_LIFETIME_MINUTES", prev_access);
    restore_env_var("AUTH_REFRESH_TOKEN_LIFETIME_DAYS", prev_refresh);
    restore_env_var("AUTH_JWT_ISSUER", prev_issuer);
    restore_env_var("AUTH_JWT_AUDIENCE", prev_aud);
    restore_env_var("AUTH_JWT_LEEWAY_SECONDS", prev_leeway);
}

#[test]
fn from_env_with_defaults_uses_typed_defaults_when_env_missing() {
    let _lock = ENV_LOCK.lock().unwrap();

    let (generated, _keys) = JwtConfig::generate().unwrap();

    let prev_priv = with_env_var("AUTH_JWT_PRIVATE_KEY", Some(generated.private_key_b64()));
    let prev_pub = with_env_var("AUTH_JWT_PUBLIC_KEY", Some(generated.public_key_b64()));
    let prev_access = with_env_var("AUTH_ACCESS_TOKEN_LIFETIME_MINUTES", None);
    let prev_refresh = with_env_var("AUTH_REFRESH_TOKEN_LIFETIME_DAYS", None);
    let prev_issuer = with_env_var("AUTH_JWT_ISSUER", None);
    let prev_aud = with_env_var("AUTH_JWT_AUDIENCE", None);
    let prev_leeway = with_env_var("AUTH_JWT_LEEWAY_SECONDS", None);

    let defaults = JwtBehaviorDefaults {
        access_token_lifetime_minutes: 42,
        refresh_token_lifetime_days: 7,
        issuer: "typed-issuer".to_string(),
        audience: Some("typed-aud".to_string()),
        leeway_seconds: 9,
    };

    let config = JwtConfig::from_env_with_defaults(&defaults).unwrap();
    assert_eq!(config.access_token_lifetime_minutes(), 42);
    assert_eq!(config.refresh_token_lifetime_days(), 7);
    assert_eq!(config.issuer(), "typed-issuer");
    assert_eq!(config.audience(), Some("typed-aud"));
    assert_eq!(config.leeway_seconds(), 9);

    restore_env_var("AUTH_JWT_PRIVATE_KEY", prev_priv);
    restore_env_var("AUTH_JWT_PUBLIC_KEY", prev_pub);
    restore_env_var("AUTH_ACCESS_TOKEN_LIFETIME_MINUTES", prev_access);
    restore_env_var("AUTH_REFRESH_TOKEN_LIFETIME_DAYS", prev_refresh);
    restore_env_var("AUTH_JWT_ISSUER", prev_issuer);
    restore_env_var("AUTH_JWT_AUDIENCE", prev_aud);
    restore_env_var("AUTH_JWT_LEEWAY_SECONDS", prev_leeway);
}

#[test]
fn from_values_uses_supplied_behavior_without_env() {
    let behavior = JwtBehaviorDefaults {
        access_token_lifetime_minutes: 50,
        refresh_token_lifetime_days: 8,
        issuer: "manual-issuer".to_string(),
        audience: Some("manual-aud".to_string()),
        leeway_seconds: 11,
    };

    let config = JwtConfig::from_values(
        "private".to_string(),
        "public".to_string(),
        behavior.clone(),
    );

    assert_eq!(config.private_key_b64(), "private");
    assert_eq!(config.public_key_b64(), "public");
    assert_eq!(
        config.access_token_lifetime_minutes(),
        behavior.access_token_lifetime_minutes
    );
    assert_eq!(
        config.refresh_token_lifetime_days(),
        behavior.refresh_token_lifetime_days
    );
    assert_eq!(config.issuer(), behavior.issuer);
    assert_eq!(config.audience(), behavior.audience.as_deref());
    assert_eq!(config.leeway_seconds(), behavior.leeway_seconds);
}

#[test]
fn from_env_rejects_invalid_numbers() {
    let _lock = ENV_LOCK.lock().unwrap();

    let (generated, _keys) = JwtConfig::generate().unwrap();

    let prev_priv = with_env_var("AUTH_JWT_PRIVATE_KEY", Some(generated.private_key_b64()));
    let prev_pub = with_env_var("AUTH_JWT_PUBLIC_KEY", Some(generated.public_key_b64()));
    let prev_access = with_env_var("AUTH_ACCESS_TOKEN_LIFETIME_MINUTES", Some("nope"));

    let result = JwtConfig::from_env();
    assert!(
        matches!(result, Err(JwtError::Config(msg)) if msg.contains("AUTH_ACCESS_TOKEN_LIFETIME_MINUTES"))
    );

    restore_env_var("AUTH_JWT_PRIVATE_KEY", prev_priv);
    restore_env_var("AUTH_JWT_PUBLIC_KEY", prev_pub);
    restore_env_var("AUTH_ACCESS_TOKEN_LIFETIME_MINUTES", prev_access);
}
