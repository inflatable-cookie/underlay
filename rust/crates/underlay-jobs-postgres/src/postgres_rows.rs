//! Database row types for the PostgreSQL job store.

use chrono::{DateTime, Utc};
use serde_json::Value;

use underlay_jobs::{DeadLetter, Job, JobErrorRecord, JobStatus, ScheduledTask};

// Helper to convert raw uuid to underlay_core::Uuid
fn from_raw(id: uuid::Uuid) -> underlay_core::Uuid {
    underlay_core::Uuid(id)
}

#[derive(sqlx::FromRow)]
pub(crate) struct JobRow {
    pub id: uuid::Uuid,
    pub job_type: String,
    pub status: String,
    pub payload: Value,
    pub attempts: i32,
    pub max_attempts: i32,
    pub scheduled_for: Option<DateTime<Utc>>,
    pub priority: i32,
    pub claimed_at: Option<DateTime<Utc>>,
    pub claimed_by: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub heartbeat_at: Option<DateTime<Utc>>,
    pub progress: Option<Value>,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<JobRow> for Job {
    fn from(row: JobRow) -> Self {
        let status = match row.status.as_str() {
            "pending" => JobStatus::Pending,
            "claimed" => JobStatus::Claimed,
            "running" => JobStatus::Running,
            "succeeded" => JobStatus::Succeeded,
            "failed" => JobStatus::Failed,
            "cancelled" => JobStatus::Cancelled,
            _ => JobStatus::Pending,
        };

        let progress = row.progress.and_then(|v| serde_json::from_value(v).ok());

        Job {
            id: from_raw(row.id),
            job_type: row.job_type,
            status,
            payload: row.payload,
            attempts: row.attempts,
            max_attempts: row.max_attempts,
            scheduled_for: row.scheduled_for,
            priority: row.priority,
            claimed_at: row.claimed_at,
            claimed_by: row.claimed_by,
            started_at: row.started_at,
            finished_at: row.finished_at,
            heartbeat_at: row.heartbeat_at,
            progress,
            last_error: row.last_error,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
pub(crate) struct ScheduledTaskRow {
    pub id: uuid::Uuid,
    pub name: String,
    pub job_type: String,
    pub schedule: String,
    pub payload: Value,
    pub max_attempts: i32,
    pub timeout_seconds: Option<i32>,
    pub allow_overlap: bool,
    pub priority: i32,
    pub enabled: bool,
    pub last_scheduled_at: Option<DateTime<Utc>>,
    pub last_completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ScheduledTaskRow> for ScheduledTask {
    fn from(row: ScheduledTaskRow) -> Self {
        ScheduledTask {
            id: from_raw(row.id),
            name: row.name,
            job_type: row.job_type,
            schedule: row.schedule,
            payload: row.payload,
            max_attempts: row.max_attempts,
            timeout_seconds: row.timeout_seconds,
            allow_overlap: row.allow_overlap,
            priority: row.priority,
            enabled: row.enabled,
            last_scheduled_at: row.last_scheduled_at,
            last_completed_at: row.last_completed_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
pub(crate) struct DeadLetterRow {
    pub id: uuid::Uuid,
    pub original_job_id: uuid::Uuid,
    pub job_type: String,
    pub payload: Value,
    pub attempts: i32,
    pub max_attempts: i32,
    pub priority: i32,
    pub last_error: String,
    pub error_history: Value,
    pub failed_at: DateTime<Utc>,
    pub retried_at: Option<DateTime<Utc>>,
    pub retried_job_id: Option<uuid::Uuid>,
    pub archived_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<DeadLetterRow> for DeadLetter {
    fn from(row: DeadLetterRow) -> Self {
        let error_history =
            serde_json::from_value::<Vec<JobErrorRecord>>(row.error_history).unwrap_or_default();

        DeadLetter {
            id: from_raw(row.id),
            original_job_id: from_raw(row.original_job_id),
            job_type: row.job_type,
            payload: row.payload,
            attempts: row.attempts,
            max_attempts: row.max_attempts,
            priority: row.priority,
            last_error: row.last_error,
            error_history,
            failed_at: row.failed_at,
            retried_at: row.retried_at,
            retried_job_id: row.retried_job_id.map(from_raw),
            archived_at: row.archived_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
