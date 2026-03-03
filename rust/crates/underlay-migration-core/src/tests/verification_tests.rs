use chrono::Utc;

use crate::{
    build_verification_artifact, AssetsStageOutput, DecideStageOutput, ExtractStageOutput,
    MaterializeStageOutput, NormalizeStageOutput, PipelineRunReport, ResumeDiagnostics,
    TransformStageOutput, VerificationIssue, VerificationSeverity, VerifyStageOutput,
};

#[test]
fn build_verification_artifact_sets_promotion_blockers() {
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
            record_count: 2,
        },
        decide: DecideStageOutput {
            decisions: Vec::new(),
            decision_count: 1,
            unresolved_count: 0,
            reused_count: 0,
            resolved_count: 0,
            invalidated_count: 0,
            invalidations: Vec::new(),
            unresolved_queue: Vec::new(),
            governance_issues: Vec::new(),
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
            issues: vec![VerificationIssue {
                code: "referential_integrity".to_string(),
                message: "orphaned relation".to_string(),
                severity: VerificationSeverity::Error,
            }],
        },
        resume_diagnostics: ResumeDiagnostics::default(),
    };

    let artifact = build_verification_artifact(&report).expect("artifact should build");
    assert!(!artifact.promotion_gate.can_promote);
    assert!(artifact
        .promotion_gate
        .blockers
        .contains(&"verify_stage_failed".to_string()));
    assert!(artifact
        .promotion_gate
        .blockers
        .contains(&"decision_coverage_mismatch".to_string()));
    assert!(!artifact.referential_integrity.passed);
}
