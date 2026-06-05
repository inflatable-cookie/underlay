use crate::verification::{VerificationCheckResult, VerificationInput, VerificationIssue};

use super::field_rules::{
    evaluate_not_null_rule, evaluate_referential_integrity_rule, evaluate_unique_rule,
};
use super::row_count::evaluate_row_count_rule;
use super::value_path::sanitize_rule_name;
use super::{RuleEngineResult, VerificationRuleKind};

pub fn evaluate_verification_rules(input: &VerificationInput) -> RuleEngineResult {
    let mut checks = Vec::with_capacity(input.rules.len());
    let mut issues = Vec::new();

    for rule in &input.rules {
        let (passed, details, issue_message) = match &rule.kind {
            VerificationRuleKind::RowCount {
                metric,
                expectation,
            } => evaluate_row_count_rule(input, metric, expectation),
            VerificationRuleKind::NotNull { field_path } => {
                evaluate_not_null_rule(&input.transform_records, field_path)
            }
            VerificationRuleKind::Unique {
                field_path,
                ignore_nulls,
            } => evaluate_unique_rule(&input.transform_records, field_path, *ignore_nulls),
            VerificationRuleKind::ReferentialIntegrity {
                field_path,
                reference_field_path,
                allow_null,
            } => evaluate_referential_integrity_rule(
                &input.transform_records,
                field_path,
                reference_field_path,
                *allow_null,
            ),
        };

        checks.push(VerificationCheckResult {
            check: format!("rule:{}", rule.name),
            passed,
            details,
        });

        if !passed {
            issues.push(VerificationIssue {
                code: format!("verification_rule_{}", sanitize_rule_name(&rule.name)),
                message: issue_message.unwrap_or_else(|| {
                    rule.description
                        .clone()
                        .unwrap_or_else(|| format!("verification rule `{}` failed", rule.name))
                }),
                severity: rule.severity,
            });
        }
    }

    RuleEngineResult { checks, issues }
}
