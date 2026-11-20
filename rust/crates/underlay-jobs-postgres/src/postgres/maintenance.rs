use tracing::instrument;

use crate::postgres_rows::JobRow;
use underlay_core::Uuid;
use underlay_jobs::Job;

use super::{to_raw, JobRepository, Result};

impl JobRepository {
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

    /// Reset a stalled job back to pending for retry, or mark it failed when
    /// it has already reached `max_attempts`.
    #[instrument(skip(self))]
    pub async fn reset_stalled(&self, job_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE platform.job
            SET status = CASE WHEN attempts < max_attempts THEN 'pending' ELSE 'failed' END,
                claimed_at = NULL,
                claimed_by = NULL,
                started_at = NULL,
                heartbeat_at = NULL,
                finished_at = CASE WHEN attempts < max_attempts THEN NULL ELSE NOW() END,
                last_error = CASE
                    WHEN attempts < max_attempts THEN 'Job stalled and was reset'
                    ELSE 'Job stalled and exceeded max_attempts'
                END
            WHERE id = $1 AND status IN ('claimed', 'running')
            "#,
        )
        .bind(to_raw(job_id))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Archive completed jobs older than the specified interval.
    #[instrument(skip(self))]
    pub async fn archive_completed(&self, older_than_days: i32) -> Result<u64> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT platform.archive_completed_jobs(($1 || ' days')::interval)
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
            SELECT platform.purge_job_history(($1 || ' days')::interval)
            "#,
        )
        .bind(older_than_days)
        .fetch_one(&self.pool)
        .await?;

        Ok(result as u64)
    }
}
