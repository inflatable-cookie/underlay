//! Repository traits for email TOTP storage.
//!
//! These traits define the storage operations needed by the email TOTP service.
//! Applications implement these traits for their specific storage backend.

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::error::EmailTotpResult;

/// A stored verification code ready for verification.
#[derive(Debug, Clone)]
pub struct StoredCode {
    /// Unique identifier for the code.
    pub id: String,
    /// Argon2 hash of the code.
    pub code_hash: String,
    /// When the code expires.
    pub expires_at: DateTime<Utc>,
    /// Number of verification attempts made.
    pub attempts: i32,
    /// Maximum allowed attempts.
    pub max_attempts: i32,
}

/// Result of checking rate limits.
#[derive(Debug, Clone)]
pub struct RateLimitStatus {
    /// Number of codes sent in the current window.
    pub send_count: i32,
    /// Number of rate limit check attempts in the current window.
    pub attempt_count: i32,
    /// Whether the user is rate limited.
    pub is_limited: bool,
}

/// A verification session created after successful code verification.
#[derive(Debug, Clone)]
pub struct VerificationSession {
    /// Unique session identifier.
    pub id: String,
    /// User who completed verification.
    pub user_id: String,
    /// Purpose the session was created for.
    pub purpose: String,
    /// Method used for verification.
    pub method: String,
    /// When the session was created.
    pub created_at: DateTime<Utc>,
    /// When the session expires.
    pub expires_at: DateTime<Utc>,
}

/// Repository trait for email TOTP code storage.
///
/// Implement this trait to provide storage for verification codes.
/// The implementation is responsible for:
/// - Storing and retrieving codes
/// - Tracking verification attempts
/// - Enforcing rate limits
#[async_trait]
pub trait EmailTotpCodeRepository: Send + Sync {
    /// Check the rate limit for a user and record an attempt.
    ///
    /// This should:
    /// 1. Record that an attempt was made (for rate limit tracking)
    /// 2. Return whether the user is currently rate limited
    ///
    /// The actual code send count is incremented separately via `increment_send_count`.
    async fn check_rate_limit(
        &self,
        user_id: &str,
        purpose: &str,
        max_per_hour: i32,
    ) -> EmailTotpResult<RateLimitStatus>;

    /// Increment the send count after successfully sending a code.
    ///
    /// Call this after the email has been sent successfully.
    async fn increment_send_count(&self, user_id: &str, purpose: &str) -> EmailTotpResult<()>;

    /// Store a new verification code.
    ///
    /// This should:
    /// 1. Invalidate any existing unused codes for this user/purpose
    /// 2. Store the new code with its hash and expiry
    async fn store_code(
        &self,
        user_id: &str,
        email: &str,
        code_hash: &str,
        purpose: &str,
        expires_at: DateTime<Utc>,
        max_attempts: i32,
    ) -> EmailTotpResult<String>;

    /// Get the active code for verification.
    ///
    /// Returns the most recent unused, unexpired code for the user/purpose.
    async fn get_active_code(
        &self,
        user_id: &str,
        purpose: &str,
    ) -> EmailTotpResult<Option<StoredCode>>;

    /// Increment the verification attempt count for a code.
    ///
    /// Returns the new attempt count.
    async fn increment_attempts(&self, code_id: &str) -> EmailTotpResult<i32>;

    /// Mark a code as used.
    async fn mark_code_used(&self, code_id: &str) -> EmailTotpResult<()>;
}

/// Repository trait for verification sessions.
///
/// Verification sessions are created after successful code verification
/// and can be consumed once to authorize sensitive operations.
#[async_trait]
pub trait VerificationSessionRepository: Send + Sync {
    /// Create a new verification session.
    ///
    /// Returns the session ID.
    async fn create_session(
        &self,
        user_id: &str,
        purpose: &str,
        method: &str,
        expires_at: DateTime<Utc>,
    ) -> EmailTotpResult<VerificationSession>;

    /// Consume a verification session.
    ///
    /// This is a one-time operation - the session is marked as used and
    /// cannot be consumed again.
    ///
    /// Returns the session if it was valid and successfully consumed.
    async fn consume_session(
        &self,
        session_id: &str,
        user_id: &str,
        purpose: &str,
    ) -> EmailTotpResult<VerificationSession>;

    /// Get a session without consuming it.
    ///
    /// Returns None if the session doesn't exist, is expired, or already used.
    async fn get_session(
        &self,
        session_id: &str,
        user_id: &str,
        purpose: &str,
    ) -> EmailTotpResult<Option<VerificationSession>>;
}

/// Combined repository trait for convenience.
///
/// Implement this if your storage backend handles both codes and sessions.
pub trait EmailTotpRepository: EmailTotpCodeRepository + VerificationSessionRepository {}

// Blanket implementation
impl<T> EmailTotpRepository for T where T: EmailTotpCodeRepository + VerificationSessionRepository {}
