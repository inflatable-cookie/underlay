//! Job runner for processing queued jobs.
//!
//! The runner fetches jobs from a store and dispatches them to registered handlers.
//! It supports two modes of operation:
//!
//! 1. **Polling mode** (`run_forever`): Polls the database at a fixed interval.
//!    Simple but creates unnecessary database load when idle.
//!
//! 2. **Notification mode** (`run_with_notifier`): Uses PostgreSQL LISTEN/NOTIFY
//!    to wake up immediately when jobs are inserted. Falls back to polling at a
//!    longer interval as a safety net. This is the recommended mode for production.
//!
//! # Example: Notification Mode (Recommended)
//!
//! ```ignore
//! use underlay_jobs::{JobRunner, JobRunnerConfig, PgJobNotifier};
//! use std::time::Duration;
//!
//! let runner = JobRunner::new(store, registry)
//!     .with_config(JobRunnerConfig {
//!         poll_interval: Duration::from_secs(30), // Fallback interval
//!         ..Default::default()
//!     });
//!
//! let mut notifier = PgJobNotifier::connect(&pool).await?;
//! runner.run_with_notifier(&mut notifier).await?;
//! ```

use std::time::Duration;

use tracing::{debug, error, info, warn};

use crate::events::{JobEvent, JobEventHub, JobEventSink};
use crate::registry::JobRegistry;
use crate::store::JobStore;

/// Configuration for the job runner.
#[derive(Debug, Clone)]
pub struct JobRunnerConfig {
    /// How often to poll for new jobs when idle (fallback interval).
    ///
    /// In notification mode, this is the maximum time between checks.
    /// In polling mode, this is the interval between each poll.
    ///
    /// Default: 30 seconds (suitable for notification mode).
    /// For polling mode without notifications, consider 250ms-1s.
    pub poll_interval: Duration,

    /// How many jobs to process before sleeping (0 = unlimited).
    pub batch_size: usize,
}

impl Default for JobRunnerConfig {
    fn default() -> Self {
        Self {
            // Default to 30 seconds - appropriate for notification mode
            // where this is just a fallback safety net
            poll_interval: Duration::from_secs(30),
            batch_size: 0,
        }
    }
}

/// Job runner that processes jobs from a store using registered handlers.
#[derive(Debug)]
pub struct JobRunner<S> {
    store: S,
    registry: JobRegistry,
    config: JobRunnerConfig,
    events: JobEventHub,
}

