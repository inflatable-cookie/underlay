use std::collections::HashMap;

use super::super::*;

pub(crate) static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub(crate) fn with_env_var(key: &str, value: Option<&str>) -> Option<String> {
    let previous = std::env::var(key).ok();
    match value {
        Some(value) => std::env::set_var(key, value),
        None => std::env::remove_var(key),
    }
    previous
}

pub(crate) fn restore_env_var(key: &str, previous: Option<String>) {
    match previous {
        Some(value) => std::env::set_var(key, value),
        None => std::env::remove_var(key),
    }
}

#[derive(Clone)]
pub(crate) struct StubProvider {
    pub(crate) token_set: TokenSet,
    pub(crate) userinfo: GoogleUserInfo,
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

pub(crate) struct MemoryRepo {
    users_by_email: tokio::sync::Mutex<HashMap<String, User>>,
    oauth_by_user_id: tokio::sync::Mutex<HashMap<Uuid, Credential>>,
}

impl MemoryRepo {
    pub(crate) fn new() -> Self {
        Self {
            users_by_email: tokio::sync::Mutex::new(HashMap::new()),
            oauth_by_user_id: tokio::sync::Mutex::new(HashMap::new()),
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

pub(crate) fn token_set_with_refresh() -> TokenSet {
    TokenSet {
        access_token: "access".to_string(),
        refresh_token: Some("refresh".to_string()),
        id_token: Some("id".to_string()),
        expires_in_seconds: Some(3600),
        scope: Some("openid email profile".to_string()),
        token_type: Some("Bearer".to_string()),
    }
}

pub(crate) fn basic_token_set() -> TokenSet {
    TokenSet {
        access_token: "access".to_string(),
        refresh_token: None,
        id_token: None,
        expires_in_seconds: None,
        scope: None,
        token_type: None,
    }
}

pub(crate) fn verified_userinfo() -> GoogleUserInfo {
    GoogleUserInfo {
        sub: "google-sub".to_string(),
        email: Some("claire@example.com".to_string()),
        email_verified: Some(true),
        name: Some("Claire".to_string()),
        given_name: None,
        family_name: None,
        picture: None,
        locale: None,
    }
}

pub(crate) fn unverified_userinfo() -> GoogleUserInfo {
    GoogleUserInfo {
        email_verified: Some(false),
        ..verified_userinfo()
    }
}

pub(crate) fn callback_request(state: &str) -> OAuthCallbackRequest {
    OAuthCallbackRequest {
        code: "code".to_string(),
        state: state.to_string(),
    }
}

pub(crate) fn login_state(state: &str) -> OAuthLoginState {
    OAuthLoginState {
        csrf_state: state.to_string(),
        pkce_verifier: "verifier".to_string(),
    }
}
