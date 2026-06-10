use chrono::Utc;
use serde_json::Value;
use tracing::{debug, instrument};

use crate::postgres_dead_letters::PgDeadLetterRepository;
use underlay_jobs::{Job, JobConfig, JobErrorRecord, JobFailureOutcome};

use super::{to_raw, JobRepository, RepoError, Result};

impl JobRepository {
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
            let delay = config
                .backoff()
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
}
