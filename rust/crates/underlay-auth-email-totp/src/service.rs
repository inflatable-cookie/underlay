//! Email TOTP verification service.
//!
//! This service provides email-based one-time password verification.
//! It handles code generation, rate limiting, and verification sessions.

use chrono::{Duration, Utc};
use tracing::{info, instrument, warn};
use underlay_auth_password::{Argon2Hasher, PasswordHasherExt, PasswordVerifierExt};

use crate::code::generate_code;
use crate::config::EmailTotpConfig;
use crate::error::{EmailTotpError, EmailTotpResult};
use crate::repository::{
    EmailTotpCodeRepository, VerificationSession, VerificationSessionRepository,
};

/// Verification method identifier for email TOTP.
pub const VERIFICATION_METHOD_EMAIL_TOTP: &str = "email_totp";

/// Email sender trait for sending verification codes.
///
/// Implement this trait to integrate with your email infrastructure.
#[async_trait::async_trait]
pub trait EmailTotpSender: Send + Sync {
    /// Send a verification code email.
    ///
    /// # Arguments
    /// * `to_email` - The recipient's email address
    /// * `code` - The verification code to send
    /// * `purpose` - The purpose of verification (for email template selection)
    /// * `expiry_minutes` - How long the code is valid for
    async fn send_code(
        &self,
        to_email: &str,
        code: &str,
        purpose: &str,
        expiry_minutes: i32,
    ) -> EmailTotpResult<()>;
}

/// Email TOTP verification service.
///
/// This service coordinates email-based OTP verification using:
/// - A code repository for storing verification codes
/// - A session repository for storing verification sessions
/// - An email sender for delivering codes
///
/// # Example
///
/// ```rust,ignore
/// use underlay_auth_email_totp::{EmailTotpService, EmailTotpConfig};
///
/// let service = EmailTotpService::new(
///     code_repository,
///     session_repository,
///     email_sender,
///     EmailTotpConfig::default(),
/// );
///
/// // Request a code
/// let expiry = service.request_code("user-123", "user@example.com", "password_change").await?;
///
/// // Verify the code (creates a verification session)
/// let session = service.verify_code("user-123", "password_change", "123456").await?;
///
/// // Consume the session when performing the sensitive action
/// let consumed = service.consume_session(&session.id, "user-123", "password_change").await?;
/// ```
pub struct EmailTotpService<C, S, E>
where
    C: EmailTotpCodeRepository,
    S: VerificationSessionRepository,
    E: EmailTotpSender,
{
    code_repository: C,
    session_repository: S,
    email_sender: E,
    hasher: Argon2Hasher,
    config: EmailTotpConfig,
}

