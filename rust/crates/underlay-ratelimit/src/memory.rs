//! In-memory rate limiting backend using DashMap.

use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use dashmap::DashMap;

use crate::backend::RateLimitBackend;
use crate::config::{RateLimitConfig, RateLimitResult};
use crate::error::Result;

/// Default maximum number of keys tracked in memory before expired or existing
/// entries are evicted to bound memory growth under key churn.
pub const DEFAULT_MAX_KEYS: usize = 100_000;

/// Minimum interval between background cleanup sweeps to prevent busy-spinning.
pub const MIN_CLEANUP_INTERVAL: Duration = Duration::from_millis(10);

/// Maximum number of entries inspected per eviction pass so hitting `max_keys`
/// stays O(1) instead of scanning the entire map on every insertion.
const EVICTION_SAMPLE_SIZE: usize = 32;

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

    /// Update the window duration if config changed, and reset if expired.
    fn maybe_reset(&mut self, window_duration: Duration) {
        self.window_duration = window_duration;
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
#[derive(Debug)]
struct CleanupGuard(Option<tokio::task::JoinHandle<()>>);

impl Drop for CleanupGuard {
    fn drop(&mut self) {
        if let Some(handle) = self.0.take() {
            handle.abort();
        }
    }
}

#[derive(Debug, Clone)]
pub struct InMemoryBackend {
    entries: Arc<DashMap<String, RateLimitEntry>>,
    max_keys: usize,
    _cleanup_guard: Arc<CleanupGuard>,
}

impl Default for InMemoryBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryBackend {
    /// Create a new in-memory backend with the default key cap ([`DEFAULT_MAX_KEYS`]).
    pub fn new() -> Self {
        Self {
            entries: Arc::new(DashMap::new()),
            max_keys: DEFAULT_MAX_KEYS,
            _cleanup_guard: Arc::new(CleanupGuard(None)),
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

    /// Configure the maximum number of tracked keys before eviction occurs.
    ///
    /// Clamped to at least 1 so the backend always tracks active keys.
    pub fn with_max_keys(mut self, max_keys: usize) -> Self {
        self.max_keys = max_keys.max(1);
        self
    }

    /// Maximum number of keys tracked by this backend.
    pub fn max_keys(&self) -> usize {
        self.max_keys
    }

    /// Create a new in-memory backend with periodic cleanup.
    ///
    /// Expired entries are cleaned up at the specified interval (clamped to at
    /// least [`MIN_CLEANUP_INTERVAL`] to prevent busy-spinning) to prevent
    /// unbounded memory growth. The background task is aborted automatically
    /// when the last clone of this backend is dropped.
    pub fn with_cleanup(cleanup_interval: Duration) -> Self {
        let interval = clamp_cleanup_interval(cleanup_interval);
        let entries: Arc<DashMap<String, RateLimitEntry>> = Arc::new(DashMap::new());
        let cleanup_entries = Arc::clone(&entries);

        let handle = tokio::spawn(async move {
            loop {
                tokio::time::sleep(interval).await;
                // Remove entries where the window has expired
                cleanup_entries
                    .retain(|_, entry| entry.window_start.elapsed() < entry.window_duration);
                tracing::trace!(
                    "Rate limit cleanup complete, {} entries remaining",
                    cleanup_entries.len()
                );
            }
        });

        Self {
            entries,
            max_keys: DEFAULT_MAX_KEYS,
            _cleanup_guard: Arc::new(CleanupGuard(Some(handle))),
        }
    }

    /// Evict expired (and, if still full, oldest-sampled) entries before
    /// inserting a new key so tracked keys remain bounded under key churn
    /// without performing an O(N) full-map scan on every insertion.
    fn evict_if_full(&self, incoming_key: &str) {
        if self.entries.len() < self.max_keys || self.entries.contains_key(incoming_key) {
            return;
        }

        while self.entries.len() >= self.max_keys {
            let mut expired = Vec::new();
            let mut oldest: Option<(String, Instant)> = None;

            for entry in self.entries.iter().take(EVICTION_SAMPLE_SIZE) {
                let value = entry.value();
                if value.window_start.elapsed() >= value.window_duration {
                    expired.push(entry.key().clone());
                } else if oldest
                    .as_ref()
                    .is_none_or(|(_, oldest_start)| value.window_start < *oldest_start)
                {
                    oldest = Some((entry.key().clone(), value.window_start));
                }
            }

            if !expired.is_empty() {
                for key in expired {
                    self.entries.remove(&key);
                }
                continue;
            }

            let Some((victim, _)) = oldest else {
                break;
            };
            self.entries.remove(&victim);
        }
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
        if config.max_requests() == 0 {
            return Ok(RateLimitResult::denied(0, config.window()));
        }

        let Some(entry) = self.entries.get(key) else {
            return Ok(RateLimitResult::allowed(config.max_requests(), 0));
        };

        let elapsed = entry.window_start.elapsed();
        let window = config.window();
        if elapsed >= window {
            return Ok(RateLimitResult::allowed(config.max_requests(), 0));
        }

        if entry.count >= config.max_requests() {
            Ok(RateLimitResult::denied(
                entry.count,
                window.saturating_sub(elapsed),
            ))
        } else {
            let remaining = config.max_requests() - entry.count;
            Ok(RateLimitResult::allowed(remaining, entry.count))
        }
    }

    async fn increment(&self, key: &str, config: &RateLimitConfig) -> Result<u64> {
        self.evict_if_full(key);

        let mut entry = self
            .entries
            .entry(key.to_string())
            .or_insert_with(|| RateLimitEntry::new(config.window()));

        entry.maybe_reset(config.window());
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
        if config.max_requests() == 0 {
            return Ok(RateLimitResult::denied(0, config.window()));
        }

        self.evict_if_full(key);

        let mut entry = self
            .entries
            .entry(key.to_string())
            .or_insert_with(|| RateLimitEntry::new(config.window()));

        entry.maybe_reset(config.window());

        if entry.count >= config.max_requests() {
            return Ok(RateLimitResult::denied(entry.count, entry.time_remaining()));
        }

        entry.count += 1;
        let remaining = config.max_requests().saturating_sub(entry.count);
        Ok(RateLimitResult::allowed(remaining, entry.count))
    }
}

fn clamp_cleanup_interval(interval: Duration) -> Duration {
    interval.max(MIN_CLEANUP_INTERVAL)
}

#[cfg(test)]
#[path = "tests/memory_tests.rs"]
mod tests;
