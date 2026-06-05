use async_trait::async_trait;
use chrono::Utc;

use crate::postgres_rows::JobRow;
use underlay_core::Uuid;
use underlay_jobs::{Job, JobConfig, JobFailureOutcome, JobHandlerError, JobId, JobStore};

use super::{JobRepository, RepoError, Result};

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