impl<S> JobRunner<S>
where
    S: JobStore,
{
    /// Create a new job runner with the given store and handler registry.
    pub fn new(store: S, registry: JobRegistry) -> Self {
        Self {
            store,
            registry,
            config: JobRunnerConfig::default(),
            events: JobEventHub::new(),
        }
    }

    /// Configure the runner.
    pub fn with_config(mut self, config: JobRunnerConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_event_sink(mut self, sink: std::sync::Arc<dyn JobEventSink>) -> Self {
        self.events = self.events.with_sink(sink);
        self
    }

    /// Process a single job if one is available.
    ///
    /// Returns `true` if a job was processed, `false` if no job was available.
    pub async fn run_once(&self) -> Result<bool, S::Error> {
        let allowed = self.registry.job_types();
        let Some(job) = self.store.fetch_next(&allowed).await? else {
            return Ok(false);
        };

        let Some(handler) = self.registry.handler(&job.job_type) else {
            warn!(
                job_type = %job.job_type,
                job_id = %job.id,
                "No handler registered for job type, skipping"
            );
            return Ok(false);
        };

        debug!(
            job_type = %job.job_type,
            job_id = %job.id,
            attempt = job.attempts,
            "Processing job"
        );

        self.events.emit(JobEvent::Claimed {
            job_id: job.id,
            job_type: job.job_type.clone(),
            worker_id: job.claimed_by.clone(),
            attempt: job.attempts,
        });
        self.events.emit(JobEvent::Started {
            job_id: job.id,
            job_type: job.job_type.clone(),
            attempt: job.attempts,
        });

        let handler_config = handler.config();

        match handler.handle(job.clone()).await {
            Ok(()) => {
                self.store.mark_success(job.id).await?;
                let duration = job
                    .started_at
                    .or(job.claimed_at)
                    .and_then(|started_at| (chrono::Utc::now() - started_at).to_std().ok());
                self.events.emit(JobEvent::Completed {
                    job_id: job.id,
                    job_type: job.job_type.clone(),
                    attempt: job.attempts,
                    duration,
                });
                info!(
                    job_type = %job.job_type,
                    job_id = %job.id,
                    "Job completed successfully"
                );
                Ok(true)
            }
            Err(err) => {
                let is_permanent = err.is_permanent;
                let error_message = err.message.clone();
                let outcome = self.store.mark_failure(&job, err, &handler_config).await?;
                self.events.emit(JobEvent::Failed {
                    job_id: job.id,
                    job_type: job.job_type.clone(),
                    error: error_message,
                    attempt: job.attempts,
                    will_retry: outcome.will_retry,
                    next_retry_delay: outcome.retry_delay,
                });
                if let Some(dead_letter_id) = outcome.dead_letter_id {
                    self.events.emit(JobEvent::DeadLettered {
                        job_id: job.id,
                        job_type: job.job_type.clone(),
                        dead_letter_id,
                    });
                }

                if is_permanent {
                    error!(
                        job_type = %job.job_type,
                        job_id = %job.id,
                        "Job failed permanently"
                    );
                } else {
                    warn!(
                        job_type = %job.job_type,
                        job_id = %job.id,
                        "Job failed (may retry)"
                    );
                }
                Ok(true)
            }
        }
    }

    /// Run the job processing loop forever using polling.
    ///
    /// This will continuously poll for and process jobs, sleeping for
    /// `poll_interval` when no jobs are available.
    ///
    /// **Note**: For production use, prefer `run_with_notifier()` which uses
    /// PostgreSQL LISTEN/NOTIFY for efficient wake-up. Polling mode is useful
    /// for testing or non-PostgreSQL backends.
    pub async fn run_forever(&self) -> Result<(), S::Error> {
        info!(
            poll_interval_ms = self.config.poll_interval.as_millis(),
            "Starting job runner in polling mode"
        );

        loop {
            let did_work = self.run_once().await?;
            if !did_work {
                tokio::time::sleep(self.config.poll_interval).await;
            }
        }
    }

    /// Run a batch of jobs.
    ///
    /// Processes up to `limit` jobs (or until the queue is empty).
    /// Returns the number of jobs processed.
    pub async fn run_batch(&self, limit: usize) -> Result<usize, S::Error> {
        let mut processed = 0;

        while processed < limit {
            let did_work = self.run_once().await?;
            if !did_work {
                break;
            }
            processed += 1;
        }

        Ok(processed)
    }
}

// ============================================================================
// Notification-based runner (PostgreSQL LISTEN/NOTIFY)
// ============================================================================

#[cfg(feature = "postgres")]
use crate::postgres::RepoError;
#[cfg(feature = "postgres")]
use crate::postgres_scheduled::PgJobNotifier;

#[cfg(feature = "postgres")]
impl<S> JobRunner<S>
where
    S: JobStore<Error = RepoError>,
{
    /// Run the job processing loop using PostgreSQL LISTEN/NOTIFY.
    ///
    /// This is the recommended way to run the job worker in production.
    /// Instead of constantly polling, the runner waits for NOTIFY events
    /// that are triggered when new jobs are inserted.
    ///
    /// # How It Works
    ///
    /// 1. Try to claim and process any available jobs
    /// 2. If no jobs are available, wait for a notification
    /// 3. When notified (or after the fallback timeout), go back to step 1
    ///
    /// # Fallback Polling
    ///
    /// The `poll_interval` from `JobRunnerConfig` is used as a fallback timeout.
    /// If no notifications arrive within this interval, the runner wakes up and
    /// checks for jobs anyway. This handles edge cases like:
    ///
    /// - Notifications missed during brief disconnections
    /// - Jobs scheduled for the future becoming ready
    /// - Any other race conditions
    ///
    /// A 30-second fallback interval is recommended (the default).
    ///
    /// # Example
    ///
    /// ```ignore
    /// use underlay_jobs::{JobRunner, JobRunnerConfig, PgJobNotifier, JobRepository};
    /// use std::time::Duration;
    ///
    /// let job_repo = JobRepository::new(pool.clone());
    /// let mut notifier = PgJobNotifier::connect(&pool).await?;
    ///
    /// let runner = JobRunner::new(job_repo, registry)
    ///     .with_config(JobRunnerConfig {
    ///         poll_interval: Duration::from_secs(30),
    ///         ..Default::default()
    ///     });
    ///
    /// // This will run forever, efficiently waiting for jobs
    /// runner.run_with_notifier(&mut notifier).await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the notifier connection fails. The caller should
    /// implement reconnection logic if needed.
    pub async fn run_with_notifier(&self, notifier: &mut PgJobNotifier) -> Result<(), S::Error> {
        info!(
            fallback_interval_secs = self.config.poll_interval.as_secs(),
            "Starting job runner with LISTEN/NOTIFY"
        );

        loop {
            // Process all available jobs
            let mut did_work = true;
            while did_work {
                did_work = self.run_once().await?;
            }

            // No jobs available - wait for notification or fallback timeout
            debug!(
                timeout_secs = self.config.poll_interval.as_secs(),
                "Waiting for job notification"
            );

            match notifier.wait(self.config.poll_interval).await {
                Ok(Some(job_type)) => {
                    debug!(job_type = %job_type, "Woke up from notification");
                }
                Ok(None) => {
                    debug!("Woke up from fallback timeout");
                }
                Err(e) => {
                    // Log the error but don't exit - the notifier may reconnect
                    // on the next wait() call, or the caller may need to handle this
                    error!(error = %e, "Notifier error, will retry");
                    // Brief sleep to avoid tight loop on persistent errors
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "tests/runner_tests.rs"]
mod tests;
