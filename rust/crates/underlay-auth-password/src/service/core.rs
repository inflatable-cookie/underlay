use std::sync::Arc;
use std::time::Duration;

use underlay_auth::hashing::{PasswordHasherExt, PasswordVerifierExt};
use underlay_ratelimit::{RateLimitBackend, RateLimitConfig};

use crate::errors::{PasswordAuthError, PasswordAuthResult};
use crate::strength::PasswordStrengthAnalyzer;

use super::{PasswordAuthRepository, PasswordConfig};
#[cfg(feature = "hibp")]
use crate::hibp::HttpHibpRangeClient;

/// Service for password-based authentication.
#[derive(Debug, Clone)]
pub struct PasswordAuthService<R, H, V, L>
where
    R: PasswordAuthRepository,
    H: PasswordHasherExt,
    V: PasswordVerifierExt,
    L: RateLimitBackend,
{
    pub(super) repository: Arc<R>,
    pub(super) hasher: Arc<H>,
    pub(super) verifier: Arc<V>,
    pub(super) rate_limiter: Arc<L>,
    pub(super) analyzer: Arc<PasswordStrengthAnalyzer>,
    pub(super) config: PasswordConfig,
    #[cfg(feature = "hibp")]
    pub(super) hibp_client: Option<PasswordAuthResult<HttpHibpRangeClient>>,
}

impl<R, H, V, L> PasswordAuthService<R, H, V, L>
where
    R: PasswordAuthRepository,
    H: PasswordHasherExt + 'static,
    V: PasswordVerifierExt + 'static,
    L: RateLimitBackend,
{
    pub(super) async fn hash_password(&self, password: &str) -> PasswordAuthResult<String> {
        let hasher = Arc::clone(&self.hasher);
        let password = password.as_bytes().to_vec();
        tokio::task::spawn_blocking(move || hasher.hash_password(&password))
            .await
            .map_err(|e| PasswordAuthError::Internal(format!("Password hashing task failed: {e}")))?
            .map_err(PasswordAuthError::from)
    }

    pub(super) async fn verify_password(
        &self,
        password: &str,
        hash: &str,
    ) -> PasswordAuthResult<bool> {
        let verifier = Arc::clone(&self.verifier);
        let password = password.as_bytes().to_vec();
        let hash = hash.to_owned();
        tokio::task::spawn_blocking(move || verifier.verify_password(&password, &hash))
            .await
            .map_err(|e| {
                PasswordAuthError::Internal(format!("Password verification task failed: {e}"))
            })?
            .map_err(PasswordAuthError::from)
    }

    pub(super) async fn check_rate_limit(
        &self,
        key: &str,
        max_attempts: u32,
    ) -> PasswordAuthResult<()> {
        let window = Duration::from_secs(self.config.rate_limit_window_seconds());
        let config = RateLimitConfig::new(u64::from(max_attempts), window);
        let result = self
            .rate_limiter
            .check_and_increment(key, &config)
            .await
            .map_err(|e| PasswordAuthError::Internal(format!("Rate limit error: {e}")))?;

        if result.is_denied() {
            return Err(PasswordAuthError::RateLimited {
                retry_after_seconds: result.retry_after_secs(),
            });
        }
        Ok(())
    }
}

impl<R, H, V, L> PasswordAuthService<R, H, V, L>
where
    R: PasswordAuthRepository,
    H: PasswordHasherExt,
    V: PasswordVerifierExt,
    L: RateLimitBackend,
{
    /// Create a new password auth service.
    pub fn new(
        repository: Arc<R>,
        hasher: Arc<H>,
        verifier: Arc<V>,
        rate_limiter: Arc<L>,
        config: Option<PasswordConfig>,
    ) -> Self {
        let config = config.unwrap_or_default();
        let analyzer =
            PasswordStrengthAnalyzer::new().with_min_length(config.min_password_length());
        #[cfg(feature = "hibp")]
        let hibp_client = match (
            config.check_compromised(),
            config.compromised_password_strategy(),
        ) {
            (
                true,
                super::CompromisedPasswordStrategy::HibpKAnonymity {
                    api_base_url,
                    user_agent,
                },
            ) => Some(HttpHibpRangeClient::new(api_base_url, user_agent)),
            _ => None,
        };

        Self {
            repository,
            hasher,
            verifier,
            rate_limiter,
            analyzer: Arc::new(analyzer),
            config,
            #[cfg(feature = "hibp")]
            hibp_client,
        }
    }
}
