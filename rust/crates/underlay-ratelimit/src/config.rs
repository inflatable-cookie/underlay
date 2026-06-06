//! Configuration and result types for rate limiting.

use std::time::Duration;

/// Configuration for a rate limit.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum number of requests allowed in the window.
    max_requests: u64,
    /// Time window for the rate limit.
    window: Duration,
}

impl RateLimitConfig {
    /// Create a new rate limit configuration.
    pub fn new(max_requests: u64, window: Duration) -> Self {
        Self {
            max_requests,
            window,
        }
    }

    /// Create a rate limit of N requests per second.
    pub fn per_second(max_requests: u64) -> Self {
        Self::new(max_requests, Duration::from_secs(1))
    }

    /// Create a rate limit of N requests per minute.
    pub fn per_minute(max_requests: u64) -> Self {
        Self::new(max_requests, Duration::from_secs(60))
    }

    /// Create a rate limit of N requests per hour.
    pub fn per_hour(max_requests: u64) -> Self {
        Self::new(max_requests, Duration::from_secs(3600))
    }

    /// Get window duration in seconds.
    pub fn window_seconds(&self) -> u64 {
        self.window.as_secs()
    }

    /// Maximum number of requests allowed in the window.
    pub fn max_requests(&self) -> u64 {
        self.max_requests
    }

    /// Time window for the rate limit.
    pub fn window(&self) -> Duration {
        self.window
    }
}

/// Result of a rate limit check.
#[derive(Debug, Clone)]
pub struct RateLimitResult {
    /// Whether the request is allowed.
    pub allowed: bool,
    /// Number of requests remaining in the current window.
    pub remaining: u64,
    /// Current count of requests in the window.
    pub count: u64,
    /// Time until the rate limit resets (if limited).
    pub reset_after: Option<Duration>,
}

impl RateLimitResult {
    /// Create an allowed result.
    pub fn allowed(remaining: u64, count: u64) -> Self {
        Self {
            allowed: true,
            remaining,
            count,
            reset_after: None,
        }
    }

    /// Create a denied (rate limited) result.
    pub fn denied(count: u64, reset_after: Duration) -> Self {
        Self {
            allowed: false,
            remaining: 0,
            count,
            reset_after: Some(reset_after),
        }
    }

    /// Check if the request is allowed.
    pub fn is_allowed(&self) -> bool {
        self.allowed
    }

    /// Check if the request is denied (rate limited).
    pub fn is_denied(&self) -> bool {
        !self.allowed
    }

    /// Get seconds until retry is allowed (0 if allowed).
    pub fn retry_after_secs(&self) -> u64 {
        self.reset_after.map(|d| d.as_secs()).unwrap_or(0)
    }
}
