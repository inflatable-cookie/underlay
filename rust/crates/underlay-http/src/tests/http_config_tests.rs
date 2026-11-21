use std::sync::Mutex;

use super::*;

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_default_config() {
    let config = HttpServerConfig::default();
    assert_eq!(config.bind_addr(), "127.0.0.1");
    assert_eq!(config.port(), 3000);
    assert_eq!(config.public_host(), "localhost");
}

#[test]
fn test_socket_addr() {
    let config = HttpServerConfig::new("0.0.0.0", 8080, "api.example.com");
    assert_eq!(config.socket_addr(), "0.0.0.0:8080");
}

#[test]
fn test_try_new_accepts_valid_config() {
    let config =
        HttpServerConfig::try_new("0.0.0.0", 8080, "api.example.com").expect("valid config");

    assert_eq!(config.bind_addr(), "0.0.0.0");
    assert_eq!(config.port(), 8080);
    assert_eq!(config.public_host(), "api.example.com");
}

#[test]
fn test_try_new_rejects_invalid_bind_addr() {
    let error = HttpServerConfig::try_new("localhost", 8080, "api.example.com")
        .expect_err("bind address must be an IP");

    assert!(matches!(
        error,
        HttpServerConfigError::InvalidBindAddr { .. }
    ));
}

#[test]
fn test_try_new_rejects_url_public_host() {
    let error = HttpServerConfig::try_new("127.0.0.1", 8080, "https://api.example.com")
        .expect_err("public host must not include a scheme");

    assert!(matches!(
        error,
        HttpServerConfigError::InvalidPublicHost { .. }
    ));
}

#[test]
fn test_try_new_rejects_path_public_host() {
    let error = HttpServerConfig::try_new("127.0.0.1", 8080, "api.example.com/path")
        .expect_err("public host must not include a path");

    assert!(matches!(
        error,
        HttpServerConfigError::InvalidPublicHost { .. }
    ));
}

#[test]
fn test_try_new_rejects_hostname_with_port_and_accepts_ipv6() {
    for bad in [
        "localhost:8080",
        "api.example.com:443",
        "[::1]:8080",
        "[localhost]",
    ] {
        let error = HttpServerConfig::try_new("127.0.0.1", 8080, bad)
            .expect_err("non-IPv6 host with ':' or brackets must be rejected");
        assert!(
            matches!(error, HttpServerConfigError::InvalidPublicHost { .. }),
            "expected InvalidPublicHost for {bad}, got {error:?}"
        );
    }

    let bare_v6 = HttpServerConfig::try_new("::1", 8080, "::1").expect("bare IPv6 should pass");
    assert_eq!(bare_v6.http_base_url(), "http://[::1]:8080");

    let bracketed_v6 =
        HttpServerConfig::try_new("::1", 8080, "[::1]").expect("bracketed IPv6 should pass");
    assert_eq!(bracketed_v6.http_base_url(), "http://[::1]:8080");
}

#[test]
fn test_base_url() {
    let config = HttpServerConfig::new("0.0.0.0", 8080, "api.example.com");
    assert_eq!(config.http_base_url(), "http://api.example.com:8080");
    assert_eq!(config.https_base_url(), "https://api.example.com:8080");
}

#[test]
fn test_local_defaults() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    // Clear env vars to test defaults
    env::remove_var("HOST");
    env::remove_var("PORT");
    env::remove_var("PUBLIC_HOST");

    let config = HttpServerConfig::from_env(Environment::Local);
    assert_eq!(config.bind_addr(), "127.0.0.1");
    assert_eq!(config.port(), 3000);
    assert_eq!(config.public_host(), "localhost");
}

#[test]
fn test_local_with_port_and_unset_host_defaults_public_host_to_localhost() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    env::remove_var("HOST");
    env::remove_var("PUBLIC_HOST");
    env::set_var("PORT", "8080");

    let from_env = HttpServerConfig::from_env(Environment::Local);
    assert_eq!(from_env.bind_addr(), "0.0.0.0");
    assert_eq!(from_env.port(), 8080);
    assert_eq!(from_env.public_host(), "localhost");

    let try_from_env =
        HttpServerConfig::try_from_env(Environment::Local).expect("valid local config");
    assert_eq!(try_from_env.bind_addr(), "0.0.0.0");
    assert_eq!(try_from_env.port(), 8080);
    assert_eq!(try_from_env.public_host(), "localhost");

    env::remove_var("PORT");
}

#[test]
fn test_try_from_env_rejects_invalid_port() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    env::set_var("PORT", "not-a-port");
    env::remove_var("HOST");
    env::remove_var("PUBLIC_HOST");

    let error =
        HttpServerConfig::try_from_env(Environment::Local).expect_err("invalid port should fail");

    assert!(matches!(error, HttpServerConfigError::InvalidPort { .. }));
    env::remove_var("PORT");
}
