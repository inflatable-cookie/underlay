use chrono::Utc;

use crate::{
    recovery_advisories_from_run, recovery_advisory_from_error, AssetsStageOutput,
    DecideStageOutput, DecisionGovernanceIssue, ExtractStageOutput, FailureClass,
    MaterializeStageOutput, MigrationError, NormalizeStageOutput, PipelineRunReport,
    ResumeDiagnostics, TransformStageOutput, VerifyStageOutput,
};

#[test]
fn recovery_advisory_from_stage_failure_contains_class_and_hint() {
    let advisory = recovery_advisory_from_error(&MigrationError::StageFailure {
        stage: "decide".to_string(),
        message: "resolver offline".to_string(),
        failure_class: FailureClass::RetrySafe,
        recovery_hint: "retry after dependency check".to_string(),
    })
    .expect("advisory should exist");

    assert_eq!(advisory.failure_class, FailureClass::RetrySafe);
    assert!(advisory.action.contains("retry"));
}

#[test]
fn recovery_advisories_from_run_surface_unresolved_and_governance() {
    let report = PipelineRunReport {
        run_id: underlay_core::Uuid::new_v7(),
        started_at: Utc::now(),
        finished_at: Utc::now(),
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
        decide: DecideStageOutput {
            decisions: Vec::new(),
            decision_count: 0,
            unresolved_count: 1,
            reused_count: 0,
            resolved_count: 0,
            invalidated_count: 0,
            invalidations: Vec::new(),
            unresolved_queue: Vec::new(),
            governance_issues: vec![DecisionGovernanceIssue {
                artifact: "decision_journal".to_string(),
                fingerprint:
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        .to_string(),
                code: "invalid".to_string(),
                message: "bad".to_string(),
            }],
        },
        integrity_gate: crate::IntegrityGateResult::default(),
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
            passed: false,
            checks: Vec::new(),
            issues: Vec::new(),
        },
        resume_diagnostics: ResumeDiagnostics::default(),
    };

    let advisories = recovery_advisories_from_run(&report);
    assert_eq!(advisories.len(), 3);
}
