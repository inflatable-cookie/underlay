use underlay_core::Uuid;

use crate::{
    AuthError, AuthResult, CredentialMetadata, CredentialRepository, CredentialType,
    GoogleOAuthService, GoogleUserInfo, OAuthCallbackRequest, OAuthLoginResult, OAuthLoginState,
    OAuthProvider, OAuthStart, TokenSet, UserRepository,
};

#[derive(Debug, Clone)]
pub struct GoogleOAuthAppService<P = GoogleOAuthService> {
    provider: P,
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
        if !constant_time_eq(&request.state, &stored_state.csrf_state) {
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

fn constant_time_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}

fn derive_display_name(userinfo: &GoogleUserInfo, email: &str) -> String {
    if let Some(name) = userinfo.name.as_deref() {
        if !name.trim().is_empty() {
            return name.trim().to_string();
        }
    }

    email.split('@').next().unwrap_or(email).trim().to_string()
}
