//! Password authentication service.

use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use underlay_core::Uuid;

use underlay_auth::{AuthError, Credential, CredentialMetadata, User, UserStatus};
use underlay_ratelimit::{RateLimitBackend, RateLimitConfig};

use crate::errors::{PasswordAuthError, PasswordAuthResult};
use crate::strength::PasswordStrengthAnalyzer;
use underlay_auth::hashing::{PasswordHasherExt, PasswordVerifierExt};

/// Configuration for password authentication.
#[derive(Debug, Clone)]
pub struct PasswordConfig {
    /// Maximum failed login attempts before lockout.
    pub max_failed_attempts: u32,
    /// Lockout duration in seconds.
    pub lockout_duration_seconds: u64,
    /// Rate limit window in seconds.
    pub rate_limit_window_seconds: u64,
    /// Maximum attempts per rate limit window.
    pub rate_limit_max_attempts: u32,
    /// Minimum password length.
    pub min_password_length: usize,
    /// Whether to check compromised passwords.
    pub check_compromised: bool,
    /// Strategy for checking if a password is compromised.
    pub compromised_password_strategy: CompromisedPasswordStrategy,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            max_failed_attempts: 5,
            lockout_duration_seconds: 900,
            rate_limit_window_seconds: 3600,
            rate_limit_max_attempts: 10,
            min_password_length: 8,
            check_compromised: false,
            compromised_password_strategy: CompromisedPasswordStrategy::LocalBlocklist,
        }
    }
}

/// Strategy for compromised-password checks.
#[derive(Debug, Clone)]
pub enum CompromisedPasswordStrategy {
    /// Offline-only: local blocklist of extremely common passwords.
    ///
    /// This is fast and has no network dependency, but does not detect all breached passwords.
    LocalBlocklist,

    /// Online (optional): HIBP Pwned Passwords k-anonymity range API.
    ///
    /// This sends only the first 5 hex chars of the SHA-1 password hash.
    #[cfg(feature = "hibp")]
    HibpKAnonymity {
        /// Base URL for the API.
        ///
        /// Example: `https://api.pwnedpasswords.com`.
        api_base_url: String,
        /// User agent string for HTTP requests.
        user_agent: String,
    },
}

/// Repository for user and credential operations.
#[async_trait]
pub trait PasswordAuthRepository: Send + Sync {
    /// Find a user by email.
    async fn find_user_by_email(&self, email: &str) -> PasswordAuthResult<Option<User>>;

    /// Find a user by ID.
    async fn find_user_by_id(&self, user_id: Uuid) -> PasswordAuthResult<Option<User>>;

    /// Find a password credential for a user.
    async fn find_password_credential(
        &self,
        user_id: Uuid,
    ) -> PasswordAuthResult<Option<Credential>>;

    /// Create a password credential for a user.
    async fn create_password_credential(
        &self,
        user_id: Uuid,
        password_hash: &str,
    ) -> PasswordAuthResult<Credential>;

    /// Update a password credential.
    async fn update_password_credential(
        &self,
        credential_id: Uuid,
        password_hash: &str,
    ) -> PasswordAuthResult<()>;

    /// Delete a password credential.
    async fn delete_password_credential(&self, credential_id: Uuid) -> PasswordAuthResult<()>;

    /// Record a failed login attempt.
    ///
    /// Implementations should increment failure counters and, when the threshold is reached,
    /// apply a lockout and return the remaining lockout seconds.
    async fn record_failed_login(
        &self,
        user_id: Uuid,
        max_failed_attempts: u32,
        lockout_duration_seconds: u64,
    ) -> PasswordAuthResult<FailedLoginAttempt>;

    /// Reset failed login attempts.
    async fn reset_failed_logins(&self, user_id: Uuid) -> PasswordAuthResult<()>;

    /// Get failed login count for a user.
    async fn get_failed_login_count(&self, user_id: Uuid) -> PasswordAuthResult<u32>;

