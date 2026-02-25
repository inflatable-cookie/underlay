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

#[derive(Clone)]
struct StubProvider {
    token_set: TokenSet,
    userinfo: GoogleUserInfo,
}

#[async_trait::async_trait]
impl OAuthProvider for StubProvider {
    type UserInfo = GoogleUserInfo;

    fn start_login(&self) -> AuthResult<OAuthStart> {
        Ok(OAuthStart {
            authorization_url: "https://example.com/auth".to_string(),
            csrf_state: "state-1".to_string(),
            pkce_verifier: "verifier-1".to_string(),
        })
    }

    fn start_login_with(&self, csrf_state: &str, _pkce_verifier: &str) -> AuthResult<String> {
        Ok(format!("https://example.com/auth?state={csrf_state}"))
    }

    async fn exchange_code(&self, _code: &str, _pkce_verifier: &str) -> AuthResult<TokenSet> {
        Ok(self.token_set.clone())
    }

    async fn refresh(&self, _refresh_token: &str) -> AuthResult<TokenSet> {
        Ok(self.token_set.clone())
    }

    async fn fetch_userinfo(&self, _access_token: &str) -> AuthResult<Self::UserInfo> {
        Ok(self.userinfo.clone())
    }
}

struct MemoryRepo {
    users_by_email: tokio::sync::Mutex<std::collections::HashMap<String, User>>,
    oauth_by_user_id: tokio::sync::Mutex<std::collections::HashMap<Uuid, Credential>>,
}

