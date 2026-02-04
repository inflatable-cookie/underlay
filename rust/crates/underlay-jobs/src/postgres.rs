//! PostgreSQL-backed job store implementation.
//!
//! Enable with the `postgres` feature flag.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{PgPool, QueryBuilder};
use thiserror::Error;
use tracing::{debug, instrument};

use crate::store::JobStore;
use crate::types::{
    Job, JobConfig, JobErrorRecord, JobFilters, JobHandlerError, JobId, JobProgress, JobStatus,
    ScheduledTask, ScheduledTaskDefinition,
};
use underlay_core::Uuid;

// Helper to convert underlay_core::Uuid to raw uuid for sqlx
fn to_raw(id: Uuid) -> uuid::Uuid {
    id.0
}

// Helper to convert raw uuid to underlay_core::Uuid
fn from_raw(id: uuid::Uuid) -> Uuid {
    Uuid(id)
}

/// Repository errors.
#[derive(Debug, Error)]
pub enum RepoError {
    #[error("Job not found: {0}")]
    NotFound(Uuid),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, RepoError>;

/// Repository for job operations.
pub struct JobRepository {
    pool: PgPool,
}

impl JobRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new job to run immediately.
    #[instrument(skip(self, payload), fields(job_type = %job_type))]
    pub async fn create(&self, job_type: &str, payload: Value, config: &JobConfig) -> Result<Uuid> {
        self.create_scheduled(job_type, payload, config, None).await
    }

    /// Create a job scheduled for a specific time.
    #[instrument(skip(self, payload), fields(job_type = %job_type))]
    pub async fn create_scheduled(
        &self,
        job_type: &str,
        payload: Value,
        config: &JobConfig,
        scheduled_for: Option<DateTime<Utc>>,
    ) -> Result<Uuid> {
        let id = Uuid::new_v7();

        sqlx::query(
            r#"
            INSERT INTO platform.job (id, job_type, payload, max_attempts, scheduled_for, priority)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(to_raw(id))
        .bind(job_type)
        .bind(&payload)
        .bind(config.max_attempts as i32)
        .bind(scheduled_for)
        .bind(config.priority)
        .execute(&self.pool)
        .await?;

        debug!(job_id = %id, "Created job");
        Ok(id)
    }

    /// Claim a batch of pending jobs for processing.
    ///
    /// Uses `FOR UPDATE SKIP LOCKED` for safe concurrent access.
    #[instrument(skip(self))]
    pub async fn claim_batch(&self, worker_id: &str, limit: usize) -> Result<Vec<Job>> {
        let now = Utc::now();

        // Claim jobs in a single query using CTE
        let rows = sqlx::query_as::<_, JobRow>(
            r#"
            WITH claimable AS (
                SELECT id
                FROM platform.job
                WHERE status = 'pending'
                  AND (scheduled_for IS NULL OR scheduled_for <= $1)
                ORDER BY priority DESC, scheduled_for NULLS FIRST, created_at
                LIMIT $2
                FOR UPDATE SKIP LOCKED
            )
            UPDATE platform.job j
            SET status = 'claimed',
                claimed_at = $1,
                claimed_by = $3,
                attempts = attempts + 1
            FROM claimable c
            WHERE j.id = c.id
            RETURNING j.*
            "#,
        )
        .bind(now)
        .bind(limit as i64)
        .bind(worker_id)
        .fetch_all(&self.pool)
        .await?;

        let jobs: Vec<Job> = rows.into_iter().map(Job::from).collect();
        debug!(count = jobs.len(), "Claimed jobs");
        Ok(jobs)
    }

    /// Mark a job as running (called when execution starts).
    #[instrument(skip(self))]
    pub async fn mark_running(&self, job_id: Uuid) -> Result<()> {
        let now = Utc::now();

        let result = sqlx::query(
            r#"
            UPDATE platform.job
            SET status = 'running',
                started_at = $2,
                heartbeat_at = $2
            WHERE id = $1 AND status = 'claimed'
            "#,
        )
        .bind(to_raw(job_id))
        .bind(now)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(RepoError::NotFound(job_id));
        }

        Ok(())
    }

