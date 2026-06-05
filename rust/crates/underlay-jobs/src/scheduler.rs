//! Scheduler configuration for recurring tasks.

use std::time::Duration;

/// Default scheduler tick interval in seconds (60 seconds).
pub const DEFAULT_SCHEDULER_TICK_INTERVAL_SECS: u64 = 60;

/// Configuration for the job scheduler.
///
/// # Example
///
/// ```
/// use underlay_jobs::SchedulerConfig;
///
/// // Default config (60 second tick interval)
/// let config = SchedulerConfig::default();
///
/// // Custom config
/// let config = SchedulerConfig::default()
///     .with_tick_interval_secs(30);
/// ```
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// Interval between scheduler ticks in seconds.
    /// Default: 60
    pub tick_interval_secs: u64,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            tick_interval_secs: DEFAULT_SCHEDULER_TICK_INTERVAL_SECS,
        }
    }
}

impl SchedulerConfig {
    /// Create a new config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the tick interval in seconds.
    pub fn with_tick_interval_secs(mut self, seconds: u64) -> Self {
        self.tick_interval_secs = seconds;
        self
    }

    /// Get the tick interval as a Duration.
    pub fn tick_interval(&self) -> Duration {
        Duration::from_secs(self.tick_interval_secs)
    }
}
