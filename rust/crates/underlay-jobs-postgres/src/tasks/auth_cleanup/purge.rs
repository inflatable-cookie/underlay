use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{info, instrument};

use underlay_jobs::{Job, JobConfig, JobHandler, JobHandlerError};

/// Purge expired sessions from auth.sessions.
///
/// Removes sessions where:
/// - `refresh_token_expires_at < NOW()`, or
/// - `status = 'expired'`
///
/// Recommended schedule: Every 15 minutes (`0 */15 * * * *`)
#[derive(Debug, Clone)]
pub struct PurgeExpiredSessionsJob {
    pool: PgPool,
}

impl PurgeExpiredSessionsJob {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JobHandler for PurgeExpiredSessionsJob {
    fn job_type(&self) -> &'static str {
        "purge_expired_sessions"
    }

    fn config(&self) -> JobConfig {
        JobConfig::maintenance()
    }

    #[instrument(skip(self, _job), fields(job_type = "purge_expired_sessions"))]
    async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            WITH deleted AS (
                DELETE FROM auth.sessions
                WHERE refresh_token_expires_at < NOW()
                   OR status = 'expired'
                RETURNING id
            )
            SELECT COUNT(*) FROM deleted
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| JobHandlerError::new(format!("Database error: {e}")))?;

        info!(deleted = result, "Purged expired sessions");
        Ok(())
    }
}

/// Purge expired auth states from auth.auth_state.
///
/// Auth states are short-lived entries used for multi-step auth flows
/// (MFA setup, passkey registration, etc.). Removes entries where
/// `expires_at < NOW()`.
///
/// Recommended schedule: Hourly (`0 0 * * * *`)
#[derive(Debug, Clone)]
pub struct PurgeAuthStatesJob {
    pool: PgPool,
}

impl PurgeAuthStatesJob {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JobHandler for PurgeAuthStatesJob {
    fn job_type(&self) -> &'static str {
        "purge_auth_states"
    }

    fn config(&self) -> JobConfig {
        JobConfig::maintenance()
    }

    #[instrument(skip(self, _job), fields(job_type = "purge_auth_states"))]
    async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            WITH deleted AS (
                DELETE FROM auth.auth_state
                WHERE expires_at < NOW()
                RETURNING id
            )
            SELECT COUNT(*) FROM deleted
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| JobHandlerError::new(format!("Database error: {e}")))?;

        info!(deleted = result, "Purged expired auth states");
        Ok(())
    }
}

/// Purge old login attempts from auth.login_attempts.
///
/// Login attempts are useful for recent audit/rate-limiting but don't need
/// indefinite retention. Default retention: 30 days.
///
/// Recommended schedule: Daily at 3 AM (`0 0 3 * * *`)
#[derive(Debug, Clone)]
pub struct PurgeLoginAttemptsJob {
    pool: PgPool,
    /// Days to retain login attempts (default: 30)
    retention_days: i32,
}

impl PurgeLoginAttemptsJob {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            retention_days: 30,
        }
    }

    pub fn with_retention_days(mut self, days: i32) -> Self {
        self.retention_days = days;
        self
    }
}

#[async_trait]
impl JobHandler for PurgeLoginAttemptsJob {
    fn job_type(&self) -> &'static str {
        "purge_login_attempts"
    }

    fn config(&self) -> JobConfig {
        JobConfig::maintenance()
    }

    #[instrument(skip(self, _job), fields(job_type = "purge_login_attempts", retention_days = self.retention_days))]
    async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            WITH deleted AS (
                DELETE FROM auth.login_attempts
                WHERE attempted_at < NOW() - ($1 || ' days')::interval
                RETURNING id
            )
            SELECT COUNT(*) FROM deleted
            "#,
        )
        .bind(self.retention_days)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| JobHandlerError::new(format!("Database error: {e}")))?;

        info!(
            deleted = result,
            retention_days = self.retention_days,
            "Purged old login attempts"
        );
        Ok(())
    }
}

/// Purge old rate limit entries from auth.email_totp_rate_limits.
///
/// Rate limit entries older than 24 hours are no longer needed.
///
/// Recommended schedule: Hourly (`0 5 * * * *`)
#[derive(Debug, Clone)]
pub struct PurgeRateLimitEntriesJob {
    pool: PgPool,
}

impl PurgeRateLimitEntriesJob {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JobHandler for PurgeRateLimitEntriesJob {
    fn job_type(&self) -> &'static str {
        "purge_rate_limit_entries"
    }

    fn config(&self) -> JobConfig {
        JobConfig::maintenance()
    }

    #[instrument(skip(self, _job), fields(job_type = "purge_rate_limit_entries"))]
    async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            WITH deleted AS (
                DELETE FROM auth.email_totp_rate_limits
                WHERE hour_bucket < NOW() - INTERVAL '24 hours'
                RETURNING user_id
            )
            SELECT COUNT(*) FROM deleted
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| JobHandlerError::new(format!("Database error: {e}")))?;

        info!(deleted = result, "Purged old rate limit entries");
        Ok(())
    }
}

/// Purge expired/used email TOTP codes from auth.email_totp_codes.
///
/// Removes codes that are expired or have been used.
///
/// Recommended schedule: Hourly (`0 10 * * * *`)
#[derive(Debug, Clone)]
pub struct PurgeEmailTotpCodesJob {
    pool: PgPool,
}

impl PurgeEmailTotpCodesJob {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JobHandler for PurgeEmailTotpCodesJob {
    fn job_type(&self) -> &'static str {
        "purge_email_totp_codes"
    }

    fn config(&self) -> JobConfig {
        JobConfig::maintenance()
    }

    #[instrument(skip(self, _job), fields(job_type = "purge_email_totp_codes"))]
    async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            WITH deleted AS (
                DELETE FROM auth.email_totp_codes
                WHERE expires_at < NOW()
                   OR used_at IS NOT NULL
                RETURNING id
            )
            SELECT COUNT(*) FROM deleted
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| JobHandlerError::new(format!("Database error: {e}")))?;

        info!(deleted = result, "Purged expired/used email TOTP codes");
        Ok(())
    }
}

/// Purge expired/used verification sessions from auth.verification_sessions.
///
/// Removes sessions that are expired or have been used.
///
/// Recommended schedule: Hourly (`0 15 * * * *`)
#[derive(Debug, Clone)]
pub struct PurgeVerificationSessionsJob {
    pool: PgPool,
}

impl PurgeVerificationSessionsJob {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JobHandler for PurgeVerificationSessionsJob {
    fn job_type(&self) -> &'static str {
        "purge_verification_sessions"
    }

    fn config(&self) -> JobConfig {
        JobConfig::maintenance()
    }

    #[instrument(skip(self, _job), fields(job_type = "purge_verification_sessions"))]
    async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            WITH deleted AS (
                DELETE FROM auth.verification_sessions
                WHERE expires_at < NOW()
                   OR used_at IS NOT NULL
                RETURNING id
            )
            SELECT COUNT(*) FROM deleted
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| JobHandlerError::new(format!("Database error: {e}")))?;

        info!(
            deleted = result,
            "Purged expired/used verification sessions"
        );
        Ok(())
    }
}
