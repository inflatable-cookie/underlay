use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FailureClass {
    RetrySafe,
    NonRetrySafe,
}

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error("stage failed [{failure_class:?}]: {stage}: {message} (hint: {recovery_hint})")]
    StageFailure {
        stage: String,
        message: String,
        failure_class: FailureClass,
        recovery_hint: String,
    },

    #[error("determinism violation: {0}")]
    DeterminismViolation(String),

    #[error("resume compatibility error ({code}): {message}")]
    ResumeCompatibility { code: String, message: String },

    #[error("run store error: {0}")]
    RunStore(String),

    #[error("plugin error: {0}")]
    Plugin(String),

    #[error("serialization error: {0}")]
    Serialization(String),
}

pub type MigrationResult<T> = Result<T, MigrationError>;
