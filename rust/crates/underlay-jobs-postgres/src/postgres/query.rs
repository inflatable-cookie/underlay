use sqlx::QueryBuilder;
use tracing::instrument;

use crate::postgres_rows::JobRow;
use underlay_core::Uuid;
use underlay_jobs::{Job, JobFilters};

use super::{to_raw, JobRepository, Result};

impl JobRepository {
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
}
