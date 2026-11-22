//! Abstract job store trait.
//!
//! This trait defines the interface for job persistence. Concrete storage
//! adapters live in backend-specific crates such as `underlay-jobs-postgres`.

use async_trait::async_trait;

use crate::types::{Job, JobConfig, JobFailureOutcome, JobHandlerError};

/// Result of attempting to mark a claimed job as completed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobCompletionOutcome {
    /// The claimed attempt still owned the running job and marked it succeeded.
    Succeeded,
    /// Another worker reclaimed or finalized the job before this attempt finished.
    LostOwnership,
}

/// Result of attempting to record a claimed job's failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobFailureRecordOutcome {
    /// The claimed attempt still owned the running job and recorded the failure.
    Recorded(JobFailureOutcome),
    /// Another worker reclaimed or finalized the job before this attempt failed.
    LostOwnership,
}

/// Abstract storage backend for jobs.
///
/// Implement this trait to provide custom storage for the job queue.
/// Use an adapter crate such as `underlay-jobs-postgres` for concrete storage.
#[async_trait]
pub trait JobStore: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Fetch the next available job that matches one of the allowed types.
    async fn fetch_next(&self, allowed_types: &[String]) -> Result<Option<Job>, Self::Error>;

    /// Mark a claimed job as successfully completed, fencing on its attempt count.
    ///
    /// Returns [`JobCompletionOutcome::LostOwnership`] when the claimed attempt
    /// no longer owns the running row (for example after stall recovery), so the
    /// runner can log and continue without emitting a duplicate completion event
    /// or terminating its loop.
    async fn mark_success(&self, job: &Job) -> Result<JobCompletionOutcome, Self::Error>;

    /// Mark a claimed job as failed with the given error, fencing on its attempt count.
    ///
    /// Returns [`JobFailureRecordOutcome::LostOwnership`] when the claimed attempt
    /// no longer owns the running row (for example after stall recovery), so the
    /// runner can log and continue without emitting a duplicate failure event
    /// or terminating its loop.
    async fn mark_failure(
        &self,
        job: &Job,
        error: JobHandlerError,
        config: &JobConfig,
    ) -> Result<JobFailureRecordOutcome, Self::Error>;
}
