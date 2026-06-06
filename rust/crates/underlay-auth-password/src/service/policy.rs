use underlay_auth::hashing::{PasswordHasherExt, PasswordVerifierExt};
use underlay_ratelimit::RateLimitBackend;

use crate::errors::{PasswordAuthError, PasswordAuthResult};
use crate::strength::PasswordStrength;

use super::{CompromisedPasswordStrategy, PasswordAuthRepository, PasswordAuthService};

impl<R, H, V, L> PasswordAuthService<R, H, V, L>
where
    R: PasswordAuthRepository,
    H: PasswordHasherExt,
    V: PasswordVerifierExt,
    L: RateLimitBackend,
{
    pub(super) async fn is_compromised_password(&self, password: &str) -> PasswordAuthResult<bool> {
        if !self.config.check_compromised() {
            return Ok(false);
        }

        match self.config.compromised_password_strategy() {
            CompromisedPasswordStrategy::LocalBlocklist => {
                Ok(self.analyzer.is_common_password(password))
            }
            #[cfg(feature = "hibp")]
            CompromisedPasswordStrategy::HibpKAnonymity {
                api_base_url,
                user_agent,
            } => crate::hibp::hibp_k_anonymity_check(password, api_base_url, user_agent).await,
        }
    }

    pub(super) async fn validate_new_password(&self, password: &str) -> PasswordAuthResult<()> {
        let analysis = self.analyzer.analyze(password);

        if self.is_compromised_password(password).await? {
            return Err(PasswordAuthError::PasswordCompromised);
        }

        if analysis.strength < PasswordStrength::Fair {
            return Err(PasswordAuthError::PasswordTooWeak(
                analysis.feedback.join(". "),
            ));
        }

        Ok(())
    }
}
