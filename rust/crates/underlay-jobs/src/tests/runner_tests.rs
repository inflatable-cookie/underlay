use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::Utc;
use serde_json::json;

use crate::types::{Job, JobConfig, JobHandler, JobHandlerError, JobId, JobStatus};
use crate::{JobRegistry, JobRunner, JobRunnerConfig, JobStore};

#[derive(Debug, Default)]
struct MemStore {
    queue: Mutex<Vec<Job>>,
    successes: Mutex<Vec<JobId>>,
    failures: Mutex<Vec<JobId>>,
    failure_calls: Mutex<Vec<FailureCall>>,
}

#[derive(Debug, Clone)]
struct FailureCall {
    job_id: JobId,
    is_permanent: bool,
    max_attempts: u32,
    retry_delay_secs: u64,
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
        error: JobHandlerError,
        config: &crate::types::JobConfig,
    ) -> Result<(), Self::Error> {
        self.failures.lock().unwrap().push(job_id);
        self.failure_calls.lock().unwrap().push(FailureCall {
            job_id,
            is_permanent: error.is_permanent,
            max_attempts: config.max_attempts,
            retry_delay_secs: config.backoff.delay_for_attempt(0).as_secs(),
        });
        Ok(())
    }
}

fn make_test_job(job_type: &str) -> Job {
    let now = Utc::now();
    Job {
        id: underlay_core::Uuid::new_v7(),
        job_type: job_type.to_string(),
        status: JobStatus::Pending,
        payload: json!({"ok": true}),
        attempts: 0,
        max_attempts: 1,
        scheduled_for: None,
        priority: 0,
        claimed_at: None,
        claimed_by: None,
        started_at: None,
        finished_at: None,
        heartbeat_at: None,
        progress: None,
        last_error: None,
        created_at: now,
        updated_at: now,
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
    let job = make_test_job("test");
    let job_id = job.id;
    store.queue.lock().unwrap().push(job);

    let mut registry = JobRegistry::new();
    registry.register(TestHandler);

    let runner = JobRunner::new(store.clone(), registry);
    let did_work = runner.run_once().await.expect("run_once");
    assert!(did_work);
    assert_eq!(store.successes.lock().unwrap().as_slice(), &[job_id]);
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
    store
        .queue
        .lock()
        .unwrap()
        .push(make_test_job("unknown_type"));

    let registry = JobRegistry::new();
    let runner = JobRunner::new(store, registry);
    let did_work = runner.run_once().await.expect("run_once");
    assert!(!did_work);
}

#[tokio::test]
async fn runner_records_failures() {
    use std::sync::atomic::{AtomicBool, Ordering};

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

    store.queue.lock().unwrap().push(make_test_job("failing"));

    let mut registry = JobRegistry::new();
    registry.register(FailingHandler {
        called: called.clone(),
    });

    let runner = JobRunner::new(store.clone(), registry);
    let did_work = runner.run_once().await.expect("run_once");
    assert!(did_work);
    assert!(called.load(Ordering::SeqCst));
    assert_eq!(store.failures.lock().unwrap().len(), 1);
    assert_eq!(store.failure_calls.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn runner_passes_handler_config_to_failure_path() {
    #[derive(Debug)]
    struct ConfiguredFailingHandler;

    #[async_trait]
    impl JobHandler for ConfiguredFailingHandler {
        fn job_type(&self) -> &'static str {
            "configured_failing"
        }

        fn config(&self) -> JobConfig {
            JobConfig::new().with_max_attempts(4).with_fixed_backoff(42)
        }

        async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
            Err(JobHandlerError::new("retryable failure"))
        }
    }

    let store = Arc::new(MemStore::default());
    let job = make_test_job("configured_failing");
    let job_id = job.id;
    store.queue.lock().unwrap().push(job);

    let mut registry = JobRegistry::new();
    registry.register(ConfiguredFailingHandler);

    let runner = JobRunner::new(store.clone(), registry);
    let did_work = runner.run_once().await.expect("run_once");

    assert!(did_work);
    let calls = store.failure_calls.lock().unwrap();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].job_id, job_id);
    assert!(!calls[0].is_permanent);
    assert_eq!(calls[0].max_attempts, 4);
    assert_eq!(calls[0].retry_delay_secs, 42);
}

#[tokio::test]
async fn runner_flags_permanent_failures_for_store() {
    #[derive(Debug)]
    struct PermanentFailingHandler;

    #[async_trait]
    impl JobHandler for PermanentFailingHandler {
        fn job_type(&self) -> &'static str {
            "permanent_failing"
        }

        fn config(&self) -> JobConfig {
            JobConfig::new().with_max_attempts(7).with_fixed_backoff(30)
        }

        async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
            Err(JobHandlerError::permanent("permanent failure"))
        }
    }

    let store = Arc::new(MemStore::default());
    let job = make_test_job("permanent_failing");
    let job_id = job.id;
    store.queue.lock().unwrap().push(job);

    let mut registry = JobRegistry::new();
    registry.register(PermanentFailingHandler);

    let runner = JobRunner::new(store.clone(), registry);
    let did_work = runner.run_once().await.expect("run_once");

    assert!(did_work);
    let calls = store.failure_calls.lock().unwrap();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].job_id, job_id);
    assert!(calls[0].is_permanent);
    assert_eq!(calls[0].max_attempts, 7);
}

#[test]
fn job_runner_config_default_values() {
    let config = JobRunnerConfig::default();
    // Default is 30 seconds (suitable for notification mode)
    assert_eq!(config.poll_interval.as_secs(), 30);
    assert_eq!(config.batch_size, 0);
}

#[tokio::test]
async fn run_batch_processes_limited_jobs() {
    let store = Arc::new(MemStore::default());

    for _ in 0..5 {
        store.queue.lock().unwrap().push(make_test_job("test"));
    }

    let mut registry = JobRegistry::new();
    registry.register(TestHandler);

    let runner = JobRunner::new(store.clone(), registry);
    let processed = runner.run_batch(3).await.expect("run_batch");

    assert_eq!(processed, 3);
    assert_eq!(store.successes.lock().unwrap().len(), 3);
    assert_eq!(store.queue.lock().unwrap().len(), 2);
}
