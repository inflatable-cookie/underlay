use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

use super::ids::{DeadLetterId, JobId};

/// A dead-letter record for a job that exhausted retries or failed permanently.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeadLetter {
    pub id: DeadLetterId,
    pub original_job_id: JobId,
    pub job_type: String,
    pub payload: Value,
    pub attempts: i32,
    pub max_attempts: i32,
    pub priority: i32,
    pub last_error: String,
    pub error_history: Vec<JobErrorRecord>,
    pub failed_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retried_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retried_job_id: Option<JobId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Filters for listing dead letters.
#[derive(Debug, Default, Clone)]
pub struct DeadLetterFilters {
    pub job_type: Option<String>,
    pub include_archived: bool,
    pub limit: usize,
    pub offset: usize,
}

impl DeadLetterFilters {
    pub fn new() -> Self {
        Self {
            limit: 50,
            ..Default::default()
        }
    }

    pub fn with_job_type(mut self, job_type: impl Into<String>) -> Self {
        self.job_type = Some(job_type.into());
        self
    }

    pub fn include_archived(mut self) -> Self {
        self.include_archived = true;
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }
}

/// Error information for failed attempts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobErrorRecord {
    pub attempt: i32,
    pub error: String,
    pub at: DateTime<Utc>,
}

/// Storage outcome for a failed job attempt.
#[derive(Debug, Clone, Default)]
pub struct JobFailureOutcome {
    pub will_retry: bool,
    pub retry_delay: Option<Duration>,
    pub dead_letter_id: Option<DeadLetterId>,
}
