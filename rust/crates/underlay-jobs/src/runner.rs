//! Job runner for processing queued jobs.
//!
//! The runner fetches jobs from a store and dispatches them to registered handlers.
//! It supports polling mode directly. Adapter crates can add notification mode
//! for storage backends that support it.
//!
//! # Example: Polling Mode
//!
//! ```ignore
//! use underlay_jobs::{JobRunner, JobRunnerConfig};
//! use std::time::Duration;
//!
//! let runner = JobRunner::new(store, registry)
//!     .with_config(JobRunnerConfig {
//!         poll_interval: Duration::from_secs(30), // Fallback interval
//!         ..Default::default()
//!     });
//!
//! runner.run_forever().await?;
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

    /// Return the runner configuration.
    pub fn config(&self) -> &JobRunnerConfig {
        &self.config
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
    /// Adapter crates may expose notification-based loops for storage backends
    /// that support wake-up events.
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

#[cfg(test)]
#[path = "tests/runner_tests/mod.rs"]
mod tests;
