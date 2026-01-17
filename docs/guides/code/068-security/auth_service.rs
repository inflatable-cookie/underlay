//! Example Rust auth service with rate limiting, lockout, and password validation.
//!
//! This demonstrates the security patterns recommended for Underlay applications.

use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use underlay_auth_password::PasswordStrengthAnalyzer;
use underlay_core::Uuid;
use underlay_ratelimit::{InMemoryBackend, RateLimitConfig, RateLimitResult, RateLimiter};

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for account lockout behavior.
#[derive(Clone, Debug)]
pub struct LockoutConfig {
    /// Maximum failed attempts before lockout (default: 5)
    pub max_failed_attempts: i32,
    /// Lockout duration in seconds (default: 900 = 15 minutes)
    pub lockout_duration_seconds: i64,
}

impl Default for LockoutConfig {
    fn default() -> Self {
        Self {
            max_failed_attempts: 5,
            lockout_duration_seconds: 900,
        }
    }
}

// =============================================================================
// Auth Service
// =============================================================================

pub struct AuthService {
    pool: PgPool,
    rate_limiter: Arc<RateLimiter<InMemoryBackend>>,
    lockout_config: LockoutConfig,
    password_analyzer: PasswordStrengthAnalyzer,
}

impl AuthService {
    pub fn new(pool: PgPool) -> Self {
        // Configure rate limiter
        let rate_limiter = Arc::new(RateLimiter::new(
            InMemoryBackend::new(),
            RateLimitConfig {
                max_requests: 10,
                window_seconds: 3600, // 10 attempts per hour
            },
        ));

        // Configure password analyzer with 12 char minimum
        let password_analyzer = PasswordStrengthAnalyzer::new().with_min_length(12);

        Self {
            pool,
            rate_limiter,
            lockout_config: LockoutConfig::default(),
            password_analyzer,
        }
    }

    // -------------------------------------------------------------------------
    // Password Validation
    // -------------------------------------------------------------------------

    /// Validate password meets security requirements.
    fn validate_password(&self, password: &str) -> Result<(), AuthError> {
        match self.password_analyzer.analyze(password) {
            Ok(_) => Ok(()),
            Err(e) => Err(AuthError::WeakPassword(e.to_string())),
        }
    }

    // -------------------------------------------------------------------------
    // Rate Limiting
    // -------------------------------------------------------------------------

    /// Check rate limit for login attempts.
    async fn check_rate_limit(&self, email: &str, ip: &str) -> Result<(), AuthError> {
        let key = format!("login:{}:{}", email.to_lowercase(), ip);

        match self.rate_limiter.check(&key).await {
            RateLimitResult::Allowed => Ok(()),
            RateLimitResult::Limited { retry_after } => {
                Err(AuthError::RateLimited { retry_after })
            }
        }
    }

    // -------------------------------------------------------------------------
    // Account Lockout
    // -------------------------------------------------------------------------

