//! JWT configuration.

use chrono::Duration;

use crate::error::{JwtError, JwtResult};
use crate::keys::KeyPair;

#[derive(Clone)]
pub struct JwtConfig {
    /// Base64-encoded PKCS#8 DER private key (Ed25519).
    pub private_key_b64: String,

    /// Base64-encoded raw Ed25519 public key bytes (32 bytes).
    pub public_key_b64: String,

    /// Access token lifetime in minutes.
    pub access_token_lifetime_minutes: i64,

    /// Refresh token lifetime in days.
    pub refresh_token_lifetime_days: i64,

    /// Token issuer.
    pub issuer: String,

    /// Token audience (optional).
    pub audience: Option<String>,

    /// Validation leeway in seconds.
    pub leeway_seconds: u64,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            private_key_b64: String::new(),
            public_key_b64: String::new(),
            access_token_lifetime_minutes: 15,
            refresh_token_lifetime_days: 30,
            issuer: "underlay".to_string(),
            audience: None,
            leeway_seconds: 30,
        }
    }
}

impl std::fmt::Debug for JwtConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtConfig")
            .field("private_key_b64", &"[REDACTED]")
            .field("public_key_b64", &"[REDACTED]")
            .field(
                "access_token_lifetime_minutes",
                &self.access_token_lifetime_minutes,
            )
            .field(
                "refresh_token_lifetime_days",
                &self.refresh_token_lifetime_days,
            )
            .field("issuer", &self.issuer)
            .field("audience", &self.audience)
            .field("leeway_seconds", &self.leeway_seconds)
            .finish()
    }
}

impl JwtConfig {
    pub fn access_token_lifetime(&self) -> Duration {
        Duration::minutes(self.access_token_lifetime_minutes)
    }

    pub fn refresh_token_lifetime(&self) -> Duration {
        Duration::days(self.refresh_token_lifetime_days)
    }

    pub fn from_env() -> JwtResult<Self> {
        let private_key_b64 = std::env::var("AUTH_JWT_PRIVATE_KEY")
            .map_err(|_| JwtError::Config("AUTH_JWT_PRIVATE_KEY not set".to_string()))?;
        let public_key_b64 = std::env::var("AUTH_JWT_PUBLIC_KEY")
            .map_err(|_| JwtError::Config("AUTH_JWT_PUBLIC_KEY not set".to_string()))?;

        let access_token_lifetime_minutes = std::env::var("AUTH_ACCESS_TOKEN_LIFETIME_MINUTES")
            .unwrap_or_else(|_| "15".to_string())
            .parse()
            .map_err(|_| {
                JwtError::Config("Invalid AUTH_ACCESS_TOKEN_LIFETIME_MINUTES".to_string())
            })?;

        let refresh_token_lifetime_days = std::env::var("AUTH_REFRESH_TOKEN_LIFETIME_DAYS")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .map_err(|_| {
                JwtError::Config("Invalid AUTH_REFRESH_TOKEN_LIFETIME_DAYS".to_string())
            })?;

        let issuer = std::env::var("AUTH_JWT_ISSUER").unwrap_or_else(|_| "underlay".to_string());
        let audience = std::env::var("AUTH_JWT_AUDIENCE").ok();

        let leeway_seconds = std::env::var("AUTH_JWT_LEEWAY_SECONDS")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .map_err(|_| JwtError::Config("Invalid AUTH_JWT_LEEWAY_SECONDS".to_string()))?;

        Ok(Self {
            private_key_b64,
            public_key_b64,
            access_token_lifetime_minutes,
            refresh_token_lifetime_days,
            issuer,
            audience,
            leeway_seconds,
        })
    }

