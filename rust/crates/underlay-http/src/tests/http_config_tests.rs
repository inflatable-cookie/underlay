use super::*;

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
fn test_base_url() {
    let config = HttpServerConfig::new("0.0.0.0", 8080, "api.example.com");
    assert_eq!(config.http_base_url(), "http://api.example.com:8080");
    assert_eq!(config.https_base_url(), "https://api.example.com:8080");
}

#[test]
fn test_local_defaults() {
    // Clear env vars to test defaults
    env::remove_var("HOST");
    env::remove_var("PORT");
    env::remove_var("PUBLIC_HOST");

    let config = HttpServerConfig::from_env(Environment::Local);
    assert_eq!(config.bind_addr(), "127.0.0.1");
    assert_eq!(config.port(), 3000);
    // public_host defaults to bind_addr
    assert_eq!(config.public_host(), "127.0.0.1");
}

#[test]
fn test_try_from_env_rejects_invalid_port() {
    env::set_var("PORT", "not-a-port");
    env::remove_var("HOST");
    env::remove_var("PUBLIC_HOST");

    let error =
        HttpServerConfig::try_from_env(Environment::Local).expect_err("invalid port should fail");

    assert!(matches!(error, HttpServerConfigError::InvalidPort { .. }));
    env::remove_var("PORT");
}
