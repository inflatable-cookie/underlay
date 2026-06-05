use crate::errors::{FailureClass, MigrationError};

use super::types::StageName;

pub(super) fn stage_failure(stage: StageName, message: String) -> MigrationError {
    let (failure_class, recovery_hint) = classify_stage_failure(stage, &message);
    MigrationError::StageFailure {
        stage: stage.as_str().to_string(),
        message,
        failure_class,
        recovery_hint,
    }
}

fn classify_stage_failure(stage: StageName, message: &str) -> (FailureClass, String) {
    if message.contains("digest mismatch")
        || message.contains("integrity")
        || message.contains("governance")
    {
        return (
            FailureClass::NonRetrySafe,
            "inspect integrity/governance inputs, correct data, then rerun from a clean checkpoint"
                .to_string(),
        );
    }

    match stage {
        StageName::Extract | StageName::Normalize | StageName::Transform | StageName::Assets => (
            FailureClass::RetrySafe,
            "retry from the last successful checkpoint after validating source connectivity".to_string(),
        ),
        StageName::Decide => (
            FailureClass::RetrySafe,
            "retry after confirming decision resolver/AI dependencies; unresolved items can be resumed"
                .to_string(),
        ),
        StageName::Materialize => (
            FailureClass::NonRetrySafe,
            "validate partial writes and run targeted cleanup or restore before retrying materialization"
                .to_string(),
        ),
        StageName::Verify => (
            FailureClass::NonRetrySafe,
            "address verification failures before promotion; do not blindly retry".to_string(),
        ),
    }
}
