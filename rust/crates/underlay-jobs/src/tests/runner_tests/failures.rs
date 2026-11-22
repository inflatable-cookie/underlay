use std::sync::atomic::{AtomicBool, Ordering};

use super::*;

#[tokio::test]
async fn runner_emits_failure_and_dead_letter_for_unregistered_claim() {
    let store = Arc::new(MemStore {
        ignore_allowed_types: true,
        ..Default::default()
    });
    let job = make_test_job("unregistered");
    let job_id = job.id;
    store.queue.lock().unwrap().push(job);
    let events = Arc::new(RecordingEventSink::default());
    let runner = JobRunner::new(store.clone(), JobRegistry::new()).with_event_sink(events.clone());

    assert!(runner.run_once().await.unwrap());
    let calls = store.failure_calls.lock().unwrap();
    assert_eq!(calls.len(), 1);
    assert!(calls[0].is_permanent);
    let events = events.events();
    assert!(matches!(
        events.as_slice(),
        [
            JobEvent::Failed { job_id: failed_id, will_retry: false, .. },
            JobEvent::DeadLettered { job_id: dead_letter_job_id, .. },
        ] if *failed_id == job_id && *dead_letter_job_id == job_id
    ));
}

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

#[tokio::test]
async fn runner_isolates_panicking_handler_and_continues() {
    #[derive(Debug)]
    struct PanickingHandler;

    #[async_trait]
    impl JobHandler for PanickingHandler {
        fn job_type(&self) -> &'static str {
            "panicking"
        }

        async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
            tokio::task::yield_now().await;
            panic!("handler exploded");
        }
    }

    let store = Arc::new(MemStore::default());
    let panic_job = make_test_job("panicking");
    let panic_job_id = panic_job.id;
    let ok_job = make_test_job("test");
    let ok_job_id = ok_job.id;
    store.queue.lock().unwrap().extend([panic_job, ok_job]);

    let events = Arc::new(RecordingEventSink::default());
    let mut registry = JobRegistry::new();
    registry.register(PanickingHandler);
    registry.register(TestHandler);

    let runner = JobRunner::new(store.clone(), registry).with_event_sink(events.clone());

    let processed = runner
        .run_batch(2)
        .await
        .expect("panicking handler must not crash runner");
    assert_eq!(processed, 2);
    assert_eq!(store.failures.lock().unwrap().as_slice(), &[panic_job_id]);
    assert_eq!(store.successes.lock().unwrap().as_slice(), &[ok_job_id]);

    let failed_event = events
        .events()
        .into_iter()
        .find_map(|event| match event {
            JobEvent::Failed { job_id, error, .. } if job_id == panic_job_id => Some(error),
            _ => None,
        })
        .expect("Failed event should be emitted for panicking handler");
    assert!(
        failed_event.contains("handler exploded"),
        "expected panic message in failure error, got: {failed_event}"
    );
}

#[tokio::test]
async fn runner_handles_lost_ownership_on_completion_and_continues() {
    let store = Arc::new(MemStore::default());
    let stale_job = make_test_job("test");
    let stale_job_id = stale_job.id;
    let healthy_job = make_test_job("test");
    let healthy_job_id = healthy_job.id;

    store
        .stale_completion_ids
        .lock()
        .unwrap()
        .push(stale_job_id);
    store.queue.lock().unwrap().extend([stale_job, healthy_job]);

    let events = Arc::new(RecordingEventSink::default());
    let mut registry = JobRegistry::new();
    registry.register(TestHandler);

    let runner = JobRunner::new(store.clone(), registry).with_event_sink(events.clone());

    let processed = runner
        .run_batch(2)
        .await
        .expect("lost ownership must not terminate runner loop");
    assert_eq!(processed, 2);
    assert_eq!(
        store.successes.lock().unwrap().as_slice(),
        &[healthy_job_id]
    );

    let completed_ids: Vec<JobId> = events
        .events()
        .into_iter()
        .filter_map(|event| match event {
            JobEvent::Completed { job_id, .. } => Some(job_id),
            _ => None,
        })
        .collect();
    assert_eq!(
        completed_ids,
        vec![healthy_job_id],
        "stale completion must not emit JobEvent::Completed"
    );
}

#[tokio::test]
async fn runner_handles_lost_ownership_on_failure_and_continues() {
    #[derive(Debug)]
    struct StaleFailingHandler;

    #[async_trait]
    impl JobHandler for StaleFailingHandler {
        fn job_type(&self) -> &'static str {
            "stale_failing"
        }

        async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
            Err(JobHandlerError::new("stale worker failed"))
        }
    }

    let store = Arc::new(MemStore::default());
    let stale_job = make_test_job("stale_failing");
    let stale_job_id = stale_job.id;
    let healthy_job = make_test_job("test");
    let healthy_job_id = healthy_job.id;

    store.stale_failure_ids.lock().unwrap().push(stale_job_id);
    store.queue.lock().unwrap().extend([stale_job, healthy_job]);

    let events = Arc::new(RecordingEventSink::default());
    let mut registry = JobRegistry::new();
    registry.register(StaleFailingHandler);
    registry.register(TestHandler);

    let runner = JobRunner::new(store.clone(), registry).with_event_sink(events.clone());

    let processed = runner
        .run_batch(2)
        .await
        .expect("lost ownership on failure must not terminate runner loop");
    assert_eq!(processed, 2);
    assert!(store.failures.lock().unwrap().is_empty());
    assert_eq!(
        store.successes.lock().unwrap().as_slice(),
        &[healthy_job_id]
    );

    let failed_events: Vec<JobId> = events
        .events()
        .into_iter()
        .filter_map(|event| match event {
            JobEvent::Failed { job_id, .. } | JobEvent::DeadLettered { job_id, .. } => Some(job_id),
            _ => None,
        })
        .collect();
    assert!(
        failed_events.is_empty(),
        "stale failure must not emit JobEvent::Failed or JobEvent::DeadLettered"
    );
}
