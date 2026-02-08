//! Background job queue system with optional PostgreSQL persistence and cron scheduling.
//!
//! This crate provides a flexible job queue system that can be used with any storage backend.
//!
//! # Features
//!
//! - **Core** (always available): Job types, handler trait, registry, and runner
//! - **`postgres`**: PostgreSQL-backed job repository with `FOR UPDATE SKIP LOCKED` claiming
//! - **`scheduler`**: Cron-based task scheduling (requires `postgres`)
//! - **`full`**: All features enabled
//!
//! # Quick Start
//!
//! ```ignore
//! use underlay_jobs::{JobRunner, JobRegistry, JobHandler, Job, JobHandlerError};
//!
//! struct MyJob;
//!
//! #[async_trait]
//! impl JobHandler for MyJob {
//!     fn job_type(&self) -> &'static str { "my_job" }
//!
//!     async fn handle(&self, job: Job) -> Result<(), JobHandlerError> {
//!         // Process the job...
//!         Ok(())
//!     }
//! }
//!
//! // Register handlers
//! let mut registry = JobRegistry::new();
//! registry.register(MyJob);
//!
//! // Create runner with your store implementation
//! let runner = JobRunner::new(store, registry);
//! runner.run_forever().await?;
//! ```
//!
//! # PostgreSQL Integration
//!
//! Enable the `postgres` feature for a complete PostgreSQL-backed implementation:
//!
//! ```ignore
//! use underlay_jobs::postgres::{JobRepository, ScheduledTaskRepository};
//!
//! let job_repo = JobRepository::new(pool.clone());
//! let task_repo = ScheduledTaskRepository::new(pool);
//!
//! // Create a job
//! job_repo.create("my_job", json!({"key": "value"}), &JobConfig::default()).await?;
//! ```
//!
//! # Database Schema
//!
//! The crate provides a migration file in `migrations/0001_create_job_tables.sql`.
//! Sync this to your application's migrations folder using `underlay-devtools`.

mod registry;
mod runner;
mod store;
pub mod types;

// PostgreSQL implementation (optional)
#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "postgres")]
mod postgres_rows;
#[cfg(feature = "postgres")]
pub mod postgres_scheduled;

// Standard maintenance tasks (optional, requires postgres)
#[cfg(feature = "postgres")]
pub mod tasks;

// Outbox pattern for domain events (optional, requires postgres)
#[cfg(feature = "outbox")]
pub mod outbox;

// Scheduler (optional, requires postgres)
#[cfg(feature = "scheduler")]
mod scheduler;

// Scheduler config is always available (doesn't require postgres)
#[cfg(feature = "scheduler")]
pub use crate::scheduler::{SchedulerConfig, DEFAULT_SCHEDULER_TICK_INTERVAL_SECS};

// Re-exports from types
pub use crate::types::{
    BackoffStrategy, Job, JobConfig, JobErrorRecord, JobFilters, JobHandler, JobHandlerError,
    JobId, JobProgress, JobResult, JobStatus, ScheduledTask, ScheduledTaskDefinition,
    DEFAULT_BACKOFF_BASE_SECS, DEFAULT_BACKOFF_MAX_SECS, DEFAULT_LONG_RUNNING_TIMEOUT_SECS,
};

// Core exports
pub use crate::registry::JobRegistry;
pub use crate::runner::{JobRunner, JobRunnerConfig};
pub use crate::store::JobStore;

// PostgreSQL exports
#[cfg(feature = "postgres")]
pub use crate::postgres::{JobRepository, RepoError};
#[cfg(feature = "postgres")]
pub use crate::postgres_scheduled::{PgJobNotifier, ScheduledTaskRepository, JOB_NOTIFY_CHANNEL};

// Scheduler exports
#[cfg(all(feature = "scheduler", feature = "postgres"))]
pub use crate::scheduler::Scheduler;

/// SQL schema for job tables.
///
/// Applications should use `underlay-devtools sync-migrations` to copy this
/// to their migrations folder, or include it directly.
pub const JOB_TABLES_SQL: &str = include_str!("../migrations/0001_create_job_tables.sql");

/// SQL for LISTEN/NOTIFY trigger.
///
/// This migration adds efficient job notification support. See the
/// `PgJobNotifier` type for usage details.
#[cfg(feature = "postgres")]
pub const JOB_NOTIFY_SQL: &str = include_str!("../migrations/0002_add_job_notify.sql");

/// SQL for domain event LISTEN/NOTIFY trigger.
///
/// This migration adds efficient outbox notification support. See the
/// `outbox::OutboxNotifier` type for usage details.
#[cfg(feature = "outbox")]
pub const DOMAIN_EVENT_NOTIFY_SQL: &str =
    include_str!("../migrations/0003_add_domain_event_notify.sql");
