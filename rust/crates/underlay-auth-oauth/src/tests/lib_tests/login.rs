use super::super::{GoogleOAuthConfig, GoogleOAuthService, OAuthProvider};

#[test]
fn google_oauth_config_accessors_and_debug_redaction() {
    let config = GoogleOAuthConfig::new("client", "secret-value-123", "https://example.com/cb")
        .with_scopes(["openid", "email"]);

    assert_eq!(config.client_id(), "client");
    assert_eq!(config.client_secret(), "secret-value-123");
    assert_eq!(config.redirect_uri(), "https://example.com/cb");
    assert_eq!(
        config.scopes(),
        &["openid".to_string(), "email".to_string()]
    );

    let debug = format!("{config:?}");
    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains("secret-value-123"));
}

#[test]
fn start_login_with_builds_url_with_state_and_challenge() {
    let svc = GoogleOAuthService::new(
        GoogleOAuthConfig::new("client", "secret", "https://example.com/cb")
            .with_scopes(["openid"]),
    )
    .unwrap();

    let url = svc
        .start_login_with(
            "state-123",
            "verifier-12345678901234567890123456789012345678901234567890",
        )
        .unwrap();

    assert!(url.contains("state=state-123"));
    assert!(url.contains("code_challenge="));
    assert!(url.contains("code_challenge_method=S256"));
    assert!(url.contains("redirect_uri="));
}

#[test]
fn start_login_generates_state_and_pkce() {
    let svc = GoogleOAuthService::new(GoogleOAuthConfig::new(
        "client",
        "secret",
        "https://example.com/cb",
    ))
    .unwrap();

    let start = svc.start_login().unwrap();
    assert!(start.authorization_url.contains("state="));
    assert!(!start.csrf_state.is_empty());
    assert!(!start.pkce_verifier.is_empty());
}