    /// Update job progress (for long-running jobs).
    #[instrument(skip(self, progress))]
    pub async fn update_progress(&self, job_id: Uuid, progress: JobProgress) -> Result<()> {
        let progress_json = serde_json::to_value(&progress)?;

        sqlx::query(
            r#"
            UPDATE platform.job
            SET progress = $2,
                heartbeat_at = NOW()
            WHERE id = $1 AND status = 'running'
            "#,
        )
        .bind(to_raw(job_id))
        .bind(progress_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Update heartbeat to prevent stall detection.
    #[instrument(skip(self))]
    pub async fn heartbeat(&self, job_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE platform.job
            SET heartbeat_at = NOW()
            WHERE id = $1 AND status = 'running'
            "#,
        )
        .bind(to_raw(job_id))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Mark a job as succeeded.
    ///
    /// Also updates `last_completed_at` on any scheduled task with a matching job_type.
    #[instrument(skip(self))]
    pub async fn mark_succeeded(&self, job_id: Uuid) -> Result<()> {
        let now = Utc::now();

        // Mark job as succeeded and update any matching scheduled task's last_completed_at
        sqlx::query(
            r#"
            WITH updated_job AS (
                UPDATE platform.job
                SET status = 'succeeded',
                    finished_at = $2
                WHERE id = $1 AND status = 'running'
                RETURNING job_type
            )
            UPDATE platform.scheduled_task
            SET last_completed_at = $2
            WHERE job_type = (SELECT job_type FROM updated_job)
            "#,
        )
        .bind(to_raw(job_id))
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Mark a job as failed and potentially schedule for retry.
    #[instrument(skip(self, config))]
    pub async fn mark_failed(&self, job_id: Uuid, error: &str, config: &JobConfig) -> Result<bool> {
        let now = Utc::now();

        // Get current attempt count
        let row: Option<(i32, i32, Value)> = sqlx::query_as(
            r#"
            SELECT attempts, max_attempts, error_history
            FROM platform.job
            WHERE id = $1
            "#,
        )
        .bind(to_raw(job_id))
        .fetch_optional(&self.pool)
        .await?;

        let Some((attempts, max_attempts, mut error_history)) = row else {
            return Err(RepoError::NotFound(job_id));
        };

        // Add error to history
        let job_error = JobErrorRecord {
            attempt: attempts,
            error: error.to_string(),
            at: now,
        };
        if let Value::Array(ref mut arr) = error_history {
            arr.push(serde_json::to_value(&job_error)?);
        }

        let should_retry = attempts < max_attempts;

        if should_retry {
            // Schedule for retry with backoff
            let delay = config.backoff.delay_for_attempt((attempts - 1) as u32);
            let retry_at = now + chrono::Duration::from_std(delay).unwrap_or_default();

            sqlx::query(
                r#"
                UPDATE platform.job
                SET status = 'pending',
                    last_error = $2,
                    error_history = $3,
                    scheduled_for = $4,
                    claimed_at = NULL,
                    claimed_by = NULL,
                    started_at = NULL,
                    heartbeat_at = NULL
                WHERE id = $1
                "#,
            )
            .bind(to_raw(job_id))
            .bind(error)
            .bind(&error_history)
            .bind(retry_at)
            .execute(&self.pool)
            .await?;

            debug!(job_id = %job_id, retry_at = %retry_at, "Job scheduled for retry");
        } else {
            // Permanent failure
            sqlx::query(
                r#"
                UPDATE platform.job
                SET status = 'failed',
                    finished_at = $2,
                    last_error = $3,
                    error_history = $4
                WHERE id = $1
                "#,
            )
            .bind(to_raw(job_id))
            .bind(now)
            .bind(error)
            .bind(&error_history)
            .execute(&self.pool)
            .await?;

            debug!(job_id = %job_id, "Job permanently failed");
        }

        Ok(should_retry)
    }

    /// Cancel a job.
    #[instrument(skip(self))]
    pub async fn cancel(&self, job_id: Uuid) -> Result<()> {
        let now = Utc::now();

        let result = sqlx::query(
            r#"
            UPDATE platform.job
            SET status = 'cancelled',
                finished_at = $2
            WHERE id = $1 AND status IN ('pending', 'claimed')
            "#,
        )
        .bind(to_raw(job_id))
        .bind(now)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(RepoError::NotFound(job_id));
        }

        Ok(())
    }

    /// Find stalled jobs (no heartbeat within timeout).
    #[instrument(skip(self))]
    pub async fn find_stalled(&self, timeout_seconds: i64) -> Result<Vec<Job>> {
        let rows = sqlx::query_as::<_, JobRow>(
            r#"
            SELECT *
            FROM platform.job
            WHERE status IN ('claimed', 'running')
              AND heartbeat_at < NOW() - ($1 || ' seconds')::interval
            "#,
        )
        .bind(timeout_seconds)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Job::from).collect())
    }

