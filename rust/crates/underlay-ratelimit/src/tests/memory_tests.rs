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