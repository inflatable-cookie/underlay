use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use underlay_migration_core::{
    build_audit_artifact, build_integrity_artifact, build_verification_artifact,
    detect_drift_from_run, detect_drift_with_lineage, evaluate_governance_policy,
    parse_decision_index, parse_decision_journal_ndjson, AuditArtifact, DecideStageOutput,
    DecisionGovernanceIssue, DecisionInvalidationReason, DecisionLineageInput,
    DriftDetectionReport, DriftThresholds, GovernanceComplianceReport, GovernancePolicy,
    IntegrityArtifact, PipelineRunReport, RecoveryAdvisory, VerificationArtifact,
};

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

pub fn load_pipeline_run_report(path: &Path) -> Result<PipelineRunReport, MigrationReportError> {
    let bytes = std::fs::read(path).map_err(MigrationReportError::Io)?;
    parse_json::<PipelineRunReport>(&bytes)
}

pub fn build_drift_report(
    report: &PipelineRunReport,
    thresholds: &DriftThresholds,
) -> DriftDetectionReport {
    detect_drift_from_run(report, thresholds)
}

pub fn build_drift_report_with_lineage(
    report: &PipelineRunReport,
    thresholds: &DriftThresholds,
    lineage: Option<&DecisionLineageInput>,
) -> DriftDetectionReport {
    detect_drift_with_lineage(report, thresholds, lineage)
}

pub fn format_drift_report(report: &DriftDetectionReport) -> Vec<String> {
    if report.issues.is_empty() {
        return vec!["no drift issues detected".to_string()];
    }

    report
        .issues
        .iter()
        .map(|issue| {
            format!(
                "{}.{} [{}]: {} -> {}",
                issue.category,
                issue.code,
                severity_label(issue.severity),
                issue.message,
                issue.remediation_hint
            )
        })
        .collect()
}

pub fn format_drift_category_summary(report: &DriftDetectionReport) -> Vec<String> {
    if report.category_summaries.is_empty() {
        return vec!["no drift categories recorded".to_string()];
    }

    report
        .category_summaries
        .iter()
        .map(|summary| {
            format!(
                "{}: issues={}, blocking={}",
                summary.category, summary.issue_count, summary.blocking_issue_count
            )
        })
        .collect()
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

pub fn build_recovery_advisories(report: &PipelineRunReport) -> Vec<RecoveryAdvisory> {
    underlay_migration_core::recovery_advisories_from_run(report)
}

pub fn format_recovery_advisories(advisories: &[RecoveryAdvisory]) -> Vec<String> {
    if advisories.is_empty() {
        return vec!["no recovery actions recommended".to_string()];
    }

    advisories
        .iter()
        .map(|advisory| {
            format!(
                "{} [{}]: {} -> {}",
                advisory.code,
                failure_class_label(advisory.failure_class),
                advisory.summary,
                advisory.action
            )
        })
        .collect()
}

pub fn build_verification_report(
    report: &PipelineRunReport,
) -> Result<VerificationArtifact, MigrationReportError> {
    build_verification_artifact(report)
        .map_err(|err| MigrationReportError::InvalidInput(err.to_string()))
}

pub fn write_verification_artifact(
    output_dir: &Path,
    artifact: &VerificationArtifact,
) -> Result<PathBuf, MigrationReportError> {
    let artifact_dir = output_dir.join("verification-artifacts");
    std::fs::create_dir_all(&artifact_dir).map_err(MigrationReportError::Io)?;
    let path = artifact_dir.join(format!("{}.json", artifact.run_id));
    let bytes = serde_json::to_vec_pretty(artifact).map_err(MigrationReportError::Json)?;
    std::fs::write(&path, bytes).map_err(MigrationReportError::Io)?;
    Ok(path)
}

pub fn format_verification_summary(artifact: &VerificationArtifact) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "verify_passed={} can_promote={}",
        artifact.verify_passed, artifact.promotion_gate.can_promote
    ));
    lines.push(format!(
        "row_counts transform={} decisions={} unresolved={}",
        artifact.row_counts.transform_record_count,
        artifact.row_counts.decision_count,
        artifact.row_counts.unresolved_decision_count
    ));
    lines.push(format!(
        "checksum_present={} checksum={}",
        artifact.checksums.transform_checksum_present, artifact.checksums.transform_checksum
    ));
    if artifact.promotion_gate.blockers.is_empty() {
        lines.push("blockers none".to_string());
    } else {
        for blocker in &artifact.promotion_gate.blockers {
            lines.push(format!("blocker {blocker}"));
        }
    }
    lines
}

pub fn build_integrity_report(report: &PipelineRunReport) -> IntegrityArtifact {
    build_integrity_artifact(report.run_id, report.integrity_gate.clone())
}

