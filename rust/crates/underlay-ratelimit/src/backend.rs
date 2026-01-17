//! Backend trait for rate limiting storage.

use async_trait::async_trait;

use crate::config::{RateLimitConfig, RateLimitResult};
use crate::error::Result;

/// Trait for rate limiting backends.
///
/// Implementations provide the storage mechanism for rate limit counters.
/// The crate provides an in-memory implementation; other backends (Redis,
/// PostgreSQL) can be implemented by consumers.
///
/// # Example Implementation
///
/// ```rust,ignore
/// use underlay_ratelimit::{RateLimitBackend, RateLimitConfig, RateLimitResult, Result};
/// use async_trait::async_trait;
///
/// struct RedisBackend { /* ... */ }
///
/// #[async_trait]
/// impl RateLimitBackend for RedisBackend {
///     async fn check(&self, key: &str, config: &RateLimitConfig) -> Result<RateLimitResult> {
///         // Check Redis for current count
///         todo!()
///     }
///
///     async fn increment(&self, key: &str, config: &RateLimitConfig) -> Result<u64> {
///         // Increment counter in Redis with TTL
///         todo!()
///     }
///
///     async fn reset(&self, key: &str) -> Result<()> {
///         // Delete key from Redis
///         todo!()
///     }
/// }
/// ```
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
    /// This is the most common operation: check if under limit, and if so,
    /// increment the counter atomically.
    ///
    /// Default implementation calls check then increment, but backends can
    /// override for atomic behavior.
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
