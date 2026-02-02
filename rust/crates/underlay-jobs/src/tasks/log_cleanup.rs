//! Log and infrastructure cleanup tasks.
//!
//! These tasks clean up error logs and captured emails to prevent
//! unbounded table growth.

use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{info, instrument};

use crate::{Job, JobConfig, JobHandler, JobHandlerError};

// ============================================================================
// Purge Error Logs
// ============================================================================

/// Purge old error logs from platform.error_log.
///
/// Error logs are useful for debugging but don't need indefinite retention.
///
/// Default retention: 90 days.
///
/// Recommended schedule: Daily at 4 AM (`0 0 4 * * *`)
#[derive(Debug, Clone)]
pub struct PurgeErrorLogsJob {
    pool: PgPool,
    /// Days to retain error logs (default: 90)
    retention_days: i32,
}

impl PurgeErrorLogsJob {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            retention_days: 90,
        }
    }

    pub fn with_retention_days(mut self, days: i32) -> Self {
        self.retention_days = days;
        self
    }
}

#[async_trait]
impl JobHandler for PurgeErrorLogsJob {
    fn job_type(&self) -> &'static str {
        "purge_error_logs"
    }

    fn config(&self) -> JobConfig {
        JobConfig::maintenance()
    }

    #[instrument(skip(self, _job), fields(job_type = "purge_error_logs", retention_days = self.retention_days))]
    async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            WITH deleted AS (
                DELETE FROM platform.error_log
                WHERE occurred_at < NOW() - ($1 || ' days')::interval
                RETURNING id
            )
            SELECT COUNT(*) FROM deleted
            "#,
        )
        .bind(self.retention_days)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| JobHandlerError::new(format!("Database error: {e}")))?;

        info!(deleted = result, retention_days = self.retention_days, "Purged old error logs");
        Ok(())
    }
}

// ============================================================================
// Purge Captured Emails
// ============================================================================

/// Purge old captured emails from platform.captured_email.
///
/// The captured_email table is used in dev/test environments to capture
/// outgoing emails instead of sending them. Old entries should be cleaned up.
///
/// Default retention: 7 days.
///
/// Recommended schedule: Daily at 4 AM (`0 30 4 * * *`)
#[derive(Debug, Clone)]
pub struct PurgeCapturedEmailsJob {
    pool: PgPool,
    /// Days to retain captured emails (default: 7)
    retention_days: i32,
}

impl PurgeCapturedEmailsJob {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            retention_days: 7,
        }
    }

    pub fn with_retention_days(mut self, days: i32) -> Self {
        self.retention_days = days;
        self
    }
}

#[async_trait]
impl JobHandler for PurgeCapturedEmailsJob {
    fn job_type(&self) -> &'static str {
        "purge_captured_emails"
    }

    fn config(&self) -> JobConfig {
        JobConfig::maintenance()
    }

    #[instrument(skip(self, _job), fields(job_type = "purge_captured_emails", retention_days = self.retention_days))]
    async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            WITH deleted AS (
                DELETE FROM platform.captured_email
                WHERE captured_at < NOW() - ($1 || ' days')::interval
                RETURNING id
            )
            SELECT COUNT(*) FROM deleted
            "#,
        )
        .bind(self.retention_days)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| JobHandlerError::new(format!("Database error: {e}")))?;

        info!(deleted = result, retention_days = self.retention_days, "Purged old captured emails");
        Ok(())
    }
}
