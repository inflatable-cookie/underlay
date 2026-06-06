use std::sync::atomic::{AtomicBool, Ordering};

use super::*;

#[tokio::test]
async fn runner_records_failures() {
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
    let event_sink = Arc::new(RecordingEventSink::default());

    store.queue.lock().unwrap().push(make_test_job("failing"));

    let mut registry = JobRegistry::new();
    registry.register(FailingHandler {
        called: called.clone(),
    });

    let runner = JobRunner::new(store.clone(), registry).with_event_sink(event_sink.clone());
    let did_work = runner.run_once().await.expect("run_once");
    assert!(did_work);
    assert!(called.load(Ordering::SeqCst));
    assert_eq!(store.failures.lock().unwrap().len(), 1);
    assert_eq!(store.failure_calls.lock().unwrap().len(), 1);

    let events = event_sink.events();
    assert!(matches!(
        events[2],
        JobEvent::Failed {
            will_retry: false,
            ..
        }
    ));
    assert!(matches!(events[3], JobEvent::DeadLettered { .. }));
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
    assert_eq!(store.dead_letters.lock().unwrap().len(), 0);
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
    assert_eq!(store.dead_letters.lock().unwrap().len(), 1);
}