    /// Check if account is currently locked out.
    async fn check_lockout(&self, user_id: Uuid) -> Result<(), AuthError> {
        let record = sqlx::query!(
            r#"
            SELECT lockout_until 
            FROM auth.users 
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        if let Some(lockout_until) = record.lockout_until {
            if lockout_until > Utc::now() {
                let remaining = (lockout_until - Utc::now()).num_seconds();
                return Err(AuthError::AccountLocked {
                    retry_after_seconds: remaining.max(0) as u64,
                });
            }
            // Lockout expired, reset counter
            self.reset_failed_logins(user_id).await?;
        }

        Ok(())
    }

    /// Record a failed login attempt.
    async fn record_failed_login(
        &self,
        user_id: Uuid,
        email: &str,
        ip: &str,
        reason: &str,
    ) -> Result<(), AuthError> {
        // Increment counter and get new value
        let new_count: i32 = sqlx::query_scalar!(
            r#"
            UPDATE auth.users
            SET failed_login_count = failed_login_count + 1
            WHERE id = $1
            RETURNING failed_login_count
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        // Check if we should lock the account
        if new_count >= self.lockout_config.max_failed_attempts {
            let lockout_until =
                Utc::now() + Duration::seconds(self.lockout_config.lockout_duration_seconds);

            sqlx::query!(
                "UPDATE auth.users SET lockout_until = $1 WHERE id = $2",
                lockout_until,
                user_id
            )
            .execute(&self.pool)
            .await?;
        }

        // Log the attempt for auditing
        sqlx::query!(
            r#"
            INSERT INTO auth.login_attempts 
                (user_id, email, ip_address, success, failure_reason)
            VALUES ($1, $2, $3, false, $4)
            "#,
            user_id,
            email,
            ip,
            reason
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Reset failed login counter after successful login.
    async fn reset_failed_logins(&self, user_id: Uuid) -> Result<(), AuthError> {
        sqlx::query!(
            r#"
            UPDATE auth.users 
            SET failed_login_count = 0, lockout_until = NULL 
            WHERE id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Log a successful login attempt.
    async fn log_successful_login(
        &self,
        user_id: Uuid,
        email: &str,
        ip: &str,
    ) -> Result<(), AuthError> {
        sqlx::query!(
            r#"
            INSERT INTO auth.login_attempts 
                (user_id, email, ip_address, success)
            VALUES ($1, $2, $3, true)
            "#,
            user_id,
            email,
            ip
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Main Login Flow
    // -------------------------------------------------------------------------

    /// Authenticate a user with email and password.
    ///
    /// This method implements the full security flow:
    /// 1. Rate limiting check
    /// 2. User lookup
    /// 3. Lockout check
    /// 4. Password verification
    /// 5. Success/failure recording
    pub async fn login_with_password_and_ip(
        &self,
        email: &str,
        password: &str,
        client_ip: &str,
    ) -> Result<SessionTokens, AuthError> {
        // 1. Rate limiting check
        self.check_rate_limit(email, client_ip).await?;

        // 2. Find user by email
        let user = match self.find_user_by_email(email).await? {
            Some(u) => u,
            None => {
                // Log attempt against non-existent user (for monitoring)
                self.log_failed_attempt_no_user(email, client_ip).await?;
                return Err(AuthError::InvalidCredentials);
            }
        };

        // 3. Check lockout status
        self.check_lockout(user.id).await?;

        // 4. Verify password
        if !self.verify_password(&user, password)? {
            self.record_failed_login(user.id, email, client_ip, "invalid_password")
                .await?;
            return Err(AuthError::InvalidCredentials);
        }

        // 5. Success - reset counters and issue session
        self.reset_failed_logins(user.id).await?;
        self.log_successful_login(user.id, email, client_ip).await?;

        // Issue session tokens
        self.issue_session(user.id).await
    }

    // -------------------------------------------------------------------------
    // Password Management
    // -------------------------------------------------------------------------

    /// Set password for a user (registration or reset).
    pub async fn set_password(&self, user_id: Uuid, password: &str) -> Result<(), AuthError> {
        // Validate password strength FIRST
        self.validate_password(password)?;

        // Hash and store
        let hash = self.hash_password(password)?;
        self.store_password_hash(user_id, &hash).await
    }

    /// Change password (requires current password).
    pub async fn change_password(
        &self,
        user_id: Uuid,
        current_password: &str,
        new_password: &str,
    ) -> Result<(), AuthError> {
        // Verify current password
        let user = self.get_user(user_id).await?;
        if !self.verify_password(&user, current_password)? {
            return Err(AuthError::InvalidCredentials);
        }

        // Validate new password strength
        self.validate_password(new_password)?;

        // Hash and store
        let hash = self.hash_password(new_password)?;
        self.store_password_hash(user_id, &hash).await
    }

    // ... (other helper methods omitted for brevity)
}

// =============================================================================
// Error Types
// =============================================================================

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Rate limited, try again in {retry_after} seconds")]
    RateLimited { retry_after: u64 },

    #[error("Account locked, try again in {retry_after_seconds} seconds")]
    AccountLocked { retry_after_seconds: u64 },

    #[error("Password too weak: {0}")]
    WeakPassword(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

// =============================================================================
// Placeholder Types (implement these based on your application)
// =============================================================================

pub struct SessionTokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}
