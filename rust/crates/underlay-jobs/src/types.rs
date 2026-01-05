use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use underlay_core::Uuid;

pub type JobId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: JobId,
    pub job_type: String,
    pub payload: Value,
    pub attempts: u32,
}

#[derive(Debug, Clone)]
pub struct JobHandlerError {
    pub message: String,
}

impl JobHandlerError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for JobHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for JobHandlerError {}

#[async_trait]
pub trait JobHandler: Send + Sync {
    fn job_type(&self) -> &'static str;
    async fn handle(&self, job: Job) -> Result<(), JobHandlerError>;
}
