//! Auth-related cleanup tasks.
//!
//! These tasks clean up expired sessions, auth states, login attempts,
//! rate limit entries, TOTP codes, and verification sessions from standard
//! Underlay auth tables.

use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{info, instrument};

use crate::{Job, JobConfig, JobHandler, JobHandlerError};

// ============================================================================
// Purge Expired Sessions
// ============================================================================

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

// ============================================================================
// Purge Auth States
// ============================================================================

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

// ============================================================================
// Purge Login Attempts
// ============================================================================

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

// ============================================================================
// Purge Rate Limit Entries
// ============================================================================

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

// ============================================================================
// Purge Email TOTP Codes
// ============================================================================

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

// ============================================================================
// Purge Verification Sessions
// ============================================================================

/// Purge expired/used verification sessions from auth.verification_sessions.
///
/// Removes sessions that are expired or have been used.
///
/// Recommended schedule: Hourly (`0 15 * * * *`)
#[derive(Debug, Clone)]
pub struct PurgeVerificationSessionsJob {
    pool: PgPool,
}

// ============================================================================
// Suspend Inactive Accounts
// ============================================================================

/// Suspend long-inactive active accounts and revoke their active sessions.
///
/// Inactivity is determined by:
/// - latest `auth.sessions.last_used_at` for the user (status = active), else
/// - `auth.users.created_at` fallback for never-logged-in accounts.
///
/// Default policy:
/// - threshold: 1095 days (3 years)
/// - target roles: student, tester
/// - batch size: 500 users per run
/// - revoke active sessions after suspension: enabled
///
/// Recommended schedule: daily (`0 40 3 * * *`)
#[derive(Debug, Clone)]
pub struct SuspendInactiveAccountsJob {
    pool: PgPool,
    inactivity_days: i32,
    roles: Vec<String>,
    batch_limit: i64,
    revoke_sessions: bool,
}

impl SuspendInactiveAccountsJob {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            inactivity_days: 1095,
            roles: vec!["student".to_string(), "tester".to_string()],
            batch_limit: 500,
            revoke_sessions: true,
        }
    }

    pub fn with_inactivity_days(mut self, days: i32) -> Self {
        self.inactivity_days = days.max(1);
        self
    }

    pub fn with_roles(mut self, roles: Vec<String>) -> Self {
        self.roles = roles
            .into_iter()
            .map(|r| r.trim().to_string())
            .filter(|r| !r.is_empty())
            .collect();
        self
    }

    pub fn with_batch_limit(mut self, batch_limit: i64) -> Self {
        self.batch_limit = batch_limit.max(1);
        self
    }

    pub fn with_revoke_sessions(mut self, revoke_sessions: bool) -> Self {
        self.revoke_sessions = revoke_sessions;
        self
    }
}

#[async_trait]
impl JobHandler for SuspendInactiveAccountsJob {
    fn job_type(&self) -> &'static str {
        "suspend_inactive_accounts"
    }

    fn config(&self) -> JobConfig {
        JobConfig::maintenance()
    }

    #[instrument(skip(self, _job), fields(job_type = "suspend_inactive_accounts", inactivity_days = self.inactivity_days, batch_limit = self.batch_limit))]
    async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
        if self.roles.is_empty() {
            info!("Skipping inactive-account suspension because no target roles were configured");
            return Ok(());
        }

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| JobHandlerError::new(format!("Database transaction begin error: {e}")))?;

        let suspended_user_ids = sqlx::query_scalar::<_, uuid::Uuid>(
            r#"
            WITH candidate AS (
                SELECT
                    u.id
                FROM auth.users u
                LEFT JOIN LATERAL (
                    SELECT MAX(s.last_used_at) AS last_used_at
                    FROM auth.sessions s
                    WHERE s.user_id = u.id
                      AND s.status = 'active'
                ) session_activity ON TRUE
                WHERE u.status = 'active'
                  AND u.role = ANY($1)
                  AND COALESCE(session_activity.last_used_at, u.created_at) <
                      NOW() - ($2 || ' days')::interval
                ORDER BY COALESCE(session_activity.last_used_at, u.created_at) ASC
                LIMIT $3
            )
            UPDATE auth.users u
            SET status = 'suspended',
                updated_at = NOW()
            FROM candidate c
            WHERE u.id = c.id
            RETURNING u.id
            "#,
        )
        .bind(&self.roles)
        .bind(self.inactivity_days)
        .bind(self.batch_limit)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| JobHandlerError::new(format!("Database error suspending inactive accounts: {e}")))?;

        let mut revoked_sessions = 0_i64;
        if self.revoke_sessions && !suspended_user_ids.is_empty() {
            revoked_sessions = sqlx::query_scalar::<_, i64>(
                r#"
                WITH revoked AS (
                    UPDATE auth.sessions
                    SET status = 'revoked',
                        is_active = FALSE,
                        revoked_at = NOW(),
                        revocation_reason = 'inactive_account_auto_suspend'
                    WHERE user_id = ANY($1)
                      AND status = 'active'
                    RETURNING id
                )
                SELECT COUNT(*) FROM revoked
                "#,
            )
            .bind(&suspended_user_ids)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| JobHandlerError::new(format!("Database error revoking inactive-account sessions: {e}")))?;
        }

        tx.commit()
            .await
            .map_err(|e| JobHandlerError::new(format!("Database transaction commit error: {e}")))?;

        info!(
            suspended_accounts = suspended_user_ids.len(),
            revoked_sessions,
            inactivity_days = self.inactivity_days,
            roles = ?self.roles,
            "Processed inactive account suspension maintenance task"
        );

        Ok(())
    }
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
