use std::path::Path;

use underlay_migration_core::{
    evaluate_governance_policy, GovernanceComplianceReport, GovernancePolicy,
};

use super::json::parse_json;
use super::MigrationReportError;

pub fn load_governance_policy(path: &Path) -> Result<GovernancePolicy, MigrationReportError> {
    let bytes = std::fs::read(path).map_err(MigrationReportError::Io)?;
    parse_json::<GovernancePolicy>(&bytes)
}

pub fn build_policy_report(policy: &GovernancePolicy) -> GovernanceComplianceReport {
    evaluate_governance_policy(policy)
}

pub fn format_policy_summary(report: &GovernanceComplianceReport) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "compliant={} issues={} blocking={}",
        report.compliant, report.issue_count, report.blocking_issue_count
    ));
    for issue in &report.issues {
        lines.push(format!(
            "{} [{:?}]: {} -> {}",
            issue.code, issue.severity, issue.message, issue.remediation_hint
        ));
    }
    lines
}
