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
//!     .with_config(JobRunnerConfig::default().with_poll_interval(Duration::from_secs(30)));
//!
//! runner.run_forever().await?;
//! ```

use std::any::Any;
use std::future::{self, Future};
use std::panic::{self, AssertUnwindSafe};
use std::pin::{pin, Pin};
use std::task::{Context, Poll};
use std::time::Duration;

use tracing::{debug, error, info, warn};

use crate::events::{JobEvent, JobEventHub, JobEventSink};
use crate::registry::JobRegistry;
use crate::store::{JobCompletionOutcome, JobFailureRecordOutcome, JobStore};
use crate::types::{Job, JobConfig, JobHandlerError};

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
    poll_interval: Duration,

    /// How many jobs to process before sleeping (0 = unlimited).
    batch_size: usize,
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

impl JobRunnerConfig {
    pub fn poll_interval(&self) -> Duration {
        self.poll_interval
    }

    pub fn batch_size(&self) -> usize {
        self.batch_size
    }

    pub fn with_poll_interval(mut self, poll_interval: Duration) -> Self {
        self.poll_interval = poll_interval;
        self
    }

    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
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
                "No handler registered for job type; failing claimed job"
            );
            let err = JobHandlerError::permanent(format!(
                "no handler registered for job type `{}`",
                job.job_type
            ));
            self.record_failure(&job, err, &JobConfig::default())
                .await?;
            return Ok(true);
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

        let handle_result = match panic::catch_unwind(AssertUnwindSafe(|| {
            handler.handle(job.clone())
        })) {
            Err(payload) => Err(JobHandlerError::new(format!(
                "job handler panicked: {}",
                panic_payload_to_string(payload)
            ))),
            Ok(fut) => {
                let guarded = CatchUnwindFuture(fut);
                let outcome = if let Some(timeout_secs) = handler_config.timeout_seconds() {
                    let timeout = std::time::Duration::from_secs(u64::from(timeout_secs.max(1)));
                    match tokio::time::timeout(timeout, guarded).await {
                        Ok(res) => res,
                        Err(_) => Ok(Err(JobHandlerError::new(format!(
                            "job timed out after {}s",
                            timeout.as_secs()
                        )))),
                    }
                } else {
                    guarded.await
                };
                match outcome {
                    Ok(res) => res,
                    Err(payload) => Err(JobHandlerError::new(format!(
                        "job handler panicked: {}",
                        panic_payload_to_string(payload)
                    ))),
                }
            }
        };

        match handle_result {
            Ok(()) => {
                match self.store.mark_success(&job).await? {
                    JobCompletionOutcome::Succeeded => {
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
                    }
                    JobCompletionOutcome::LostOwnership => {
                        warn!(
                            job_type = %job.job_type,
                            job_id = %job.id,
                            attempt = job.attempts,
                            "Job completion ignored because attempt no longer owns the running claim"
                        );
                    }
                }
                Ok(true)
            }
            Err(err) => {
                let is_permanent = err.is_permanent;
                if self.record_failure(&job, err, &handler_config).await? {
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
                }
                Ok(true)
            }
        }
    }

    async fn record_failure(
        &self,
        job: &Job,
        error: JobHandlerError,
        config: &JobConfig,
    ) -> Result<bool, S::Error> {
        let message = error.message.clone();
        let outcome = match self.store.mark_failure(job, error, config).await? {
            JobFailureRecordOutcome::Recorded(outcome) => outcome,
            JobFailureRecordOutcome::LostOwnership => {
                warn!(
                    job_type = %job.job_type,
                    job_id = %job.id,
                    attempt = job.attempts,
                    "Job failure ignored because attempt no longer owns the running claim"
                );
                return Ok(false);
            }
        };
        self.events.emit(JobEvent::Failed {
            job_id: job.id,
            job_type: job.job_type.clone(),
            error: message,
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
        Ok(true)
    }

    /// Run the job processing loop forever using polling.
    ///
    /// This will continuously poll for and process jobs, sleeping for
    /// `poll_interval` when no jobs are available.
    ///
    /// Adapter crates may expose notification-based loops for storage backends
    /// that support wake-up events.
    pub async fn run_forever(&self) -> Result<(), S::Error> {
        self.run_until_shutdown(future::pending()).await
    }

    /// Poll for jobs until the supplied shutdown future resolves.
    ///
    /// A shutdown wakes an idle runner immediately. If a fetch or handler is
    /// already in progress, the runner settles that job before shutting down.
    ///
    /// # Errors
    ///
    /// Returns the store error if fetching a job or recording its outcome fails.
    pub async fn run_until_shutdown(
        &self,
        shutdown: impl Future<Output = ()>,
    ) -> Result<(), S::Error> {
        info!(
            poll_interval_ms = self.config.poll_interval().as_millis(),
            "Starting job runner in polling mode"
        );

        let batch_size = self.config.batch_size();
        let mut shutdown = pin!(shutdown);
        let mut should_sleep = false;
        let mut processed_in_batch = 0usize;

        loop {
            tokio::select! {
                biased;
                () = &mut shutdown => return Ok(()),
                () = async {
                    if should_sleep {
                        tokio::time::sleep(self.config.poll_interval()).await;
                    }
                } => {}
            }

            let did_work = self.run_once().await?;
            if did_work && batch_size > 0 {
                processed_in_batch += 1;
            }
            should_sleep = !did_work || (batch_size > 0 && processed_in_batch >= batch_size);
            if should_sleep {
                processed_in_batch = 0;
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

struct CatchUnwindFuture<F>(F);

impl<F> Future for CatchUnwindFuture<F>
where
    F: Future + Unpin,
{
    type Output = std::thread::Result<F::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match panic::catch_unwind(AssertUnwindSafe(|| Pin::new(&mut self.0).poll(cx))) {
            Ok(Poll::Ready(output)) => Poll::Ready(Ok(output)),
            Ok(Poll::Pending) => Poll::Pending,
            Err(payload) => Poll::Ready(Err(payload)),
        }
    }
}

fn panic_payload_to_string(payload: Box<dyn Any + Send>) -> String {
    if let Some(msg) = payload.downcast_ref::<&str>() {
        (*msg).to_string()
    } else if let Some(msg) = payload.downcast_ref::<String>() {
        msg.clone()
    } else {
        "unknown panic payload".to_string()
    }
}

#[cfg(test)]
#[path = "tests/runner_tests/mod.rs"]
mod tests;
