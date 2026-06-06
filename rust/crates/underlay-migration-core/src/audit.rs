use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

use crate::pipeline::PipelineRunReport;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    BundlePull,
    IntegrityVerify,
    ApplyMaterialize,
    Resume,
    CleanupAdvisory,
    PromotionCheck,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuditOutcome {
    Success,
    Failure,
    Advisory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditRecord {
    pub audit_id: underlay_core::Uuid,
    pub run_id: underlay_core::Uuid,
    pub action: AuditAction,
    pub occurred_at: DateTime<Utc>,
    pub outcome: AuditOutcome,
    pub summary: String,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditArtifact {
    pub run_id: underlay_core::Uuid,
    pub generated_at: DateTime<Utc>,
    pub record_count: usize,
    pub records: Vec<AuditRecord>,
}

pub fn build_audit_artifact(report: &PipelineRunReport) -> AuditArtifact {
    let mut records = Vec::new();

    records.push(AuditRecord {
        audit_id: deterministic_audit_id(report.run_id, "bundle_pull"),
        run_id: report.run_id,
        action: AuditAction::BundlePull,
        occurred_at: report.started_at,
        outcome: if report.integrity_gate.evidence.digest_verified
            && report.integrity_gate.evidence.sidecar_checksums_verified
        {
            AuditOutcome::Success
        } else {
            AuditOutcome::Failure
        },
        summary: "bundle pull and initial evidence capture".to_string(),
        metadata: BTreeMap::from([
            (
                "digest_verified".to_string(),
                report.integrity_gate.evidence.digest_verified.to_string(),
            ),
            (
                "sidecar_checksums_verified".to_string(),
                report
                    .integrity_gate
                    .evidence
                    .sidecar_checksums_verified
                    .to_string(),
            ),
        ]),
    });

    records.push(AuditRecord {
        audit_id: deterministic_audit_id(report.run_id, "integrity_verify"),
        run_id: report.run_id,
        action: AuditAction::IntegrityVerify,
        occurred_at: report.started_at,
        outcome: if report.integrity_gate.passed {
            AuditOutcome::Success
        } else {
            AuditOutcome::Failure
        },
        summary: "integrity gate evaluation".to_string(),
        metadata: BTreeMap::from([
            (
                "blocker_count".to_string(),
                report.integrity_gate.blockers.len().to_string(),
            ),
            (
                "require_signature_verification".to_string(),
                report
                    .integrity_gate
                    .policy
                    .require_signature_verification()
                    .to_string(),
            ),
        ]),
    });

    records.push(AuditRecord {
        audit_id: deterministic_audit_id(report.run_id, "apply_materialize"),
        run_id: report.run_id,
        action: AuditAction::ApplyMaterialize,
        occurred_at: report.finished_at,
        outcome: if report.integrity_gate.passed {
            AuditOutcome::Success
        } else {
            AuditOutcome::Failure
        },
        summary: "materialization/apply stage execution".to_string(),
        metadata: BTreeMap::from([
            (
                "inserted".to_string(),
                report.materialize.inserted.to_string(),
            ),
            (
                "updated".to_string(),
                report.materialize.updated.to_string(),
            ),
            (
                "skipped".to_string(),
                report.materialize.skipped.to_string(),
            ),
        ]),
    });

    records.push(AuditRecord {
        audit_id: deterministic_audit_id(report.run_id, "resume"),
        run_id: report.run_id,
        action: AuditAction::Resume,
        occurred_at: report.started_at,
        outcome: if report.resume_diagnostics.resume_attempted {
            AuditOutcome::Success
        } else {
            AuditOutcome::Advisory
        },
        summary: "resume/checkpoint decision".to_string(),
        metadata: BTreeMap::from([
            (
                "resume_attempted".to_string(),
                report.resume_diagnostics.resume_attempted.to_string(),
            ),
            (
                "status".to_string(),
                report.resume_diagnostics.status.clone(),
            ),
        ]),
    });

    let cleanup_required = !report.verify.passed || !report.integrity_gate.passed;
    records.push(AuditRecord {
        audit_id: deterministic_audit_id(report.run_id, "cleanup_advisory"),
        run_id: report.run_id,
        action: AuditAction::CleanupAdvisory,
        occurred_at: report.finished_at,
        outcome: if cleanup_required {
            AuditOutcome::Advisory
        } else {
            AuditOutcome::Success
        },
        summary: "cleanup and rerun recommendation status".to_string(),
        metadata: BTreeMap::from([("cleanup_required".to_string(), cleanup_required.to_string())]),
    });

    records.push(AuditRecord {
        audit_id: deterministic_audit_id(report.run_id, "promotion_check"),
        run_id: report.run_id,
        action: AuditAction::PromotionCheck,
        occurred_at: report.finished_at,
        outcome: if report.verify.passed && report.integrity_gate.passed {
            AuditOutcome::Success
        } else {
            AuditOutcome::Failure
        },
        summary: "promotion readiness gate".to_string(),
        metadata: BTreeMap::from([
            (
                "verify_passed".to_string(),
                report.verify.passed.to_string(),
            ),
            (
                "integrity_passed".to_string(),
                report.integrity_gate.passed.to_string(),
            ),
        ]),
    });

    AuditArtifact {
        run_id: report.run_id,
        generated_at: report.finished_at,
        record_count: records.len(),
        records,
    }
}

fn deterministic_audit_id(run_id: underlay_core::Uuid, action: &str) -> underlay_core::Uuid {
    let mut hasher = Sha256::new();
    hasher.update(run_id.to_string().as_bytes());
    hasher.update(b":");
    hasher.update(action.as_bytes());
    let hash = hasher.finalize();

    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&hash[..16]);
    bytes[6] = (bytes[6] & 0x0f) | 0x70;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    underlay_core::Uuid(underlay_core::RawUuid::from_bytes(bytes))
}

#[cfg(test)]
#[path = "tests/audit_tests.rs"]
mod tests;
