use std::sync::Arc;

use underlay_auth::hashing::{PasswordHasherExt, PasswordVerifierExt};
use underlay_ratelimit::RateLimitBackend;

use crate::strength::PasswordStrengthAnalyzer;

use super::{PasswordAuthRepository, PasswordConfig};

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
        let analyzer = PasswordStrengthAnalyzer::new().with_min_length(config.min_password_length);

        Self {
            repository,
            hasher,
            verifier,
            rate_limiter,
            analyzer: Arc::new(analyzer),
            config,
        }
    }
}
