//! Background job system for Acme.
//!
//! This crate wraps `underlay-jobs` and provides Acme-specific job handlers.
//!
//! The initial implementation intentionally keeps the registry empty; it exists
//! to provide the standard entrypoint and wiring so jobs can be added safely.

// Re-export everything from underlay-jobs.
pub use underlay_jobs::{
    BackoffStrategy, Job, JobConfig, JobErrorRecord, JobFilters, JobHandler, JobHandlerError,
    JobId, JobProgress, JobRegistry, JobRepository, JobResult, JobRunner, JobRunnerConfig,
    JobStatus, JobStore, PgJobNotifier, RepoError, ScheduledTask, ScheduledTaskDefinition,
    ScheduledTaskRepository, Scheduler, JOB_NOTIFY_CHANNEL, JOB_NOTIFY_SQL, JOB_TABLES_SQL,
};