impl MemoryRepo {
    fn new() -> Self {
        Self {
            users_by_email: tokio::sync::Mutex::new(std::collections::HashMap::new()),
            oauth_by_user_id: tokio::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl underlay_auth::UserRepository for MemoryRepo {
    async fn find_by_id(&self, _user_id: Uuid) -> underlay_auth::RepoResult<Option<User>> {
        Ok(None)
    }

    async fn find_by_email(&self, email: &str) -> underlay_auth::RepoResult<Option<User>> {
        Ok(self.users_by_email.lock().await.get(email).cloned())
    }

    async fn create(
        &self,
        email: &str,
        display_name: Option<&str>,
    ) -> underlay_auth::RepoResult<User> {
        let user = User {
            id: Uuid::new_v7(),
            email: email.to_string(),
            display_name: display_name.map(|s| s.to_string()),
            status: underlay_auth::UserStatus::Active,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.users_by_email
            .lock()
            .await
            .insert(email.to_string(), user.clone());

        Ok(user)
    }

    async fn update_status(
        &self,
        _user_id: Uuid,
        _status: underlay_auth::UserStatus,
    ) -> underlay_auth::RepoResult<()> {
        Ok(())
    }

    async fn delete(&self, _user_id: Uuid) -> underlay_auth::RepoResult<()> {
        Ok(())
    }

    async fn email_available(&self, _email: &str) -> underlay_auth::RepoResult<bool> {
        Ok(true)
    }
}

#[async_trait::async_trait]
impl underlay_auth::CredentialRepository for MemoryRepo {
    async fn find_by_id(
        &self,
        _credential_id: Uuid,
    ) -> underlay_auth::RepoResult<Option<Credential>> {
        Ok(None)
    }

    async fn find_by_user_id(&self, _user_id: Uuid) -> underlay_auth::RepoResult<Vec<Credential>> {
        Ok(vec![])
    }

    async fn find_password_by_user_id(
        &self,
        _user_id: Uuid,
    ) -> underlay_auth::RepoResult<Option<Credential>> {
        Ok(None)
    }

    async fn find_totp_by_user_id(
        &self,
        _user_id: Uuid,
    ) -> underlay_auth::RepoResult<Option<Credential>> {
        Ok(None)
    }

    async fn find_passkeys_by_user_id(
        &self,
        _user_id: Uuid,
    ) -> underlay_auth::RepoResult<Vec<Credential>> {
        Ok(vec![])
    }

    async fn find_oauth_by_user_and_provider(
        &self,
        user_id: Uuid,
        provider: CredentialType,
    ) -> underlay_auth::RepoResult<Option<Credential>> {
        if provider != CredentialType::OAuthGoogle {
            return Ok(None);
        }
        Ok(self.oauth_by_user_id.lock().await.get(&user_id).cloned())
    }

    async fn create(
        &self,
        user_id: Uuid,
        credential_type: CredentialType,
        secret_encrypted: &str,
        metadata: &CredentialMetadata,
    ) -> underlay_auth::RepoResult<Credential> {
        let now = chrono::Utc::now();
        let credential = Credential {
            id: Uuid::new_v7(),
            user_id,
            credential_type,
            secret_encrypted: secret_encrypted.to_string(),
            metadata: metadata.clone(),
            verified: true,
            created_at: now,
            updated_at: now,
            last_used_at: None,
        };

        if credential_type == CredentialType::OAuthGoogle {
            self.oauth_by_user_id
                .lock()
                .await
                .insert(user_id, credential.clone());
        }

        Ok(credential)
    }

    async fn set_verified(
        &self,
        _credential_id: Uuid,
        _verified: bool,
    ) -> underlay_auth::RepoResult<()> {
        Ok(())
    }

    async fn update_last_used(&self, _credential_id: Uuid) -> underlay_auth::RepoResult<()> {
        Ok(())
    }

    async fn delete(&self, credential_id: Uuid) -> underlay_auth::RepoResult<()> {
        let mut oauth = self.oauth_by_user_id.lock().await;
        if let Some((user_id, _)) = oauth
            .iter()
            .find(|(_k, v)| v.id == credential_id)
            .map(|(k, v)| (*k, v.clone()))
        {
            oauth.remove(&user_id);
        }
        Ok(())
    }

    async fn delete_all_for_user(&self, _user_id: Uuid) -> underlay_auth::RepoResult<()> {
        Ok(())
    }
}

#[tokio::test]
async fn handle_google_callback_creates_user_and_credential() {
    let repo = MemoryRepo::new();

    let provider = StubProvider {
        token_set: TokenSet {
            access_token: "access".to_string(),
            refresh_token: Some("refresh".to_string()),
            id_token: Some("id".to_string()),
            expires_in_seconds: Some(3600),
            scope: Some("openid email profile".to_string()),
            token_type: Some("Bearer".to_string()),
        },
        userinfo: GoogleUserInfo {
            sub: "google-sub".to_string(),
            email: Some("claire@example.com".to_string()),
            email_verified: Some(true),
            name: Some("Claire".to_string()),
            given_name: None,
            family_name: None,
            picture: Some("https://example.com/p.png".to_string()),
            locale: None,
        },
    };

    let svc = GoogleOAuthAppService::new(provider);

    let result = svc
        .handle_google_callback(
            &repo,
            OAuthCallbackRequest {
                code: "code".to_string(),
                state: "state-1".to_string(),
            },
            OAuthLoginState {
                csrf_state: "state-1".to_string(),
                pkce_verifier: "verifier".to_string(),
            },
            |secret| Ok(format!("enc:{secret}")),
        )
        .await
        .unwrap();

    assert!(result.is_new_user);
    assert_eq!(result.user.email, "claire@example.com");
    assert_eq!(result.user.display_name, Some("Claire".to_string()));
    assert_eq!(
        result.credential.credential_type,
        CredentialType::OAuthGoogle
    );
    assert!(result.credential.secret_encrypted.contains("enc:"));
}

#[tokio::test]
async fn handle_google_callback_rejects_state_mismatch() {
    let repo = MemoryRepo::new();

    let provider = StubProvider {
        token_set: TokenSet {
            access_token: "access".to_string(),
            refresh_token: None,
            id_token: None,
            expires_in_seconds: None,
            scope: None,
            token_type: None,
        },
        userinfo: GoogleUserInfo {
            sub: "google-sub".to_string(),
            email: Some("claire@example.com".to_string()),
            email_verified: Some(true),
            name: Some("Claire".to_string()),
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
        },
    };

    let svc = GoogleOAuthAppService::new(provider);

    let err = svc
        .handle_google_callback(
            &repo,
            OAuthCallbackRequest {
                code: "code".to_string(),
                state: "state-bad".to_string(),
            },
            OAuthLoginState {
                csrf_state: "state-expected".to_string(),
                pkce_verifier: "verifier".to_string(),
            },
            |_secret| Ok("enc".to_string()),
        )
        .await
        .unwrap_err();

    assert!(matches!(err, AuthError::BadRequest(_)));
}

#[tokio::test]
async fn handle_google_callback_requires_verified_email() {
    let repo = MemoryRepo::new();

    let provider = StubProvider {
        token_set: TokenSet {
            access_token: "access".to_string(),
            refresh_token: None,
            id_token: None,
            expires_in_seconds: None,
            scope: None,
            token_type: None,
        },
        userinfo: GoogleUserInfo {
            sub: "google-sub".to_string(),
            email: Some("claire@example.com".to_string()),
            email_verified: Some(false),
            name: Some("Claire".to_string()),
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
        },
    };

    let mut svc = GoogleOAuthAppService::new(provider);
    svc.require_verified_email = true;

    let err = svc
        .handle_google_callback(
            &repo,
            OAuthCallbackRequest {
                code: "code".to_string(),
                state: "state-1".to_string(),
            },
            OAuthLoginState {
                csrf_state: "state-1".to_string(),
                pkce_verifier: "verifier".to_string(),
            },
            |_secret| Ok("enc".to_string()),
        )
        .await
        .unwrap_err();

    assert!(matches!(err, AuthError::OAuthError(_)));
}

#[tokio::test]
async fn disconnect_google_removes_credential() {
    let repo = MemoryRepo::new();

    // Create user + credential via callback.
    let provider = StubProvider {
        token_set: TokenSet {
            access_token: "access".to_string(),
            refresh_token: Some("refresh".to_string()),
            id_token: None,
            expires_in_seconds: None,
            scope: None,
            token_type: None,
        },
        userinfo: GoogleUserInfo {
            sub: "google-sub".to_string(),
            email: Some("claire@example.com".to_string()),
            email_verified: Some(true),
            name: Some("Claire".to_string()),
            given_name: None,
            family_name: None,
            picture: None,
            locale: None,
        },
    };

    let svc = GoogleOAuthAppService::new(provider);
    let result = svc
        .handle_google_callback(
            &repo,
            OAuthCallbackRequest {
                code: "code".to_string(),
                state: "state-1".to_string(),
            },
            OAuthLoginState {
                csrf_state: "state-1".to_string(),
                pkce_verifier: "verifier".to_string(),
            },
            |secret| Ok(format!("enc:{secret}")),
        )
        .await
        .unwrap();

    assert!(repo
        .find_oauth_by_user_and_provider(result.user.id, CredentialType::OAuthGoogle)
        .await
        .unwrap()
        .is_some());

    svc.disconnect_google(&repo, result.user.id).await.unwrap();

    assert!(repo
        .find_oauth_by_user_and_provider(result.user.id, CredentialType::OAuthGoogle)
        .await
        .unwrap()
        .is_none());
}
