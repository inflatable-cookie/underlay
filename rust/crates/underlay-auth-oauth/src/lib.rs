//! OAuth2 primitives for Underlay-based apps.
//!
//! This crate provides an app-agnostic boundary around OAuth2 flows (Google first):
//! - builds authorization URLs (state + PKCE)
//! - exchanges authorization codes for tokens
//! - fetches user info from the provider
//! - refreshes tokens
//!
//! Apps remain responsible for:
//! - persisting state/PKCE verifier between start/callback steps (server-side)
//! - storing refresh tokens and linking to users/credentials
//! - user creation/linking policy and sessions

mod token_cipher;

pub use token_cipher::{OAuthTokenCipher, AUTH_OAUTH_SECRET_KEY_ENV};

use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl,
    Scope, TokenUrl,
};
use serde::{Deserialize, Serialize};
use underlay_auth::{
    AuthError, AuthResult, Credential, CredentialMetadata, CredentialRepository, CredentialType,
    User, UserRepository,
};
use underlay_core::Uuid;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSet {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub expires_in_seconds: Option<u64>,
    pub scope: Option<String>,
    pub token_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthStart {
    pub authorization_url: String,
    pub csrf_state: String,
    pub pkce_verifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthCallbackRequest {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthLoginState {
    pub csrf_state: String,
    pub pkce_verifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthLoginResult {
    pub user: User,
    pub is_new_user: bool,
    pub credential: Credential,
    pub token_set: TokenSet,
    pub userinfo: GoogleUserInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub locale: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GoogleOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,

    /// Scopes to request. If empty, uses a secure default.
    pub scopes: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum OAuthServiceError {
    #[error("invalid oauth configuration")]
    InvalidConfig,
    #[error("oauth exchange failed")]
    ExchangeFailed,
    #[error("oauth userinfo failed")]
    UserInfoFailed,
    #[error("oauth refresh failed")]
    RefreshFailed,
}

impl From<OAuthServiceError> for AuthError {
    fn from(value: OAuthServiceError) -> Self {
        match value {
            OAuthServiceError::InvalidConfig => {
                AuthError::Internal("invalid oauth configuration".into())
            }
            OAuthServiceError::ExchangeFailed => AuthError::OAuthError("exchange failed".into()),
            OAuthServiceError::UserInfoFailed => AuthError::OAuthError("userinfo failed".into()),
            OAuthServiceError::RefreshFailed => AuthError::OAuthTokenRefreshFailed,
        }
    }
}

#[async_trait::async_trait]
pub trait OAuthProvider {
    type UserInfo;

    fn start_login(&self) -> AuthResult<OAuthStart>;

    fn start_login_with(&self, csrf_state: &str, pkce_verifier: &str) -> AuthResult<String>;

    async fn exchange_code(&self, code: &str, pkce_verifier: &str) -> AuthResult<TokenSet>;

    async fn refresh(&self, refresh_token: &str) -> AuthResult<TokenSet>;

    async fn fetch_userinfo(&self, access_token: &str) -> AuthResult<Self::UserInfo>;
}

#[derive(Debug, Clone)]
pub struct GoogleOAuthService {
    client: BasicClient,
    http: reqwest::Client,
    token_url: Url,
    userinfo_url: Url,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    scopes: Vec<Scope>,
}

#[derive(Debug, Clone)]
pub struct GoogleOAuthAppService<P = GoogleOAuthService> {
    provider: P,

    /// Require `email_verified=true` from Google.
    pub require_verified_email: bool,
}

impl<P> GoogleOAuthAppService<P>
where
    P: OAuthProvider<UserInfo = GoogleUserInfo> + Send + Sync,
{
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            require_verified_email: true,
        }
    }

    pub fn initiate_google_login(&self) -> AuthResult<OAuthStart> {
        self.provider.start_login()
    }

    pub fn initiate_google_login_with(
        &self,
        csrf_state: &str,
        pkce_verifier: &str,
    ) -> AuthResult<String> {
        self.provider.start_login_with(csrf_state, pkce_verifier)
    }

    pub async fn refresh_google_token(&self, refresh_token: &str) -> AuthResult<TokenSet> {
        self.provider.refresh(refresh_token).await
    }

    pub async fn disconnect_google<R>(&self, repo: &R, user_id: Uuid) -> AuthResult<()>
    where
        R: UserRepository + CredentialRepository,
    {
        let existing = repo
            .find_oauth_by_user_and_provider(user_id, CredentialType::OAuthGoogle)
            .await?;

        let Some(existing) = existing else {
            return Err(AuthError::OAuthNotConnected);
        };

        CredentialRepository::delete(repo, existing.id).await?;
        Ok(())
    }

    pub async fn handle_google_callback<R, E>(
        &self,
        repo: &R,
        request: OAuthCallbackRequest,
        stored_state: OAuthLoginState,
        encrypt_secret: E,
    ) -> AuthResult<OAuthLoginResult>
    where
        R: UserRepository + CredentialRepository,
        E: FnOnce(&str) -> AuthResult<String>,
    {
        if request.state != stored_state.csrf_state {
            return Err(AuthError::BadRequest("invalid oauth state".into()));
        }

        let token_set = self
            .provider
            .exchange_code(&request.code, &stored_state.pkce_verifier)
            .await?;

        let userinfo = self
            .provider
            .fetch_userinfo(&token_set.access_token)
            .await?;

        let email = userinfo
            .email
            .clone()
            .ok_or_else(|| AuthError::OAuthError("google userinfo missing email".into()))?;

        if self.require_verified_email && userinfo.email_verified != Some(true) {
            return Err(AuthError::OAuthError(
                "google userinfo email not verified".into(),
            ));
        }

        let (user, is_new_user) = match repo.find_by_email(&email).await? {
            Some(existing) => (existing, false),
            None => {
                let display_name = derive_display_name(&userinfo, &email);
                (
                    UserRepository::create(repo, &email, &display_name).await?,
                    true,
                )
            }
        };

        let existing_oauth = repo
            .find_oauth_by_user_and_provider(user.id, CredentialType::OAuthGoogle)
            .await?;
        if existing_oauth.is_some() {
            return Err(AuthError::OAuthAlreadyConnected);
        }

        let refresh_token = token_set.refresh_token.clone().unwrap_or_default();
        let secret_encrypted = encrypt_secret(&refresh_token)?;

        let scopes = token_set
            .scope
            .as_deref()
            .unwrap_or("")
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let metadata = CredentialMetadata::OAuthGoogle {
            google_user_id: userinfo.sub.clone(),
            scopes,
        };

        let credential = CredentialRepository::create(
            repo,
            user.id,
            CredentialType::OAuthGoogle,
            &secret_encrypted,
            &metadata,
        )
        .await?;

        Ok(OAuthLoginResult {
            user,
            is_new_user,
            credential,
            token_set,
            userinfo,
        })
    }
}

fn derive_display_name(userinfo: &GoogleUserInfo, email: &str) -> String {
    if let Some(name) = userinfo.name.as_deref() {
        if !name.trim().is_empty() {
            return name.trim().to_string();
        }
    }

    email.split('@').next().unwrap_or(email).trim().to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
    scope: Option<String>,
    token_type: Option<String>,
    id_token: Option<String>,
}

impl GoogleOAuthService {
    pub fn new(config: GoogleOAuthConfig) -> AuthResult<Self> {
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .map_err(|_| OAuthServiceError::InvalidConfig)?;

        let token_url_string = "https://oauth2.googleapis.com/token".to_string();
        let token_url = TokenUrl::new(token_url_string.clone())
            .map_err(|_| OAuthServiceError::InvalidConfig)?;

        let redirect_uri = config.redirect_uri;
        let redirect_url =
            RedirectUrl::new(redirect_uri.clone()).map_err(|_| OAuthServiceError::InvalidConfig)?;

        let client_id = config.client_id;
        let client_secret = config.client_secret;

        let client = BasicClient::new(
            ClientId::new(client_id.clone()),
            Some(ClientSecret::new(client_secret.clone())),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(redirect_url);

        let token_url =
            Url::parse(&token_url_string).map_err(|_| OAuthServiceError::InvalidConfig)?;
        let userinfo_url = Url::parse("https://openidconnect.googleapis.com/v1/userinfo")
            .map_err(|_| OAuthServiceError::InvalidConfig)?;

        let scopes = if config.scopes.is_empty() {
            vec![
                Scope::new("openid".to_string()),
                Scope::new("email".to_string()),
                Scope::new("profile".to_string()),
            ]
        } else {
            config.scopes.into_iter().map(Scope::new).collect()
        };

        Ok(Self {
            client,
            http: reqwest::Client::new(),
            token_url,
            userinfo_url,
            client_id,
            client_secret,
            redirect_uri,
            scopes,
        })
    }

    pub fn from_env() -> AuthResult<Self> {
        let client_id =
            std::env::var("AUTH_GOOGLE_CLIENT_ID").map_err(|_| OAuthServiceError::InvalidConfig)?;
        let client_secret = std::env::var("AUTH_GOOGLE_CLIENT_SECRET")
            .map_err(|_| OAuthServiceError::InvalidConfig)?;
        let redirect_uri = std::env::var("AUTH_GOOGLE_REDIRECT_URI")
            .map_err(|_| OAuthServiceError::InvalidConfig)?;

        Self::new(GoogleOAuthConfig {
            client_id,
            client_secret,
            redirect_uri,
            scopes: vec![],
        })
    }

    fn to_tokenset(token: GoogleTokenResponse) -> TokenSet {
        TokenSet {
            access_token: token.access_token,
            refresh_token: token.refresh_token,
            id_token: token.id_token,
            expires_in_seconds: token.expires_in,
            scope: token.scope,
            token_type: token.token_type,
        }
    }
}

#[async_trait::async_trait]
impl OAuthProvider for GoogleOAuthService {
    type UserInfo = GoogleUserInfo;

    fn start_login(&self) -> AuthResult<OAuthStart> {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (authorization_url, csrf_state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.scopes.clone())
            .set_pkce_challenge(pkce_challenge)
            .url();

        Ok(OAuthStart {
            authorization_url: authorization_url.to_string(),
            csrf_state: csrf_state.secret().to_string(),
            pkce_verifier: pkce_verifier.secret().to_string(),
        })
    }

    fn start_login_with(&self, csrf_state: &str, pkce_verifier: &str) -> AuthResult<String> {
        let pkce_verifier = PkceCodeVerifier::new(pkce_verifier.to_string());
        let pkce_challenge = PkceCodeChallenge::from_code_verifier_sha256(&pkce_verifier);

        let (authorization_url, _csrf_state) = self
            .client
            .authorize_url(|| CsrfToken::new(csrf_state.to_string()))
            .add_scopes(self.scopes.clone())
            .set_pkce_challenge(pkce_challenge)
            .url();

        Ok(authorization_url.to_string())
    }

    async fn exchange_code(&self, code: &str, pkce_verifier: &str) -> AuthResult<TokenSet> {
        let resp = self
            .http
            .post(self.token_url.clone())
            .form(&[
                ("code", code),
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("redirect_uri", self.redirect_uri.as_str()),
                ("grant_type", "authorization_code"),
                ("code_verifier", pkce_verifier),
            ])
            .send()
            .await
            .map_err(|_| OAuthServiceError::ExchangeFailed)?;

        if !resp.status().is_success() {
            return Err(OAuthServiceError::ExchangeFailed.into());
        }

        let token = resp
            .json::<GoogleTokenResponse>()
            .await
            .map_err(|_| OAuthServiceError::ExchangeFailed)?;

        Ok(GoogleOAuthService::to_tokenset(token))
    }

    async fn refresh(&self, refresh_token: &str) -> AuthResult<TokenSet> {
        let resp = self
            .http
            .post(self.token_url.clone())
            .form(&[
                ("refresh_token", refresh_token),
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("grant_type", "refresh_token"),
            ])
            .send()
            .await
            .map_err(|_| OAuthServiceError::RefreshFailed)?;

        if !resp.status().is_success() {
            return Err(OAuthServiceError::RefreshFailed.into());
        }

        let token = resp
            .json::<GoogleTokenResponse>()
            .await
            .map_err(|_| OAuthServiceError::RefreshFailed)?;

        Ok(GoogleOAuthService::to_tokenset(token))
    }

    async fn fetch_userinfo(&self, access_token: &str) -> AuthResult<Self::UserInfo> {
        let resp = self
            .http
            .get(self.userinfo_url.clone())
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|_| OAuthServiceError::UserInfoFailed)?;

        if !resp.status().is_success() {
            return Err(OAuthServiceError::UserInfoFailed.into());
        }

        resp.json::<GoogleUserInfo>()
            .await
            .map_err(|_| OAuthServiceError::UserInfoFailed.into())
    }
}

#[cfg(test)]
mod tests {
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
        let prev_redirect =
            with_env_var("AUTH_GOOGLE_REDIRECT_URI", Some("https://example.com/cb"));

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

        async fn create(&self, email: &str, display_name: &str) -> underlay_auth::RepoResult<User> {
            let user = User {
                id: Uuid::new_v7(),
                email: email.to_string(),
                display_name: display_name.to_string(),
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

        async fn find_by_user_id(
            &self,
            _user_id: Uuid,
        ) -> underlay_auth::RepoResult<Vec<Credential>> {
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
        assert_eq!(result.user.display_name, "Claire");
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
}