    /// Reset a stalled job back to pending for retry.
    #[instrument(skip(self))]
    pub async fn reset_stalled(&self, job_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE platform.job
            SET status = 'pending',
                claimed_at = NULL,
                claimed_by = NULL,
                started_at = NULL,
                heartbeat_at = NULL,
                last_error = 'Job stalled and was reset'
            WHERE id = $1 AND status IN ('claimed', 'running')
            "#,
        )
        .bind(to_raw(job_id))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get a job by ID.
    #[instrument(skip(self))]
    pub async fn get(&self, job_id: Uuid) -> Result<Option<Job>> {
        let row = sqlx::query_as::<_, JobRow>(
            r#"
            SELECT *
            FROM platform.job
            WHERE id = $1
            "#,
        )
        .bind(to_raw(job_id))
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Job::from))
    }

    /// List jobs with filters.
    #[instrument(skip(self))]
    pub async fn list(&self, filters: JobFilters) -> Result<Vec<Job>> {
        let mut qb: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            r#"
            SELECT *
            FROM platform.job
            WHERE 1=1
            "#,
        );

        if let Some(status) = &filters.status {
            qb.push(" AND status = ").push_bind(status.as_str());
        }
        if let Some(job_type) = &filters.job_type {
            qb.push(" AND job_type ILIKE ").push_bind(format!("%{}%", job_type));
        }

        qb.push(" ORDER BY created_at DESC LIMIT ")
            .push_bind(filters.limit as i64)
            .push(" OFFSET ")
            .push_bind(filters.offset as i64);

        let rows = qb.build_query_as::<JobRow>().fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Job::from).collect())
    }

    /// Archive completed jobs older than the specified interval.
    #[instrument(skip(self))]
    pub async fn archive_completed(&self, older_than_days: i32) -> Result<u64> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT platform.archive_completed_jobs($1 || ' days')
            "#,
        )
        .bind(older_than_days)
        .fetch_one(&self.pool)
        .await?;

        Ok(result as u64)
    }

    /// Purge old job history.
    #[instrument(skip(self))]
    pub async fn purge_history(&self, older_than_days: i32) -> Result<u64> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT platform.purge_job_history($1 || ' days')
            "#,
        )
        .bind(older_than_days)
        .fetch_one(&self.pool)
        .await?;

        Ok(result as u64)
    }
}

/// Implement JobStore trait for JobRepository to allow use with JobRunner.
#[async_trait]
impl JobStore for JobRepository {
    type Error = RepoError;

    async fn fetch_next(&self, allowed_types: &[String]) -> Result<Option<Job>> {
        if allowed_types.is_empty() {
            return Ok(None);
        }

        let now = Utc::now();
        let worker_id = format!("worker-{}", Uuid::new_v7());

        // Claim a single job
        let row = sqlx::query_as::<_, JobRow>(
            r#"
            WITH claimable AS (
                SELECT id
                FROM platform.job
                WHERE status = 'pending'
                  AND job_type = ANY($1)
                  AND (scheduled_for IS NULL OR scheduled_for <= $2)
                ORDER BY priority DESC, scheduled_for NULLS FIRST, created_at
                LIMIT 1
                FOR UPDATE SKIP LOCKED
            )
            UPDATE platform.job j
            SET status = 'running',
                claimed_at = $2,
                claimed_by = $3,
                started_at = $2,
                heartbeat_at = $2,
                attempts = attempts + 1
            FROM claimable c
            WHERE j.id = c.id
            RETURNING j.*
            "#,
        )
        .bind(allowed_types)
        .bind(now)
        .bind(&worker_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Job::from))
    }

    async fn mark_success(&self, job_id: JobId) -> Result<()> {
        self.mark_succeeded(job_id).await
    }

    async fn mark_failure(&self, job_id: JobId, error: JobHandlerError) -> Result<()> {
        // Use default config for retry logic (single attempt, no retry)
        let config = JobConfig::default();
        self.mark_failed(job_id, &error.to_string(), &config)
            .await?;
        Ok(())
    }
}

/// Repository for scheduled task operations.
pub struct ScheduledTaskRepository {
    pool: PgPool,
}

