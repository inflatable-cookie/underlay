use super::*;

#[tokio::test]
async fn test_basic_rate_limiting() {
    let backend = InMemoryBackend::new();
    let config = RateLimitConfig::new(3, Duration::from_secs(60));

    // First 3 requests should be allowed
    for i in 0..3 {
        let result = backend.check_and_increment("test", &config).await.unwrap();
        assert!(result.is_allowed(), "Request {} should be allowed", i + 1);
        assert_eq!(result.remaining, 2 - i as u64);
    }

    // 4th request should be denied
    let result = backend.check_and_increment("test", &config).await.unwrap();
    assert!(result.is_denied());
    assert!(result.retry_after_secs() > 0);
}

#[tokio::test]
async fn test_window_reset() {
    let backend = InMemoryBackend::new();
    let config = RateLimitConfig::new(2, Duration::from_millis(50));

    // Use up the limit
    backend.check_and_increment("test", &config).await.unwrap();
    backend.check_and_increment("test", &config).await.unwrap();

    let result = backend.check_and_increment("test", &config).await.unwrap();
    assert!(result.is_denied());

    // Wait for window to expire
    tokio::time::sleep(Duration::from_millis(60)).await;

    // Should be allowed again
    let result = backend.check_and_increment("test", &config).await.unwrap();
    assert!(result.is_allowed());
}

#[tokio::test]
async fn test_reset() {
    let backend = InMemoryBackend::new();
    let config = RateLimitConfig::new(2, Duration::from_secs(60));

    // Use up the limit
    backend.check_and_increment("test", &config).await.unwrap();
    backend.check_and_increment("test", &config).await.unwrap();

    let result = backend.check_and_increment("test", &config).await.unwrap();
    assert!(result.is_denied());

    // Reset the counter
    backend.reset("test").await.unwrap();

    // Should be allowed again
    let result = backend.check_and_increment("test", &config).await.unwrap();
    assert!(result.is_allowed());
    assert_eq!(result.remaining, 1);
}

#[tokio::test]
async fn test_separate_keys() {
    let backend = InMemoryBackend::new();
    let config = RateLimitConfig::new(1, Duration::from_secs(60));

    let result = backend
        .check_and_increment("user:1", &config)
        .await
        .unwrap();
    assert!(result.is_allowed());

    let result = backend
        .check_and_increment("user:1", &config)
        .await
        .unwrap();
    assert!(result.is_denied());

    // Different key should still be allowed
    let result = backend
        .check_and_increment("user:2", &config)
        .await
        .unwrap();
    assert!(result.is_allowed());
}

#[tokio::test]
async fn test_window_duration_updates_when_config_changes() {
    let backend = InMemoryBackend::new();
    let long_window = RateLimitConfig::new(1, Duration::from_secs(60));
    let short_window = RateLimitConfig::new(1, Duration::from_millis(30));

    let res = backend
        .check_and_increment("key", &long_window)
        .await
        .unwrap();
    assert!(res.is_allowed());

    let denied = backend
        .check_and_increment("key", &long_window)
        .await
        .unwrap();
    assert!(denied.is_denied());

    tokio::time::sleep(Duration::from_millis(40)).await;

    // Switching to the shorter window should immediately expire the entry.
    let res = backend
        .check_and_increment("key", &short_window)
        .await
        .unwrap();
    assert!(res.is_allowed());
}

#[tokio::test]
async fn test_max_keys_caps_tracked_entries_under_churn() {
    let backend = InMemoryBackend::new().with_max_keys(10);
    let config = RateLimitConfig::new(5, Duration::from_secs(60));

    for i in 0..50 {
        let key = format!("churn:{i}");
        let res = backend.check_and_increment(&key, &config).await.unwrap();
        assert!(res.is_allowed());
        assert!(
            backend.len() <= 10,
            "tracked key count {} exceeded cap 10",
            backend.len()
        );
    }

    assert_eq!(backend.len(), 10);
}

#[tokio::test]
async fn test_zero_max_requests_denies_without_inserting_entry() {
    let backend = InMemoryBackend::new();
    let config = RateLimitConfig::new(0, Duration::from_millis(500));

    let result = backend
        .check_and_increment("blocked:key", &config)
        .await
        .expect("check_and_increment should succeed");
    assert!(result.is_denied());
    assert_eq!(result.count, 0);
    assert_eq!(result.retry_after_secs(), 1);
    assert!(
        backend.is_empty(),
        "max_requests == 0 must not insert a map entry"
    );
}

