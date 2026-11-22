use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use super::*;

#[derive(Default)]
struct MemOutbox {
    events: Mutex<Vec<(OutboxEvent, bool)>>,
}

impl MemOutbox {
    fn push(&self, event_type: &str) -> uuid::Uuid {
        let id = uuid::Uuid::now_v7();
        self.events.lock().unwrap().push((
            OutboxEvent {
                id,
                event_type: event_type.to_string(),
                payload: serde_json::json!({}),
                occurred_at: Utc::now(),
            },
            false,
        ));
        id
    }

    fn processed_ids(&self) -> Vec<uuid::Uuid> {
        self.events
            .lock()
            .unwrap()
            .iter()
            .filter_map(|(event, processed)| processed.then_some(event.id))
            .collect()
    }
}

impl OutboxBackend for MemOutbox {
    type Error = std::convert::Infallible;

    async fn process_next<'a, F, Fut>(
        &'a self,
        skip_ids: &'a [uuid::Uuid],
        after: Option<DrainCursor>,
        timeout: Duration,
        handler: &'a F,
    ) -> Result<ProcessOneOutcome, Self::Error>
    where
        F: Fn(OutboxEvent) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'a,
    {
        let next = {
            let guard = self.events.lock().unwrap();
            let mut candidates: Vec<OutboxEvent> = guard
                .iter()
                .filter(|(event, processed)| {
                    !processed
                        && !skip_ids.contains(&event.id)
                        && after.is_none_or(|cursor| {
                            (event.occurred_at, event.id) > (cursor.occurred_at, cursor.id)
                        })
                })
                .map(|(event, _)| event.clone())
                .collect();
            candidates.sort_by_key(|event| (event.occurred_at, event.id));
            candidates.into_iter().next()
        };

        let Some(event) = next else {
            return Ok(ProcessOneOutcome::Empty);
        };

        let event_id = event.id;
        let event_type = event.event_type.clone();
        let occurred_at = event.occurred_at;
        match invoke_handler_guarded(timeout, handler, event).await {
            Ok(()) => {
                let mut guard = self.events.lock().unwrap();
                if let Some((_, processed)) = guard.iter_mut().find(|(e, _)| e.id == event_id) {
                    *processed = true;
                }
                Ok(ProcessOneOutcome::Succeeded(event_id))
            }
            Err(error) => Ok(ProcessOneOutcome::Failed {
                event_id,
                event_type,
                occurred_at,
                error,
            }),
        }
    }
}

struct IdleNotifier;

#[async_trait]
impl OutboxWakeup for IdleNotifier {
    type Error = &'static str;

    async fn wait_notification(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<String>, Self::Error> {
        tokio::time::sleep(timeout).await;
        Ok(None)
    }
}

#[test]
fn outbox_config_defaults() {
    let config = OutboxConfig::default();
    assert_eq!(config.batch_size(), 100);
    assert_eq!(config.fallback_interval(), Duration::from_secs(30));
    assert_eq!(config.handler_timeout(), Duration::from_secs(30));
    assert_eq!(config.failure_backoff(), Duration::from_secs(5));
    assert_eq!(config.max_failure_backoff(), Duration::from_secs(300));
}

#[test]
fn outbox_config_builder() {
    let config = OutboxConfig::default()
        .with_batch_size(50)
        .with_fallback_interval(Duration::from_secs(60))
        .with_handler_timeout(Duration::from_secs(5))
        .with_failure_backoff(Duration::from_secs(2))
        .with_max_failure_backoff(Duration::from_secs(60));

    assert_eq!(config.batch_size(), 50);
    assert_eq!(config.fallback_interval(), Duration::from_secs(60));
    assert_eq!(config.handler_timeout(), Duration::from_secs(5));
    assert_eq!(config.failure_backoff(), Duration::from_secs(2));
    assert_eq!(config.max_failure_backoff(), Duration::from_secs(60));
}

#[test]
fn outbox_event_debug() {
    let event = OutboxEvent {
        id: uuid::Uuid::nil(),
        event_type: "test.event".to_string(),
        payload: serde_json::json!({"key": "value"}),
        occurred_at: Utc::now(),
    };
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("OutboxEvent"));
    assert!(debug_str.contains("test.event"));
}

#[test]
fn domain_event_notify_sql_present() {
    assert!(!DOMAIN_EVENT_NOTIFY_SQL.is_empty());
    assert!(DOMAIN_EVENT_NOTIFY_SQL.contains("notify_domain_event_inserted"));
}

