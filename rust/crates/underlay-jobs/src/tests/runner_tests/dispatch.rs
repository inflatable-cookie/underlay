use super::*;

#[tokio::test]
async fn runner_dispatches_jobs_to_registered_handler() {
    let store = Arc::new(MemStore::default());
    let job = make_test_job("test");
    let job_id = job.id;
    store.queue.lock().unwrap().push(job);
    let event_sink = Arc::new(RecordingEventSink::default());

    let mut registry = JobRegistry::new();
    registry.register(TestHandler);

    let runner = JobRunner::new(store.clone(), registry).with_event_sink(event_sink.clone());
    let did_work = runner.run_once().await.expect("run_once");
    assert!(did_work);
    assert_eq!(store.successes.lock().unwrap().as_slice(), &[job_id]);

    let events = event_sink.events();
    assert!(
        matches!(events[0], JobEvent::Claimed { job_id: event_job_id, .. } if event_job_id == job_id)
    );
    assert!(
        matches!(events[1], JobEvent::Started { job_id: event_job_id, .. } if event_job_id == job_id)
    );
    assert!(
        matches!(events[2], JobEvent::Completed { job_id: event_job_id, .. } if event_job_id == job_id)
    );
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
