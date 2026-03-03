use super::{
    build_audit_report, build_drift_report, build_integrity_report, build_policy_report,
    build_recovery_advisories, build_verification_report, format_audit_summary,
    format_decision_governance_report, format_decision_invalidation_report, format_drift_report,
    format_integrity_summary, format_policy_summary, format_recovery_advisories,
    format_verification_summary, load_decide_stage_output, load_governance_policy,
    load_pipeline_run_report, load_pipeline_run_report_from_path, require_env,
    top_governance_issues, write_audit_artifact, write_verification_artifact, DevtoolError,
};
use underlay_migration_core::{
    AccessControlPolicy, AccessControlRule, AssetsStageOutput, DecideStageOutput,
    DecisionGovernanceIssue, DecisionInvalidationEvent, DecisionInvalidationReason,
    DecisionOutcome, DecisionProvenance, DriftThresholds, ExtractStageOutput, GovernancePolicy,
    MaterializeStageOutput, NormalizeStageOutput, PipelineRunReport, PolicyOwner, RedactionPolicy,
    ResumeDiagnostics, RetentionPolicy, RetentionRule, TransformStageOutput, VerifyStageOutput,
};

#[test]
fn require_env_returns_value_when_present() {
    let value = require_env("PATH").expect("PATH should be present in test environment");
    assert!(!value.is_empty());
}

#[test]
fn require_env_returns_missing_error_when_absent() {
    let err = require_env("UNDERLAY_DEVTOOLS_TEST_MISSING_ENV_SHOULD_NOT_EXIST")
        .expect_err("missing env var should return an error");
    match err {
        DevtoolError::MissingEnvVar { name } => {
            assert_eq!(name, "UNDERLAY_DEVTOOLS_TEST_MISSING_ENV_SHOULD_NOT_EXIST")
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

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

#[test]
fn load_decide_stage_output_supports_decide_and_pipeline_shapes() {
    let dir = temp_dir("underlay_report_load");
    let decide_path = dir.join("decide.json");
    let pipeline_path = dir.join("pipeline.json");

    let decide = DecideStageOutput {
        decisions: Vec::new(),
        decision_count: 0,
        unresolved_count: 0,
        reused_count: 0,
        resolved_count: 0,
        invalidated_count: 0,
        invalidations: Vec::new(),
        unresolved_queue: Vec::new(),
        governance_issues: Vec::new(),
    };
    std::fs::write(
        &decide_path,
        serde_json::to_vec_pretty(&decide).expect("serialize decide"),
    )
    .expect("write decide");

    let loaded_decide = load_decide_stage_output(&decide_path).expect("load decide");
    assert_eq!(loaded_decide.decision_count, 0);

    let report = PipelineRunReport {
        run_id: underlay_core::Uuid::new_v7(),
        started_at: chrono::Utc::now(),
        finished_at: chrono::Utc::now(),
        extract: ExtractStageOutput {
            batches: Vec::new(),
            batch_count: 0,
            record_count: 0,
        },
        normalize: NormalizeStageOutput {
            batches: Vec::new(),
            batch_count: 0,
            record_count: 0,
        },
        transform: TransformStageOutput {
            batches: Vec::new(),
            batch_count: 0,
            record_count: 0,
        },
        decide,
        integrity_gate: underlay_migration_core::IntegrityGateResult::default(),
        materialize: MaterializeStageOutput {
            inserted: 0,
            updated: 0,
            skipped: 0,
        },
        assets: AssetsStageOutput {
            resolved_count: 0,
            unresolved_count: 0,
        },
        verify: VerifyStageOutput {
            passed: true,
            checks: Vec::new(),
            issues: Vec::new(),
        },
        resume_diagnostics: ResumeDiagnostics::default(),
    };
    std::fs::write(
        &pipeline_path,
        serde_json::to_vec_pretty(&report).expect("serialize report"),
    )
    .expect("write report");

    let loaded_from_report = load_decide_stage_output(&pipeline_path).expect("load report");
    assert_eq!(loaded_from_report.decision_count, 0);

    let loaded_report = load_pipeline_run_report(&pipeline_path).expect("load pipeline report");
    let drift = build_drift_report(
        &loaded_report,
        &DriftThresholds {
            max_unresolved_decisions: 0,
            max_governance_issues: 0,
            max_lineage_mismatches: 0,
            require_verify_passed: true,
        },
    );
    assert_eq!(drift.issue_count, 0);
    let drift_lines = format_drift_report(&drift);
    assert_eq!(drift_lines, vec!["no drift issues detected".to_string()]);

    let recovery = build_recovery_advisories(&loaded_report);
    let recovery_lines = format_recovery_advisories(&recovery);
    assert_eq!(
        recovery_lines,
        vec!["no recovery actions recommended".to_string()]
    );

    let verification = build_verification_report(&loaded_report).expect("build verification");
    let verification_lines = format_verification_summary(&verification);
    assert!(verification_lines
        .iter()
        .any(|line| line.contains("can_promote=true")));
    let written = write_verification_artifact(&dir, &verification).expect("write artifact");
    assert!(written.exists());

    let integrity = build_integrity_report(&loaded_report);
    let integrity_lines = format_integrity_summary(&integrity);
    assert!(integrity_lines[0].contains("passed=true"));
    assert!(integrity_lines[0].contains("signature_enforcement_phase=observe"));
    assert!(integrity_lines[0].contains("run_scope=demo"));
    assert!(integrity_lines[1].contains("signature_verified=None"));

    let loaded_from_dir = load_pipeline_run_report_from_path(&dir).expect("load from dir");
    assert_eq!(loaded_from_dir.run_id, loaded_report.run_id);

    let audit = build_audit_report(&loaded_report);
    let audit_lines = format_audit_summary(&audit);
    assert!(audit_lines[0].contains("record_count="));
    let audit_written = write_audit_artifact(&dir, &audit).expect("write audit artifact");
    assert!(audit_written.exists());
}

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

fn temp_dir(prefix: &str) -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time should be monotonic")
        .as_nanos();
    path.push(format!("{prefix}_{}_{}", std::process::id(), nanos));
    std::fs::create_dir_all(&path).expect("temp dir should be created");
    path
}
