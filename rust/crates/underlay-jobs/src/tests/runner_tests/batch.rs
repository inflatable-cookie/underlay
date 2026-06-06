use super::*;

#[test]
fn job_runner_config_default_values() {
    let config = JobRunnerConfig::default();
    assert_eq!(config.poll_interval().as_secs(), 30);
    assert_eq!(config.batch_size(), 0);
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