#[tokio::test(start_paused = true)]
async fn poison_pill_event_does_not_starve_newer_events_and_recovers_after_backoff() {
    let outbox = MemOutbox::default();
    let poison_id = outbox.push("poison.event");
    let healthy_a = outbox.push("healthy.a");
    let healthy_b = outbox.push("healthy.b");

    let processor = OutboxProcessor::new(
        OutboxConfig::default()
            .with_failure_backoff(Duration::from_secs(10))
            .with_max_failure_backoff(Duration::from_secs(60)),
    );

    let fail_poison = Arc::new(AtomicBool::new(true));
    let poison_attempts = Arc::new(AtomicUsize::new(0));

    let handler = {
        let fail_poison = Arc::clone(&fail_poison);
        let poison_attempts = Arc::clone(&poison_attempts);
        move |event: OutboxEvent| {
            let fail_poison = Arc::clone(&fail_poison);
            let poison_attempts = Arc::clone(&poison_attempts);
            async move {
                if event.event_type == "poison.event" {
                    poison_attempts.fetch_add(1, Ordering::SeqCst);
                    if fail_poison.load(Ordering::SeqCst) {
                        return Err("poison failure".into());
                    }
                }
                Ok(())
            }
        }
    };

    // First batch: poison event fails once, but both newer healthy events still succeed.
    let mut drain_cursor = None;
    let report = processor
        .process_batch_with(
            &outbox,
            &handler,
            pin!(future::pending::<()>()),
            &mut drain_cursor,
        )
        .await
        .unwrap();
    assert_eq!(report.processed, 2);
    assert_eq!(poison_attempts.load(Ordering::SeqCst), 1);
    assert_eq!(outbox.processed_ids(), vec![healthy_a, healthy_b]);

    // While poison event is still in backoff, a new healthy event is processed
    // without re-running the failing poison event.
    let healthy_c = outbox.push("healthy.c");
    drain_cursor = None;
    let report = processor
        .process_batch_with(
            &outbox,
            &handler,
            pin!(future::pending::<()>()),
            &mut drain_cursor,
        )
        .await
        .unwrap();
    assert_eq!(report.processed, 1);
    assert_eq!(poison_attempts.load(Ordering::SeqCst), 1);
    assert_eq!(
        outbox.processed_ids(),
        vec![healthy_a, healthy_b, healthy_c]
    );

    // After backoff expires, the poison event is retried and succeeds once fixed.
    tokio::time::advance(Duration::from_secs(10)).await;
    fail_poison.store(false, Ordering::SeqCst);
    drain_cursor = None;
    let report = processor
        .process_batch_with(
            &outbox,
            &handler,
            pin!(future::pending::<()>()),
            &mut drain_cursor,
        )
        .await
        .unwrap();
    assert_eq!(report.processed, 1);
    assert_eq!(poison_attempts.load(Ordering::SeqCst), 2);
    assert_eq!(
        outbox.processed_ids(),
        vec![poison_id, healthy_a, healthy_b, healthy_c]
    );
}

#[tokio::test(start_paused = true)]
async fn drain_cursor_advances_past_more_failures_than_tracker_cap() {
    let outbox = MemOutbox::default();
    let failing_count = MAX_TRACKED_FAILURES + 5;
    for _ in 0..failing_count {
        outbox.push("poison.event");
    }
    let healthy_id = outbox.push("healthy.event");

    let processor = OutboxProcessor::new(
        OutboxConfig::default()
            .with_batch_size(1)
            .with_failure_backoff(Duration::ZERO)
            .with_max_failure_backoff(Duration::ZERO)
            .with_fallback_interval(Duration::from_secs(30)),
    );

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let shutdown_tx = Arc::new(Mutex::new(Some(shutdown_tx)));
    let poison_attempts = Arc::new(AtomicUsize::new(0));

    tokio::time::timeout(
        Duration::from_secs(5),
        processor.run_polling_backend_until_shutdown(
            &outbox,
            {
                let poison_attempts = Arc::clone(&poison_attempts);
                let shutdown_tx = Arc::clone(&shutdown_tx);
                move |event: OutboxEvent| {
                    let poison_attempts = Arc::clone(&poison_attempts);
                    let shutdown_tx = Arc::clone(&shutdown_tx);
                    async move {
                        if event.event_type == "poison.event" {
                            poison_attempts.fetch_add(1, Ordering::SeqCst);
                            return Err("poison failure".into());
                        }
                        if let Some(tx) = shutdown_tx.lock().unwrap().take() {
                            let _ = tx.send(());
                        }
                        Ok(())
                    }
                }
            },
            async move {
                let _ = shutdown_rx.await;
            },
        ),
    )
    .await
    .expect("healthy event beyond MAX_TRACKED_FAILURES must still be reached")
    .unwrap();

    assert_eq!(poison_attempts.load(Ordering::SeqCst), failing_count);
    assert_eq!(outbox.processed_ids(), vec![healthy_id]);
}

