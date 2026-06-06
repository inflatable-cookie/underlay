use crate::errors::MigrationResult;
use crate::pipeline::PipelineRunReport;

use super::checksum::transform_checksum;
use super::{
    VerificationArtifact, VerificationChecksumSection, VerificationIntegrityGateSection,
    VerificationPromotionGate, VerificationReferentialIntegritySection,
    VerificationRowCountSection, VerificationSeverity,
};

pub fn build_verification_artifact(
    report: &PipelineRunReport,
) -> MigrationResult<VerificationArtifact> {
    let transform_checksum = transform_checksum(&report.transform)?;
    let decision_coverage_pass = report.transform.record_count == report.decide.decision_count;

    let row_counts = VerificationRowCountSection {
        transform_record_count: report.transform.record_count,
        decision_count: report.decide.decision_count,
        unresolved_decision_count: report.decide.unresolved_count,
        decision_coverage_pass,
    };

    let checksums = VerificationChecksumSection {
        transform_checksum_present: !transform_checksum.is_empty(),
        transform_checksum,
    };

    let referential_integrity = VerificationReferentialIntegritySection {
        passed: !report.verify.issues.iter().any(|issue| {
            issue.code.contains("referential") && issue.severity == VerificationSeverity::Error
        }),
        source: "verify_stage_issues".to_string(),
    };
    let integrity_gate = VerificationIntegrityGateSection {
        passed: report.integrity_gate.passed,
        blocker_count: report.integrity_gate.blockers.len(),
    };

    let mut blockers = Vec::new();
    if !report.verify.passed {
        blockers.push("verify_stage_failed".to_string());
    }
    if !decision_coverage_pass {
        blockers.push("decision_coverage_mismatch".to_string());
    }
    if !checksums.transform_checksum_present {
        blockers.push("transform_checksum_missing".to_string());
    }
    if !referential_integrity.passed {
        blockers.push("referential_integrity_failed".to_string());
    }
    if !integrity_gate.passed {
        blockers.push("integrity_gate_failed".to_string());
    }
    for issue in &report.verify.issues {
        if issue.severity == VerificationSeverity::Error {
            blockers.push(format!("verification_issue:{}", issue.code));
        }
    }
    blockers.sort();
    blockers.dedup();

    Ok(VerificationArtifact {
        run_id: report.run_id,
        generated_at: chrono::Utc::now(),
        verify_passed: report.verify.passed,
        row_counts,
        checksums,
        referential_integrity,
        integrity_gate,
        issues: report.verify.issues.clone(),
        promotion_gate: VerificationPromotionGate {
            can_promote: blockers.is_empty(),
            blockers,
        },
    })
}
