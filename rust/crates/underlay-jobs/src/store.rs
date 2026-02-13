//! Abstract job store trait.
//!
//! This trait defines the interface for job persistence. The crate provides
//! a PostgreSQL implementation when the `postgres` feature is enabled.

use async_trait::async_trait;

use crate::types::{Job, JobConfig, JobHandlerError, JobId};

/// Abstract storage backend for jobs.
///
/// Implement this trait to provide custom storage for the job queue.
/// The crate provides `JobRepository` when the `postgres` feature is enabled.
#[async_trait]
pub trait JobStore: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Fetch the next available job that matches one of the allowed types.
    async fn fetch_next(&self, allowed_types: &[String]) -> Result<Option<Job>, Self::Error>;

    /// Mark a job as successfully completed.
    async fn mark_success(&self, job_id: JobId) -> Result<(), Self::Error>;

    /// Mark a job as failed with the given error.
    async fn mark_failure(
        &self,
        job_id: JobId,
        error: JobHandlerError,
        config: &JobConfig,
    ) -> Result<(), Self::Error>;
}
