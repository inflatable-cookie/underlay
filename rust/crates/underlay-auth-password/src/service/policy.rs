use underlay_auth::hashing::{PasswordHasherExt, PasswordVerifierExt};
use underlay_ratelimit::RateLimitBackend;

use crate::errors::{PasswordAuthError, PasswordAuthResult};
use crate::strength::MIN_PASSWORD_STRENGTH;

use super::core::MAX_PASSWORD_BYTES;
use super::{PasswordAuthRepository, PasswordAuthService};

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

        #[cfg(feature = "hibp")]
        if let Some(client) = &self.hibp_client {
            let client = client.as_ref().map_err(Clone::clone)?;
            return crate::hibp::hibp_k_anonymity_check_with_client(password, client).await;
        }

        Ok(self.analyzer.is_common_password(password))
    }

    pub(super) async fn validate_new_password(&self, password: &str) -> PasswordAuthResult<()> {
        if password.len() > MAX_PASSWORD_BYTES {
            return Err(PasswordAuthError::PasswordTooWeak(format!(
                "Password must be at most {MAX_PASSWORD_BYTES} bytes"
            )));
        }

        let analysis = self.analyzer.analyze(password);

        if self.is_compromised_password(password).await? {
            return Err(PasswordAuthError::PasswordCompromised);
        }

        if analysis.strength < MIN_PASSWORD_STRENGTH {
            let feedback = if analysis.feedback.is_empty() {
                "Password is too weak".to_string()
            } else {
                analysis.feedback.join(". ")
            };
            return Err(PasswordAuthError::PasswordTooWeak(feedback));
        }

        Ok(())
    }
}
