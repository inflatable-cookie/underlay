use std::time::Duration;

use crate::{JobRegistry, JobStore};

#[derive(Debug, Clone)]
pub struct JobRunnerConfig {
    pub poll_interval: Duration,
}

impl Default for JobRunnerConfig {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_millis(250),
        }
    }
}

#[derive(Debug)]
pub struct JobRunner<S> {
    store: S,
    registry: JobRegistry,
    config: JobRunnerConfig,
}

impl<S> JobRunner<S>
where
    S: JobStore,
{
    pub fn new(store: S, registry: JobRegistry) -> Self {
        Self {
            store,
            registry,
            config: JobRunnerConfig::default(),
        }
    }

    pub fn with_config(mut self, config: JobRunnerConfig) -> Self {
        self.config = config;
        self
    }

    pub async fn run_once(&self) -> Result<bool, S::Error> {
        let allowed = self.registry.job_types();
        let Some(job) = self.store.fetch_next(&allowed).await? else {
            return Ok(false);
        };

        let Some(handler) = self.registry.handler(&job.job_type) else {
            // Not our job; treat as not processed.
            return Ok(false);
        };

        match handler.handle(job.clone()).await {
            Ok(()) => {
                self.store.mark_success(job.id).await?;
                Ok(true)
            }
            Err(err) => {
                self.store.mark_failure(job.id, err).await?;
                Ok(true)
            }
        }
    }

    pub async fn run_forever(&self) -> Result<(), S::Error> {
        loop {
            let did_work = self.run_once().await?;
            if !did_work {
                tokio::time::sleep(self.config.poll_interval).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use serde_json::json;

    use crate::{Job, JobHandler, JobHandlerError, JobId, JobRegistry, JobRunner, JobRunnerConfig, JobStore};

    #[derive(Debug, Default)]
    struct MemStore {
        queue: Mutex<Vec<Job>>,
        successes: Mutex<Vec<JobId>>,
        failures: Mutex<Vec<JobId>>,
    }

    #[derive(Debug)]
    struct MemError;

    impl std::fmt::Display for MemError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "mem error")
        }
    }

    impl std::error::Error for MemError {}

    #[async_trait]
    impl JobStore for Arc<MemStore> {
        type Error = MemError;

        async fn fetch_next(&self, allowed_types: &[String]) -> Result<Option<Job>, Self::Error> {
            let mut q = self.queue.lock().unwrap();
            let idx = q
                .iter()
                .position(|j| allowed_types.iter().any(|t| t == &j.job_type));
            Ok(idx.map(|i| q.remove(i)))
        }

        async fn mark_success(&self, job_id: JobId) -> Result<(), Self::Error> {
            self.successes.lock().unwrap().push(job_id);
            Ok(())
        }

        async fn mark_failure(
            &self,
            job_id: JobId,
            _error: JobHandlerError,
        ) -> Result<(), Self::Error> {
            self.failures.lock().unwrap().push(job_id);
            Ok(())
        }
    }

    #[derive(Debug)]
    struct TestHandler;

    #[async_trait]
    impl JobHandler for TestHandler {
        fn job_type(&self) -> &'static str {
            "test"
        }

        async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn runner_dispatches_jobs_to_registered_handler() {
        let store = Arc::new(MemStore::default());

        let id = underlay_core::Uuid::new_v7();
        store.queue.lock().unwrap().push(Job {
            id,
            job_type: "test".to_string(),
            payload: serde_json::json!({"ok": true}),
            attempts: 0,
        });

        let mut registry = JobRegistry::new();
        registry.register(TestHandler);

        let runner = JobRunner::new(store.clone(), registry);
        let did_work = runner.run_once().await.expect("run_once");
        assert!(did_work);
        assert_eq!(store.successes.lock().unwrap().as_slice(), &[id]);
    }

    #[tokio::test]
    async fn runner_returns_false_when_no_jobs_available() {
        let store = Arc::new(MemStore::default());
        let registry = JobRegistry::new();
        let runner = JobRunner::new(store, registry);
        let did_work = runner.run_once().await.expect("run_once");
        assert!(!did_work);
    }

    #[tokio::test]
    async fn runner_ignores_unknown_job_types() {
        let store = Arc::new(MemStore::default());

        store.queue.lock().unwrap().push(Job {
            id: underlay_core::Uuid::new_v7(),
            job_type: "unknown_type".to_string(),
            payload: json!({}),
            attempts: 0,
        });

        let registry = JobRegistry::new();
        let runner = JobRunner::new(store, registry);
        let did_work = runner.run_once().await.expect("run_once");
        assert!(!did_work);
    }

    #[tokio::test]
    async fn runner_records_failures() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        #[derive(Debug)]
        struct FailingHandler {
            called: Arc<AtomicBool>,
        }

        #[async_trait]
        impl JobHandler for FailingHandler {
            fn job_type(&self) -> &'static str {
                "failing"
            }

            async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
                self.called.store(true, Ordering::SeqCst);
                Err(JobHandlerError::new("intentional failure"))
            }
        }

        let store = Arc::new(MemStore::default());
        let called = Arc::new(AtomicBool::new(false));

        store.queue.lock().unwrap().push(Job {
            id: underlay_core::Uuid::new_v7(),
            job_type: "failing".to_string(),
            payload: json!({}),
            attempts: 0,
        });

        let mut registry = JobRegistry::new();
        registry.register(FailingHandler { called: called.clone() });

        let runner = JobRunner::new(store.clone(), registry);
        let did_work = runner.run_once().await.expect("run_once");
        assert!(did_work);
        assert!(called.load(Ordering::SeqCst));
        assert_eq!(store.failures.lock().unwrap().len(), 1);
    }

    #[test]
    fn job_runner_config_default_values() {
        let config = JobRunnerConfig::default();
        assert_eq!(config.poll_interval.as_millis(), 250);
    }

    #[test]
    fn job_runner_config_with_custom_values() {
        let config = JobRunnerConfig {
            poll_interval: std::time::Duration::from_secs(5),
        };
        assert_eq!(config.poll_interval.as_secs(), 5);
    }

    #[test]
    fn job_runner_debug_format() {
        let registry = JobRegistry::new();
        let runner = JobRunner::new(
            std::sync::Arc::new(MemStore::default()),
            registry
        );
        let debug_str = format!("{:?}", runner);
        assert!(debug_str.contains("JobRunner"));
    }
}
