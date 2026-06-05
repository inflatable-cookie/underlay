use super::super::{AuthError, GoogleOAuthService};
use super::support::{restore_env_var, with_env_var, ENV_LOCK};

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