    /// Returns remaining lockout seconds, or `None` if not locked out.
    async fn get_lockout_remaining_seconds(&self, user_id: Uuid)
        -> PasswordAuthResult<Option<u64>>;
}

/// Result of recording a failed login attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FailedLoginAttempt {
    pub count: u32,
    pub lockout_remaining_seconds: Option<u64>,
}

/// Service for password-based authentication.
#[derive(Debug, Clone)]
pub struct PasswordAuthService<R, H, V, L>
where
    R: PasswordAuthRepository,
    H: PasswordHasherExt,
    V: PasswordVerifierExt,
    L: RateLimitBackend,
{
    repository: Arc<R>,
    hasher: Arc<H>,
    verifier: Arc<V>,
    rate_limiter: Arc<L>,
    analyzer: Arc<PasswordStrengthAnalyzer>,
    config: PasswordConfig,
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

    async fn is_compromised_password(&self, password: &str) -> PasswordAuthResult<bool> {
        if !self.config.check_compromised {
            return Ok(false);
        }

        match &self.config.compromised_password_strategy {
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

    /// Create or replace a user's password credential.
    pub async fn set_password(
        &self,
        user_id: Uuid,
        password: &str,
    ) -> PasswordAuthResult<Credential> {
        let analysis = self.analyzer.analyze(password);

        if self.is_compromised_password(password).await? {
            return Err(PasswordAuthError::PasswordCompromised.into());
        }

        if analysis.strength < crate::strength::PasswordStrength::Fair {
            return Err(PasswordAuthError::PasswordTooWeak(analysis.feedback.join(". ")).into());
        }

        let new_hash = self.hasher.hash_password(password.as_bytes())?;

        if let Some(existing) = self.repository.find_password_credential(user_id).await? {
            if self
                .verifier
                .verify_password(password.as_bytes(), &existing.secret_encrypted)?
            {
                return Err(PasswordAuthError::PasswordSameAsCurrent.into());
            }

            self.repository
                .update_password_credential(existing.id, &new_hash)
                .await?;

            let updated = self
                .repository
                .find_password_credential(user_id)
                .await?
                .ok_or(PasswordAuthError::Internal(
                    "Password credential missing after update".to_string(),
                ))?;

            Ok(updated)
        } else {
            self.repository
                .create_password_credential(user_id, &new_hash)
                .await
        }
    }

    /// Verify login credentials.
    pub async fn verify_login(&self, email: &str, password: &str) -> PasswordAuthResult<User> {
        self.verify_login_with_context(email, password, None).await
    }

    /// Verify login credentials with an optional context (e.g. IP address).
    pub async fn verify_login_with_context(
        &self,
        email: &str,
        password: &str,
        ip: Option<&str>,
    ) -> PasswordAuthResult<User> {
        let normalized_email = email.trim().to_lowercase();
        let rate_limit_key = match ip {
            Some(ip) => format!("login:{}:{}", normalized_email, ip),
            None => format!("login:{}", normalized_email),
        };

        // Check rate limit using the injected backend
        let rate_limit_config = RateLimitConfig::new(
            self.config.rate_limit_max_attempts as u64,
            Duration::from_secs(self.config.rate_limit_window_seconds),
        );
        let rate_result = self
            .rate_limiter
            .check_and_increment(&rate_limit_key, &rate_limit_config)
            .await
            .map_err(|e| PasswordAuthError::Internal(format!("Rate limit error: {}", e)))?;

        if rate_result.is_denied() {
            return Err(PasswordAuthError::RateLimited {
                retry_after_seconds: rate_result.retry_after_secs(),
            }
            .into());
        }

        let user = match self
            .repository
            .find_user_by_email(&normalized_email)
            .await?
        {
            Some(u) => u,
            None => return Err(PasswordAuthError::CredentialNotFound),
        };

        if user.status == UserStatus::Suspended {
            return Err(AuthError::AccountSuspended.into());
        }
        if user.status == UserStatus::Deleted {
            return Err(AuthError::AccountDeleted.into());
        }

        if let Some(retry_after_seconds) = self
            .repository
            .get_lockout_remaining_seconds(user.id)
            .await?
        {
            return Err(PasswordAuthError::AccountLocked {
                retry_after_seconds,
            }
            .into());
        }

        let credential = match self.repository.find_password_credential(user.id).await? {
            Some(c) => c,
            None => return Err(PasswordAuthError::CredentialNotFound),
        };

        let password_hash = match &credential.metadata {
            CredentialMetadata::Password { .. } => &credential.secret_encrypted,
            _ => {
                return Err(
                    PasswordAuthError::Internal("Invalid credential type".to_string()).into(),
                );
            }
        };

        let is_valid = self
            .verifier
            .verify_password(password.as_bytes(), password_hash)?;

        if is_valid {
            self.repository.reset_failed_logins(user.id).await?;
            Ok(user)
        } else {
            let attempt = self
                .repository
                .record_failed_login(
                    user.id,
                    self.config.max_failed_attempts,
                    self.config.lockout_duration_seconds,
                )
                .await?;

            if let Some(retry_after_seconds) = attempt.lockout_remaining_seconds {
                Err(PasswordAuthError::AccountLocked {
                    retry_after_seconds,
                }
                .into())
            } else if attempt.count >= self.config.max_failed_attempts {
                Err(PasswordAuthError::AccountLocked {
                    retry_after_seconds: self.config.lockout_duration_seconds,
                }
                .into())
            } else {
                Err(PasswordAuthError::WrongPassword)
            }
        }
    }

    /// Change a user's password.
    pub async fn change_password(
        &self,
        user_id: Uuid,
        current_password: &str,
        new_password: &str,
    ) -> PasswordAuthResult<()> {
        let analysis = self.analyzer.analyze(new_password);

        if self.is_compromised_password(new_password).await? {
            return Err(PasswordAuthError::PasswordCompromised.into());
        }

        if analysis.strength < crate::strength::PasswordStrength::Fair {
            return Err(PasswordAuthError::PasswordTooWeak(analysis.feedback.join(". ")).into());
        }

        let credential = match self.repository.find_password_credential(user_id).await? {
            Some(c) => c,
            None => return Err(PasswordAuthError::CredentialNotFound.into()),
        };

        let current_hash = &credential.secret_encrypted;
        let is_valid = self
            .verifier
            .verify_password(current_password.as_bytes(), current_hash)?;

        if !is_valid {
            return Err(PasswordAuthError::WrongPassword);
        }

        if self
            .verifier
            .verify_password(new_password.as_bytes(), current_hash)?
        {
            return Err(PasswordAuthError::PasswordSameAsCurrent.into());
        }

        let new_hash = self.hasher.hash_password(new_password.as_bytes())?;

        self.repository
            .update_password_credential(credential.id, &new_hash)
            .await?;

        Ok(())
    }

    /// Reset a user's password (admin/internal use).
    pub async fn reset_password(
        &self,
        user_id: Uuid,
        new_password: &str,
    ) -> PasswordAuthResult<()> {
        let analysis = self.analyzer.analyze(new_password);

        if self.is_compromised_password(new_password).await? {
            return Err(PasswordAuthError::PasswordCompromised.into());
        }

        if analysis.strength < crate::strength::PasswordStrength::Fair {
            return Err(PasswordAuthError::PasswordTooWeak(analysis.feedback.join(". ")).into());
        }

        let credential = match self.repository.find_password_credential(user_id).await? {
            Some(c) => c,
            None => return Err(PasswordAuthError::CredentialNotFound.into()),
        };

        let new_hash = self.hasher.hash_password(new_password.as_bytes())?;

        self.repository
            .update_password_credential(credential.id, &new_hash)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
#[path = "service_tests.rs"]
mod tests;
