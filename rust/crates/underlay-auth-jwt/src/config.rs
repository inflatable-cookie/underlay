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

/// Non-secret JWT behavior tuning that can be supplied by typed app config.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JwtBehaviorDefaults {
    pub access_token_lifetime_minutes: i64,
    pub refresh_token_lifetime_days: i64,
    pub issuer: String,
    pub audience: Option<String>,
    pub leeway_seconds: u64,
}

impl Default for JwtBehaviorDefaults {
    fn default() -> Self {
        Self {
            access_token_lifetime_minutes: 15,
            refresh_token_lifetime_days: 30,
            issuer: "underlay".to_string(),
            audience: None,
            leeway_seconds: 30,
        }
    }
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            private_key_b64: String::new(),
            public_key_b64: String::new(),
            access_token_lifetime_minutes: JwtBehaviorDefaults::default()
                .access_token_lifetime_minutes,
            refresh_token_lifetime_days: JwtBehaviorDefaults::default().refresh_token_lifetime_days,
            issuer: JwtBehaviorDefaults::default().issuer,
            audience: JwtBehaviorDefaults::default().audience,
            leeway_seconds: JwtBehaviorDefaults::default().leeway_seconds,
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
        Self::from_env_with_defaults(&JwtBehaviorDefaults::default())
    }

    pub fn from_env_with_defaults(defaults: &JwtBehaviorDefaults) -> JwtResult<Self> {
        let private_key_b64 = std::env::var("AUTH_JWT_PRIVATE_KEY")
            .map_err(|_| JwtError::Config("AUTH_JWT_PRIVATE_KEY not set".to_string()))?;
        let public_key_b64 = std::env::var("AUTH_JWT_PUBLIC_KEY")
            .map_err(|_| JwtError::Config("AUTH_JWT_PUBLIC_KEY not set".to_string()))?;

        let access_token_lifetime_minutes = std::env::var("AUTH_ACCESS_TOKEN_LIFETIME_MINUTES")
            .ok()
            .map(|raw| {
                raw.parse().map_err(|_| {
                    JwtError::Config("Invalid AUTH_ACCESS_TOKEN_LIFETIME_MINUTES".to_string())
                })
            })
            .transpose()?
            .unwrap_or(defaults.access_token_lifetime_minutes);

        let refresh_token_lifetime_days = std::env::var("AUTH_REFRESH_TOKEN_LIFETIME_DAYS")
            .ok()
            .map(|raw| {
                raw.parse().map_err(|_| {
                    JwtError::Config("Invalid AUTH_REFRESH_TOKEN_LIFETIME_DAYS".to_string())
                })
            })
            .transpose()?
            .unwrap_or(defaults.refresh_token_lifetime_days);

        let issuer = std::env::var("AUTH_JWT_ISSUER").unwrap_or_else(|_| defaults.issuer.clone());
        let audience = std::env::var("AUTH_JWT_AUDIENCE")
            .ok()
            .or_else(|| defaults.audience.clone());

        let leeway_seconds = std::env::var("AUTH_JWT_LEEWAY_SECONDS")
            .ok()
            .map(|raw| {
                raw.parse()
                    .map_err(|_| JwtError::Config("Invalid AUTH_JWT_LEEWAY_SECONDS".to_string()))
            })
            .transpose()?
            .unwrap_or(defaults.leeway_seconds);

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

    pub fn from_values(
        private_key_b64: String,
        public_key_b64: String,
        behavior: JwtBehaviorDefaults,
    ) -> Self {
        Self {
            private_key_b64,
            public_key_b64,
            access_token_lifetime_minutes: behavior.access_token_lifetime_minutes,
            refresh_token_lifetime_days: behavior.refresh_token_lifetime_days,
            issuer: behavior.issuer,
            audience: behavior.audience,
            leeway_seconds: behavior.leeway_seconds,
        }
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
#[path = "tests/config_tests.rs"]
mod tests;
