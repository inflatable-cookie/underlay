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
