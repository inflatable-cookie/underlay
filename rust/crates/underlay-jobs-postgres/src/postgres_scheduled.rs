//! Scheduled task repository and job notification listener.

use sqlx::PgPool;
use sqlx::postgres::PgListener;
use std::time::Duration;
use tracing::debug;

use crate::postgres::{RepoError, Result};
use crate::postgres_rows::ScheduledTaskRow;
use underlay_core::Uuid;
use underlay_jobs::{ScheduledTask, ScheduledTaskDefinition};

// Helper to convert underlay_core::Uuid to raw uuid for sqlx
fn to_raw(id: Uuid) -> uuid::Uuid {
    id.0
}

fn from_raw(id: uuid::Uuid) -> Uuid {
    Uuid(id)
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
    #[tracing::instrument(skip(self, task))]
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
        .bind(task.config.max_attempts() as i32)
        .bind(task.config.timeout_seconds().map(|s| s as i32))
        .bind(task.config.allow_overlap_enabled())
        .bind(task.config.priority())
        .fetch_one(&self.pool)
        .await?;

        Ok(from_raw(row.0))
    }

    /// Get all enabled scheduled tasks.
    #[tracing::instrument(skip(self))]
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
    #[tracing::instrument(skip(self))]
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
    #[tracing::instrument(skip(self))]
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
    #[tracing::instrument(skip(self))]
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
    #[tracing::instrument(skip(self, active_names))]
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
// Job Notifier (LISTEN/NOTIFY support)
// ============================================================================

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
/// use underlay_jobs_postgres::PgJobNotifier;
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
