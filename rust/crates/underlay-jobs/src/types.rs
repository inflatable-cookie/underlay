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

#[cfg(test)]
mod tests {
    use super::{Job, JobHandlerError};
    use serde_json::json;

    #[test]
    fn job_type_has_required_fields() {
        let job = Job {
            id: underlay_core::Uuid::new_v7(),
            job_type: "test_job".to_string(),
            payload: json!({"key": "value"}),
            attempts: 1,
        };
        assert!(!job.id.to_string().is_empty());
        assert_eq!(job.job_type, "test_job");
        assert_eq!(job.attempts, 1);
    }

    #[test]
    fn job_payload_is_valid_json() {
        let payload = json!({"nested": {"key": 123}});
        let job = Job {
            id: underlay_core::Uuid::new_v7(),
            job_type: "test".to_string(),
            payload: payload.clone(),
            attempts: 0,
        };
        assert_eq!(job.payload, payload);
    }

    #[test]
    fn job_handler_error_new() {
        let err = JobHandlerError::new("test message");
        assert_eq!(err.message, "test message");
    }

    #[test]
    fn job_handler_error_display() {
        let err = JobHandlerError::new("error message");
        assert_eq!(err.to_string(), "error message");
    }

    #[test]
    fn job_handler_error_is_error_trait() {
        fn assert_error<T: std::error::Error>() {}
        assert_error::<JobHandlerError>();
    }

    #[test]
    fn job_handler_error_implements_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<JobHandlerError>();
        assert_sync::<JobHandlerError>();
    }
}
