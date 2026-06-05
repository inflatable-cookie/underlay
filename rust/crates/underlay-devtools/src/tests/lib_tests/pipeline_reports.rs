use super::support::temp_dir;
use super::*;

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
