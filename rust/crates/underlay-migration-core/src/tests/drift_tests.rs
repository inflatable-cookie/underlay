use chrono::Utc;

use crate::{
    detect_drift_from_run, detect_drift_with_lineage, AssetsStageOutput, DecideStageOutput,
    DecisionIndex, DecisionIndexEntry, DriftSeverity, DriftThresholds, ExtractStageOutput,
    MaterializeStageOutput, NormalizeStageOutput, PipelineRunReport, TransformStageOutput,
    VerifyStageOutput,
};
use std::collections::BTreeMap;

#[test]
fn detect_drift_flags_unresolved_and_governance_issues() {
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
            unresolved_count: 2,
            reused_count: 0,
            resolved_count: 0,
            invalidated_count: 0,
            invalidations: Vec::new(),
            unresolved_queue: Vec::new(),
            governance_issues: vec![crate::DecisionGovernanceIssue {
                artifact: "decision_journal".to_string(),
                fingerprint:
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        .to_string(),
                code: "invalid".to_string(),
                message: "bad record".to_string(),
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
        resume_diagnostics: crate::ResumeDiagnostics::default(),
    };

    let drift = detect_drift_from_run(
        &report,
        &DriftThresholds {
            max_unresolved_decisions: 0,
            max_governance_issues: 0,
            max_lineage_mismatches: 0,
            require_verify_passed: true,
        },
    );

    assert_eq!(drift.issue_count, 3);
    assert_eq!(drift.blocking_issue_count, 3);
    assert!(drift
        .issues
        .iter()
        .all(|issue| issue.severity == DriftSeverity::Error));
    assert_eq!(drift.category_summaries.len(), 3);
}

#[test]
fn detect_drift_with_lineage_flags_index_to_journal_mismatch() {
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
            passed: true,
            checks: Vec::new(),
            issues: Vec::new(),
        },
        resume_diagnostics: crate::ResumeDiagnostics::default(),
    };

    let fingerprint =
        "sha256:1111111111111111111111111111111111111111111111111111111111111111".to_string();
    let mut entries = BTreeMap::new();
    entries.insert(
        fingerprint.clone(),
        DecisionIndexEntry {
            bundle_digest:
                "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    .to_string(),
            decision_id: underlay_core::Uuid::new_v7(),
            created_at: Utc::now(),
        },
    );
    let lineage = crate::DecisionLineageInput {
        index: DecisionIndex {
            schema_version: "1".to_string(),
            bundle_digest:
                "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    .to_string(),
            entries,
        },
        journal_records: Vec::new(),
        expected_bundle_digest: Some(
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        ),
    };

    let drift = detect_drift_with_lineage(
        &report,
        &DriftThresholds {
            max_unresolved_decisions: 0,
            max_governance_issues: 0,
            max_lineage_mismatches: 0,
            require_verify_passed: true,
        },
        Some(&lineage),
    );

    assert!(drift
        .issues
        .iter()
        .any(|issue| issue.code == "index_fingerprint_missing_in_journal"));
    assert!(drift
        .issues
        .iter()
        .any(|issue| issue.code == "lineage_mismatches_exceed_threshold"));
}
