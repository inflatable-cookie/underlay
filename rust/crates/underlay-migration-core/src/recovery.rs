use serde::{Deserialize, Serialize};

use crate::errors::{FailureClass, MigrationError};
use crate::pipeline::PipelineRunReport;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecoveryAdvisory {
    pub code: String,
    pub failure_class: FailureClass,
    pub summary: String,
    pub action: String,
}

pub fn recovery_advisory_from_error(error: &MigrationError) -> Option<RecoveryAdvisory> {
    match error {
        MigrationError::StageFailure {
            stage,
            message,
            failure_class,
            recovery_hint,
        } => Some(RecoveryAdvisory {
            code: format!("stage_failure_{stage}"),
            failure_class: *failure_class,
            summary: message.clone(),
            action: recovery_hint.clone(),
        }),
        MigrationError::ResumeCompatibility { code, message } => Some(RecoveryAdvisory {
            code: format!("resume_compatibility_{code}"),
            failure_class: FailureClass::NonRetrySafe,
            summary: message.clone(),
            action:
                "align plugin/schema versions with the persisted checkpoint or clear resume state"
                    .to_string(),
        }),
        _ => None,
    }
}

pub fn recovery_advisories_from_run(report: &PipelineRunReport) -> Vec<RecoveryAdvisory> {
    let mut advisories = Vec::new();

    if report.decide.unresolved_count > 0 {
        advisories.push(RecoveryAdvisory {
            code: "unresolved_decisions_present".to_string(),
            failure_class: FailureClass::RetrySafe,
            summary: format!(
                "{} unresolved decision(s) remain",
                report.decide.unresolved_count
            ),
            action: "review unresolved queue, apply human overrides, then rerun from checkpoint"
                .to_string(),
        });
    }

    if !report.decide.governance_issues.is_empty() {
        advisories.push(RecoveryAdvisory {
            code: "governance_issues_present".to_string(),
            failure_class: FailureClass::NonRetrySafe,
            summary: format!(
                "{} governance issue(s) detected",
                report.decide.governance_issues.len()
            ),
            action: "fix contract/integrity issues in decision artifacts before rerun".to_string(),
        });
    }

    if !report.verify.passed {
        advisories.push(RecoveryAdvisory {
            code: "verify_stage_failed".to_string(),
            failure_class: FailureClass::NonRetrySafe,
            summary: "verify stage failed".to_string(),
            action: "address verification errors, then rerun from the last safe checkpoint"
                .to_string(),
        });
    }

    advisories
}

#[cfg(test)]
#[path = "tests/recovery_tests.rs"]
mod tests;
