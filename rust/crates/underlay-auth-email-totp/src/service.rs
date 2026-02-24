//! Email TOTP verification service.
//!
//! This service provides email-based one-time password verification.
//! It handles code generation, rate limiting, and verification sessions.

use chrono::{Duration, Utc};
use tracing::{info, instrument, warn};
use underlay_auth::hashing::{Argon2Hasher, PasswordHasherExt, PasswordVerifierExt};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::{RateLimitStatus, StoredCode, VerificationSession};
    use chrono::{DateTime, Duration};
    use std::sync::Mutex;
    use underlay_auth::hashing::{Argon2Hasher, PasswordHasherExt};

    #[derive(Debug)]
    struct MockCodeRepoState {
        rate_limit: RateLimitStatus,
        active_code: Option<StoredCode>,
        increment_attempts_result: i32,
        stored: Vec<(String, String, String, DateTime<Utc>, i32)>,
        increment_send_calls: usize,
        increment_attempt_calls: Vec<String>,
        marked_used: Vec<String>,
    }

    impl Default for MockCodeRepoState {
        fn default() -> Self {
            Self {
                rate_limit: RateLimitStatus {
                    send_count: 0,
                    attempt_count: 0,
                    is_limited: false,
                },
                active_code: None,
                increment_attempts_result: 0,
                stored: Vec::new(),
                increment_send_calls: 0,
                increment_attempt_calls: Vec::new(),
                marked_used: Vec::new(),
            }
        }
    }

    struct MockCodeRepo {
        state: Mutex<MockCodeRepoState>,
    }

    impl MockCodeRepo {
        fn new(state: MockCodeRepoState) -> Self {
            Self {
                state: Mutex::new(state),
            }
        }
    }

    #[async_trait::async_trait]
    impl EmailTotpCodeRepository for MockCodeRepo {
        async fn check_rate_limit(
            &self,
            _user_id: &str,
            _purpose: &str,
            _max_per_hour: i32,
        ) -> EmailTotpResult<RateLimitStatus> {
            Ok(self.state.lock().expect("lock").rate_limit.clone())
        }

        async fn increment_send_count(&self, _user_id: &str, _purpose: &str) -> EmailTotpResult<()> {
            self.state.lock().expect("lock").increment_send_calls += 1;
            Ok(())
        }

        async fn store_code(
            &self,
            user_id: &str,
            email: &str,
            code_hash: &str,
            _purpose: &str,
            expires_at: DateTime<Utc>,
            max_attempts: i32,
        ) -> EmailTotpResult<String> {
            self.state.lock().expect("lock").stored.push((
                user_id.to_string(),
                email.to_string(),
                code_hash.to_string(),
                expires_at,
                max_attempts,
            ));
            Ok("stored-code-id".to_string())
        }

        async fn get_active_code(
            &self,
            _user_id: &str,
            _purpose: &str,
        ) -> EmailTotpResult<Option<StoredCode>> {
            Ok(self.state.lock().expect("lock").active_code.clone())
        }

        async fn increment_attempts(&self, code_id: &str) -> EmailTotpResult<i32> {
            let mut state = self.state.lock().expect("lock");
            state.increment_attempt_calls.push(code_id.to_string());
            Ok(state.increment_attempts_result)
        }

        async fn mark_code_used(&self, code_id: &str) -> EmailTotpResult<()> {
            self.state
                .lock()
                .expect("lock")
                .marked_used
                .push(code_id.to_string());
            Ok(())
        }
    }

    #[derive(Debug, Default)]
    struct MockSessionRepoState {
        session_to_create: Option<VerificationSession>,
        consumed: Option<VerificationSession>,
        fetched: Option<VerificationSession>,
        create_calls: usize,
        consume_calls: usize,
        get_calls: usize,
    }

    struct MockSessionRepo {
        state: Mutex<MockSessionRepoState>,
    }

    impl MockSessionRepo {
        fn new(state: MockSessionRepoState) -> Self {
            Self {
                state: Mutex::new(state),
            }
        }
    }

    #[async_trait::async_trait]
    impl VerificationSessionRepository for MockSessionRepo {
        async fn create_session(
            &self,
            _user_id: &str,
            _purpose: &str,
            _method: &str,
            _expires_at: DateTime<Utc>,
        ) -> EmailTotpResult<VerificationSession> {
            let mut state = self.state.lock().expect("lock");
            state.create_calls += 1;
            state
                .session_to_create
                .clone()
                .ok_or(EmailTotpError::Storage("missing session".to_string()))
        }

        async fn consume_session(
            &self,
            _session_id: &str,
            _user_id: &str,
            _purpose: &str,
        ) -> EmailTotpResult<VerificationSession> {
            let mut state = self.state.lock().expect("lock");
            state.consume_calls += 1;
            state
                .consumed
                .clone()
                .ok_or(EmailTotpError::SessionNotFound)
        }

        async fn get_session(
            &self,
            _session_id: &str,
            _user_id: &str,
            _purpose: &str,
        ) -> EmailTotpResult<Option<VerificationSession>> {
            let mut state = self.state.lock().expect("lock");
            state.get_calls += 1;
            Ok(state.fetched.clone())
        }
    }

    #[derive(Debug, Default)]
    struct MockSenderState {
        sent: Vec<(String, String, String, i32)>,
    }

    struct MockSender {
        state: Mutex<MockSenderState>,
    }

    impl MockSender {
        fn new() -> Self {
            Self {
                state: Mutex::new(MockSenderState::default()),
            }
        }
    }

    #[async_trait::async_trait]
    impl EmailTotpSender for MockSender {
        async fn send_code(
            &self,
            to_email: &str,
            code: &str,
            purpose: &str,
            expiry_minutes: i32,
        ) -> EmailTotpResult<()> {
            self.state.lock().expect("lock").sent.push((
                to_email.to_string(),
                code.to_string(),
                purpose.to_string(),
                expiry_minutes,
            ));
            Ok(())
        }
    }

    fn hash(code: &str) -> String {
        Argon2Hasher::default()
            .hash_password(code.as_bytes())
            .expect("hashing should succeed")
    }

    fn session(id: &str, user_id: &str, purpose: &str) -> VerificationSession {
        VerificationSession {
            id: id.to_string(),
            user_id: user_id.to_string(),
            purpose: purpose.to_string(),
            method: VERIFICATION_METHOD_EMAIL_TOTP.to_string(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::minutes(10),
        }
    }

    fn default_rate_limit() -> RateLimitStatus {
        RateLimitStatus {
            send_count: 0,
            attempt_count: 0,
            is_limited: false,
        }
    }

    #[tokio::test]
    async fn request_code_returns_rate_limited_when_limit_hit() {
        let code_repo = MockCodeRepo::new(MockCodeRepoState {
            rate_limit: RateLimitStatus {
                send_count: 10,
                attempt_count: 10,
                is_limited: true,
            },
            ..Default::default()
        });
        let service = EmailTotpService::new(
            code_repo,
            MockSessionRepo::new(MockSessionRepoState::default()),
            MockSender::new(),
            EmailTotpConfig::default(),
        );

        let err = service
            .request_code("user-1", "u@example.com", "login")
            .await
            .expect_err("rate limited requests should fail");
        assert!(matches!(err, EmailTotpError::RateLimited));
    }

    #[tokio::test]
    async fn request_code_stores_hash_and_sends_email() {
        let code_repo = MockCodeRepo::new(MockCodeRepoState {
            rate_limit: default_rate_limit(),
            ..Default::default()
        });
        let sender = MockSender::new();
        let service = EmailTotpService::new(
            code_repo,
            MockSessionRepo::new(MockSessionRepoState::default()),
            sender,
            EmailTotpConfig::default(),
        );

        let expires_at = service
            .request_code("user-1", "u@example.com", "login")
            .await
            .expect("request should succeed");

        assert!(expires_at > Utc::now() - Duration::seconds(1));
        let state = service.code_repository.state.lock().expect("lock");
        assert_eq!(state.stored.len(), 1);
        assert_eq!(state.increment_send_calls, 1);
        assert_eq!(state.stored[0].0, "user-1");
        assert_eq!(state.stored[0].1, "u@example.com");
    }

    #[tokio::test]
    async fn verify_code_returns_expected_error_conditions() {
        let base = MockCodeRepoState {
            rate_limit: default_rate_limit(),
            ..Default::default()
        };
        let sender = MockSender::new();
        let session_repo = MockSessionRepo::new(MockSessionRepoState {
            session_to_create: Some(session("s1", "user-1", "login")),
            ..Default::default()
        });

        let svc_missing = EmailTotpService::new(
            MockCodeRepo::new(base),
            session_repo,
            sender,
            EmailTotpConfig::default(),
        );
        let err = svc_missing
            .verify_code("user-1", "login", "123456")
            .await
            .expect_err("missing code should fail");
        assert!(matches!(err, EmailTotpError::NoActiveCode));

        let expired = StoredCode {
            id: "code1".to_string(),
            code_hash: hash("123456"),
            expires_at: Utc::now() - Duration::minutes(1),
            attempts: 0,
            max_attempts: 5,
        };
        let svc_expired = EmailTotpService::new(
            MockCodeRepo::new(MockCodeRepoState {
                rate_limit: default_rate_limit(),
                active_code: Some(expired),
                ..Default::default()
            }),
            MockSessionRepo::new(MockSessionRepoState::default()),
            MockSender::new(),
            EmailTotpConfig::default(),
        );
        let err = svc_expired
            .verify_code("user-1", "login", "123456")
            .await
            .expect_err("expired code should fail");
        assert!(matches!(err, EmailTotpError::CodeExpired));

        let exhausted = StoredCode {
            id: "code2".to_string(),
            code_hash: hash("123456"),
            expires_at: Utc::now() + Duration::minutes(5),
            attempts: 3,
            max_attempts: 3,
        };
        let svc_exhausted = EmailTotpService::new(
            MockCodeRepo::new(MockCodeRepoState {
                rate_limit: default_rate_limit(),
                active_code: Some(exhausted),
                ..Default::default()
            }),
            MockSessionRepo::new(MockSessionRepoState::default()),
            MockSender::new(),
            EmailTotpConfig::default(),
        );
        let err = svc_exhausted
            .verify_code("user-1", "login", "123456")
            .await
            .expect_err("exhausted code should fail");
        assert!(matches!(err, EmailTotpError::TooManyAttempts));
    }

    #[tokio::test]
    async fn verify_code_invalid_increments_attempts() {
        let service = EmailTotpService::new(
            MockCodeRepo::new(MockCodeRepoState {
                rate_limit: default_rate_limit(),
                active_code: Some(StoredCode {
                    id: "code1".to_string(),
                    code_hash: hash("123456"),
                    expires_at: Utc::now() + Duration::minutes(5),
                    attempts: 0,
                    max_attempts: 5,
                }),
                increment_attempts_result: 1,
                ..Default::default()
            }),
            MockSessionRepo::new(MockSessionRepoState::default()),
            MockSender::new(),
            EmailTotpConfig::default(),
        );

        let err = service
            .verify_code("user-1", "login", "000000")
            .await
            .expect_err("invalid code should fail");
        assert!(matches!(err, EmailTotpError::InvalidCode));
        let state = service.code_repository.state.lock().expect("lock");
        assert_eq!(state.increment_attempt_calls, vec!["code1".to_string()]);
    }

    #[tokio::test]
    async fn verify_code_success_marks_used_and_creates_session() {
        let service = EmailTotpService::new(
            MockCodeRepo::new(MockCodeRepoState {
                rate_limit: default_rate_limit(),
                active_code: Some(StoredCode {
                    id: "code1".to_string(),
                    code_hash: hash("123456"),
                    expires_at: Utc::now() + Duration::minutes(5),
                    attempts: 0,
                    max_attempts: 5,
                }),
                ..Default::default()
            }),
            MockSessionRepo::new(MockSessionRepoState {
                session_to_create: Some(session("session-1", "user-1", "login")),
                ..Default::default()
            }),
            MockSender::new(),
            EmailTotpConfig::default(),
        );

        let created = service
            .verify_code("user-1", "login", "123456")
            .await
            .expect("valid code should succeed");
        assert_eq!(created.id, "session-1");
        assert_eq!(created.method, VERIFICATION_METHOD_EMAIL_TOTP);

        let code_state = service.code_repository.state.lock().expect("lock");
        assert_eq!(code_state.marked_used, vec!["code1".to_string()]);
        let session_state = service.session_repository.state.lock().expect("lock");
        assert_eq!(session_state.create_calls, 1);
    }

    #[tokio::test]
    async fn verify_code_only_success_marks_used_without_session_creation() {
        let service = EmailTotpService::new(
            MockCodeRepo::new(MockCodeRepoState {
                rate_limit: default_rate_limit(),
                active_code: Some(StoredCode {
                    id: "code-only".to_string(),
                    code_hash: hash("123456"),
                    expires_at: Utc::now() + Duration::minutes(5),
                    attempts: 0,
                    max_attempts: 5,
                }),
                ..Default::default()
            }),
            MockSessionRepo::new(MockSessionRepoState::default()),
            MockSender::new(),
            EmailTotpConfig::default(),
        );

        service
            .verify_code_only("user-1", "login", " 123456 ")
            .await
            .expect("code-only verification should succeed");

        let code_state = service.code_repository.state.lock().expect("lock");
        assert_eq!(code_state.marked_used, vec!["code-only".to_string()]);
        let session_state = service.session_repository.state.lock().expect("lock");
        assert_eq!(session_state.create_calls, 0);
    }

    #[tokio::test]
    async fn consume_and_get_session_delegate_to_repository() {
        let service = EmailTotpService::new(
            MockCodeRepo::new(MockCodeRepoState {
                rate_limit: default_rate_limit(),
                ..Default::default()
            }),
            MockSessionRepo::new(MockSessionRepoState {
                consumed: Some(session("s-consume", "user-1", "login")),
                fetched: Some(session("s-fetch", "user-1", "login")),
                ..Default::default()
            }),
            MockSender::new(),
            EmailTotpConfig::default(),
        );

        let consumed = service
            .consume_session("s-consume", "user-1", "login")
            .await
            .expect("consume should succeed");
        assert_eq!(consumed.id, "s-consume");

        let fetched = service
            .get_session("s-fetch", "user-1", "login")
            .await
            .expect("get should succeed")
            .expect("session should exist");
        assert_eq!(fetched.id, "s-fetch");
    }
}