impl<C, S, E> EmailTotpService<C, S, E>
where
    C: EmailTotpCodeRepository,
    S: VerificationSessionRepository,
    E: EmailTotpSender,
{
    /// Create a new email TOTP service.
    pub fn new(
        code_repository: C,
        session_repository: S,
        email_sender: E,
        config: EmailTotpConfig,
    ) -> Self {
        Self {
            code_repository,
            session_repository,
            email_sender,
            hasher: Argon2Hasher::default(),
            config,
        }
    }

    /// Get the service configuration.
    pub fn config(&self) -> &EmailTotpConfig {
        &self.config
    }

    /// Request a new verification code.
    ///
    /// Generates a code, stores its hash, and sends it via email.
    /// Returns the code's expiry time.
    ///
    /// # Arguments
    /// * `user_id` - The user requesting the code
    /// * `email` - The email address to send the code to
    /// * `purpose` - The purpose of verification (e.g., "password_change", "login")
    ///
    /// # Errors
    /// * `EmailTotpError::RateLimited` - If the user has requested too many codes
    /// * `EmailTotpError::EmailSendFailed` - If the email could not be sent
    #[instrument(skip(self, email), fields(purpose = %purpose))]
    pub async fn request_code(
        &self,
        user_id: &str,
        email: &str,
        purpose: &str,
    ) -> EmailTotpResult<chrono::DateTime<chrono::Utc>> {
        // Check rate limit
        let rate_limit = self
            .code_repository
            .check_rate_limit(user_id, purpose, self.config.max_codes_per_hour)
            .await?;

        if rate_limit.is_limited {
            warn!(
                user_id = %user_id,
                send_count = rate_limit.send_count,
                attempt_count = rate_limit.attempt_count,
                "Email TOTP rate limit exceeded"
            );
            return Err(EmailTotpError::RateLimited);
        }

        // Generate code
        let code = generate_code(self.config.code_length);

        // Hash the code for storage
        let code_hash = self
            .hasher
            .hash_password(code.as_bytes())
            .map_err(|e| EmailTotpError::Storage(format!("failed to hash code: {}", e)))?;

        // Calculate expiry
        let expires_at = Utc::now() + Duration::minutes(self.config.code_expiry_minutes as i64);

        // Store the code
        self.code_repository
            .store_code(
                user_id,
                email,
                &code_hash,
                purpose,
                expires_at,
                self.config.max_attempts,
            )
            .await?;

        // Send the email
        self.email_sender
            .send_code(email, &code, purpose, self.config.code_expiry_minutes)
            .await?;

        // Increment send count after successful send
        self.code_repository
            .increment_send_count(user_id, purpose)
            .await?;

        info!(
            user_id = %user_id,
            purpose = %purpose,
            expires_at = %expires_at,
            "Email TOTP code sent"
        );

        Ok(expires_at)
    }

    /// Verify a code and create a verification session.
    ///
    /// # Arguments
    /// * `user_id` - The user verifying the code
    /// * `purpose` - The purpose of verification
    /// * `code` - The code to verify
    ///
    /// # Errors
    /// * `EmailTotpError::NoActiveCode` - If no active code exists
    /// * `EmailTotpError::CodeExpired` - If the code has expired
    /// * `EmailTotpError::TooManyAttempts` - If too many verification attempts
    /// * `EmailTotpError::InvalidCode` - If the code is incorrect
    #[instrument(skip(self, code), fields(purpose = %purpose))]
    pub async fn verify_code(
        &self,
        user_id: &str,
        purpose: &str,
        code: &str,
    ) -> EmailTotpResult<VerificationSession> {
        // Get the active code
        let stored_code = self
            .code_repository
            .get_active_code(user_id, purpose)
            .await?
            .ok_or(EmailTotpError::NoActiveCode)?;

        // Check expiry
        if stored_code.expires_at < Utc::now() {
            return Err(EmailTotpError::CodeExpired);
        }

        // Check attempts
        if stored_code.attempts >= stored_code.max_attempts {
            return Err(EmailTotpError::TooManyAttempts);
        }

        // Verify the code
        let is_valid = self
            .hasher
            .verify_password(code.trim().as_bytes(), &stored_code.code_hash)
            .unwrap_or(false);

        if !is_valid {
            // Increment attempts
            let new_attempts = self
                .code_repository
                .increment_attempts(&stored_code.id)
                .await?;

            warn!(
                user_id = %user_id,
                attempts = new_attempts,
                max_attempts = stored_code.max_attempts,
                "Invalid email TOTP code attempt"
            );

            return Err(EmailTotpError::InvalidCode);
        }

        // Mark code as used
        self.code_repository.mark_code_used(&stored_code.id).await?;

        // Create verification session
        let session_expires_at =
            Utc::now() + Duration::minutes(self.config.session_expiry_minutes as i64);

        let session = self
            .session_repository
            .create_session(
                user_id,
                purpose,
                VERIFICATION_METHOD_EMAIL_TOTP,
                session_expires_at,
            )
            .await?;

        info!(
            user_id = %user_id,
            session_id = %session.id,
            purpose = %purpose,
            "Email TOTP verification successful"
        );

        Ok(session)
    }

    /// Verify a code without creating a verification session.
    ///
    /// Use this for login flows where you don't need a verification session.
    #[instrument(skip(self, code), fields(purpose = %purpose))]
    pub async fn verify_code_only(
        &self,
        user_id: &str,
        purpose: &str,
        code: &str,
    ) -> EmailTotpResult<()> {
        // Get the active code
        let stored_code = self
            .code_repository
            .get_active_code(user_id, purpose)
            .await?
            .ok_or(EmailTotpError::NoActiveCode)?;

        // Check expiry
        if stored_code.expires_at < Utc::now() {
            return Err(EmailTotpError::CodeExpired);
        }

        // Check attempts
        if stored_code.attempts >= stored_code.max_attempts {
            return Err(EmailTotpError::TooManyAttempts);
        }

        // Verify the code
        let is_valid = self
            .hasher
            .verify_password(code.trim().as_bytes(), &stored_code.code_hash)
            .unwrap_or(false);

        if !is_valid {
            // Increment attempts
            let new_attempts = self
                .code_repository
                .increment_attempts(&stored_code.id)
                .await?;

            warn!(
                user_id = %user_id,
                attempts = new_attempts,
                max_attempts = stored_code.max_attempts,
                "Invalid email TOTP code attempt"
            );

            return Err(EmailTotpError::InvalidCode);
        }

        // Mark code as used
        self.code_repository.mark_code_used(&stored_code.id).await?;

        info!(
            user_id = %user_id,
            purpose = %purpose,
            "Email TOTP verification for login successful"
        );

        Ok(())
    }

    /// Consume a verification session.
    ///
    /// This is a one-time operation - the session cannot be consumed again.
    ///
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `user_id` - The user ID (must match the session's user)
    /// * `purpose` - The purpose (must match the session's purpose)
    #[instrument(skip(self))]
    pub async fn consume_session(
        &self,
        session_id: &str,
        user_id: &str,
        purpose: &str,
    ) -> EmailTotpResult<VerificationSession> {
        self.session_repository
            .consume_session(session_id, user_id, purpose)
            .await
    }

    /// Get a verification session without consuming it.
    ///
    /// Returns None if the session doesn't exist, is expired, or already used.
    pub async fn get_session(
        &self,
        session_id: &str,
        user_id: &str,
        purpose: &str,
    ) -> EmailTotpResult<Option<VerificationSession>> {
        self.session_repository
            .get_session(session_id, user_id, purpose)
            .await
    }
}
