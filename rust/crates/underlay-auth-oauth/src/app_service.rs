use underlay_core::Uuid;

use crate::{
    AuthError, AuthResult, Credential, CredentialMetadata, CredentialRepository, CredentialType,
    GoogleOAuthService, GoogleUserInfo, OAuthCallbackRequest, OAuthLoginResult, OAuthLoginState,
    OAuthProvider, OAuthStart, TokenSet, UserRepository,
};

/// Stored in the credential secret field for Google sign-ins without offline access.
pub const NO_GOOGLE_REFRESH_TOKEN: &str = "oauth:no-refresh-token";

/// Persistence for resolving and refreshing a linked Google credential.
///
/// The store must enforce uniqueness per user and per Google subject for
/// Google credentials, and reject conflicting creates.
#[async_trait::async_trait]
pub trait GoogleOAuthCredentialRepository: CredentialRepository {
    /// Find an `OAuthGoogle` credential by the Google subject stored in its metadata.
    async fn find_google_credential_by_subject(
        &self,
        google_user_id: &str,
    ) -> AuthResult<Option<Credential>>;

    /// Atomically replace the encrypted secret and return the updated row.
    /// Match the credential ID, Google type, and stored Google subject.
    /// Return `None` if the matching link no longer exists.
    async fn replace_google_refresh_token(
        &self,
        credential_id: Uuid,
        google_user_id: &str,
        secret_encrypted: &str,
    ) -> AuthResult<Option<Credential>>;
}

#[derive(Debug, Clone)]
pub struct GoogleOAuthAppService<P = GoogleOAuthService> {
    provider: P,
    require_verified_email: bool,
    auto_link_by_verified_email: bool,
}

impl<P> GoogleOAuthAppService<P>
where
    P: OAuthProvider<UserInfo = GoogleUserInfo> + Send + Sync,
{
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            require_verified_email: true,
            auto_link_by_verified_email: false,
        }
    }

    pub fn require_verified_email(&self) -> bool {
        self.require_verified_email
    }

    pub fn with_require_verified_email(mut self, require: bool) -> Self {
        self.require_verified_email = require;
        self
    }

    /// Allow linking existing accounts by a Google-verified email address.
    ///
    /// Use this only when the application trusts Google to prove ownership of
    /// the local account's email address. The provider email must be verified
    /// even when `with_require_verified_email(false)` is configured.
    #[must_use]
    pub fn with_verified_email_auto_linking(mut self) -> Self {
        self.auto_link_by_verified_email = true;
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
        R: UserRepository + GoogleOAuthCredentialRepository,
        E: FnOnce(&str) -> AuthResult<String>,
    {
        if stored_state.csrf_state.trim().is_empty()
            || !constant_time_eq(&request.state, &stored_state.csrf_state)
        {
            return Err(AuthError::BadRequest("invalid oauth state".into()));
        }

        if request.code.trim().is_empty() {
            return Err(AuthError::BadRequest("missing oauth code".into()));
        }

        if stored_state.pkce_verifier.trim().is_empty() {
            return Err(AuthError::BadRequest("missing oauth pkce verifier".into()));
        }

        let token_set = self
            .provider
            .exchange_code(&request.code, &stored_state.pkce_verifier)
            .await?;

        let userinfo = self
            .provider
            .fetch_userinfo(&token_set.access_token)
            .await?;

        if userinfo.sub.trim().is_empty() {
            return Err(AuthError::OAuthError(
                "google userinfo missing subject".into(),
            ));
        }

        if self.require_verified_email && userinfo.email_verified != Some(true) {
            return Err(AuthError::OAuthError(
                "google userinfo email not verified".into(),
            ));
        }

        let linked_credential = repo
            .find_google_credential_by_subject(&userinfo.sub)
            .await?;
        let (user, is_new_user) = if let Some(credential) = &linked_credential {
            let user = UserRepository::find_by_id(repo, credential.user_id)
                .await?
                .ok_or(AuthError::UserNotFound)?;
            (user, false)
        } else {
            let email = userinfo
                .email
                .as_deref()
                .map(|email| email.trim().to_ascii_lowercase())
                .filter(|email| !email.is_empty())
                .ok_or_else(|| AuthError::OAuthError("google userinfo missing email".into()))?;
            match repo.find_by_email(&email).await? {
                Some(existing)
                    if self.auto_link_by_verified_email
                        && userinfo.email_verified == Some(true) =>
                {
                    (existing, false)
                }
                Some(_) => {
                    return Err(AuthError::OAuthError(
                        "sign in to the existing account before linking Google".into(),
                    ));
                }
                None => {
                    let display_name = derive_display_name(&userinfo, &email);
                    (
                        UserRepository::create(repo, &email, Some(display_name.as_str())).await?,
                        true,
                    )
                }
            }
        };

        if user.is_suspended() {
            return Err(AuthError::AccountSuspended);
        }
        if user.is_deleted() {
            return Err(AuthError::AccountDeleted);
        }

        let existing_oauth = match linked_credential {
            Some(credential) => Some(credential),
            None => {
                repo.find_oauth_by_user_and_provider(user.id, CredentialType::OAuthGoogle)
                    .await?
            }
        };
        let refresh_token = token_set
            .refresh_token
            .as_deref()
            .filter(|token| !token.is_empty());

        if let Some(credential) = existing_oauth {
            if credential.credential_type != CredentialType::OAuthGoogle
                || !matches!(
                    &credential.metadata,
                    CredentialMetadata::OAuthGoogle { google_user_id, .. }
                        if *google_user_id == userinfo.sub
                )
            {
                return Err(AuthError::OAuthError(
                    "google account does not match the linked account".into(),
                ));
            }

            let credential = match refresh_token {
                Some(token) => {
                    let secret_encrypted = encrypt_secret(token)?;
                    repo.replace_google_refresh_token(
                        credential.id,
                        &userinfo.sub,
                        &secret_encrypted,
                    )
                    .await?
                    .ok_or(AuthError::OAuthNotConnected)?
                }
                None => credential,
            };

            return Ok(OAuthLoginResult {
                user,
                is_new_user,
                credential,
                token_set,
                userinfo,
            });
        }

        let secret_encrypted = refresh_token.map(encrypt_secret).transpose()?;

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
            secret_encrypted
                .as_deref()
                .unwrap_or(NO_GOOGLE_REFRESH_TOKEN),
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
