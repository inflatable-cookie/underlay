//! Rate limiting with pluggable backends.
//!
//! This crate provides a trait-based abstraction for rate limiting that allows
//! swapping backends (in-memory, Redis, database) without changing application code.
//!
//! # Example
//!
//! ```rust
//! use underlay_ratelimit::{RateLimitBackend, InMemoryBackend, RateLimitConfig};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() {
//!     let backend = InMemoryBackend::new();
//!     let config = RateLimitConfig::new(10, Duration::from_secs(60)); // 10 per minute
//!
//!     let result = backend.check("user:123:login", &config).await.unwrap();
//!     if result.is_allowed() {
//!         println!("Request allowed, {} remaining", result.remaining);
//!     } else {
//!         println!("Rate limited, retry after {} seconds", result.retry_after_secs());
//!     }
//! }
//! ```

mod backend;
mod config;
mod error;
mod memory;
#[cfg(feature = "postgres")]
mod postgres;

#[cfg(all(test, feature = "postgres"))]
#[path = "tests/postgres_integration.rs"]
mod postgres_integration;

pub use backend::RateLimitBackend;
pub use config::{RateLimitConfig, RateLimitResult};
pub use error::{RateLimitError, Result};
pub use memory::InMemoryBackend;
#[cfg(feature = "postgres")]
pub use postgres::PostgresBackend;
