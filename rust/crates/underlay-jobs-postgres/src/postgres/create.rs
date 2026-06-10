use chrono::{DateTime, Utc};
use serde_json::Value;
use tracing::{debug, instrument};

use crate::postgres_rows::JobRow;
use underlay_core::Uuid;
use underlay_jobs::{Job, JobConfig, JobEvent};

use super::{to_raw, JobRepository, Result};

impl JobRepository {
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
        .bind(config.max_attempts() as i32)
        .bind(scheduled_for)
        .bind(config.priority())
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
}
