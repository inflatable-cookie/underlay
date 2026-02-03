//! Job system self-maintenance tasks.
//!
//! These tasks archive completed jobs, purge old history,
//! and recover abandoned/stalled jobs.

use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{info, instrument, warn};

use crate::{Job, JobConfig, JobHandler, JobHandlerError};

// ============================================================================
// Archive Completed Jobs
// ============================================================================

/// Archive completed jobs to platform.job_history.
///
/// Moves succeeded/failed/cancelled jobs older than the retention period
/// to the history table. This keeps the main job table small and performant.
///
/// Default retention: 7 days.
///
/// Recommended schedule: Daily at 5 AM (`0 0 5 * * *`)
#[derive(Debug, Clone)]
pub struct ArchiveCompletedJobsJob {
    pool: PgPool,
    /// Days to keep jobs before archiving (default: 7)
    retention_days: i32,
}

impl ArchiveCompletedJobsJob {
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
impl JobHandler for ArchiveCompletedJobsJob {
    fn job_type(&self) -> &'static str {
        "archive_completed_jobs"
    }

    fn config(&self) -> JobConfig {
        JobConfig::maintenance()
    }

    #[instrument(skip(self, _job), fields(job_type = "archive_completed_jobs", retention_days = self.retention_days))]
    async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT platform.archive_completed_jobs(($1 || ' days')::interval)
            "#,
        )
        .bind(self.retention_days)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| JobHandlerError::new(format!("Database error: {e}")))?;

        info!(
            archived = result,
            retention_days = self.retention_days,
            "Archived completed jobs"
        );
        Ok(())
    }
}

// ============================================================================
// Purge Job History
// ============================================================================

/// Purge old job history from platform.job_history.
///
/// Removes archived job history older than the retention period.
///
/// Default retention: 90 days.
///
/// Recommended schedule: Weekly on Sunday at 5 AM (`0 0 5 * * SUN`)
#[derive(Debug, Clone)]
pub struct PurgeJobHistoryJob {
    pool: PgPool,
    /// Days to keep job history (default: 90)
    retention_days: i32,
}

impl PurgeJobHistoryJob {
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
impl JobHandler for PurgeJobHistoryJob {
    fn job_type(&self) -> &'static str {
        "purge_job_history"
    }

    fn config(&self) -> JobConfig {
        JobConfig::maintenance()
    }

    #[instrument(skip(self, _job), fields(job_type = "purge_job_history", retention_days = self.retention_days))]
    async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT platform.purge_job_history(($1 || ' days')::interval)
            "#,
        )
        .bind(self.retention_days)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| JobHandlerError::new(format!("Database error: {e}")))?;

        info!(
            purged = result,
            retention_days = self.retention_days,
            "Purged old job history"
        );
        Ok(())
    }
}

// ============================================================================
// Recover Abandoned Jobs
// ============================================================================

/// Recover abandoned/stalled jobs.
///
/// Finds jobs stuck in `claimed` or `running` status without a recent heartbeat
/// and either:
/// - Resets them to `pending` if they have retries remaining
/// - Marks them as `failed` if they've exhausted retries
///
/// Default stall timeout: 300 seconds (5 minutes).
///
/// Recommended schedule: Every 5 minutes (`0 */5 * * * *`)
#[derive(Debug, Clone)]
pub struct RecoverAbandonedJobsJob {
    pool: PgPool,
    /// Seconds without heartbeat before considering a job stalled (default: 300)
    stall_timeout_seconds: i64,
}

impl RecoverAbandonedJobsJob {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            stall_timeout_seconds: 300,
        }
    }

    pub fn with_stall_timeout_seconds(mut self, seconds: i64) -> Self {
        self.stall_timeout_seconds = seconds;
        self
    }
}

#[async_trait]
impl JobHandler for RecoverAbandonedJobsJob {
    fn job_type(&self) -> &'static str {
        "recover_abandoned_jobs"
    }

    fn config(&self) -> JobConfig {
        JobConfig::maintenance()
    }

    #[instrument(skip(self, _job), fields(job_type = "recover_abandoned_jobs", stall_timeout = self.stall_timeout_seconds))]
    async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
        // Find and reset stalled jobs that have retries remaining
        let recovered = sqlx::query_scalar::<_, i64>(
            r#"
            WITH stalled AS (
                UPDATE platform.job
                SET status = 'pending',
                    claimed_at = NULL,
                    claimed_by = NULL,
                    started_at = NULL,
                    heartbeat_at = NULL,
                    last_error = 'Job stalled and was automatically recovered'
                WHERE status IN ('claimed', 'running')
                  AND heartbeat_at < NOW() - ($1 || ' seconds')::interval
                  AND attempts < max_attempts
                RETURNING id
            )
            SELECT COUNT(*) FROM stalled
            "#,
        )
        .bind(self.stall_timeout_seconds)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| JobHandlerError::new(format!("Database error: {e}")))?;

        if recovered > 0 {
            warn!(recovered = recovered, "Recovered stalled jobs");
        } else {
            info!("No stalled jobs to recover");
        }

        // Mark as failed any jobs that have exceeded max_attempts
        let failed = sqlx::query_scalar::<_, i64>(
            r#"
            WITH failed AS (
                UPDATE platform.job
                SET status = 'failed',
                    finished_at = NOW(),
                    last_error = 'Job stalled and exceeded max attempts'
                WHERE status IN ('claimed', 'running')
                  AND heartbeat_at < NOW() - ($1 || ' seconds')::interval
                  AND attempts >= max_attempts
                RETURNING id
            )
            SELECT COUNT(*) FROM failed
            "#,
        )
        .bind(self.stall_timeout_seconds)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| JobHandlerError::new(format!("Database error: {e}")))?;

        if failed > 0 {
            warn!(
                failed = failed,
                "Marked stalled jobs as failed (max attempts exceeded)"
            );
        }

        Ok(())
    }
}