impl ScheduledTaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Upsert a scheduled task definition.
    #[instrument(skip(self, task))]
    pub async fn upsert(&self, task: &ScheduledTaskDefinition) -> Result<Uuid> {
        let row: (uuid::Uuid,) = sqlx::query_as(
            r#"
            INSERT INTO platform.scheduled_task (name, job_type, schedule, payload, max_attempts, timeout_seconds, allow_overlap, priority)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (name) DO UPDATE SET
                job_type = EXCLUDED.job_type,
                schedule = EXCLUDED.schedule,
                payload = EXCLUDED.payload,
                max_attempts = EXCLUDED.max_attempts,
                timeout_seconds = EXCLUDED.timeout_seconds,
                allow_overlap = EXCLUDED.allow_overlap,
                priority = EXCLUDED.priority,
                updated_at = NOW()
            RETURNING id
            "#,
        )
        .bind(task.name)
        .bind(task.job_type)
        .bind(task.schedule)
        .bind(&task.payload)
        .bind(task.config.max_attempts as i32)
        .bind(task.config.timeout_seconds.map(|s| s as i32))
        .bind(task.config.allow_overlap)
        .bind(task.config.priority)
        .fetch_one(&self.pool)
        .await?;

        Ok(from_raw(row.0))
    }

    /// Get all enabled scheduled tasks.
    #[instrument(skip(self))]
    pub async fn get_enabled(&self) -> Result<Vec<ScheduledTask>> {
        let rows = sqlx::query_as::<_, ScheduledTaskRow>(
            r#"
            SELECT *
            FROM platform.scheduled_task
            WHERE enabled = TRUE
            ORDER BY name
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(ScheduledTask::from).collect())
    }

    /// Mark a task as scheduled (update last_scheduled_at).
    #[instrument(skip(self))]
    pub async fn mark_scheduled(&self, task_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE platform.scheduled_task
            SET last_scheduled_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(to_raw(task_id))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Mark a task as completed (update last_completed_at).
    #[instrument(skip(self))]
    pub async fn mark_completed(&self, task_name: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE platform.scheduled_task
            SET last_completed_at = NOW()
            WHERE name = $1
            "#,
        )
        .bind(task_name)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Check if a task is currently running (for overlap prevention).
    #[instrument(skip(self))]
    pub async fn is_running(&self, job_type: &str) -> Result<bool> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM platform.job
            WHERE job_type = $1 AND status IN ('pending', 'claimed', 'running')
            "#,
        )
        .bind(job_type)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }

    /// Disable all tasks not in the provided list (cleanup stale tasks).
    #[instrument(skip(self, active_names))]
    pub async fn disable_stale(&self, active_names: &[&str]) -> Result<u64> {
        let result = sqlx::query(
            r#"
            UPDATE platform.scheduled_task
            SET enabled = FALSE
            WHERE name != ALL($1)
            "#,
        )
        .bind(active_names)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}

// ============================================================================
// Database row types (internal)
// ============================================================================

