use async_trait::async_trait;

use super::config::JobConfig;
use super::records::Job;

/// Result of job execution.
#[derive(Debug)]
pub enum JobResult {
    /// Job completed successfully
    Success,
    /// Job failed with a retryable error
    RetryableError(String),
    /// Job failed with a permanent error (no retry)
    PermanentError(String),
}

impl JobResult {
    /// Create a success result.
    pub fn success() -> Self {
        Self::Success
    }

    /// Create a retryable error result.
    pub fn retryable(error: impl Into<String>) -> Self {
        Self::RetryableError(error.into())
    }

    /// Create a permanent error result.
    pub fn permanent(error: impl Into<String>) -> Self {
        Self::PermanentError(error.into())
    }
}

/// Error type for job handlers.
#[derive(Debug, Clone)]
pub struct JobHandlerError {
    pub message: String,
    pub is_permanent: bool,
}

impl JobHandlerError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            is_permanent: false,
        }
    }

    pub fn permanent(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            is_permanent: true,
        }
    }
}

impl std::fmt::Display for JobHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for JobHandlerError {}

impl From<JobHandlerError> for JobResult {
    fn from(err: JobHandlerError) -> Self {
        if err.is_permanent {
            JobResult::PermanentError(err.message)
        } else {
            JobResult::RetryableError(err.message)
        }
    }
}

/// Trait for job handlers.
///
/// Implement this trait to define how a specific job type is executed.
#[async_trait]
pub trait JobHandler: Send + Sync {
    /// The canonical job type string (e.g., "learning.reindex").
    ///
    /// Convention: `namespace.action` where namespace is the domain
    /// (learning, assessment, platform, etc.)
    fn job_type(&self) -> &'static str;

    /// Configuration for this job type.
    ///
    /// Defines retry behavior, timeouts, progress tracking, etc.
    fn config(&self) -> JobConfig {
        JobConfig::default()
    }

    /// Execute the job with the given payload.
    ///
    /// Legacy interface - implement `handle_with_context` for full functionality.
    async fn handle(&self, job: Job) -> Result<(), JobHandlerError>;
}
