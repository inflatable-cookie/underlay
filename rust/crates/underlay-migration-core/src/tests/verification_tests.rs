use std::io;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::Utc;
use serde_json::json;

use crate::{
    benchmark_verification_paths, build_verification_artifact, evaluate_verification_rules,
    standard_verification_rules, AssetsStageOutput, CountExpectation, DecideStageOutput,
    DecisionReusePolicy, ExtractStageOutput, LegacyRecordBatch, MaterializeResult,
    MaterializeStageOutput, MigrationContext, MigrationPlugin, NormalizeStageOutput,
    PipelinePolicy, PipelineRunReport, ResumeDiagnostics, RunMetadata, TransformBatch,
    TransformStageOutput, VerificationInput, VerificationIssue, VerificationMetric,
    VerificationRule, VerificationRuleKind, VerificationSeverity, VerifyStageOutput,
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

#[test]
fn evaluate_verification_rules_reports_readable_failures() {
    let input = VerificationInput {
        transform_record_count: 2,
        transform_records: vec![
            json!({ "id": 1, "parent_id": null }),
            json!({ "id": 1, "parent_id": 99 }),
        ],
        decision_count: 2,
        unresolved_decision_count: 0,
        decision_governance_issue_count: 0,
        transform_checksum: "sha256:test".to_string(),
        materialize: MaterializeStageOutput {
            inserted: 2,
            updated: 0,
            skipped: 0,
        },
        assets: AssetsStageOutput {
            resolved_count: 0,
            unresolved_count: 0,
        },
        rules: vec![
            standard_verification_rules::unique("id"),
            standard_verification_rules::not_null("parent_id"),
            standard_verification_rules::referential_integrity("parent_id", "id"),
            VerificationRule {
                name: "row_count_warning".to_string(),
                description: Some("row count should stay below warning threshold".to_string()),
                severity: VerificationSeverity::Warning,
                kind: VerificationRuleKind::RowCount {
                    metric: VerificationMetric::TransformRecordCount,
                    expectation: CountExpectation::Maximum(1),
                },
            },
        ],
    };

    let result = evaluate_verification_rules(&input);
    assert_eq!(result.checks.len(), 4);
    assert_eq!(result.issues.len(), 4);
    assert!(result
        .issues
        .iter()
        .any(|issue| issue.message.contains("duplicate values")));
    assert!(result
        .issues
        .iter()
        .any(|issue| issue.message.contains("null or missing")));
    assert!(result
        .issues
        .iter()
        .any(|issue| issue.message.contains("could not resolve")));
    assert!(result
        .issues
        .iter()
        .any(|issue| issue.severity == VerificationSeverity::Warning));
}

#[tokio::test]
async fn benchmark_verification_paths_measures_declarative_and_plugin_checks() {
    #[derive(Debug)]
    struct BenchmarkPlugin {
        calls: Arc<Mutex<usize>>,
    }

    #[async_trait]
    impl MigrationPlugin for BenchmarkPlugin {
        type Error = io::Error;

        fn plugin_version(&self) -> &str {
            "benchmark-plugin-v1"
        }

        async fn normalize(
            &self,
            _ctx: &MigrationContext,
            batch: LegacyRecordBatch,
        ) -> Result<crate::NormalizedBatch, Self::Error> {
            Ok(crate::NormalizedBatch {
                records: batch.records,
            })
        }

        async fn transform(
            &self,
            _ctx: &MigrationContext,
            batch: crate::NormalizedBatch,
        ) -> Result<TransformBatch, Self::Error> {
            Ok(TransformBatch {
                records: batch.records,
            })
        }

        async fn materialize(
            &self,
            _ctx: &MigrationContext,
            batch: TransformBatch,
        ) -> Result<MaterializeResult, Self::Error> {
            Ok(MaterializeResult {
                inserted: batch.records.len() as u64,
                updated: 0,
                skipped: 0,
            })
        }

        async fn verify_semantics(
            &self,
            _ctx: &MigrationContext,
            _input: &VerificationInput,
        ) -> Result<Vec<VerificationIssue>, Self::Error> {
            let mut calls = self.calls.lock().expect("lock should succeed");
            *calls += 1;
            Ok(Vec::new())
        }

        fn should_invalidate_decision(
            &self,
            _ctx: &MigrationContext,
            _fingerprint: &str,
            _reuse_policy: DecisionReusePolicy,
        ) -> bool {
            false
        }
    }

    let plugin = BenchmarkPlugin {
        calls: Arc::new(Mutex::new(0)),
    };
    let ctx = MigrationContext::new(
        RunMetadata::new("benchmark-plugin-v1", "schema-v1"),
        PipelinePolicy::default(),
    );
    let input = VerificationInput {
        transform_record_count: 2,
        transform_records: vec![
            json!({ "id": 1, "email": "a@example.com" }),
            json!({ "id": 2, "email": "b@example.com" }),
        ],
        decision_count: 2,
        unresolved_decision_count: 0,
        decision_governance_issue_count: 0,
        transform_checksum: "sha256:test".to_string(),
        materialize: MaterializeStageOutput {
            inserted: 2,
            updated: 0,
            skipped: 0,
        },
        assets: AssetsStageOutput {
            resolved_count: 0,
            unresolved_count: 0,
        },
        rules: vec![
            standard_verification_rules::unique("id"),
            standard_verification_rules::not_null("email"),
            standard_verification_rules::row_count_min(VerificationMetric::TransformRecordCount, 2),
        ],
    };

    let benchmark = benchmark_verification_paths(&plugin, &ctx, &input, 5)
        .await
        .expect("benchmark should succeed");

    assert_eq!(benchmark.iterations, 5);
    assert_eq!(*plugin.calls.lock().expect("lock should succeed"), 5);
}
