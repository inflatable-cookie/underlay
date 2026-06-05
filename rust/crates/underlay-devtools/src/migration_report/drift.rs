use underlay_migration_core::{
    detect_drift_from_run, detect_drift_with_lineage, DecisionLineageInput, DriftDetectionReport,
    DriftSeverity, DriftThresholds, PipelineRunReport,
};

pub fn build_drift_report(
    report: &PipelineRunReport,
    thresholds: &DriftThresholds,
) -> DriftDetectionReport {
    detect_drift_from_run(report, thresholds)
}

pub fn build_drift_report_with_lineage(
    report: &PipelineRunReport,
    thresholds: &DriftThresholds,
    lineage: Option<&DecisionLineageInput>,
) -> DriftDetectionReport {
    detect_drift_with_lineage(report, thresholds, lineage)
}

pub fn format_drift_report(report: &DriftDetectionReport) -> Vec<String> {
    if report.issues.is_empty() {
        return vec!["no drift issues detected".to_string()];
    }

    report
        .issues
        .iter()
        .map(|issue| {
            format!(
                "{}.{} [{}]: {} -> {}",
                issue.category,
                issue.code,
                severity_label(issue.severity),
                issue.message,
                issue.remediation_hint
            )
        })
        .collect()
}

pub fn format_drift_category_summary(report: &DriftDetectionReport) -> Vec<String> {
    if report.category_summaries.is_empty() {
        return vec!["no drift categories recorded".to_string()];
    }

    report
        .category_summaries
        .iter()
        .map(|summary| {
            format!(
                "{}: issues={}, blocking={}",
                summary.category, summary.issue_count, summary.blocking_issue_count
            )
        })
        .collect()
}

fn severity_label(severity: DriftSeverity) -> &'static str {
    match severity {
        DriftSeverity::Warning => "warning",
        DriftSeverity::Error => "error",
    }
}
