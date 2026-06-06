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
            let remaining = config.max_requests.saturating_sub(new_count);
            Ok(RateLimitResult::allowed(remaining, new_count))
        } else {
            Ok(result)
        }
    }
}
