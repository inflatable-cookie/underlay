use chrono::Utc;
use tracing::instrument;

use underlay_core::Uuid;
use underlay_jobs::{Job, JobProgress};

use super::{to_raw, JobRepository, RepoError, Result};

impl JobRepository {
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

    /// Legacy unfenced compatibility entry point for ID-only callers.
    ///
    /// Worker completion paths must use [`Self::mark_succeeded_for_job`] or
    /// [`Self::mark_succeeded_attempt`] so a stale worker cannot finalize a
    /// newer claim after stall recovery.
    ///
    /// Also updates `last_completed_at` on any scheduled task with a matching job_type.
    ///
    /// # Errors
    ///
    /// Returns [`RepoError::NotFound`] if the job does not exist or
    /// [`RepoError::Conflict`] if the job is no longer running.
    #[instrument(skip(self))]
    pub async fn mark_succeeded(&self, job_id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        let row: Option<(i32, bool)> = sqlx::query_as(
            r#"
            SELECT attempts, status = 'running'
            FROM platform.job
            WHERE id = $1
            FOR UPDATE
            "#,
        )
        .bind(to_raw(job_id))
        .fetch_optional(&mut *tx)
        .await?;

        let Some((attempts, is_running)) = row else {
            return Err(RepoError::NotFound(job_id));
        };
        if !is_running {
            return Err(RepoError::Conflict(format!(
                "job {job_id} is no longer running"
            )));
        }

        Self::finalize_succeeded_in_tx(&mut tx, job_id, attempts).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Mark a claimed job as succeeded, fencing on its claimed attempt count.
    ///
    /// Also updates `last_completed_at` on any scheduled task with a matching job_type.
    ///
    /// # Errors
    ///
    /// Returns [`RepoError::NotFound`] if the job does not exist or
    /// [`RepoError::Conflict`] if the supplied attempt no longer owns the running row.
    #[instrument(skip(self, job), fields(job_id = %job.id, attempts = job.attempts))]
    pub async fn mark_succeeded_for_job(&self, job: &Job) -> Result<()> {
        self.mark_succeeded_attempt(job.id, job.attempts).await
    }

    /// Mark a job as succeeded only if it is currently running the expected attempt.
    ///
    /// Also updates `last_completed_at` on any scheduled task with a matching job_type.
    ///
    /// # Errors
    ///
    /// Returns [`RepoError::NotFound`] if the job does not exist or
    /// [`RepoError::Conflict`] if the supplied attempt no longer owns the running row.
    #[instrument(skip(self))]
    pub async fn mark_succeeded_attempt(&self, job_id: Uuid, expected_attempts: i32) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        let row: Option<(i32, bool)> = sqlx::query_as(
            r#"
            SELECT attempts, status = 'running'
            FROM platform.job
            WHERE id = $1
            FOR UPDATE
            "#,
        )
        .bind(to_raw(job_id))
        .fetch_optional(&mut *tx)
        .await?;

        let Some((attempts, is_running)) = row else {
            return Err(RepoError::NotFound(job_id));
        };
        if !is_running || attempts != expected_attempts {
            return Err(RepoError::Conflict(format!(
                "job {job_id} is no longer running attempt {expected_attempts}"
            )));
        }

        Self::finalize_succeeded_in_tx(&mut tx, job_id, expected_attempts).await?;
        tx.commit().await?;
        Ok(())
    }

    async fn finalize_succeeded_in_tx(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        job_id: Uuid,
        attempts: i32,
    ) -> Result<()> {
        let now = Utc::now();

        sqlx::query(
            r#"
            WITH updated_job AS (
                UPDATE platform.job
                SET status = 'succeeded',
                    finished_at = $2
                WHERE id = $1 AND status = 'running' AND attempts = $3
                RETURNING job_type
            )
            UPDATE platform.scheduled_task
            SET last_completed_at = $2
            WHERE job_type = (SELECT job_type FROM updated_job)
            "#,
        )
        .bind(to_raw(job_id))
        .bind(now)
        .bind(attempts)
        .execute(&mut **tx)
        .await?;

        Ok(())
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
}