#[tokio::test(start_paused = true)]
async fn single_event_batch_keeps_draining_healthy_events_past_failed_head() {
    for failure_backoff in [Duration::from_millis(10), Duration::ZERO] {
        let outbox = MemOutbox::default();
        let _poison_id = outbox.push("poison.event");
        let healthy_id = outbox.push("healthy.event");

        // Mirrors the Postgres reproductions: batch_size = 1 with both short (10ms)
        // and zero failure backoff under a longer fallback_interval (75ms).
        let processor = OutboxProcessor::new(
            OutboxConfig::default()
                .with_batch_size(1)
                .with_failure_backoff(failure_backoff)
                .with_max_failure_backoff(failure_backoff)
                .with_fallback_interval(Duration::from_millis(75)),
        );

        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
        let shutdown_tx = Arc::new(Mutex::new(Some(shutdown_tx)));
        let poison_attempts = Arc::new(AtomicUsize::new(0));

        // Independent deadline so starvation fails promptly instead of hanging.
        tokio::time::timeout(
            Duration::from_millis(500),
            processor.run_polling_backend_until_shutdown(
                &outbox,
                {
                    let poison_attempts = Arc::clone(&poison_attempts);
                    let shutdown_tx = Arc::clone(&shutdown_tx);
                    move |event: OutboxEvent| {
                        let poison_attempts = Arc::clone(&poison_attempts);
                        let shutdown_tx = Arc::clone(&shutdown_tx);
                        async move {
                            if event.event_type == "poison.event" {
                                poison_attempts.fetch_add(1, Ordering::SeqCst);
                                return Err("poison failure".into());
                            }
                            if let Some(tx) = shutdown_tx.lock().unwrap().take() {
                                let _ = tx.send(());
                            }
                            Ok(())
                        }
                    }
                },
                async move {
                    let _ = shutdown_rx.await;
                },
            ),
        )
        .await
        .expect("healthy event must be reached before independent deadline")
        .unwrap();

        assert_eq!(
            poison_attempts.load(Ordering::SeqCst),
            1,
            "poison event should be attempted once before healthy event (backoff={failure_backoff:?})"
        );
        assert_eq!(outbox.processed_ids(), vec![healthy_id]);
    }
}

#[tokio::test(start_paused = true)]
async fn active_handler_shutdown_settles_current_event_and_claims_no_more() {
    for use_notifier in [false, true] {
        let outbox = MemOutbox::default();
        let first_id = outbox.push("first.event");
        let _second_id = outbox.push("second.event");

        let processor = OutboxProcessor::new(
            OutboxConfig::default()
                .with_batch_size(10)
                .with_fallback_interval(Duration::from_secs(30)),
        );

        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
        let shutdown_tx = Arc::new(Mutex::new(Some(shutdown_tx)));
        let handled = Arc::new(AtomicUsize::new(0));

        let handler = {
            let shutdown_tx = Arc::clone(&shutdown_tx);
            let handled = Arc::clone(&handled);
            move |_event: OutboxEvent| {
                let shutdown_tx = Arc::clone(&shutdown_tx);
                let handled = Arc::clone(&handled);
                async move {
                    handled.fetch_add(1, Ordering::SeqCst);
                    if let Some(tx) = shutdown_tx.lock().unwrap().take() {
                        let _ = tx.send(());
                    }
                    tokio::time::sleep(Duration::from_millis(20)).await;
                    Ok(())
                }
            }
        };

        let shutdown = async move {
            let _ = shutdown_rx.await;
        };

        if use_notifier {
            let mut notifier = IdleNotifier;
            processor
                .run_with_notifier_backend_until_shutdown(&outbox, &mut notifier, handler, shutdown)
                .await
                .unwrap();
        } else {
            processor
                .run_polling_backend_until_shutdown(&outbox, handler, shutdown)
                .await
                .unwrap();
        }

        assert_eq!(
            handled.load(Ordering::SeqCst),
            1,
            "must not claim a second event after shutdown is signaled (use_notifier={use_notifier})"
        );
        assert_eq!(outbox.processed_ids(), vec![first_id]);
    }
}

