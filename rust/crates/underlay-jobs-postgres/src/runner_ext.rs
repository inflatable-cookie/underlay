use async_trait::async_trait;
use tracing::{debug, error, info};

use crate::{PgJobNotifier, RepoError};
use underlay_jobs::{JobRunner, JobStore};

/// PostgreSQL LISTEN/NOTIFY runner support for `JobRunner`.
#[async_trait]
pub trait PostgresJobRunnerExt {
    /// Run the job processing loop using PostgreSQL LISTEN/NOTIFY.
    async fn run_with_notifier(&self, notifier: &mut PgJobNotifier) -> Result<(), RepoError>;
}

#[async_trait]
impl<S> PostgresJobRunnerExt for JobRunner<S>
where
    S: JobStore<Error = RepoError> + Sync,
{
    async fn run_with_notifier(&self, notifier: &mut PgJobNotifier) -> Result<(), RepoError> {
        let poll_interval = self.config().poll_interval();

        info!(
            fallback_interval_secs = poll_interval.as_secs(),
            "Starting job runner with LISTEN/NOTIFY"
        );

        loop {
            let mut did_work = true;
            while did_work {
                did_work = self.run_once().await?;
            }

            debug!(
                timeout_secs = poll_interval.as_secs(),
                "Waiting for job notification"
            );

            match notifier.wait(poll_interval).await {
                Ok(Some(job_type)) => {
                    debug!(job_type = %job_type, "Woke up from notification");
                }
                Ok(None) => {
                    debug!("Woke up from fallback timeout");
                }
                Err(error) => {
                    error!(error = %error, "Notifier error, will retry");
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }
    }
}