    /// Generate a new keypair + config with defaults.
    pub fn generate() -> JwtResult<(Self, KeyPair)> {
        let key_pair = KeyPair::generate()?;
        let config = Self {
            private_key_b64: key_pair.private_key_pkcs8_der_b64.clone(),
            public_key_b64: key_pair.public_key_raw_b64.clone(),
            ..Self::default()
        };

        Ok((config, key_pair))
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
    fn from_env_requires_private_key() {
        let _lock = ENV_LOCK.lock().unwrap();

        let prev_priv = with_env_var("AUTH_JWT_PRIVATE_KEY", None);
        let prev_pub = with_env_var("AUTH_JWT_PUBLIC_KEY", Some("abc"));

        let result = JwtConfig::from_env();
        assert!(matches!(result, Err(JwtError::Config(_))));

        restore_env_var("AUTH_JWT_PRIVATE_KEY", prev_priv);
        restore_env_var("AUTH_JWT_PUBLIC_KEY", prev_pub);
    }

    #[test]
    fn from_env_requires_public_key() {
        let _lock = ENV_LOCK.lock().unwrap();

        let prev_priv = with_env_var("AUTH_JWT_PRIVATE_KEY", Some("abc"));
        let prev_pub = with_env_var("AUTH_JWT_PUBLIC_KEY", None);

        let result = JwtConfig::from_env();
        assert!(matches!(result, Err(JwtError::Config(_))));

        restore_env_var("AUTH_JWT_PRIVATE_KEY", prev_priv);
        restore_env_var("AUTH_JWT_PUBLIC_KEY", prev_pub);
    }

    #[test]
    fn from_env_parses_defaults_and_overrides() {
        let _lock = ENV_LOCK.lock().unwrap();

        let (generated, _keys) = JwtConfig::generate().unwrap();

        let prev_priv = with_env_var("AUTH_JWT_PRIVATE_KEY", Some(&generated.private_key_b64));
        let prev_pub = with_env_var("AUTH_JWT_PUBLIC_KEY", Some(&generated.public_key_b64));
        let prev_access = with_env_var("AUTH_ACCESS_TOKEN_LIFETIME_MINUTES", Some("42"));
        let prev_refresh = with_env_var("AUTH_REFRESH_TOKEN_LIFETIME_DAYS", Some("7"));
        let prev_issuer = with_env_var("AUTH_JWT_ISSUER", Some("issuer-x"));
        let prev_aud = with_env_var("AUTH_JWT_AUDIENCE", Some("aud-x"));
        let prev_leeway = with_env_var("AUTH_JWT_LEEWAY_SECONDS", Some("5"));

        let config = JwtConfig::from_env().unwrap();

        assert_eq!(config.private_key_b64, generated.private_key_b64);
        assert_eq!(config.public_key_b64, generated.public_key_b64);
        assert_eq!(config.access_token_lifetime_minutes, 42);
        assert_eq!(config.refresh_token_lifetime_days, 7);
        assert_eq!(config.issuer, "issuer-x");
        assert_eq!(config.audience, Some("aud-x".to_string()));
        assert_eq!(config.leeway_seconds, 5);

        restore_env_var("AUTH_JWT_PRIVATE_KEY", prev_priv);
        restore_env_var("AUTH_JWT_PUBLIC_KEY", prev_pub);
        restore_env_var("AUTH_ACCESS_TOKEN_LIFETIME_MINUTES", prev_access);
        restore_env_var("AUTH_REFRESH_TOKEN_LIFETIME_DAYS", prev_refresh);
        restore_env_var("AUTH_JWT_ISSUER", prev_issuer);
        restore_env_var("AUTH_JWT_AUDIENCE", prev_aud);
        restore_env_var("AUTH_JWT_LEEWAY_SECONDS", prev_leeway);
    }

    #[test]
    fn from_env_rejects_invalid_numbers() {
        let _lock = ENV_LOCK.lock().unwrap();

        let (generated, _keys) = JwtConfig::generate().unwrap();

        let prev_priv = with_env_var("AUTH_JWT_PRIVATE_KEY", Some(&generated.private_key_b64));
        let prev_pub = with_env_var("AUTH_JWT_PUBLIC_KEY", Some(&generated.public_key_b64));
        let prev_access = with_env_var("AUTH_ACCESS_TOKEN_LIFETIME_MINUTES", Some("nope"));

        let result = JwtConfig::from_env();
        assert!(
            matches!(result, Err(JwtError::Config(msg)) if msg.contains("AUTH_ACCESS_TOKEN_LIFETIME_MINUTES"))
        );

        restore_env_var("AUTH_JWT_PRIVATE_KEY", prev_priv);
        restore_env_var("AUTH_JWT_PUBLIC_KEY", prev_pub);
        restore_env_var("AUTH_ACCESS_TOKEN_LIFETIME_MINUTES", prev_access);
    }
}
