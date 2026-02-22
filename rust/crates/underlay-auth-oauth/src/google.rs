//! Google OAuth2 provider implementation.

use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl,
    Scope, TokenUrl,
};
use serde::{Deserialize, Serialize};
use underlay_auth::AuthResult;
use url::Url;

use crate::{
    GoogleOAuthConfig, GoogleUserInfo, OAuthProvider, OAuthServiceError, OAuthStart, TokenSet,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
    scope: Option<String>,
    token_type: Option<String>,
    id_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GoogleOAuthService {
    http: reqwest::Client,
    token_url: Url,
    userinfo_url: Url,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    scopes: Vec<Scope>,
}

impl GoogleOAuthService {
    pub fn new(config: GoogleOAuthConfig) -> AuthResult<Self> {
        let token_url_string = "https://oauth2.googleapis.com/token".to_string();
        let redirect_uri = config.redirect_uri;

        let client_id = config.client_id;
        let client_secret = config.client_secret;

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
        let client = BasicClient::new(ClientId::new(self.client_id.clone()))
            .set_client_secret(ClientSecret::new(self.client_secret.clone()))
            .set_auth_uri(
                AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                    .map_err(|_| OAuthServiceError::InvalidConfig)?,
            )
            .set_token_uri(
                TokenUrl::new(self.token_url.to_string())
                    .map_err(|_| OAuthServiceError::InvalidConfig)?,
            )
            .set_redirect_uri(
                RedirectUrl::new(self.redirect_uri.clone())
                    .map_err(|_| OAuthServiceError::InvalidConfig)?,
            );

        let (authorization_url, csrf_state) = client
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
        let client = BasicClient::new(ClientId::new(self.client_id.clone()))
            .set_client_secret(ClientSecret::new(self.client_secret.clone()))
            .set_auth_uri(
                AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                    .map_err(|_| OAuthServiceError::InvalidConfig)?,
            )
            .set_token_uri(
                TokenUrl::new(self.token_url.to_string())
                    .map_err(|_| OAuthServiceError::InvalidConfig)?,
            )
            .set_redirect_uri(
                RedirectUrl::new(self.redirect_uri.clone())
                    .map_err(|_| OAuthServiceError::InvalidConfig)?,
            );

        let (authorization_url, _csrf_state) = client
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
