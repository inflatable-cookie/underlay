use serde::{Deserialize, Serialize};

/// Job status enum matching the database constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    /// Job is waiting to be claimed by a worker
    Pending,
    /// Job has been claimed but not yet started
    Claimed,
    /// Job is currently executing
    Running,
    /// Job completed successfully
    Succeeded,
    /// Job failed (exhausted retries or permanent failure)
    Failed,
    /// Job was manually cancelled
    Cancelled,
}

impl JobStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            JobStatus::Pending => "pending",
            JobStatus::Claimed => "claimed",
            JobStatus::Running => "running",
            JobStatus::Succeeded => "succeeded",
            JobStatus::Failed => "failed",
            JobStatus::Cancelled => "cancelled",
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            JobStatus::Succeeded | JobStatus::Failed | JobStatus::Cancelled
        )
    }
}
