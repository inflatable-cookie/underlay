use std::path::{Path, PathBuf};

use underlay_migration_core::{build_audit_artifact, AuditArtifact, PipelineRunReport};

use super::MigrationReportError;

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
