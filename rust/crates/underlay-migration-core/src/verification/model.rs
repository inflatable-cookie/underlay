use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::pipeline::{AssetsStageOutput, MaterializeStageOutput};
use crate::verification_rules::VerificationRule;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationIssue {
    pub code: String,
    pub message: String,
    pub severity: VerificationSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationCheckResult {
    pub check: String,
    pub passed: bool,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationInput {
    pub transform_record_count: usize,
    #[serde(default)]
    pub transform_records: Vec<Value>,
    pub decision_count: usize,
    pub unresolved_decision_count: usize,
    pub decision_governance_issue_count: usize,
    pub transform_checksum: String,
    pub materialize: MaterializeStageOutput,
    pub assets: AssetsStageOutput,
    #[serde(default)]
    pub rules: Vec<VerificationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationReport {
    pub passed: bool,
    pub checks: Vec<VerificationCheckResult>,
    pub issues: Vec<VerificationIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationArtifact {
    pub run_id: underlay_core::Uuid,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub verify_passed: bool,
    pub row_counts: VerificationRowCountSection,
    pub checksums: VerificationChecksumSection,
    pub referential_integrity: VerificationReferentialIntegritySection,
    pub integrity_gate: VerificationIntegrityGateSection,
    pub issues: Vec<VerificationIssue>,
    pub promotion_gate: VerificationPromotionGate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationRowCountSection {
    pub transform_record_count: usize,
    pub decision_count: usize,
    pub unresolved_decision_count: usize,
    pub decision_coverage_pass: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationChecksumSection {
    pub transform_checksum: String,
    pub transform_checksum_present: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationReferentialIntegritySection {
    pub passed: bool,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationIntegrityGateSection {
    pub passed: bool,
    pub blocker_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationPromotionGate {
    pub can_promote: bool,
    pub blockers: Vec<String>,
}
