use super::super::{GoogleOAuthConfig, GoogleOAuthService, OAuthProvider};

#[test]
fn start_login_with_builds_url_with_state_and_challenge() {
    let svc = GoogleOAuthService::new(GoogleOAuthConfig {
        client_id: "client".to_string(),
        client_secret: "secret".to_string(),
        redirect_uri: "https://example.com/cb".to_string(),
        scopes: vec!["openid".to_string()],
    })
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
    let svc = GoogleOAuthService::new(GoogleOAuthConfig {
        client_id: "client".to_string(),
        client_secret: "secret".to_string(),
        redirect_uri: "https://example.com/cb".to_string(),
        scopes: vec![],
    })
    .unwrap();

    let start = svc.start_login().unwrap();
    assert!(start.authorization_url.contains("state="));
    assert!(!start.csrf_state.is_empty());
    assert!(!start.pkce_verifier.is_empty());
}