#[test]
fn test_cleanup_interval_clamps_zero_to_minimum() {
    assert_eq!(clamp_cleanup_interval(Duration::ZERO), MIN_CLEANUP_INTERVAL);
    assert_eq!(
        clamp_cleanup_interval(Duration::from_millis(1)),
        MIN_CLEANUP_INTERVAL
    );
    assert_eq!(
        clamp_cleanup_interval(Duration::from_secs(5)),
        Duration::from_secs(5)
    );
}

#[tokio::test]
async fn test_with_cleanup_zero_duration_does_not_busy_spin() {
    let backend = InMemoryBackend::with_cleanup(Duration::ZERO);
    let config = RateLimitConfig::new(5, Duration::from_secs(60));
    let res = backend
        .check_and_increment("cleanup:zero", &config)
        .await
        .expect("backend should remain responsive");
    assert!(res.is_allowed());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrent_churn_stays_bounded_under_max_keys() {
    let backend = InMemoryBackend::new().with_max_keys(16);
    let config = RateLimitConfig::new(5, Duration::from_secs(60));

    let mut tasks = Vec::new();
    for worker in 0..8 {
        let backend = backend.clone();
        let config = config.clone();
        tasks.push(tokio::spawn(async move {
            for i in 0..40 {
                let key = format!("w{worker}:k{i}");
                let _ = backend.check_and_increment(&key, &config).await;
                assert!(
                    backend.len() <= 16,
                    "tracked key count {} exceeded cap 16 under concurrency",
                    backend.len()
                );
            }
        }));
    }

    for task in tasks {
        task.await.expect("worker task should complete");
    }
    assert!(backend.len() <= 16);
}

#[tokio::test]
async fn test_admission_rejects_rather_than_overfilling_when_all_sampled_entries_refresh() {
    let backend = InMemoryBackend::new().with_max_keys(2);
    let short_window = RateLimitConfig::new(5, Duration::from_millis(20));

    backend
        .check_and_increment("k1", &short_window)
        .await
        .unwrap();
    backend
        .check_and_increment("k2", &short_window)
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_millis(30)).await;

    // Simulate concurrent refreshes occurring right after sampling on every pass:
    // `remove_if` must preserve every refreshed entry, and capacity admission
    // must reject rather than overfilling `max_keys`.
    let _admission = backend
        .admission_lock
        .lock()
        .expect("admission lock should not be poisoned");
    let result = backend.ensure_capacity_for_new_key("k3", || {
        for mut entry in backend.entries.iter_mut() {
            entry.window_start = Instant::now();
            entry.window_duration = Duration::from_secs(60);
            entry.count = 1;
        }
    });

    assert!(
        matches!(result, Err(RateLimitError::Backend(_))),
        "admission must fail when bounded eviction frees no slot, got: {result:?}"
    );
    assert_eq!(backend.len(), 2, "map must not overfill past max_keys");
    assert_eq!(
        backend.entries.get("k1").map(|e| e.count),
        Some(1),
        "refreshed k1 must be preserved"
    );
    assert_eq!(
        backend.entries.get("k2").map(|e| e.count),
        Some(1),
        "refreshed k2 must be preserved"
    );
}

#[tokio::test]
async fn test_eviction_preserves_refreshed_entry() {
    let backend = InMemoryBackend::new().with_max_keys(2);
    let short_window = RateLimitConfig::new(5, Duration::from_millis(30));

    backend
        .check_and_increment("k1", &short_window)
        .await
        .unwrap();
    backend
        .check_and_increment("k2", &short_window)
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_millis(40)).await;

    // Refresh k1 with a fresh window before admitting k3 (k2 remains expired).
    let refreshed = backend
        .check_and_increment("k1", &short_window)
        .await
        .unwrap();
    assert!(refreshed.is_allowed());
    assert_eq!(refreshed.count, 1);

    // Admitting k3 should evict expired k2 while keeping refreshed k1 intact.
    backend
        .check_and_increment("k3", &short_window)
        .await
        .unwrap();
    assert_eq!(backend.len(), 2);
    let check_k1 = backend.check("k1", &short_window).await.unwrap();
    assert_eq!(check_k1.count, 1, "refreshed counter must not be evicted");
}
