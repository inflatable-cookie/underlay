//! Backend trait for rate limiting storage.

use async_trait::async_trait;

use crate::config::{RateLimitConfig, RateLimitResult};
use crate::error::Result;

/// Storage contract for rate limit counters.
///
/// The crate ships an in-memory backend. Consumers can implement this trait for
/// Redis, PostgreSQL, or another shared counter store.
#[async_trait]
pub trait RateLimitBackend: Send + Sync {
    /// Check if a request is allowed under the rate limit.
    ///
    /// This should NOT increment the counter - use `check_and_increment` for that.
    async fn check(&self, key: &str, config: &RateLimitConfig) -> Result<RateLimitResult>;

    /// Increment the counter for a key and return the new count.
    ///
    /// The counter should automatically expire after the window duration.
    async fn increment(&self, key: &str, config: &RateLimitConfig) -> Result<u64>;

    /// Reset the counter for a key.
    ///
    /// Use this after successful authentication to clear failed attempt counters.
    async fn reset(&self, key: &str) -> Result<()>;

    /// Check if allowed and increment in one operation.
    ///
    /// Backends should override this when they can make the operation atomic.
    async fn check_and_increment(
        &self,
        key: &str,
        config: &RateLimitConfig,
    ) -> Result<RateLimitResult> {
        let result = self.check(key, config).await?;
        if result.is_allowed() {
            let new_count = self.increment(key, config).await?;
            if new_count > config.max_requests() {
                return Ok(RateLimitResult::denied(new_count, config.window()));
            }
            let remaining = config.max_requests() - new_count;
            Ok(RateLimitResult::allowed(remaining, new_count))
        } else {
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::Duration;

    use super::*;

    struct RacingBackend {
        next_count: AtomicU64,
    }

    #[async_trait]
    impl RateLimitBackend for RacingBackend {
        async fn check(&self, _key: &str, config: &RateLimitConfig) -> Result<RateLimitResult> {
            Ok(RateLimitResult::allowed(config.max_requests(), 0))
        }

        async fn increment(&self, _key: &str, _config: &RateLimitConfig) -> Result<u64> {
            Ok(self.next_count.fetch_add(1, Ordering::SeqCst) + 1)
        }

        async fn reset(&self, _key: &str) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn default_check_and_increment_denies_when_new_count_exceeds_max() {
        let backend = RacingBackend {
            next_count: AtomicU64::new(5),
        };
        let config = RateLimitConfig::new(5, Duration::from_secs(30));

        let result = backend
            .check_and_increment("k", &config)
            .await
            .expect("check_and_increment should succeed");
        assert!(result.is_denied());
        assert_eq!(result.count, 6);
        assert_eq!(result.remaining, 0);
        assert_eq!(result.retry_after_secs(), 30);
    }
}
