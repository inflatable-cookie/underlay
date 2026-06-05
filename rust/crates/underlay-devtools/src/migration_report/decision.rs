use std::collections::HashMap;
use std::path::Path;

use underlay_migration_core::{
    parse_decision_index, parse_decision_journal_ndjson, DecideStageOutput,
    DecisionGovernanceIssue, DecisionInvalidationReason, PipelineRunReport,
};

use super::json::parse_json;
use super::MigrationReportError;

pub fn format_decision_invalidation_report(decide: &DecideStageOutput) -> Vec<String> {
    if decide.invalidations.is_empty() {
        return vec!["no invalidations recorded".to_string()];
    }

    let mut counts: HashMap<DecisionInvalidationReason, usize> = HashMap::new();
    for invalidation in &decide.invalidations {
        *counts.entry(invalidation.reason).or_insert(0) += 1;
    }

    let mut rows = counts
        .into_iter()
        .map(|(reason, count)| format!("{}: {count}", reason_label(reason)))
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

pub fn format_decision_governance_report(decide: &DecideStageOutput) -> Vec<String> {
    if decide.governance_issues.is_empty() {
        return vec!["no governance issues recorded".to_string()];
    }

    let mut counts: HashMap<(String, String), usize> = HashMap::new();
    for issue in &decide.governance_issues {
        *counts
            .entry((issue.artifact.clone(), issue.code.clone()))
            .or_insert(0) += 1;
    }

    let mut rows = counts
        .into_iter()
        .map(|((artifact, code), count)| format!("{artifact}.{code}: {count}"))
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

pub fn load_decide_stage_output(path: &Path) -> Result<DecideStageOutput, MigrationReportError> {
    let bytes = std::fs::read(path).map_err(MigrationReportError::Io)?;
    if let Ok(decide) = parse_json::<DecideStageOutput>(&bytes) {
        return Ok(decide);
    }

    let report: PipelineRunReport = parse_json(&bytes)?;
    Ok(report.decide)
}

pub fn load_decision_index(
    path: &Path,
) -> Result<underlay_migration_core::DecisionIndex, MigrationReportError> {
    let bytes = std::fs::read(path).map_err(MigrationReportError::Io)?;
    let text = String::from_utf8(bytes)
        .map_err(|err| MigrationReportError::InvalidInput(err.to_string()))?;
    parse_decision_index(&text).map_err(|err| MigrationReportError::InvalidInput(err.to_string()))
}

pub fn load_decision_journal(
    path: &Path,
) -> Result<Vec<underlay_migration_core::DecisionJournalRecord>, MigrationReportError> {
    let bytes = std::fs::read(path).map_err(MigrationReportError::Io)?;
    let text = String::from_utf8(bytes)
        .map_err(|err| MigrationReportError::InvalidInput(err.to_string()))?;
    parse_decision_journal_ndjson(&text)
        .map_err(|err| MigrationReportError::InvalidInput(err.to_string()))
}

pub fn top_governance_issues(
    decide: &DecideStageOutput,
    limit: usize,
) -> Vec<DecisionGovernanceIssue> {
    decide
        .governance_issues
        .iter()
        .take(limit)
        .cloned()
        .collect()
}

fn reason_label(reason: DecisionInvalidationReason) -> &'static str {
    match reason {
        DecisionInvalidationReason::FingerprintMismatch => "fingerprint_mismatch",
        DecisionInvalidationReason::ResolverVersionMismatch => "resolver_version_mismatch",
        DecisionInvalidationReason::PromptVersionMismatch => "prompt_version_mismatch",
        DecisionInvalidationReason::TargetSchemaVersionMismatch => "target_schema_version_mismatch",
        DecisionInvalidationReason::PluginDependencyChanged => "plugin_dependency_changed",
    }
}
