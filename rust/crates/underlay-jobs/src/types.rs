//! Core types for the job system.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

use underlay_core::Uuid;

pub type JobId = Uuid;

/// Job status enum matching the database constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    /// Job is waiting to be claimed by a worker
    Pending,
    /// Job has been claimed but not yet started
    Claimed,
    /// Job is currently executing
    Running,
    /// Job completed successfully
    Succeeded,
    /// Job failed (exhausted retries or permanent failure)
    Failed,
    /// Job was manually cancelled
    Cancelled,
}

impl JobStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            JobStatus::Pending => "pending",
            JobStatus::Claimed => "claimed",
            JobStatus::Running => "running",
            JobStatus::Succeeded => "succeeded",
            JobStatus::Failed => "failed",
            JobStatus::Cancelled => "cancelled",
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            JobStatus::Succeeded | JobStatus::Failed | JobStatus::Cancelled
        )
    }
}

/// Configuration for a job type.
///
/// Tasks can opt-in to features they need:
/// - Retries with backoff
/// - Progress tracking
/// - Overlap prevention
/// - Timeouts
#[derive(Debug, Clone)]
pub struct JobConfig {
    /// Maximum retry attempts (default: 1, meaning no retries)
    pub max_attempts: u32,
    /// Timeout in seconds (None = no timeout)
    pub timeout_seconds: Option<u32>,
    /// Allow multiple instances of this job to run simultaneously
    pub allow_overlap: bool,
    /// Job priority (higher = more urgent, default: 0)
    pub priority: i32,
    /// Whether this job reports progress
    pub tracks_progress: bool,
    /// Retry backoff strategy
    pub backoff: BackoffStrategy,
}

impl Default for JobConfig {
    fn default() -> Self {
        Self {
            max_attempts: 1,
            timeout_seconds: None,
            allow_overlap: false,
            priority: 0,
            tracks_progress: false,
            backoff: BackoffStrategy::None,
        }
    }
}

/// Default base delay for exponential backoff (60 seconds).
pub const DEFAULT_BACKOFF_BASE_SECS: u64 = 60;

/// Default maximum delay for exponential backoff (1 hour).
pub const DEFAULT_BACKOFF_MAX_SECS: u64 = 3600;

/// Default timeout for long-running jobs (1 hour).
pub const DEFAULT_LONG_RUNNING_TIMEOUT_SECS: u32 = 3600;

impl JobConfig {
    /// Create a new config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Config for simple maintenance tasks (no retries, no overlap).
    pub fn maintenance() -> Self {
        Self::default()
    }

    /// Config for critical tasks that should retry on failure.
    ///
    /// Uses default exponential backoff (60s base, 3600s max).
    /// Use `with_backoff` to customize the backoff strategy.
    pub fn with_retries(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            backoff: BackoffStrategy::Exponential {
                base: Duration::from_secs(DEFAULT_BACKOFF_BASE_SECS),
                max: Duration::from_secs(DEFAULT_BACKOFF_MAX_SECS),
            },
            ..Self::default()
        }
    }

    /// Config for long-running tasks with progress tracking.
    ///
    /// Default timeout is 1 hour. Use `with_timeout` to customize.
    pub fn long_running() -> Self {
        Self {
            tracks_progress: true,
            timeout_seconds: Some(DEFAULT_LONG_RUNNING_TIMEOUT_SECS),
            ..Self::default()
        }
    }

    /// Config for long-running tasks with retries.
    ///
    /// Uses default exponential backoff (60s base, 3600s max) and 1 hour timeout.
    pub fn long_running_with_retries(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            tracks_progress: true,
            timeout_seconds: Some(DEFAULT_LONG_RUNNING_TIMEOUT_SECS),
            backoff: BackoffStrategy::Exponential {
                base: Duration::from_secs(DEFAULT_BACKOFF_BASE_SECS),
                max: Duration::from_secs(DEFAULT_BACKOFF_MAX_SECS),
            },
            ..Self::default()
        }
    }

    /// Set the maximum retry attempts.
    pub fn with_max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = attempts;
        self
    }

    /// Set the priority (higher = more urgent).
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Set the timeout in seconds.
    pub fn with_timeout(mut self, seconds: u32) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }

    /// Set the backoff strategy.
    ///
    /// # Example
    ///
    /// ```
    /// use underlay_jobs::{JobConfig, BackoffStrategy};
    /// use std::time::Duration;
    ///
    /// let config = JobConfig::with_retries(3)
    ///     .with_backoff(BackoffStrategy::Exponential {
    ///         base: Duration::from_secs(30),  // Start with 30s delay
    ///         max: Duration::from_secs(600),  // Cap at 10 minutes
    ///     });
    /// ```
    pub fn with_backoff(mut self, backoff: BackoffStrategy) -> Self {
        self.backoff = backoff;
        self
    }

    /// Set exponential backoff with custom base and max delays.
    ///
    /// Shorthand for `with_backoff(BackoffStrategy::Exponential { ... })`.
    pub fn with_exponential_backoff(mut self, base_secs: u64, max_secs: u64) -> Self {
        self.backoff = BackoffStrategy::Exponential {
            base: Duration::from_secs(base_secs),
            max: Duration::from_secs(max_secs),
        };
        self
    }

    /// Set fixed delay backoff.
    ///
    /// All retry attempts will wait the same duration.
    pub fn with_fixed_backoff(mut self, delay_secs: u64) -> Self {
        self.backoff = BackoffStrategy::Fixed(Duration::from_secs(delay_secs));
        self
    }

    /// Enable progress tracking.
    pub fn with_progress_tracking(mut self) -> Self {
        self.tracks_progress = true;
        self
    }

    /// Allow overlapping executions.
    pub fn allow_overlap(mut self) -> Self {
        self.allow_overlap = true;
        self
    }
}

