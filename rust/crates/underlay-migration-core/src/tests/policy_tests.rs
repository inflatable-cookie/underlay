use crate::{
    evaluate_governance_policy, AccessControlPolicy, AccessControlRule, GovernancePolicy,
    PolicyOwner, RedactionPolicy, RetentionPolicy, RetentionRule,
};

fn valid_policy() -> GovernancePolicy {
    GovernancePolicy {
        policy_id: "policy-2026-03".to_string(),
        owners: vec![PolicyOwner {
            domain: "migration_ops".to_string(),
            owner: "Platform Team".to_string(),
            contact: "platform@example.com".to_string(),
        }],
        retention: RetentionPolicy {
            rules: vec![
                RetentionRule {
                    artifact: "decision_journal".to_string(),
                    min_days: 365,
                },
                RetentionRule {
                    artifact: "decision_index".to_string(),
                    min_days: 365,
                },
                RetentionRule {
                    artifact: "audit_artifact".to_string(),
                    min_days: 365,
                },
                RetentionRule {
                    artifact: "verification_artifact".to_string(),
                    min_days: 365,
                },
            ],
        },
        access_control: AccessControlPolicy {
            rules: vec![
                AccessControlRule {
                    artifact: "decision_journal".to_string(),
                    allowed_roles: vec!["migration_admin".to_string()],
                    break_glass_role: Some("security_ops".to_string()),
                },
                AccessControlRule {
                    artifact: "decision_index".to_string(),
                    allowed_roles: vec!["migration_admin".to_string()],
                    break_glass_role: Some("security_ops".to_string()),
                },
                AccessControlRule {
                    artifact: "audit_artifact".to_string(),
                    allowed_roles: vec!["audit_reader".to_string()],
                    break_glass_role: Some("security_ops".to_string()),
                },
                AccessControlRule {
                    artifact: "verification_artifact".to_string(),
                    allowed_roles: vec!["release_manager".to_string()],
                    break_glass_role: Some("security_ops".to_string()),
                },
            ],
        },
        redaction: RedactionPolicy {
            allowed_redacted_fields: vec!["canonical_decision_input.pii".to_string()],
            forbidden_redacted_fields: vec!["fingerprint".to_string(), "decision_id".to_string()],
        },
    }
}

#[test]
fn governance_policy_compliance_passes_for_valid_policy() {
    let report = evaluate_governance_policy(&valid_policy());
    assert!(report.compliant);
    assert_eq!(report.blocking_issue_count, 0);
}

#[test]
fn governance_policy_flags_missing_retention_and_access_rules() {
    let mut policy = valid_policy();
    policy.retention.rules.clear();
    policy.access_control.rules.clear();

    let report = evaluate_governance_policy(&policy);
    assert!(!report.compliant);
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.code == "retention_rule_missing"));
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.code == "access_rule_missing"));
}
