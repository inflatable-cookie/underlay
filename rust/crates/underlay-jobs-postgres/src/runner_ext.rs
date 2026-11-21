use std::future::{self, Future};
use std::pin::pin;
use std::time::Duration;

use async_trait::async_trait;
use tracing::{debug, error, info};

use crate::{PgJobNotifier, RepoError};
use underlay_jobs::{JobRunner, JobStore};

const MAX_COALESCED_NOTIFICATIONS: usize = 64;
const COALESCE_SOCKET_TIMEOUT: Duration = Duration::from_millis(2);

/// PostgreSQL LISTEN/NOTIFY runner support for `JobRunner`.
#[async_trait]
pub trait PostgresJobRunnerExt {
    /// Run the job processing loop using PostgreSQL LISTEN/NOTIFY.
    async fn run_with_notifier(&self, notifier: &mut PgJobNotifier) -> Result<(), RepoError>;
}

/// Graceful-shutdown LISTEN/NOTIFY runner support for runners that can stop
/// between job claims and settle an in-flight job before returning.
#[async_trait]
pub trait PostgresJobRunnerShutdownExt {
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
}

#[async_trait]
impl<S> PostgresJobRunnerShutdownExt for JobRunner<S>
where
    S: JobStore<Error = RepoError> + Sync,
{
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

                processed_in_batch += 1;
                if batch_size > 0 && processed_in_batch >= batch_size {
                    hit_batch_limit = true;
                    break;
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

            // If we processed jobs during this pass, coalesce burst notifications
            // that arrived on the LISTEN socket while draining before sleeping.
            if processed_in_batch > 0 {
                tokio::select! {
                    biased;
                    () = &mut shutdown => return Ok(()),
                    _ = notifier.coalesce_pending(MAX_COALESCED_NOTIFICATIONS, COALESCE_SOCKET_TIMEOUT) => {}
                }
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
                            // Coalesce additional burst notifications already queued on the socket
                            // while staying responsive to shutdown.
                            tokio::select! {
                                biased;
                                () = &mut shutdown => return Ok(()),
                                _ = notifier.coalesce_pending(MAX_COALESCED_NOTIFICATIONS, COALESCE_SOCKET_TIMEOUT) => {}
                            }
                        }
                        Ok(None) => {
                            debug!("Woke up from fallback timeout");
                        }
                        Err(error) => {
                            error!(error = %error, "Notifier error, will retry");
                            tokio::select! {
                                biased;
                                () = &mut shutdown => return Ok(()),
                                () = tokio::time::sleep(Duration::from_secs(1)) => {}
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
