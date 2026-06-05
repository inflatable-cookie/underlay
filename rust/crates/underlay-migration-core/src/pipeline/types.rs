use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::decision_memory::DecisionInvalidationReason;
use crate::integrity::IntegrityGateResult;
use crate::plugin::{
    DecisionOutcome, DecisionProvenance, LegacyRecordBatch, NormalizedBatch, TransformBatch,
};
use crate::verification::{VerificationCheckResult, VerificationIssue};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum StageName {
    Extract,
    Normalize,
    Transform,
    Decide,
    Materialize,
    Assets,
    Verify,
}

impl StageName {
    pub fn as_str(self) -> &'static str {
        match self {
            StageName::Extract => "extract",
            StageName::Normalize => "normalize",
            StageName::Transform => "transform",
            StageName::Decide => "decide",
            StageName::Materialize => "materialize",
            StageName::Assets => "assets",
            StageName::Verify => "verify",
        }
    }

    pub(crate) fn index(self) -> usize {
        match self {
            StageName::Extract => 0,
            StageName::Normalize => 1,
            StageName::Transform => 2,
            StageName::Decide => 3,
            StageName::Materialize => 4,
            StageName::Assets => 5,
            StageName::Verify => 6,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractStageOutput {
    pub batches: Vec<LegacyRecordBatch>,
    pub batch_count: usize,
    pub record_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NormalizeStageOutput {
    pub batches: Vec<NormalizedBatch>,
    pub batch_count: usize,
    pub record_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransformStageOutput {
    pub batches: Vec<TransformBatch>,
    pub batch_count: usize,
    pub record_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecideStageOutput {
    pub decisions: Vec<DecisionOutcome>,
    pub decision_count: usize,
    pub unresolved_count: usize,
    #[serde(default)]
    pub reused_count: usize,
    #[serde(default)]
    pub resolved_count: usize,
    #[serde(default)]
    pub invalidated_count: usize,
    #[serde(default)]
    pub invalidations: Vec<DecisionInvalidationEvent>,
    #[serde(default)]
    pub unresolved_queue: Vec<UnresolvedDecisionItem>,
    #[serde(default)]
    pub governance_issues: Vec<DecisionGovernanceIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecisionInvalidationEvent {
    pub fingerprint: String,
    pub reason: DecisionInvalidationReason,
    pub decision_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnresolvedDecisionItem {
    pub unresolved_id: underlay_core::Uuid,
    pub fingerprint: String,
    pub decision_type: String,
    pub confidence: Option<f64>,
    pub threshold: f64,
    pub reason: String,
    pub provenance: DecisionProvenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecisionGovernanceIssue {
    pub artifact: String,
    pub fingerprint: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MaterializeStageOutput {
    pub inserted: u64,
    pub updated: u64,
    pub skipped: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssetsStageOutput {
    pub resolved_count: u64,
    pub unresolved_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyStageOutput {
    pub passed: bool,
    pub checks: Vec<VerificationCheckResult>,
    pub issues: Vec<VerificationIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelineRunReport {
    pub run_id: underlay_core::Uuid,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub extract: ExtractStageOutput,
    pub normalize: NormalizeStageOutput,
    pub transform: TransformStageOutput,
    pub decide: DecideStageOutput,
    #[serde(default)]
    pub integrity_gate: IntegrityGateResult,
    pub materialize: MaterializeStageOutput,
    pub assets: AssetsStageOutput,
    pub verify: VerifyStageOutput,
    #[serde(default)]
    pub resume_diagnostics: ResumeDiagnostics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResumeDiagnostics {
    pub resume_attempted: bool,
    pub resumed_from_stage: Option<StageName>,
    pub status: String,
    pub reason: Option<String>,
}

impl Default for ResumeDiagnostics {
    fn default() -> Self {
        Self {
            resume_attempted: false,
            resumed_from_stage: None,
            status: "fresh_start".to_string(),
            reason: None,
        }
    }
}
