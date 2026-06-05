use super::*;

#[test]
fn format_decision_invalidation_report_summarizes_by_reason() {
    let decide = DecideStageOutput {
        decisions: vec![DecisionOutcome {
            fingerprint: "sha256:1111111111111111111111111111111111111111111111111111111111111111"
                .to_string(),
            outcome: serde_json::json!({"ok": true}),
            confidence: Some(0.99),
            provenance: DecisionProvenance::Ai,
        }],
        decision_count: 1,
        unresolved_count: 0,
        reused_count: 0,
        resolved_count: 1,
        invalidated_count: 2,
        invalidations: vec![
            DecisionInvalidationEvent {
                fingerprint:
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        .to_string(),
                reason: DecisionInvalidationReason::PromptVersionMismatch,
                decision_type: "migration_record_resolution".to_string(),
            },
            DecisionInvalidationEvent {
                fingerprint:
                    "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                        .to_string(),
                reason: DecisionInvalidationReason::PromptVersionMismatch,
                decision_type: "migration_record_resolution".to_string(),
            },
        ],
        unresolved_queue: Vec::new(),
        governance_issues: Vec::new(),
    };

    let lines = format_decision_invalidation_report(&decide);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], "prompt_version_mismatch: 2");
}

#[test]
fn governance_report_formats_and_lists_top_issues() {
    let decide = DecideStageOutput {
        decisions: Vec::new(),
        decision_count: 0,
        unresolved_count: 0,
        reused_count: 0,
        resolved_count: 0,
        invalidated_count: 0,
        invalidations: Vec::new(),
        unresolved_queue: Vec::new(),
        governance_issues: vec![
            DecisionGovernanceIssue {
                artifact: "decision_journal".to_string(),
                fingerprint:
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        .to_string(),
                code: "decision_journal_invalid_record".to_string(),
                message: "bad record".to_string(),
            },
            DecisionGovernanceIssue {
                artifact: "decision_journal".to_string(),
                fingerprint:
                    "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                        .to_string(),
                code: "decision_journal_invalid_record".to_string(),
                message: "another bad record".to_string(),
            },
        ],
    };

    let lines = format_decision_governance_report(&decide);
    assert_eq!(lines.len(), 1);
    assert_eq!(
        lines[0],
        "decision_journal.decision_journal_invalid_record: 2"
    );

    let top = top_governance_issues(&decide, 1);
    assert_eq!(top.len(), 1);
    assert_eq!(top[0].artifact, "decision_journal");
}
