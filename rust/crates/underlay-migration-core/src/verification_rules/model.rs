use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::verification::{VerificationCheckResult, VerificationIssue, VerificationSeverity};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationRule {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub severity: VerificationSeverity,
    pub kind: VerificationRuleKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationRuleKind {
    RowCount {
        metric: VerificationMetric,
        expectation: CountExpectation,
    },
    NotNull {
        field_path: String,
    },
    Unique {
        field_path: String,
        #[serde(default)]
        ignore_nulls: bool,
    },
    ReferentialIntegrity {
        field_path: String,
        reference_field_path: String,
        #[serde(default)]
        allow_null: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationMetric {
    TransformRecordCount,
    DecisionCount,
    UnresolvedDecisionCount,
    MaterializeInserted,
    MaterializeUpdated,
    MaterializeSkipped,
    AssetsResolvedCount,
    AssetsUnresolvedCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CountExpectation {
    Exact(usize),
    Minimum(usize),
    Maximum(usize),
    Between { min: usize, max: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RuleEngineResult {
    pub checks: Vec<VerificationCheckResult>,
    pub issues: Vec<VerificationIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationBenchmarkResult {
    pub iterations: usize,
    pub declarative_elapsed: Duration,
    pub plugin_elapsed: Duration,
    pub declarative_avg_micros: u128,
    pub plugin_avg_micros: u128,
}
