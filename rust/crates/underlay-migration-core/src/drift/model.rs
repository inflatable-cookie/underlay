use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::decision_memory::DecisionIndex;
use crate::run_store::DecisionJournalRecord;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DriftSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DriftIssue {
    pub category: String,
    pub code: String,
    pub severity: DriftSeverity,
    pub message: String,
    pub remediation_hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DriftDetectionReport {
    pub run_id: underlay_core::Uuid,
    pub generated_at: DateTime<Utc>,
    pub issue_count: usize,
    pub blocking_issue_count: usize,
    pub issues: Vec<DriftIssue>,
    pub category_summaries: Vec<DriftCategorySummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DriftCategorySummary {
    pub category: String,
    pub issue_count: usize,
    pub blocking_issue_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DriftThresholds {
    pub max_unresolved_decisions: usize,
    pub max_governance_issues: usize,
    pub max_lineage_mismatches: usize,
    pub require_verify_passed: bool,
}

impl Default for DriftThresholds {
    fn default() -> Self {
        Self {
            max_unresolved_decisions: 0,
            max_governance_issues: 0,
            max_lineage_mismatches: 0,
            require_verify_passed: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecisionLineageInput {
    pub index: DecisionIndex,
    pub journal_records: Vec<DecisionJournalRecord>,
    pub expected_bundle_digest: Option<String>,
}
