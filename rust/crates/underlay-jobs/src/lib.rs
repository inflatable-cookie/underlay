//! Background job queue system.
//!
//! This crate provides a flexible job queue system that can be used with any storage backend.
//!
//! # Features
//!
//! - Job types, handler trait, store trait, registry, runner, scheduler config,
//!   dead-letter contracts, and job event hooks.
//! - PostgreSQL repositories, outbox processing, scheduled task runtime, and
//!   maintenance task helpers live in `underlay-jobs-postgres`.
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
//! # PostgreSQL Adapter
//!
//! Use `underlay-jobs-postgres` for the concrete PostgreSQL-backed implementation:
//!
//! ```ignore
//! use underlay_jobs_postgres::{JobRepository, ScheduledTaskRepository};
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
//! The PostgreSQL adapter crate provides migration SQL constants. Sync them to
//! your application's migrations folder using `underlay-devtools`.

mod dead_letters;
mod events;
mod registry;
mod runner;
mod scheduler;
mod store;
pub mod types;

pub use crate::scheduler::{SchedulerConfig, DEFAULT_SCHEDULER_TICK_INTERVAL_SECS};

// Re-exports from types
pub use crate::dead_letters::DeadLetterStore;
pub use crate::events::{JobEvent, JobEventHub, JobEventSink};
pub use crate::types::{
    BackoffJitter, BackoffStrategy, DeadLetter, DeadLetterFilters, DeadLetterId, Job, JobConfig,
    JobErrorRecord, JobFailureOutcome, JobFilters, JobHandler, JobHandlerError, JobId, JobProgress,
    JobResult, JobStatus, ScheduledTask, ScheduledTaskDefinition, DEFAULT_BACKOFF_BASE_SECS,
    DEFAULT_BACKOFF_MAX_SECS, DEFAULT_LONG_RUNNING_TIMEOUT_SECS,
};

// Core exports
pub use crate::registry::JobRegistry;
pub use crate::runner::{JobRunner, JobRunnerConfig};
pub use crate::store::JobStore;
