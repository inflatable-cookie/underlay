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

mod google;
mod token_cipher;

pub use google::GoogleOAuthService;
pub use token_cipher::{OAuthTokenCipher, AUTH_OAUTH_SECRET_KEY_ENV};

use serde::{Deserialize, Serialize};
use underlay_auth::{
    AuthError, AuthResult, Credential, CredentialMetadata, CredentialRepository, CredentialType,
    User, UserRepository,
};
use underlay_core::Uuid;

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

#[derive(Clone)]
pub struct GoogleOAuthConfig {
    client_id: String,
    client_secret: String,
    redirect_uri: String,

    /// Scopes to request. If empty, uses a secure default.
    scopes: Vec<String>,
}

impl std::fmt::Debug for GoogleOAuthConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GoogleOAuthConfig")
            .field("client_id", &self.client_id)
            .field("client_secret", &"[REDACTED]")
            .field("redirect_uri", &self.redirect_uri)
            .field("scopes", &self.scopes)
            .finish()
    }
}

impl GoogleOAuthConfig {
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.into(),
            scopes: Vec::new(),
        }
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn client_secret(&self) -> &str {
        &self.client_secret
    }

    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    pub fn scopes(&self) -> &[String] {
        &self.scopes
    }

    pub fn with_scopes(mut self, scopes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.scopes = scopes.into_iter().map(Into::into).collect();
        self
    }

    pub(crate) fn into_parts(self) -> (String, String, String, Vec<String>) {
        (
            self.client_id,
            self.client_secret,
            self.redirect_uri,
            self.scopes,
        )
    }
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

underlay_auth::impl_auth_error_from!(OAuthServiceError, err, {
    OAuthServiceError::InvalidConfig => {
        AuthError::Internal("invalid oauth configuration".into())
    }
    OAuthServiceError::ExchangeFailed => AuthError::OAuthError("exchange failed".into()),
    OAuthServiceError::UserInfoFailed => AuthError::OAuthError("userinfo failed".into()),
    OAuthServiceError::RefreshFailed => AuthError::OAuthTokenRefreshFailed,
});

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
pub struct GoogleOAuthAppService<P = GoogleOAuthService> {
    provider: P,

    /// Require `email_verified=true` from Google.
    require_verified_email: bool,
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

    pub fn require_verified_email(&self) -> bool {
        self.require_verified_email
    }

    pub fn with_require_verified_email(mut self, require: bool) -> Self {
        self.require_verified_email = require;
        self
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
                    UserRepository::create(repo, &email, Some(display_name.as_str())).await?,
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

#[cfg(test)]
#[path = "tests/lib_tests.rs"]
mod tests;