/// Retry backoff strategy.
#[derive(Debug, Clone)]
pub enum BackoffStrategy {
    /// No delay between retries
    None,
    /// Fixed delay between retries
    Fixed(Duration),
    /// Exponential backoff: min(base * 2^attempt, max)
    Exponential { base: Duration, max: Duration },
}

impl BackoffStrategy {
    /// Calculate the delay for a given attempt number (0-indexed).
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        match self {
            BackoffStrategy::None => Duration::ZERO,
            BackoffStrategy::Fixed(d) => *d,
            BackoffStrategy::Exponential { base, max } => {
                let multiplier = 2u64.saturating_pow(attempt);
                let delay = base.saturating_mul(multiplier as u32);
                std::cmp::min(delay, *max)
            }
        }
    }
}

/// Progress information for long-running jobs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobProgress {
    /// Current progress value
    pub current: u64,
    /// Total expected value (for percentage calculation)
    pub total: u64,
    /// Human-readable progress message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// When this progress was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl JobProgress {
    pub fn new(current: u64, total: u64) -> Self {
        Self {
            current,
            total,
            message: None,
            updated_at: Some(Utc::now()),
        }
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn percentage(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.current as f64 / self.total as f64) * 100.0
        }
    }
}

/// A job record from the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Job {
    pub id: JobId,
    pub job_type: String,
    pub status: JobStatus,
    pub payload: Value,
    pub attempts: i32,
    pub max_attempts: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_for: Option<DateTime<Utc>>,
    pub priority: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimed_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<JobProgress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Error information for failed attempts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobErrorRecord {
    pub attempt: i32,
    pub error: String,
    pub at: DateTime<Utc>,
}

/// A scheduled task definition from the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduledTask {
    pub id: Uuid,
    pub name: String,
    pub job_type: String,
    pub schedule: String,
    pub payload: Value,
    pub max_attempts: i32,
    pub timeout_seconds: Option<i32>,
    pub allow_overlap: bool,
    pub priority: i32,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scheduled_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Definition for registering a scheduled task in code.
#[derive(Debug, Clone)]
pub struct ScheduledTaskDefinition {
    /// Unique task name (used for upsert)
    pub name: &'static str,
    /// Job type to create when scheduled
    pub job_type: &'static str,
    /// Cron schedule expression
    pub schedule: &'static str,
    /// Payload to pass to the job
    pub payload: Value,
    /// Job configuration
    pub config: JobConfig,
}

/// Filters for listing jobs.
#[derive(Debug, Default, Clone)]
pub struct JobFilters {
    pub status: Option<JobStatus>,
    pub job_type: Option<String>,
    pub limit: usize,
    pub offset: usize,
}

impl JobFilters {
    pub fn new() -> Self {
        Self {
            limit: 50,
            ..Default::default()
        }
    }

    pub fn with_status(mut self, status: JobStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_job_type(mut self, job_type: impl Into<String>) -> Self {
        self.job_type = Some(job_type.into());
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

#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;
