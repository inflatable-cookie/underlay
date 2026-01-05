use async_trait::async_trait;

use crate::{Job, JobHandlerError, JobId};

#[async_trait]
pub trait JobStore: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn fetch_next(&self, allowed_types: &[String]) -> Result<Option<Job>, Self::Error>;

    async fn mark_success(&self, job_id: JobId) -> Result<(), Self::Error>;

    async fn mark_failure(
        &self,
        job_id: JobId,
        error: JobHandlerError,
    ) -> Result<(), Self::Error>;
}
