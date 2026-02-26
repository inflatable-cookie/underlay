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
fn encrypt_is_nondeterministic_and_decrypts() {
    let key = vec![42u8; 32];
    let b64 = STANDARD.encode(&key);
    let cipher = OAuthTokenCipher::from_key_string("TEST_KEY", &b64).unwrap();

    let enc1 = cipher.encrypt_refresh_token("refresh-token").unwrap();
    let enc2 = cipher.encrypt_refresh_token("refresh-token").unwrap();
    assert_ne!(enc1, enc2);

    let dec = cipher.decrypt_refresh_token(&enc1).unwrap();
    assert_eq!(dec, "refresh-token");

    assert_eq!(cipher.decrypt_refresh_token("plain:abc").unwrap(), "abc");
}

#[test]
fn from_env_optional_returns_none_when_unset() {
    let _lock = ENV_LOCK.lock().unwrap();
    let prev = with_env_var(AUTH_OAUTH_SECRET_KEY_ENV, None);
    let loaded = OAuthTokenCipher::from_env_optional().unwrap();
    assert!(loaded.is_none());
    restore_env_var(AUTH_OAUTH_SECRET_KEY_ENV, prev);
}

#[test]
fn from_env_errors_on_wrong_length() {
    let _lock = ENV_LOCK.lock().unwrap();
    let prev = with_env_var(AUTH_OAUTH_SECRET_KEY_ENV, Some("AA=="));

    let err = OAuthTokenCipher::from_env().err();
    assert!(matches!(err, Some(AuthError::Internal(_))));

    restore_env_var(AUTH_OAUTH_SECRET_KEY_ENV, prev);
}
