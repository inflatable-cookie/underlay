use super::super::{AuthError, GoogleOAuthConfig, GoogleOAuthService};
use super::support::{restore_env_var, with_env_var, ENV_LOCK};

#[test]
fn constructor_validates_the_redirect_uri() {
    for uri in [
        "relative/callback",
        "ftp://example.com/callback",
        "http://example.com/callback",
        "https://example.com/callback#fragment",
        "https://name@example.com/callback",
    ] {
        assert!(
            GoogleOAuthService::new(GoogleOAuthConfig::new("id", "secret", uri)).is_err(),
            "invalid redirect URI {uri}"
        );
    }
    for uri in [
        "https://example.com/callback?variant=web",
        "http://localhost:8080/callback",
        "http://[::1]:8080/callback",
    ] {
        assert!(
            GoogleOAuthService::new(GoogleOAuthConfig::new("id", "secret", uri)).is_ok(),
            "valid redirect URI {uri}"
        );
    }
}

#[test]
fn from_env_requires_config_vars() {
    let _lock = ENV_LOCK.lock().unwrap();

    let prev_id = with_env_var("AUTH_GOOGLE_CLIENT_ID", None);
    let prev_secret = with_env_var("AUTH_GOOGLE_CLIENT_SECRET", Some("secret"));
    let prev_redirect = with_env_var("AUTH_GOOGLE_REDIRECT_URI", Some("https://example.com/cb"));

    let result = GoogleOAuthService::from_env();
    assert!(matches!(
        result,
        Err(AuthError::Internal(_)) | Err(AuthError::OAuthError(_))
    ));

    restore_env_var("AUTH_GOOGLE_CLIENT_ID", prev_id);
    restore_env_var("AUTH_GOOGLE_CLIENT_SECRET", prev_secret);
    restore_env_var("AUTH_GOOGLE_REDIRECT_URI", prev_redirect);
}
