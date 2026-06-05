use super::support::temp_dir;
use super::*;

#[test]
fn governance_policy_report_loads_and_formats_summary() {
    let dir = temp_dir("underlay_report_policy");
    let policy_path = dir.join("governance-policy.json");

    let policy = GovernancePolicy {
        policy_id: "migration-governance-v1".to_string(),
        owners: vec![PolicyOwner {
            domain: "migration".to_string(),
            owner: "platform-team".to_string(),
            contact: "platform@example.test".to_string(),
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
                    break_glass_role: Some("migration_break_glass".to_string()),
                },
                AccessControlRule {
                    artifact: "decision_index".to_string(),
                    allowed_roles: vec!["migration_admin".to_string()],
                    break_glass_role: Some("migration_break_glass".to_string()),
                },
                AccessControlRule {
                    artifact: "audit_artifact".to_string(),
                    allowed_roles: vec!["migration_auditor".to_string()],
                    break_glass_role: Some("migration_break_glass".to_string()),
                },
                AccessControlRule {
                    artifact: "verification_artifact".to_string(),
                    allowed_roles: vec!["migration_auditor".to_string()],
                    break_glass_role: Some("migration_break_glass".to_string()),
                },
            ],
        },
        redaction: RedactionPolicy {
            allowed_redacted_fields: vec!["email".to_string(), "full_name".to_string()],
            forbidden_redacted_fields: vec!["fingerprint".to_string()],
        },
    };

    std::fs::write(
        &policy_path,
        serde_json::to_vec_pretty(&policy).expect("serialize policy"),
    )
    .expect("write policy");

    let loaded = load_governance_policy(&policy_path).expect("load policy");
    let report = build_policy_report(&loaded);
    assert!(report.compliant);
    assert_eq!(report.blocking_issue_count, 0);

    let lines = format_policy_summary(&report);
    assert!(lines[0].contains("compliant=true"));
    assert!(lines[0].contains("blocking=0"));
}
