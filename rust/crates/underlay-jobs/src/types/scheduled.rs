use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::config::JobConfig;
use underlay_core::Uuid;

/// A scheduled task definition from the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduledTask {
    pub id: Uuid,
    pub name: String,
    pub job_type: String,
    pub schedule: String,
    pub payload: Value,
    pub max_attempts: i32,
    pub timeout_seconds: Option<i32>,
    pub allow_overlap: bool,
    pub priority: i32,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scheduled_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Definition for registering a scheduled task in code.
#[derive(Debug, Clone)]
pub struct ScheduledTaskDefinition {
    /// Unique task name (used for upsert)
    pub name: &'static str,
    /// Job type to create when scheduled
    pub job_type: &'static str,
    /// Cron schedule expression
    pub schedule: &'static str,
    /// Payload to pass to the job
    pub payload: Value,
    /// Job configuration
    pub config: JobConfig,
}
