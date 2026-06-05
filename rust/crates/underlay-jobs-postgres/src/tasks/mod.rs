//! Standard maintenance task handlers for Underlay applications.
//!
//! These job handlers clean up expired data from standard Underlay tables:
//! - `auth.*` tables (sessions, auth state, login attempts, TOTP codes)
//! - `platform.*` tables (jobs, error logs, captured emails)
//!
//! # Usage
//!
//! Register the handlers you need in your job registry:
//!
//! ```ignore
//! use underlay_jobs::JobRegistry;
//! use underlay_jobs_postgres::tasks;
//!
//! let mut registry = JobRegistry::new();
//!
//! // Auth cleanup
//! registry.register(tasks::PurgeExpiredSessionsJob::new(pool.clone()));
//! registry.register(tasks::PurgeAuthStatesJob::new(pool.clone()));
//!
//! // Job maintenance
//! registry.register(tasks::RecoverAbandonedJobsJob::new(pool.clone()));
//! ```
//!
//! Then define scheduled tasks to run them:
//!
//! ```ignore
//! ScheduledTaskDefinition {
//!     name: "purge_expired_sessions",
//!     job_type: "purge_expired_sessions",
//!     schedule: "0 */15 * * * *", // Every 15 minutes
//!     payload: json!({}),
//!     config: JobConfig::maintenance(),
//! }
//! ```

mod auth_cleanup;
mod job_maintenance;
mod log_cleanup;

pub use auth_cleanup::{
    PurgeAuthStatesJob, PurgeEmailTotpCodesJob, PurgeExpiredSessionsJob, PurgeLoginAttemptsJob,
    PurgeRateLimitEntriesJob, PurgeVerificationSessionsJob, SuspendInactiveAccountsJob,
};
pub use job_maintenance::{ArchiveCompletedJobsJob, PurgeJobHistoryJob, RecoverAbandonedJobsJob};
pub use log_cleanup::{PurgeCapturedEmailsJob, PurgeErrorLogsJob};
