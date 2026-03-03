use chrono::Utc;

use crate::{
    build_audit_artifact, AssetsStageOutput, DecideStageOutput, ExtractStageOutput,
    IntegrityGateResult, MaterializeStageOutput, NormalizeStageOutput, PipelineRunReport,
    ResumeDiagnostics, TransformStageOutput, VerifyStageOutput,
};

#[test]
fn build_audit_artifact_emits_expected_critical_actions() {
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
            unresolved_count: 0,
            reused_count: 0,
            resolved_count: 0,
            invalidated_count: 0,
            invalidations: Vec::new(),
            unresolved_queue: Vec::new(),
            governance_issues: Vec::new(),
        },
        integrity_gate: IntegrityGateResult::default(),
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

    let artifact = build_audit_artifact(&report);
    assert_eq!(artifact.record_count, artifact.records.len());
    assert!(artifact
        .records
        .iter()
        .any(|r| matches!(r.action, crate::AuditAction::BundlePull)));
    assert!(artifact
        .records
        .iter()
        .any(|r| matches!(r.action, crate::AuditAction::PromotionCheck)));
}
