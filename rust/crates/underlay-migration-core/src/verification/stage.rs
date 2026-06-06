use crate::context::MigrationContext;
use crate::errors::{MigrationError, MigrationResult};
use crate::plugin::MigrationPlugin;
use crate::verification_rules::evaluate_verification_rules;

use super::{
    VerificationCheckResult, VerificationInput, VerificationIssue, VerificationReport,
    VerificationSeverity,
};

pub async fn verify_stage<P>(
    plugin: &P,
    ctx: &MigrationContext,
    input: &VerificationInput,
) -> MigrationResult<VerificationReport>
where
    P: MigrationPlugin,
{
    let mut checks = Vec::new();
    let mut issues = Vec::new();

    let rule_result = evaluate_verification_rules(input);
    checks.extend(rule_result.checks);
    issues.extend(rule_result.issues);

    let decision_coverage_pass = input.transform_record_count == input.decision_count;
    checks.push(VerificationCheckResult {
        check: "decision_coverage".to_string(),
        passed: decision_coverage_pass,
        details: format!(
            "transform_record_count={}, decision_count={}",
            input.transform_record_count, input.decision_count
        ),
    });
    if !decision_coverage_pass {
        issues.push(VerificationIssue {
            code: "decision_coverage_mismatch".to_string(),
            message: format!(
                "decision count mismatch: expected {}, got {}",
                input.transform_record_count, input.decision_count
            ),
            severity: VerificationSeverity::Error,
        });
    }

    let unresolved_pass =
        !ctx.policy.fail_on_unresolved_decisions || input.unresolved_decision_count == 0;
    checks.push(VerificationCheckResult {
        check: "unresolved_decisions".to_string(),
        passed: unresolved_pass,
        details: format!(
            "unresolved_decision_count={}, fail_on_unresolved_decisions={}",
            input.unresolved_decision_count, ctx.policy.fail_on_unresolved_decisions
        ),
    });
    if !unresolved_pass {
        issues.push(VerificationIssue {
            code: "unresolved_decisions".to_string(),
            message: format!(
                "unresolved decisions detected: {}",
                input.unresolved_decision_count
            ),
            severity: VerificationSeverity::Error,
        });
    }

    let governance_pass = input.decision_governance_issue_count == 0;
    checks.push(VerificationCheckResult {
        check: "decision_governance_integrity".to_string(),
        passed: governance_pass,
        details: format!(
            "decision_governance_issue_count={}",
            input.decision_governance_issue_count
        ),
    });
    if !governance_pass {
        issues.push(VerificationIssue {
            code: "decision_governance_integrity".to_string(),
            message: format!(
                "decision governance issues detected: {}",
                input.decision_governance_issue_count
            ),
            severity: VerificationSeverity::Error,
        });
    }

    let checksum_pass = !input.transform_checksum.is_empty();
    checks.push(VerificationCheckResult {
        check: "transform_checksum".to_string(),
        passed: checksum_pass,
        details: format!("sha256={}", input.transform_checksum),
    });
    if !checksum_pass {
        issues.push(VerificationIssue {
            code: "transform_checksum_missing".to_string(),
            message: "transform checksum was empty".to_string(),
            severity: VerificationSeverity::Error,
        });
    }

    let plugin_issues = plugin
        .verify_semantics(ctx, input)
        .await
        .map_err(|err| MigrationError::Plugin(err.to_string()))?;

    checks.push(VerificationCheckResult {
        check: "plugin_semantics".to_string(),
        passed: plugin_issues
            .iter()
            .all(|issue| issue.severity != VerificationSeverity::Error),
        details: format!("issue_count={}", plugin_issues.len()),
    });

    issues.extend(plugin_issues);

    let passed = issues
        .iter()
        .all(|issue| issue.severity != VerificationSeverity::Error);

    Ok(VerificationReport {
        passed,
        checks,
        issues,
    })
}
