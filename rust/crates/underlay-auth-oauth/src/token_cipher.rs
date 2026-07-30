use underlay_auth::{AuthResult, SecretCipher};

pub const AUTH_OAUTH_SECRET_KEY_ENV: &str = "AUTH_OAUTH_SECRET_KEY";

/// OAuth refresh-token cipher. Thin wrapper over the canonical
/// [`SecretCipher`]; the `enc:v1:` wire format is identical.
#[derive(Debug, Clone)]
pub struct OAuthTokenCipher {
    inner: SecretCipher,
}

impl OAuthTokenCipher {
    pub fn from_env() -> AuthResult<Self> {
        Self::from_env_var(AUTH_OAUTH_SECRET_KEY_ENV)
    }

    pub fn from_env_optional() -> AuthResult<Option<Self>> {
        Self::from_env_var_optional(AUTH_OAUTH_SECRET_KEY_ENV)
    }

    pub fn from_env_var(key: &str) -> AuthResult<Self> {
        SecretCipher::from_env_var(key).map(|inner| Self { inner })
    }

    pub fn from_env_var_optional(key: &str) -> AuthResult<Option<Self>> {
        SecretCipher::from_env_var_optional(key).map(|opt| opt.map(|inner| Self { inner }))
    }

    #[cfg(test)]
    fn from_key_string(key_name: &str, key_b64: &str) -> AuthResult<Self> {
        SecretCipher::from_key_string(key_name, key_b64).map(|inner| Self { inner })
    }

    /// Opt into reading legacy `plain:`-prefixed secrets during a migration.
    ///
    /// Use only while re-encrypting stored plaintext tokens; leave off in
    /// steady state so a `plain:` value is rejected rather than trusted.
    pub fn with_plain_migration(mut self, allow: bool) -> Self {
        self.inner = self.inner.with_plain_migration(allow);
        self
    }

    pub fn encrypt_refresh_token(&self, refresh_token: &str) -> AuthResult<String> {
        self.inner.encrypt(refresh_token)
    }

    pub fn decrypt_refresh_token(&self, secret_encrypted: &str) -> AuthResult<String> {
        self.inner.decrypt(secret_encrypted)
    }
}

#[cfg(test)]
#[path = "tests/token_cipher_tests.rs"]
mod tests;