#[derive(sqlx::FromRow)]
struct JobRow {
    id: uuid::Uuid,
    job_type: String,
    status: String,
    payload: Value,
    attempts: i32,
    max_attempts: i32,
    scheduled_for: Option<DateTime<Utc>>,
    priority: i32,
    claimed_at: Option<DateTime<Utc>>,
    claimed_by: Option<String>,
    started_at: Option<DateTime<Utc>>,
    finished_at: Option<DateTime<Utc>>,
    heartbeat_at: Option<DateTime<Utc>>,
    progress: Option<Value>,
    last_error: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<JobRow> for Job {
    fn from(row: JobRow) -> Self {
        let status = match row.status.as_str() {
            "pending" => JobStatus::Pending,
            "claimed" => JobStatus::Claimed,
            "running" => JobStatus::Running,
            "succeeded" => JobStatus::Succeeded,
            "failed" => JobStatus::Failed,
            "cancelled" => JobStatus::Cancelled,
            _ => JobStatus::Pending,
        };

        let progress = row.progress.and_then(|v| serde_json::from_value(v).ok());

        Job {
            id: from_raw(row.id),
            job_type: row.job_type,
            status,
            payload: row.payload,
            attempts: row.attempts,
            max_attempts: row.max_attempts,
            scheduled_for: row.scheduled_for,
            priority: row.priority,
            claimed_at: row.claimed_at,
            claimed_by: row.claimed_by,
            started_at: row.started_at,
            finished_at: row.finished_at,
            heartbeat_at: row.heartbeat_at,
            progress,
            last_error: row.last_error,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct ScheduledTaskRow {
    id: uuid::Uuid,
    name: String,
    job_type: String,
    schedule: String,
    payload: Value,
    max_attempts: i32,
    timeout_seconds: Option<i32>,
    allow_overlap: bool,
    priority: i32,
    enabled: bool,
    last_scheduled_at: Option<DateTime<Utc>>,
    last_completed_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<ScheduledTaskRow> for ScheduledTask {
    fn from(row: ScheduledTaskRow) -> Self {
        ScheduledTask {
            id: from_raw(row.id),
            name: row.name,
            job_type: row.job_type,
            schedule: row.schedule,
            payload: row.payload,
            max_attempts: row.max_attempts,
            timeout_seconds: row.timeout_seconds,
            allow_overlap: row.allow_overlap,
            priority: row.priority,
            enabled: row.enabled,
            last_scheduled_at: row.last_scheduled_at,
            last_completed_at: row.last_completed_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

// ============================================================================
// Job Notifier (LISTEN/NOTIFY support)
// ============================================================================

use sqlx::postgres::PgListener;
use std::time::Duration;

/// The PostgreSQL NOTIFY channel name used for job notifications.
pub const JOB_NOTIFY_CHANNEL: &str = "underlay_job_notify";

/// Listens for PostgreSQL NOTIFY events when new jobs are inserted.
///
/// This provides efficient wake-up for job workers without constant polling.
/// When a job is inserted into `platform.job`, a trigger sends a NOTIFY on
/// the `underlay_job_notify` channel. Workers waiting on this notifier wake
/// up immediately to claim the new job.
///
/// # Example
///
/// ```ignore
/// use underlay_jobs::postgres::PgJobNotifier;
/// use std::time::Duration;
///
/// let mut notifier = PgJobNotifier::connect(&pool).await?;
///
/// loop {
///     // Try to claim and process a job
///     if let Some(job) = store.fetch_next(&allowed_types).await? {
///         process(job).await?;
///     } else {
///         // No jobs available - wait for notification or timeout
///         notifier.wait(Duration::from_secs(30)).await?;
///     }
/// }
/// ```
///
/// # Reliability
///
/// NOTIFY messages can be missed if:
/// - The connection drops and reconnects
/// - The worker wasn't listening when the NOTIFY was sent
///
/// Always use a fallback timeout (e.g., 30 seconds) to catch any missed
/// notifications. The fallback poll ensures correctness while LISTEN/NOTIFY
/// provides responsiveness.
pub struct PgJobNotifier {
    listener: PgListener,
}

impl PgJobNotifier {
    /// Connect to PostgreSQL and start listening for job notifications.
    ///
    /// This establishes a dedicated connection for LISTEN/NOTIFY, separate
    /// from the connection pool used for queries.
    pub async fn connect(pool: &PgPool) -> Result<Self> {
        let mut listener = PgListener::connect_with(pool).await?;
        listener.listen(JOB_NOTIFY_CHANNEL).await?;
        debug!("Job notifier listening on channel '{}'", JOB_NOTIFY_CHANNEL);
        Ok(Self { listener })
    }

    /// Wait for a job notification or until the timeout expires.
    ///
    /// Returns:
    /// - `Ok(Some(job_type))` - A notification was received with the job type
    /// - `Ok(None)` - The timeout expired without a notification
    /// - `Err(_)` - A connection error occurred
    ///
    /// # Timeout Behavior
    ///
    /// The timeout serves as a fallback poll interval. Even if no notifications
    /// arrive (due to missed messages or quiet periods), the worker will wake
    /// up periodically to check for jobs.
    ///
    /// Recommended timeout: 30 seconds. This balances:
    /// - Fast response to missed notifications
    /// - Low database load during idle periods
    pub async fn wait(&mut self, timeout: Duration) -> Result<Option<String>> {
        match tokio::time::timeout(timeout, self.listener.recv()).await {
            Ok(Ok(notification)) => {
                debug!(
                    channel = %notification.channel(),
                    payload = %notification.payload(),
                    "Received job notification"
                );
                Ok(Some(notification.payload().to_string()))
            }
            Ok(Err(e)) => {
                // Connection error - caller should handle reconnection
                Err(RepoError::Database(e))
            }
            Err(_) => {
                // Timeout - no notification received
                Ok(None)
            }
        }
    }

    /// Drain any pending notifications without blocking.
    ///
    /// Useful after processing a batch of jobs to clear queued notifications
    /// that are no longer relevant (since we just checked the queue).
    pub fn drain(&mut self) {
        // PgListener doesn't have a direct drain method, but try_recv would work
        // For now, this is a no-op placeholder. The notifications will be
        // consumed on the next wait() call anyway.
    }
}
