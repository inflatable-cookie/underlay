//! In-memory rate limiting backend using DashMap.

use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use dashmap::DashMap;
use tokio::sync::RwLock;

use crate::backend::RateLimitBackend;
use crate::config::{RateLimitConfig, RateLimitResult};
use crate::error::Result;

/// Entry in the rate limit store.
#[derive(Debug, Clone)]
struct RateLimitEntry {
    /// Number of requests in the current window.
    count: u64,
    /// When the current window started.
    window_start: Instant,
    /// Duration of the window.
    window_duration: Duration,
}

impl RateLimitEntry {
    fn new(window_duration: Duration) -> Self {
        Self {
            count: 0,
            window_start: Instant::now(),
            window_duration,
        }
    }

    /// Check if the window has expired and reset if needed.
    fn maybe_reset(&mut self) {
        if self.window_start.elapsed() >= self.window_duration {
            self.count = 0;
            self.window_start = Instant::now();
        }
    }

    /// Get time remaining in the current window.
    fn time_remaining(&self) -> Duration {
        let elapsed = self.window_start.elapsed();
        if elapsed >= self.window_duration {
            Duration::ZERO
        } else {
            self.window_duration - elapsed
        }
    }
}

/// In-memory rate limiting backend.
///
/// **Single-instance / non-production only.** Counters are process-local
/// (a `DashMap` keyed on `Instant`), so in a multi-replica deployment each
/// replica keeps its own counter - an attacker spreads attempts across
/// replicas and a restart wipes every counter. Production deployments with
/// more than one app instance must use [`crate::PostgresBackend`] (feature
/// `postgres`) or another shared-counter backend. Construct this only via
/// [`InMemoryBackend::single_instance`] in production code paths so the
/// choice is explicit.
///
/// # Features
///
/// - Thread-safe concurrent access
/// - Automatic window expiry
/// - Optional background cleanup of expired entries
///
/// # Example
///
/// ```rust
/// use underlay_ratelimit::{InMemoryBackend, RateLimitBackend, RateLimitConfig};
/// use std::time::Duration;
///
/// #[tokio::main]
/// async fn main() {
///     let backend = InMemoryBackend::new();
///     let config = RateLimitConfig::per_minute(10);
///
///     // Check and increment atomically
///     let result = backend.check_and_increment("user:123", &config).await.unwrap();
///     println!("Allowed: {}, Remaining: {}", result.is_allowed(), result.remaining);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct InMemoryBackend {
    entries: Arc<DashMap<String, RateLimitEntry>>,
    cleanup_handle: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl Default for InMemoryBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryBackend {
    /// Create a new in-memory backend.
    pub fn new() -> Self {
        Self {
            entries: Arc::new(DashMap::new()),
            cleanup_handle: Arc::new(RwLock::new(None)),
        }
    }

    /// Explicit single-instance constructor.
    ///
    /// Identical to [`InMemoryBackend::new`], but names the operational
    /// constraint at the call site: this backend is safe only when exactly
    /// one app instance enforces the limit. Use this in production wiring so
    /// the single-instance assumption is a deliberate, greppable decision
    /// rather than an accidental default.
    pub fn single_instance() -> Self {
        Self::new()
    }

    /// Create a new in-memory backend with periodic cleanup.
    ///
    /// Expired entries are cleaned up at the specified interval to prevent
    /// unbounded memory growth.
    pub fn with_cleanup(cleanup_interval: Duration) -> Self {
        let backend = Self::new();
        let entries = Arc::clone(&backend.entries);

        let handle = tokio::spawn(async move {
            loop {
                tokio::time::sleep(cleanup_interval).await;
                // Remove entries where the window has expired
                entries.retain(|_, entry| entry.window_start.elapsed() < entry.window_duration);
                tracing::trace!(
                    "Rate limit cleanup complete, {} entries remaining",
                    entries.len()
                );
            }
        });

        // Store handle but don't await it
        let cleanup_handle = Arc::clone(&backend.cleanup_handle);
        tokio::spawn(async move {
            *cleanup_handle.write().await = Some(handle);
        });

        backend
    }

    /// Get the current number of tracked keys.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the backend has no tracked keys.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clear all rate limit entries.
    pub fn clear(&self) {
        self.entries.clear();
    }
}

#[async_trait]
impl RateLimitBackend for InMemoryBackend {
    async fn check(&self, key: &str, config: &RateLimitConfig) -> Result<RateLimitResult> {
        let mut entry = self
            .entries
            .entry(key.to_string())
            .or_insert_with(|| RateLimitEntry::new(config.window()));

        entry.maybe_reset();

        if entry.count >= config.max_requests() {
            Ok(RateLimitResult::denied(entry.count, entry.time_remaining()))
        } else {
            let remaining = config.max_requests() - entry.count;
            Ok(RateLimitResult::allowed(remaining, entry.count))
        }
    }

    async fn increment(&self, key: &str, config: &RateLimitConfig) -> Result<u64> {
        let mut entry = self
            .entries
            .entry(key.to_string())
            .or_insert_with(|| RateLimitEntry::new(config.window()));

        entry.maybe_reset();
        entry.count += 1;

        Ok(entry.count)
    }

    async fn reset(&self, key: &str) -> Result<()> {
        self.entries.remove(key);
        Ok(())
    }

    async fn check_and_increment(
        &self,
        key: &str,
        config: &RateLimitConfig,
    ) -> Result<RateLimitResult> {
        let mut entry = self
            .entries
            .entry(key.to_string())
            .or_insert_with(|| RateLimitEntry::new(config.window()));

        entry.maybe_reset();

        if entry.count >= config.max_requests() {
            return Ok(RateLimitResult::denied(entry.count, entry.time_remaining()));
        }

        entry.count += 1;
        let remaining = config.max_requests().saturating_sub(entry.count);
        Ok(RateLimitResult::allowed(remaining, entry.count))
    }
}

#[cfg(test)]
#[path = "tests/memory_tests.rs"]
mod tests;
