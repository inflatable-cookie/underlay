use std::time::Duration;

use super::backoff::{BackoffJitter, BackoffStrategy};

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
    max_attempts: u32,
    /// Timeout in seconds (None = no timeout)
    timeout_seconds: Option<u32>,
    /// Allow multiple instances of this job to run simultaneously
    allow_overlap: bool,
    /// Job priority (higher = more urgent, default: 0)
    priority: i32,
    /// Whether this job reports progress
    tracks_progress: bool,
    /// Retry backoff strategy
    backoff: BackoffStrategy,
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
                jitter: None,
            },
            ..Self::default()
        }
    }

    /// Config for critical tasks that should retry on failure with deterministic jitter.
    ///
    /// Existing retry presets keep their previous timing. Use this preset when you want
    /// retry spread for new or updated jobs without changing unrelated callers.
    pub fn with_retries_and_jitter(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            backoff: BackoffStrategy::Exponential {
                base: Duration::from_secs(DEFAULT_BACKOFF_BASE_SECS),
                max: Duration::from_secs(DEFAULT_BACKOFF_MAX_SECS),
                jitter: Some(BackoffJitter::default()),
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
                jitter: None,
            },
            ..Self::default()
        }
    }

    /// Config for long-running tasks with retries and deterministic jitter.
    pub fn long_running_with_retries_and_jitter(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            tracks_progress: true,
            timeout_seconds: Some(DEFAULT_LONG_RUNNING_TIMEOUT_SECS),
            backoff: BackoffStrategy::Exponential {
                base: Duration::from_secs(DEFAULT_BACKOFF_BASE_SECS),
                max: Duration::from_secs(DEFAULT_BACKOFF_MAX_SECS),
                jitter: Some(BackoffJitter::default()),
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

    /// Set the optional timeout in seconds.
    pub fn with_optional_timeout(mut self, seconds: Option<u32>) -> Self {
        self.timeout_seconds = seconds;
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
    ///         jitter: None,
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
            jitter: None,
        };
        self
    }

    /// Set exponential backoff with deterministic jitter spread.
    pub fn with_jittered_exponential_backoff(mut self, base_secs: u64, max_secs: u64) -> Self {
        self.backoff = BackoffStrategy::Exponential {
            base: Duration::from_secs(base_secs),
            max: Duration::from_secs(max_secs),
            jitter: Some(BackoffJitter::default()),
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

    /// Set whether overlapping executions are allowed.
    pub fn with_allow_overlap(mut self, allow_overlap: bool) -> Self {
        self.allow_overlap = allow_overlap;
        self
    }

    /// Return the maximum retry attempts.
    pub fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    /// Return the optional timeout in seconds.
    pub fn timeout_seconds(&self) -> Option<u32> {
        self.timeout_seconds
    }

    /// Return whether overlapping executions are allowed.
    pub fn allow_overlap_enabled(&self) -> bool {
        self.allow_overlap
    }

    /// Return the configured priority.
    pub fn priority(&self) -> i32 {
        self.priority
    }

    /// Return whether this job reports progress.
    pub fn tracks_progress(&self) -> bool {
        self.tracks_progress
    }

    /// Return the retry backoff strategy.
    pub fn backoff(&self) -> &BackoffStrategy {
        &self.backoff
    }
}
