use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::QueryBuilder;
use tracing::{debug, instrument};

use crate::postgres::{RepoError, Result};
use crate::postgres_rows::DeadLetterRow;
use underlay_core::Uuid;
use underlay_jobs::{
    DeadLetter, DeadLetterFilters, DeadLetterId, DeadLetterStore, Job, JobEvent, JobEventHub,
    JobEventSink, JobId,
};

fn to_raw(id: Uuid) -> uuid::Uuid {
    id.0
}

pub struct PgDeadLetterRepository {
    pool: sqlx::PgPool,
    events: JobEventHub,
}

impl PgDeadLetterRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool,
            events: JobEventHub::new(),
        }
    }

    pub fn with_event_sink(mut self, sink: std::sync::Arc<dyn JobEventSink>) -> Self {
        self.events = self.events.with_sink(sink);
        self
    }

    #[instrument(skip(self))]
    pub async fn get(&self, dead_letter_id: DeadLetterId) -> Result<Option<DeadLetter>> {
        let row = sqlx::query_as::<_, DeadLetterRow>(
            r#"
            SELECT *
            FROM platform.job_dead_letter
            WHERE id = $1
            "#,
        )
        .bind(to_raw(dead_letter_id))
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(DeadLetter::from))
    }

    #[instrument(skip(self))]
    pub async fn list(&self, filters: DeadLetterFilters) -> Result<Vec<DeadLetter>> {
        let mut qb: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            r#"
            SELECT *
            FROM platform.job_dead_letter
            WHERE 1=1
            "#,
        );

        if let Some(job_type) = &filters.job_type {
            qb.push(" AND job_type ILIKE ")
                .push_bind(format!("%{}%", job_type));
        }
        if !filters.include_archived {
            qb.push(" AND archived_at IS NULL");
        }

        qb.push(" ORDER BY failed_at DESC LIMIT ")
            .push_bind(filters.limit as i64)
            .push(" OFFSET ")
            .push_bind(filters.offset as i64);

        let rows = qb
            .build_query_as::<DeadLetterRow>()
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(DeadLetter::from).collect())
    }

    #[instrument(skip(self))]
    pub async fn retry(&self, dead_letter_id: DeadLetterId) -> Result<JobId> {
        let mut tx = self.pool.begin().await?;

        let row = sqlx::query_as::<_, DeadLetterRow>(
            r#"
            SELECT *
            FROM platform.job_dead_letter
            WHERE id = $1
            FOR UPDATE
            "#,
        )
        .bind(to_raw(dead_letter_id))
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(RepoError::NotFound(dead_letter_id))?;

        if row.archived_at.is_some() {
            return Err(RepoError::Conflict(format!(
                "Dead letter {} is archived and cannot be retried",
                dead_letter_id
            )));
        }
        if row.retried_job_id.is_some() {
            return Err(RepoError::Conflict(format!(
                "Dead letter {} has already been retried",
                dead_letter_id
            )));
        }

        let new_job_id = Uuid::new_v7();
        sqlx::query(
            r#"
            INSERT INTO platform.job (id, job_type, payload, max_attempts, priority)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(to_raw(new_job_id))
        .bind(&row.job_type)
        .bind(&row.payload)
        .bind(row.max_attempts)
        .bind(row.priority)
        .execute(&mut *tx)
        .await?;

        let retried_at = Utc::now();
        sqlx::query(
            r#"
            UPDATE platform.job_dead_letter
            SET retried_at = $2,
                retried_job_id = $3
            WHERE id = $1
            "#,
        )
        .bind(to_raw(dead_letter_id))
        .bind(retried_at)
        .bind(to_raw(new_job_id))
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        self.events.emit(JobEvent::Enqueued {
            job_id: new_job_id,
            job_type: row.job_type.clone(),
            scheduled_for: None,
        });

        debug!(
            dead_letter_id = %dead_letter_id,
            new_job_id = %new_job_id,
            "Retried dead letter as new job"
        );
        Ok(new_job_id)
    }

    #[instrument(skip(self))]
    pub async fn archive_old(&self, before: DateTime<Utc>) -> Result<u64> {
        let result = sqlx::query(
            r#"
            UPDATE platform.job_dead_letter
            SET archived_at = NOW()
            WHERE failed_at < $1
              AND archived_at IS NULL
            "#,
        )
        .bind(before)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    pub(crate) async fn insert_dead_letter(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        job: &Job,
        error: &str,
        error_history: &serde_json::Value,
    ) -> Result<DeadLetterId> {
        let dead_letter_id = Uuid::new_v7();
        sqlx::query(
            r#"
            INSERT INTO platform.job_dead_letter (
                id,
                original_job_id,
                job_type,
                payload,
                attempts,
                max_attempts,
                priority,
                last_error,
                error_history,
                failed_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(to_raw(dead_letter_id))
        .bind(to_raw(job.id))
        .bind(&job.job_type)
        .bind(&job.payload)
        .bind(job.attempts)
        .bind(job.max_attempts)
        .bind(job.priority)
        .bind(error)
        .bind(error_history)
        .bind(Utc::now())
        .execute(&mut **tx)
        .await?;

        Ok(dead_letter_id)
    }
}

#[async_trait]
impl DeadLetterStore for PgDeadLetterRepository {
    type Error = RepoError;

    async fn list_dead_letters(
        &self,
        filters: DeadLetterFilters,
    ) -> std::result::Result<Vec<DeadLetter>, Self::Error> {
        self.list(filters).await
    }

    async fn retry_dead_letter(
        &self,
        dead_letter_id: DeadLetterId,
    ) -> std::result::Result<JobId, Self::Error> {
        self.retry(dead_letter_id).await
    }

    async fn archive_old_dead_letters(
        &self,
        before: DateTime<Utc>,
    ) -> std::result::Result<u64, Self::Error> {
        self.archive_old(before).await
    }
}
