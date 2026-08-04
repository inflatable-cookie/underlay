//! Versioned AES-256-GCM cipher for secrets at rest (TOTP seeds, OAuth
//! tokens, email codes — any credential persisted to a database).
//!
//! Format: `enc:v1:{nonce_b64url}:{ciphertext_b64url}` with a random 96-bit
//! nonce per encryption. Decryption fails closed: unknown formats and
//! `plain:`-prefixed legacy values are rejected unless the caller explicitly
//! opts into a bounded migration window.

use aes_gcm::aead::{Aead, Generate, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;

use crate::{AuthError, AuthResult};

const FORMAT_PREFIX: &str = "enc:v1:";
const PLAIN_PREFIX: &str = "plain:";

/// Generic cipher for secrets at rest. Clone freely; the key never leaves
/// the struct and is redacted from `Debug`.
#[derive(Clone)]
pub struct SecretCipher {
    cipher: Aes256Gcm,
    /// When true, `plain:`-prefixed secrets are read back as-is. Off by
    /// default so plaintext secrets are never silently accepted; enable only
    /// for a bounded migration window that re-encrypts legacy rows.
    allow_plain_migration: bool,
}

impl SecretCipher {
    /// Load the key from a base64/base64url-encoded 32-byte env var.
    pub fn from_env_var(key: &str) -> AuthResult<Self> {
        let value =
            std::env::var(key).map_err(|_| AuthError::Internal(format!("{key} must be set")))?;
        Self::from_key_string(key, &value)
    }

    /// Like [`Self::from_env_var`], but returns `None` when the var is unset.
    pub fn from_env_var_optional(key: &str) -> AuthResult<Option<Self>> {
        let Ok(value) = std::env::var(key) else {
            return Ok(None);
        };

        Self::from_key_string(key, &value).map(Some)
    }

    /// Build from a base64/base64url-encoded 32-byte key (e.g. from a vault).
    pub fn from_key_string(key_name: &str, key_b64: &str) -> AuthResult<Self> {
        let key_bytes = decode_key_bytes(key_b64)
            .map_err(|_| AuthError::Internal(format!("invalid {key_name}")))?;

        if key_bytes.len() != 32 {
            return Err(AuthError::Internal(format!(
                "{key_name} must be 32 bytes (base64/base64url)"
            )));
        }

        let cipher = Aes256Gcm::new_from_slice(&key_bytes)
            .map_err(|_| AuthError::Internal("failed to initialize secret cipher".into()))?;

        Ok(Self {
            cipher,
            allow_plain_migration: false,
        })
    }

    /// Generate a fresh base64url-encoded 32-byte key suitable for
    /// [`Self::from_key_string`].
    pub fn generate_key() -> String {
        let key = Key::<Aes256Gcm>::generate_from_rng(&mut rand::rng());
        URL_SAFE_NO_PAD.encode(key)
    }

    /// Opt into reading legacy `plain:`-prefixed secrets during a migration.
    ///
    /// Use only while re-encrypting stored plaintext secrets; leave off in
    /// steady state so a `plain:` value is rejected rather than trusted.
    pub fn with_plain_migration(mut self, allow: bool) -> Self {
        self.allow_plain_migration = allow;
        self
    }

    /// Returns true if the value carries the current encrypted format prefix.
    pub fn is_encrypted(value: &str) -> bool {
        value.starts_with(FORMAT_PREFIX)
    }

    pub fn encrypt(&self, secret: &str) -> AuthResult<String> {
        let nonce = aes_gcm::aead::Nonce::<Aes256Gcm>::generate_from_rng(&mut rand::rng());
        let ciphertext = self
            .cipher
            .encrypt(&nonce, secret.as_bytes())
            .map_err(|_| AuthError::Internal("failed to encrypt secret".into()))?;

        let nonce_b64 = URL_SAFE_NO_PAD.encode(nonce);
        let ct_b64 = URL_SAFE_NO_PAD.encode(ciphertext);
        Ok(format!("{FORMAT_PREFIX}{nonce_b64}:{ct_b64}"))
    }

    pub fn decrypt(&self, secret_encrypted: &str) -> AuthResult<String> {
        if let Some(raw) = secret_encrypted.strip_prefix(PLAIN_PREFIX) {
            if self.allow_plain_migration {
                return Ok(raw.to_string());
            }
            return Err(AuthError::Internal(
                "plaintext secret rejected (enable plain migration explicitly)".into(),
            ));
        }

        let Some(rest) = secret_encrypted.strip_prefix(FORMAT_PREFIX) else {
            return Err(AuthError::Internal("unknown secret format".into()));
        };

        let (nonce_b64, ct_b64) = rest
            .split_once(':')
            .ok_or_else(|| AuthError::Internal("invalid secret format".into()))?;

        let nonce_bytes = URL_SAFE_NO_PAD
            .decode(nonce_b64.as_bytes())
            .map_err(|_| AuthError::Internal("invalid secret nonce".into()))?;

        let nonce_arr: [u8; 12] = nonce_bytes
            .try_into()
            .map_err(|_| AuthError::Internal("invalid secret nonce".into()))?;
        let nonce = Nonce::from(nonce_arr);

        let ct = URL_SAFE_NO_PAD
            .decode(ct_b64.as_bytes())
            .map_err(|_| AuthError::Internal("invalid secret ciphertext".into()))?;

        let plaintext = self
            .cipher
            .decrypt(&nonce, ct.as_ref())
            .map_err(|_| AuthError::Internal("failed to decrypt secret".into()))?;

        String::from_utf8(plaintext).map_err(|_| AuthError::Internal("secret not utf-8".into()))
    }
}

impl std::fmt::Debug for SecretCipher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SecretCipher")
            .field("cipher", &"[REDACTED]")
            .field("allow_plain_migration", &self.allow_plain_migration)
            .finish()
    }
}

fn decode_key_bytes(key_b64: &str) -> Result<Vec<u8>, base64::DecodeError> {
    URL_SAFE_NO_PAD
        .decode(key_b64.as_bytes())
        .or_else(|_| STANDARD.decode(key_b64.as_bytes()))
}

#[cfg(test)]
#[path = "tests/secret_cipher_tests.rs"]
mod tests;
