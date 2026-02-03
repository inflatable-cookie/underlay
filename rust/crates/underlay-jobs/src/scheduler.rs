//! Cron-based scheduler for recurring tasks.
//!
//! Enable with the `scheduler` feature flag. Requires the `postgres` feature
//! for database persistence.

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

#[cfg(all(feature = "scheduler", feature = "postgres"))]
mod inner {
    use chrono::{DateTime, Utc};
    use cron::Schedule;
    use std::str::FromStr;
    use tracing::{debug, error, info, instrument, warn};

    use crate::postgres::{JobRepository, Result, ScheduledTaskRepository};
    use crate::types::{JobConfig, ScheduledTask, ScheduledTaskDefinition};

    /// Scheduler for recurring tasks.
    ///
    /// Checks scheduled tasks and creates jobs when they're due.
    pub struct Scheduler {
        job_repo: JobRepository,
        task_repo: ScheduledTaskRepository,
    }

    impl Scheduler {
        /// Create a new scheduler.
        pub fn new(job_repo: JobRepository, task_repo: ScheduledTaskRepository) -> Self {
            Self { job_repo, task_repo }
        }

        /// Register scheduled task definitions.
        ///
        /// This upserts the definitions into the database and disables any
        /// tasks that are no longer in the provided list.
        #[instrument(skip(self, tasks))]
        pub async fn register_tasks(&self, tasks: &[ScheduledTaskDefinition]) -> Result<()> {
            info!(count = tasks.len(), "Registering scheduled tasks");

            for task in tasks {
                // Validate cron expression
                if Schedule::from_str(task.schedule).is_err() {
                    error!(
                        task_name = task.name,
                        schedule = task.schedule,
                        "Invalid cron expression, skipping task"
                    );
                    continue;
                }

                self.task_repo.upsert(task).await?;
                debug!(task_name = task.name, "Registered scheduled task");
            }

            // Disable tasks not in the list
            let active_names: Vec<&str> = tasks.iter().map(|t| t.name).collect();
            let disabled = self.task_repo.disable_stale(&active_names).await?;
            if disabled > 0 {
                info!(count = disabled, "Disabled stale scheduled tasks");
            }

            Ok(())
        }

        /// Run one scheduling tick.
        ///
        /// Checks all enabled tasks and creates jobs for those that are due.
        #[instrument(skip(self))]
        pub async fn tick(&self) -> Result<()> {
            let tasks = self.task_repo.get_enabled().await?;
            let now = Utc::now();

            for task in tasks {
                if let Err(e) = self.check_task(&task, now).await {
                    error!(
                        task_name = %task.name,
                        error = %e,
                        "Failed to check scheduled task"
                    );
                }
            }

            Ok(())
        }

        /// Check a single task and create a job if due.
        async fn check_task(&self, task: &ScheduledTask, now: DateTime<Utc>) -> Result<()> {
            // Parse cron schedule
            let schedule = match Schedule::from_str(&task.schedule) {
                Ok(s) => s,
                Err(e) => {
                    warn!(
                        task_name = %task.name,
                        schedule = %task.schedule,
                        error = %e,
                        "Invalid cron expression"
                    );
                    return Ok(());
                }
            };

            // Determine if we should schedule
            let should_schedule = match task.last_scheduled_at {
                None => true, // Never scheduled
                Some(last) => {
                    // Find the next occurrence after the last schedule
                    let next = schedule.after(&last).next();
                    next.map(|n| n <= now).unwrap_or(false)
                }
            };

            if !should_schedule {
                return Ok(());
            }

            // Check for overlap if not allowed
            if !task.allow_overlap {
                let is_running = self.task_repo.is_running(&task.job_type).await?;
                if is_running {
                    debug!(
                        task_name = %task.name,
                        "Skipping schedule - previous job still running"
                    );
                    return Ok(());
                }
            }

            // Create the job
            let config = JobConfig {
                max_attempts: task.max_attempts as u32,
                timeout_seconds: task.timeout_seconds.map(|s| s as u32),
                allow_overlap: task.allow_overlap,
                priority: task.priority,
                ..JobConfig::default()
            };

            let job_id = self
                .job_repo
                .create(&task.job_type, task.payload.clone(), &config)
                .await?;

            // Update last_scheduled_at
            self.task_repo.mark_scheduled(task.id).await?;

            info!(
                task_name = %task.name,
                job_id = %job_id,
                job_type = %task.job_type,
                "Created scheduled job"
            );

            Ok(())
        }
    }
}

#[cfg(all(feature = "scheduler", feature = "postgres"))]
pub use inner::Scheduler;
