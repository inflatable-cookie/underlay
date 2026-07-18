use std::time::SystemTime;

use underlay_auth::{AuthError, AuthResult, CredentialMetadata};
use underlay_ratelimit::{RateLimitBackend, RateLimitConfig};

use crate::{TotpConfig, TotpSetup, TwoFactorCode, TwoFactorVerified};

#[derive(Debug, Clone)]
pub struct TotpService {
    pub(crate) config: TotpConfig,
}

impl TotpService {
    pub fn new(config: Option<TotpConfig>) -> Self {
        Self {
            config: config.unwrap_or_default(),
        }
    }

    pub fn config(&self) -> &TotpConfig {
        &self.config
    }

    pub fn metadata(&self) -> CredentialMetadata {
        CredentialMetadata::Totp {
            issuer: self.config.issuer().to_string(),
            algorithm: self.config.algorithm().as_str().to_string(),
            digits: self.config.digits() as u8,
            period: self.config.period_seconds() as u8,
        }
    }

    pub fn setup(&self, account_name: &str, backup_code_count: usize) -> AuthResult<TotpSetup> {
        let secret = self.generate_secret();
        let otpauth_uri = self.provisioning_uri(account_name, &secret.base32);
        let qr_svg = self.qr_svg(&otpauth_uri)?;
        let (backup_codes, backup_code_hashes) = self.generate_backup_codes(backup_code_count);

        Ok(TotpSetup {
            secret,
            otpauth_uri,
            qr_svg,
            backup_codes,
            backup_code_hashes,
            metadata: self.metadata(),
        })
    }

    /// Helper for app login flows: verify either a TOTP code or a backup code.
    ///
    /// Apps can call this after password verification when a user has 2FA enabled.
    pub fn verify_second_factor(
        &self,
        secret_base32: &str,
        last_counter: Option<u64>,
        input: TwoFactorCode<'_>,
        backup_code_hashes: &[String],
        now: SystemTime,
    ) -> AuthResult<TwoFactorVerified> {
        match input {
            TwoFactorCode::Totp(code) => {
                let verified = match last_counter {
                    Some(last) => self
                        .verify_totp_with_replay_protection(secret_base32, code, now, last)
                        .map_err(AuthError::from)?,
                    None => self
                        .verify_totp_at(secret_base32, code, now)
                        .map_err(AuthError::from)?,
                };
                Ok(TwoFactorVerified::Totp(verified))
            }
            TwoFactorCode::BackupCode(code) => {
                let index = self.verify_backup_code(code, backup_code_hashes)?;
                Ok(TwoFactorVerified::BackupCode { index })
            }
        }
    }

    /// Verify a second factor with a per-user attempt cap.
    ///
    /// Second-factor codes have a tiny keyspace (6 digits / short backup
    /// codes), so unlimited guesses are brute-forceable. This checks a rate
    /// limit keyed on `rate_limit_key` (use something like
    /// `"2fa:<user_id>"`) before verifying, and increments only on a failed
    /// attempt so legitimate success is never penalized. A successful verify
    /// resets the counter.
    #[allow(clippy::too_many_arguments)] // limiter + key + config + the full verify inputs
    pub async fn verify_second_factor_throttled<L: RateLimitBackend>(
        &self,
        limiter: &L,
        rate_limit_key: &str,
        rate_limit_config: &RateLimitConfig,
        secret_base32: &str,
        last_counter: Option<u64>,
        input: TwoFactorCode<'_>,
        backup_code_hashes: &[String],
        now: SystemTime,
    ) -> AuthResult<TwoFactorVerified> {
        let check = limiter
            .check(rate_limit_key, rate_limit_config)
            .await
            .map_err(|e| AuthError::Internal(format!("rate limit error: {e}")))?;
        if check.is_denied() {
            return Err(AuthError::RateLimited {
                retry_after_seconds: check.retry_after_secs(),
            });
        }

        match self.verify_second_factor(secret_base32, last_counter, input, backup_code_hashes, now)
        {
            Ok(verified) => {
                let _ = limiter.reset(rate_limit_key).await;
                Ok(verified)
            }
            Err(err) => {
                let _ = limiter.increment(rate_limit_key, rate_limit_config).await;
                Err(err)
            }
        }
    }
}
