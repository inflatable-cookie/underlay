use chrono::Utc;

use crate::pipeline::PipelineRunReport;

use super::lineage::append_lineage_issues;
use super::model::{
    DecisionLineageInput, DriftDetectionReport, DriftIssue, DriftSeverity, DriftThresholds,
};
use super::summary::summarize_categories;

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
