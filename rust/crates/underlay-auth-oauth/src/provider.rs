use crate::{AuthResult, OAuthStart, TokenSet};

#[async_trait::async_trait]
pub trait OAuthProvider {
    type UserInfo;

    fn start_login(&self) -> AuthResult<OAuthStart>;

    fn start_login_with(&self, csrf_state: &str, pkce_verifier: &str) -> AuthResult<String>;

    async fn exchange_code(&self, code: &str, pkce_verifier: &str) -> AuthResult<TokenSet>;

    async fn refresh(&self, refresh_token: &str) -> AuthResult<TokenSet>;

    async fn fetch_userinfo(&self, access_token: &str) -> AuthResult<Self::UserInfo>;
}
