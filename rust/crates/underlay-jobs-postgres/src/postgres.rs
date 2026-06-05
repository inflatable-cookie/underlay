//! PostgreSQL-backed job store implementation.
//!
//! Enable with the `postgres` feature flag.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{PgPool, QueryBuilder};
use thiserror::Error;
use tracing::{debug, instrument};

use crate::postgres_dead_letters::PgDeadLetterRepository;
use crate::postgres_rows::JobRow;
use underlay_core::Uuid;
use underlay_jobs::{
    Job, JobConfig, JobErrorRecord, JobEvent, JobEventHub, JobEventSink, JobFailureOutcome,
    JobFilters, JobHandlerError, JobId, JobProgress, JobStore,
};

// Helper to convert underlay_core::Uuid to raw uuid for sqlx
fn to_raw(id: Uuid) -> uuid::Uuid {
    id.0
}

/// Repository errors.
#[derive(Debug, Error)]
pub enum RepoError {
    #[error("Job not found: {0}")]
    NotFound(Uuid),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, RepoError>;

/// Repository for job operations.
pub struct JobRepository {
    pool: PgPool,
    events: JobEventHub,
}

impl JobRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            events: JobEventHub::new(),
        }
    }

    pub fn with_event_sink(mut self, sink: std::sync::Arc<dyn JobEventSink>) -> Self {
        self.events = self.events.with_sink(sink);
        self
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

        self.events.emit(JobEvent::Enqueued {
            job_id: id,
            job_type: job_type.to_string(),
            scheduled_for,
        });
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
    pub async fn mark_failed(
        &self,
        job: &Job,
        error: &str,
        config: &JobConfig,
        is_permanent: bool,
    ) -> Result<JobFailureOutcome> {
        let job_id = job.id;
        let now = Utc::now();
        let mut tx = self.pool.begin().await?;

        // Get current attempt count
        let row: Option<(i32, i32, Value)> = sqlx::query_as(
            r#"
            SELECT attempts, max_attempts, error_history
            FROM platform.job
            WHERE id = $1
            "#,
        )
        .bind(to_raw(job_id))
        .fetch_optional(&mut *tx)
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

        let should_retry = !is_permanent && attempts < max_attempts;

        if should_retry {
            // Schedule for retry with backoff
            let delay = config
                .backoff
                .delay_for_attempt_with_seed((attempts - 1) as u32, job_id.0.as_u128() as u64);
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
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;
            debug!(job_id = %job_id, retry_at = %retry_at, "Job scheduled for retry");
            Ok(JobFailureOutcome {
                will_retry: true,
                retry_delay: Some(delay),
                dead_letter_id: None,
            })
        } else {
            let dead_letter_id =
                PgDeadLetterRepository::insert_dead_letter(&mut tx, job, error, &error_history)
                    .await?;
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
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;
            debug!(job_id = %job_id, "Job permanently failed");
            Ok(JobFailureOutcome {
                will_retry: false,
                retry_delay: None,
                dead_letter_id: Some(dead_letter_id),
            })
        }
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
              AND COALESCE(heartbeat_at, claimed_at, started_at, created_at)
                  < NOW() - ($1 || ' seconds')::interval
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
            qb.push(" AND job_type ILIKE ")
                .push_bind(format!("%{}%", job_type));
        }

        qb.push(" ORDER BY created_at DESC LIMIT ")
            .push_bind(filters.limit as i64)
            .push(" OFFSET ")
            .push_bind(filters.offset as i64);

        let rows = qb.build_query_as::<JobRow>().fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Job::from).collect())
    }

    /// Count jobs with filters.
    #[instrument(skip(self))]
    pub async fn count(&self, filters: JobFilters) -> Result<usize> {
        let mut qb: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            r#"
            SELECT COUNT(*)
            FROM platform.job
            WHERE 1=1
            "#,
        );

        if let Some(status) = &filters.status {
            qb.push(" AND status = ").push_bind(status.as_str());
        }
        if let Some(job_type) = &filters.job_type {
            qb.push(" AND job_type ILIKE ")
                .push_bind(format!("%{}%", job_type));
        }

        let count = qb.build_query_scalar::<i64>().fetch_one(&self.pool).await?;
        Ok(count as usize)
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

    async fn mark_failure(
        &self,
        job: &Job,
        error: JobHandlerError,
        config: &JobConfig,
    ) -> Result<JobFailureOutcome> {
        self.mark_failed(job, &error.message, config, error.is_permanent)
            .await
    }
}
