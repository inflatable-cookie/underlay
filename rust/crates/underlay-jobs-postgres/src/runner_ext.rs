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
    ///
    /// Kept `Self: Sized` with a default implementation so existing
    /// implementations and `dyn PostgresJobRunnerExt` trait objects remain
    /// source-compatible.
    async fn run_with_notifier_until_shutdown<F>(
        &self,
        notifier: &mut PgJobNotifier,
        shutdown: F,
    ) -> Result<(), RepoError>
    where
        Self: Sized + Sync,
        F: Future<Output = ()> + Send,
    {
        let mut shutdown = pin!(shutdown);
        tokio::select! {
            biased;
            () = &mut shutdown => Ok(()),
            res = self.run_with_notifier(notifier) => res,
        }
    }
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
            let mut hit_batch_limit = false;

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
                        hit_batch_limit = true;
                        break;
                    }
                }
            }

            // Coalesce already-buffered notifications without blocking on the socket.
            notifier.drain();

            // When a bounded batch filled completely, more jobs may still be pending.
            // Yield once so shutdown and other tasks get a turn, then continue draining
            // immediately instead of sleeping for `poll_interval`.
            if hit_batch_limit {
                tokio::task::yield_now().await;
                continue;
            }

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
                            notifier.drain();
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

#[cfg(test)]
mod tests {
    use super::PostgresJobRunnerExt;

    #[test]
    fn postgres_job_runner_ext_remains_object_safe() {
        fn _assert_object_safe(_: &dyn PostgresJobRunnerExt) {}
    }
}
