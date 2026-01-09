use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;

use underlay_auth::{AuthError, AuthResult};

pub const AUTH_OAUTH_SECRET_KEY_ENV: &str = "AUTH_OAUTH_SECRET_KEY";

#[derive(Clone)]
pub struct OAuthTokenCipher {
    cipher: Aes256Gcm,
}

impl OAuthTokenCipher {
    pub fn from_env() -> AuthResult<Self> {
        Self::from_env_var(AUTH_OAUTH_SECRET_KEY_ENV)
    }

    pub fn from_env_optional() -> AuthResult<Option<Self>> {
        Self::from_env_var_optional(AUTH_OAUTH_SECRET_KEY_ENV)
    }

    pub fn from_env_var(key: &str) -> AuthResult<Self> {
        let value =
            std::env::var(key).map_err(|_| AuthError::Internal(format!("{key} must be set")))?;
        Self::from_key_string(key, &value)
    }

    pub fn from_env_var_optional(key: &str) -> AuthResult<Option<Self>> {
        let Ok(value) = std::env::var(key) else {
            return Ok(None);
        };

        Self::from_key_string(key, &value).map(Some)
    }

    fn from_key_string(key_name: &str, key_b64: &str) -> AuthResult<Self> {
        let key_bytes = decode_key_bytes(key_b64)
            .map_err(|_| AuthError::Internal(format!("invalid {key_name}")))?;

        if key_bytes.len() != 32 {
            return Err(AuthError::Internal(format!(
                "{key_name} must be 32 bytes (base64/base64url)"
            )));
        }

        let cipher = Aes256Gcm::new_from_slice(&key_bytes)
            .map_err(|_| AuthError::Internal("failed to initialize oauth cipher".into()))?;

        Ok(Self { cipher })
    }

    pub fn encrypt_refresh_token(&self, refresh_token: &str) -> AuthResult<String> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = self
            .cipher
            .encrypt(&nonce, refresh_token.as_bytes())
            .map_err(|_| AuthError::Internal("failed to encrypt oauth secret".into()))?;

        let nonce_b64 = URL_SAFE_NO_PAD.encode(nonce);
        let ct_b64 = URL_SAFE_NO_PAD.encode(ciphertext);
        Ok(format!("enc:v1:{nonce_b64}:{ct_b64}"))
    }

    pub fn decrypt_refresh_token(&self, secret_encrypted: &str) -> AuthResult<String> {
        if let Some(raw) = secret_encrypted.strip_prefix("plain:") {
            return Ok(raw.to_string());
        }

        let Some(rest) = secret_encrypted.strip_prefix("enc:v1:") else {
            return Err(AuthError::Internal("unknown oauth secret format".into()));
        };

        let (nonce_b64, ct_b64) = rest
            .split_once(':')
            .ok_or_else(|| AuthError::Internal("invalid oauth secret format".into()))?;

        let nonce_bytes = URL_SAFE_NO_PAD
            .decode(nonce_b64.as_bytes())
            .map_err(|_| AuthError::Internal("invalid oauth nonce".into()))?;

        if nonce_bytes.len() != 12 {
            return Err(AuthError::Internal("invalid oauth nonce".into()));
        }

        let nonce = Nonce::from_slice(&nonce_bytes);

        let ct = URL_SAFE_NO_PAD
            .decode(ct_b64.as_bytes())
            .map_err(|_| AuthError::Internal("invalid oauth ciphertext".into()))?;

        let plaintext = self
            .cipher
            .decrypt(nonce, ct.as_ref())
            .map_err(|_| AuthError::Internal("failed to decrypt oauth secret".into()))?;

        String::from_utf8(plaintext)
            .map_err(|_| AuthError::Internal("oauth secret not utf-8".into()))
    }
}

fn decode_key_bytes(key_b64: &str) -> Result<Vec<u8>, base64::DecodeError> {
    URL_SAFE_NO_PAD
        .decode(key_b64.as_bytes())
        .or_else(|_| STANDARD.decode(key_b64.as_bytes()))
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
    fn encrypt_is_nondeterministic_and_decrypts() {
        let key = vec![42u8; 32];
        let b64 = STANDARD.encode(&key);
        let cipher = OAuthTokenCipher::from_key_string("TEST_KEY", &b64).unwrap();

        let enc1 = cipher.encrypt_refresh_token("refresh-token").unwrap();
        let enc2 = cipher.encrypt_refresh_token("refresh-token").unwrap();
        assert_ne!(enc1, enc2);

        let dec = cipher.decrypt_refresh_token(&enc1).unwrap();
        assert_eq!(dec, "refresh-token");

        assert_eq!(cipher.decrypt_refresh_token("plain:abc").unwrap(), "abc");
    }

    #[test]
    fn from_env_optional_returns_none_when_unset() {
        let _lock = ENV_LOCK.lock().unwrap();
        let prev = with_env_var(AUTH_OAUTH_SECRET_KEY_ENV, None);
        let loaded = OAuthTokenCipher::from_env_optional().unwrap();
        assert!(loaded.is_none());
        restore_env_var(AUTH_OAUTH_SECRET_KEY_ENV, prev);
    }

    #[test]
    fn from_env_errors_on_wrong_length() {
        let _lock = ENV_LOCK.lock().unwrap();
        let prev = with_env_var(AUTH_OAUTH_SECRET_KEY_ENV, Some("AA=="));

        let err = OAuthTokenCipher::from_env().err();
        assert!(matches!(err, Some(AuthError::Internal(_))));

        restore_env_var(AUTH_OAUTH_SECRET_KEY_ENV, prev);
    }
}
