use super::*;

#[tokio::test(start_paused = true)]
async fn shutdown_interrupts_idle_poll_delay() {
    use std::time::Duration;

    let runner = JobRunner::new(Arc::new(MemStore::default()), JobRegistry::new());
    let start = tokio::time::Instant::now();
    runner
        .run_until_shutdown(tokio::time::sleep(Duration::from_secs(1)))
        .await
        .unwrap();
    assert_eq!(start.elapsed(), Duration::from_secs(1));
}

#[tokio::test(start_paused = true)]
async fn shutdown_settles_in_flight_job_before_stopping() {
    use std::time::Duration;

    #[derive(Debug)]
    struct SlowHandler;

    #[async_trait]
    impl JobHandler for SlowHandler {
        fn job_type(&self) -> &'static str {
            "test"
        }

        async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
            tokio::time::sleep(Duration::from_secs(2)).await;
            Ok(())
        }
    }

    let store = Arc::new(MemStore::default());
    let first = make_test_job("test");
    let first_id = first.id;
    store
        .queue
        .lock()
        .unwrap()
        .extend([first, make_test_job("test")]);

    let mut registry = JobRegistry::new();
    registry.register(SlowHandler);
    let runner = JobRunner::new(store.clone(), registry);

    runner
        .run_until_shutdown(tokio::time::sleep(Duration::from_secs(1)))
        .await
        .unwrap();

    assert_eq!(store.successes.lock().unwrap().as_slice(), &[first_id]);
    assert_eq!(store.queue.lock().unwrap().len(), 1);
}

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
