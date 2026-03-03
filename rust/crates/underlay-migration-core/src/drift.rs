use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::decision_memory::{validate_decision_index, DecisionIndex};
use crate::pipeline::PipelineRunReport;
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

pub fn detect_drift_from_run(
    report: &PipelineRunReport,
    thresholds: &DriftThresholds,
) -> DriftDetectionReport {
    detect_drift_with_lineage(report, thresholds, None)
}

pub fn detect_drift_with_lineage(
    report: &PipelineRunReport,
    thresholds: &DriftThresholds,
    lineage: Option<&DecisionLineageInput>,
) -> DriftDetectionReport {
    let mut issues = Vec::new();

    if report.decide.unresolved_count > thresholds.max_unresolved_decisions {
        issues.push(DriftIssue {
            category: "decision_state".to_string(),
            code: "unresolved_decisions_exceed_threshold".to_string(),
            severity: DriftSeverity::Error,
            message: format!(
                "unresolved decisions {} exceed threshold {}",
                report.decide.unresolved_count, thresholds.max_unresolved_decisions
            ),
            remediation_hint:
                "review unresolved queue, apply overrides where needed, then rerun from checkpoint"
                    .to_string(),
        });
    }

    if report.decide.governance_issues.len() > thresholds.max_governance_issues {
        issues.push(DriftIssue {
            category: "governance".to_string(),
            code: "governance_issues_exceed_threshold".to_string(),
            severity: DriftSeverity::Error,
            message: format!(
                "governance issues {} exceed threshold {}",
                report.decide.governance_issues.len(),
                thresholds.max_governance_issues
            ),
            remediation_hint:
                "resolve governance contract violations in decision artifacts before promotion"
                    .to_string(),
        });
    }

    if thresholds.require_verify_passed && !report.verify.passed {
        issues.push(DriftIssue {
            category: "verification".to_string(),
            code: "verify_stage_failed".to_string(),
            severity: DriftSeverity::Error,
            message: "verify stage did not pass".to_string(),
            remediation_hint: "address verify-stage errors and rerun validation before promoting"
                .to_string(),
        });
    }

    if let Some(lineage) = lineage {
        append_lineage_issues(lineage, thresholds, &mut issues);
    }

    let blocking_issue_count = issues
        .iter()
        .filter(|issue| issue.severity == DriftSeverity::Error)
        .count();
    let category_summaries = summarize_categories(&issues);

    DriftDetectionReport {
        run_id: report.run_id,
        generated_at: Utc::now(),
        issue_count: issues.len(),
        blocking_issue_count,
        issues,
        category_summaries,
    }
}

fn append_lineage_issues(
    lineage: &DecisionLineageInput,
    thresholds: &DriftThresholds,
    issues: &mut Vec<DriftIssue>,
) {
    if let Err(err) = validate_decision_index(&lineage.index) {
        issues.push(DriftIssue {
            category: "lineage".to_string(),
            code: "decision_index_invalid".to_string(),
            severity: DriftSeverity::Error,
            message: err.to_string(),
            remediation_hint:
                "rebuild or republish decision index sidecar from a valid decision journal"
                    .to_string(),
        });
        return;
    }

    let mut lineage_mismatches = 0usize;
    if let Some(expected_digest) = &lineage.expected_bundle_digest {
        if &lineage.index.bundle_digest != expected_digest {
            lineage_mismatches += 1;
            issues.push(DriftIssue {
                category: "lineage".to_string(),
                code: "index_bundle_digest_mismatch".to_string(),
                severity: DriftSeverity::Error,
                message: format!(
                    "decision index bundle_digest {} differs from expected {}",
                    lineage.index.bundle_digest, expected_digest
                ),
                remediation_hint:
                    "ensure index sidecar is linked to the promoted bundle digest, then republish"
                        .to_string(),
            });
        }
    }

    for (fingerprint, entry) in &lineage.index.entries {
        if entry.bundle_digest != lineage.index.bundle_digest {
            lineage_mismatches += 1;
            issues.push(DriftIssue {
                category: "lineage".to_string(),
                code: "entry_bundle_digest_mismatch".to_string(),
                severity: DriftSeverity::Error,
                message: format!(
                    "entry bundle_digest {} for {} differs from index bundle_digest {}",
                    entry.bundle_digest, fingerprint, lineage.index.bundle_digest
                ),
                remediation_hint:
                    "regenerate index entries from the canonical journal for this bundle"
                        .to_string(),
            });
        }

        let matches = lineage
            .journal_records
            .iter()
            .filter(|record| record.fingerprint == *fingerprint)
            .collect::<Vec<_>>();
        if matches.is_empty() {
            lineage_mismatches += 1;
            issues.push(DriftIssue {
                category: "lineage".to_string(),
                code: "index_fingerprint_missing_in_journal".to_string(),
                severity: DriftSeverity::Error,
                message: format!(
                    "fingerprint {} from decision index missing in journal",
                    fingerprint
                ),
                remediation_hint:
                    "append missing journal record or rebuild index from valid journal".to_string(),
            });
            continue;
        }

        if !matches
            .iter()
            .any(|record| record.decision_id == entry.decision_id)
        {
            lineage_mismatches += 1;
            issues.push(DriftIssue {
                category: "lineage".to_string(),
                code: "index_decision_id_mismatch".to_string(),
                severity: DriftSeverity::Error,
                message: format!(
                    "fingerprint {} maps to decision_id {} in index but not in journal lineage",
                    fingerprint, entry.decision_id
                ),
                remediation_hint:
                    "rebuild decision index so fingerprint->decision_id mapping matches journal"
                        .to_string(),
            });
        }
    }

    for record in &lineage.journal_records {
        if !lineage.index.entries.contains_key(&record.fingerprint) {
            issues.push(DriftIssue {
                category: "lineage".to_string(),
                code: "journal_fingerprint_missing_in_index".to_string(),
                severity: DriftSeverity::Warning,
                message: format!(
                    "journal fingerprint {} is not present in decision index",
                    record.fingerprint
                ),
                remediation_hint:
                    "merge/rebuild sidecar index to include latest journal fingerprints".to_string(),
            });
        }
    }

    if lineage_mismatches > thresholds.max_lineage_mismatches {
        issues.push(DriftIssue {
            category: "lineage".to_string(),
            code: "lineage_mismatches_exceed_threshold".to_string(),
            severity: DriftSeverity::Error,
            message: format!(
                "lineage mismatches {} exceed threshold {}",
                lineage_mismatches, thresholds.max_lineage_mismatches
            ),
            remediation_hint: "resolve decision index/journal mismatches before promotion"
                .to_string(),
        });
    }
}

fn summarize_categories(issues: &[DriftIssue]) -> Vec<DriftCategorySummary> {
    let mut map: std::collections::BTreeMap<String, DriftCategorySummary> =
        std::collections::BTreeMap::new();
    for issue in issues {
        let entry = map
            .entry(issue.category.clone())
            .or_insert_with(|| DriftCategorySummary {
                category: issue.category.clone(),
                issue_count: 0,
                blocking_issue_count: 0,
            });
        entry.issue_count += 1;
        if issue.severity == DriftSeverity::Error {
            entry.blocking_issue_count += 1;
        }
    }
    map.into_values().collect()
}

#[cfg(test)]
#[path = "tests/drift_tests.rs"]
mod tests;
