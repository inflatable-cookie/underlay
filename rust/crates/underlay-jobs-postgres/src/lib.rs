//! PostgreSQL adapter for Underlay jobs.
//!
//! This crate owns the SQLx-backed repositories, LISTEN/NOTIFY support,
//! outbox processing, scheduled task runtime, maintenance task handlers, and
//! Postgres migration SQL for the core `underlay-jobs` contract.

mod postgres;
mod postgres_dead_letters;
mod postgres_rows;
mod postgres_scheduled;
mod runner_ext;
mod scheduler;

pub mod outbox;
pub mod tasks;

pub use crate::outbox::DOMAIN_EVENT_NOTIFY_SQL;
pub use crate::postgres::{JobRepository, RepoError};
pub use crate::postgres_dead_letters::PgDeadLetterRepository;
pub use crate::postgres_scheduled::{PgJobNotifier, ScheduledTaskRepository, JOB_NOTIFY_CHANNEL};
pub use crate::runner_ext::PostgresJobRunnerExt;
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
pub const JOB_NOTIFY_SQL: &str = include_str!("../migrations/0002_add_job_notify.sql");

/// SQL for dead-letter persistence.
///
/// This migration adds `platform.job_dead_letter` for failed job inspection and retry.
pub const JOB_DEAD_LETTERS_SQL: &str = include_str!("../migrations/0004_add_job_dead_letters.sql");