#[tokio::test(start_paused = true)]
async fn polling_until_shutdown_interrupts_idle_sleep() {
    let outbox = MemOutbox::default();
    let processor = OutboxProcessor::new(
        OutboxConfig::default().with_fallback_interval(Duration::from_secs(30)),
    );
    let start = Instant::now();

    processor
        .run_polling_backend_until_shutdown(
            &outbox,
            |_| async { Ok(()) },
            tokio::time::sleep(Duration::from_secs(1)),
        )
        .await
        .unwrap();

    assert_eq!(start.elapsed(), Duration::from_secs(1));
}

#[tokio::test(start_paused = true)]
async fn notifier_until_shutdown_interrupts_idle_wait() {
    let outbox = MemOutbox::default();
    let mut notifier = IdleNotifier;
    let processor = OutboxProcessor::new(
        OutboxConfig::default().with_fallback_interval(Duration::from_secs(30)),
    );
    let start = Instant::now();

    processor
        .run_with_notifier_backend_until_shutdown(
            &outbox,
            &mut notifier,
            |_| async { Ok(()) },
            tokio::time::sleep(Duration::from_secs(1)),
        )
        .await
        .unwrap();

    assert_eq!(start.elapsed(), Duration::from_secs(1));
}

#[tokio::test(start_paused = true)]
async fn panicking_outbox_handler_is_isolated_backed_off_and_does_not_kill_processor() {
    let outbox = MemOutbox::default();
    let sync_panic_id = outbox.push("panic.sync");
    let async_panic_id = outbox.push("panic.async");
    let healthy_id = outbox.push("healthy.event");

    let processor = OutboxProcessor::new(
        OutboxConfig::default()
            .with_failure_backoff(Duration::from_secs(10))
            .with_max_failure_backoff(Duration::from_secs(60)),
    );

    let should_panic = Arc::new(AtomicBool::new(true));
    let panic_attempts = Arc::new(AtomicUsize::new(0));

    let handler = {
        let should_panic = Arc::clone(&should_panic);
        let panic_attempts = Arc::clone(&panic_attempts);
        move |event: OutboxEvent| {
            let should_panic = Arc::clone(&should_panic);
            let panic_attempts = Arc::clone(&panic_attempts);
            if event.event_type == "panic.sync" && should_panic.load(Ordering::SeqCst) {
                panic_attempts.fetch_add(1, Ordering::SeqCst);
                panic!("sync outbox panic");
            }
            async move {
                if event.event_type == "panic.async" && should_panic.load(Ordering::SeqCst) {
                    panic_attempts.fetch_add(1, Ordering::SeqCst);
                    tokio::task::yield_now().await;
                    panic!("async outbox panic");
                }
                Ok(())
            }
        }
    };

    let mut drain_cursor = None;
    let report = processor
        .process_batch_with(
            &outbox,
            &handler,
            pin!(future::pending::<()>()),
            &mut drain_cursor,
        )
        .await
        .expect("panicking handler must not crash outbox processor");

    assert_eq!(report.processed, 1);
    assert_eq!(panic_attempts.load(Ordering::SeqCst), 2);
    assert_eq!(outbox.processed_ids(), vec![healthy_id]);

    // While backed off, neither panicking event is re-run.
    drain_cursor = None;
    let report = processor
        .process_batch_with(
            &outbox,
            &handler,
            pin!(future::pending::<()>()),
            &mut drain_cursor,
        )
        .await
        .unwrap();
    assert_eq!(report.processed, 0);
    assert_eq!(panic_attempts.load(Ordering::SeqCst), 2);

    // After backoff expires and handler stops panicking, both events succeed.
    tokio::time::advance(Duration::from_secs(10)).await;
    should_panic.store(false, Ordering::SeqCst);
    drain_cursor = None;
    let report = processor
        .process_batch_with(
            &outbox,
            &handler,
            pin!(future::pending::<()>()),
            &mut drain_cursor,
        )
        .await
        .unwrap();
    assert_eq!(report.processed, 2);
    assert_eq!(
        outbox.processed_ids(),
        vec![sync_panic_id, async_panic_id, healthy_id]
    );
}
