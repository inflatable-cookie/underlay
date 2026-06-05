use std::path::{Path, PathBuf};

use underlay_migration_core::{
    build_verification_artifact, PipelineRunReport, VerificationArtifact,
};

use super::MigrationReportError;

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
