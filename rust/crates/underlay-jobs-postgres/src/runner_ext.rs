use std::future::{self, Future};
use std::pin::pin;

use async_trait::async_trait;
use tracing::{debug, error, info};

use crate::{PgJobNotifier, RepoError};
use underlay_jobs::{JobRunner, JobStore};

/// PostgreSQL LISTEN/NOTIFY runner support for `JobRunner`.
#[async_trait]
pub trait PostgresJobRunnerExt {
    /// Run the job processing loop using PostgreSQL LISTEN/NOTIFY.
    async fn run_with_notifier(&self, notifier: &mut PgJobNotifier) -> Result<(), RepoError>;

    /// Run the job processing loop using PostgreSQL LISTEN/NOTIFY until the
    /// supplied shutdown future resolves.
    ///
    /// A shutdown wakes an idle runner immediately. If a job is already in
    /// progress, the runner settles that job before shutting down.
    async fn run_with_notifier_until_shutdown<F>(
        &self,
        notifier: &mut PgJobNotifier,
        shutdown: F,
    ) -> Result<(), RepoError>
    where
        F: Future<Output = ()> + Send;
}

#[async_trait]
impl<S> PostgresJobRunnerExt for JobRunner<S>
where
    S: JobStore<Error = RepoError> + Sync,
{
    async fn run_with_notifier(&self, notifier: &mut PgJobNotifier) -> Result<(), RepoError> {
        self.run_with_notifier_until_shutdown(notifier, future::pending())
            .await
    }

    async fn run_with_notifier_until_shutdown<F>(
        &self,
        notifier: &mut PgJobNotifier,
        shutdown: F,
    ) -> Result<(), RepoError>
    where
        F: Future<Output = ()> + Send,
    {
        let poll_interval = self.config().poll_interval();
        let batch_size = self.config().batch_size();
        let mut shutdown = pin!(shutdown);

        info!(
            fallback_interval_secs = poll_interval.as_secs(),
            batch_size, "Starting job runner with LISTEN/NOTIFY"
        );

        loop {
            let mut processed_in_batch = 0usize;
            loop {
                tokio::select! {
                    biased;
                    () = &mut shutdown => return Ok(()),
                    () = future::ready(()) => {}
                }

                let did_work = self.run_once().await?;
                if !did_work {
                    break;
                }

                if batch_size > 0 {
                    processed_in_batch += 1;
                    if processed_in_batch >= batch_size {
                        break;
                    }
                }
            }

            // Coalesce burst notifications that arrived while draining the batch.
            notifier.drain_pending().await;

            debug!(
                timeout_secs = poll_interval.as_secs(),
                "Waiting for job notification"
            );

            tokio::select! {
                biased;
                () = &mut shutdown => return Ok(()),
                wait_result = notifier.wait(poll_interval) => {
                    match wait_result {
                        Ok(Some(job_type)) => {
                            debug!(job_type = %job_type, "Woke up from notification");
                            notifier.drain_pending().await;
                        }
                        Ok(None) => {
                            debug!("Woke up from fallback timeout");
                        }
                        Err(error) => {
                            error!(error = %error, "Notifier error, will retry");
                            tokio::select! {
                                biased;
                                () = &mut shutdown => return Ok(()),
                                () = tokio::time::sleep(std::time::Duration::from_secs(1)) => {}
                            }
                        }
                    }
                }
            }
        }
    }
}
