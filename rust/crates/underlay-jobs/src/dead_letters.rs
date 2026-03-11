use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::types::{DeadLetter, DeadLetterFilters, DeadLetterId, JobId};

#[async_trait]
pub trait DeadLetterStore: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn list_dead_letters(
        &self,
        filters: DeadLetterFilters,
    ) -> Result<Vec<DeadLetter>, Self::Error>;

    async fn retry_dead_letter(&self, dead_letter_id: DeadLetterId) -> Result<JobId, Self::Error>;

    async fn archive_old_dead_letters(&self, before: DateTime<Utc>) -> Result<u64, Self::Error>;
}