pub fn format_integrity_summary(artifact: &IntegrityArtifact) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "passed={} require_digest_verification={} require_sidecar_checksum_verification={} require_signature_verification={} effective_require_signature_verification={} signature_enforcement_phase={} run_scope={}",
        artifact.gate.passed,
        artifact.gate.policy.require_digest_verification,
        artifact.gate.policy.require_sidecar_checksum_verification,
        artifact.gate.policy.require_signature_verification,
        artifact.gate.effective_require_signature_verification,
        signature_enforcement_phase_label(artifact.gate.policy.signature_enforcement_phase),
        integrity_run_scope_label(artifact.gate.policy.run_scope)
    ));
    lines.push(format!(
        "signature_verified={:?} signature_verified_at={:?} signer_identity={:?} signature_key_id={:?}",
        artifact.gate.evidence.signature_verified,
        artifact.gate.evidence.signature_verified_at,
        artifact.gate.evidence.signer_identity,
        artifact.gate.evidence.signature_key_id
    ));
    if artifact.gate.blockers.is_empty() {
        lines.push("blockers none".to_string());
    } else {
        for blocker in &artifact.gate.blockers {
            lines.push(format!(
                "blocker {}: {} -> {}",
                blocker.code, blocker.message, blocker.remediation_hint
            ));
        }
    }
    lines
}

pub fn build_audit_report(report: &PipelineRunReport) -> AuditArtifact {
    build_audit_artifact(report)
}

pub fn format_audit_summary(artifact: &AuditArtifact) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "run_id={} record_count={}",
        artifact.run_id, artifact.record_count
    ));
    for record in &artifact.records {
        lines.push(format!(
            "{} {:?} {:?}: {}",
            record.audit_id, record.action, record.outcome, record.summary
        ));
    }
    lines
}

pub fn write_audit_artifact(
    output_dir: &Path,
    artifact: &AuditArtifact,
) -> Result<PathBuf, MigrationReportError> {
    let artifact_dir = output_dir.join("audit-artifacts");
    std::fs::create_dir_all(&artifact_dir).map_err(MigrationReportError::Io)?;
    let path = artifact_dir.join(format!("{}.json", artifact.run_id));
    let bytes = serde_json::to_vec_pretty(artifact).map_err(MigrationReportError::Json)?;
    std::fs::write(&path, bytes).map_err(MigrationReportError::Io)?;
    Ok(path)
}

pub fn load_pipeline_run_report_from_path(
    input: &Path,
) -> Result<PipelineRunReport, MigrationReportError> {
    if input.is_file() {
        return load_pipeline_run_report(input);
    }

    if input.is_dir() {
        let mut files = std::fs::read_dir(input)
            .map_err(MigrationReportError::Io)?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|path| path.is_file())
            .collect::<Vec<_>>();
        files.sort();
        for file in files {
            if let Ok(report) = load_pipeline_run_report(&file) {
                return Ok(report);
            }
        }
        return Err(MigrationReportError::InvalidInput(format!(
            "no pipeline run report JSON found in {}",
            input.display()
        )));
    }

    Err(MigrationReportError::InvalidInput(format!(
        "input path does not exist: {}",
        input.display()
    )))
}

pub fn load_governance_policy(path: &Path) -> Result<GovernancePolicy, MigrationReportError> {
    let bytes = std::fs::read(path).map_err(MigrationReportError::Io)?;
    parse_json::<GovernancePolicy>(&bytes)
}

pub fn build_policy_report(policy: &GovernancePolicy) -> GovernanceComplianceReport {
    evaluate_governance_policy(policy)
}

pub fn format_policy_summary(report: &GovernanceComplianceReport) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "compliant={} issues={} blocking={}",
        report.compliant, report.issue_count, report.blocking_issue_count
    ));
    for issue in &report.issues {
        lines.push(format!(
            "{} [{:?}]: {} -> {}",
            issue.code, issue.severity, issue.message, issue.remediation_hint
        ));
    }
    lines
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

#[derive(Debug)]
pub enum MigrationReportError {
    Io(std::io::Error),
    Json(serde_json::Error),
    InvalidInput(String),
}

impl std::fmt::Display for MigrationReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationReportError::Io(err) => write!(f, "{err}"),
            MigrationReportError::Json(err) => write!(f, "{err}"),
            MigrationReportError::InvalidInput(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for MigrationReportError {}

fn parse_json<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, MigrationReportError> {
    serde_json::from_slice(bytes).map_err(MigrationReportError::Json)
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

fn severity_label(severity: underlay_migration_core::DriftSeverity) -> &'static str {
    match severity {
        underlay_migration_core::DriftSeverity::Warning => "warning",
        underlay_migration_core::DriftSeverity::Error => "error",
    }
}

fn failure_class_label(class: underlay_migration_core::FailureClass) -> &'static str {
    match class {
        underlay_migration_core::FailureClass::RetrySafe => "retry_safe",
        underlay_migration_core::FailureClass::NonRetrySafe => "non_retry_safe",
    }
}

fn signature_enforcement_phase_label(
    phase: underlay_migration_core::SignatureEnforcementPhase,
) -> &'static str {
    match phase {
        underlay_migration_core::SignatureEnforcementPhase::Observe => "observe",
        underlay_migration_core::SignatureEnforcementPhase::EnforcePreprod => "enforce_preprod",
        underlay_migration_core::SignatureEnforcementPhase::EnforceAll => "enforce_all",
    }
}

fn integrity_run_scope_label(scope: underlay_migration_core::IntegrityRunScope) -> &'static str {
    match scope {
        underlay_migration_core::IntegrityRunScope::Demo => "demo",
        underlay_migration_core::IntegrityRunScope::PreProduction => "pre_production",
        underlay_migration_core::IntegrityRunScope::Production => "production",
    }
}
