use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;

use crate::{
    DecisionReusePolicy, LegacyRecordBatch, MigrationContext, MigrationError,
    MigrationOrchestrator, PipelinePolicy, RunMetadata, RunStore, StageCheckpoint, StageName,
    StageSnapshot,
};

use super::support::{
    InMemoryRunStore, MockAssetResolver, MockDecisionResolver, MockPlugin, MockSource,
};

#[test]
fn stage_order_is_stable() {
    let expected = [
        StageName::Extract,
        StageName::Normalize,
        StageName::Transform,
        StageName::Decide,
        StageName::Materialize,
        StageName::Assets,
        StageName::Verify,
    ];

    assert_eq!(
        MigrationOrchestrator::<(), (), (), ()>::stage_order(),
        expected
    );
}

#[test]
fn reuse_policy_has_strict_default_choice_available() {
    assert_eq!(DecisionReusePolicy::Strict, DecisionReusePolicy::Strict);
}

#[tokio::test]
async fn run_executes_all_stages_and_returns_report() {
    let source_calls = Arc::new(Mutex::new(0u64));
    let normalize_calls = Arc::new(Mutex::new(0u64));
    let resolve_calls = Arc::new(Mutex::new(0u64));

    let source = MockSource {
        calls: Arc::clone(&source_calls),
    };
    let plugin = MockPlugin {
        fail_transform: false,
        semantic_error: false,
        invalidate_decisions: false,
        normalize_calls: Arc::clone(&normalize_calls),
    };
    let resolver = MockDecisionResolver {
        resolve_calls: Arc::clone(&resolve_calls),
    };
    let assets = MockAssetResolver {};
    let store = InMemoryRunStore::default();

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);
    let ctx = MigrationContext::new(
        RunMetadata::new("plugin-v1", "schema-v1"),
        PipelinePolicy::default(),
    );

    let report = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect("run should succeed");

    assert_eq!(report.extract.batch_count, 1);
    assert_eq!(report.extract.record_count, 2);
    assert_eq!(report.transform.record_count, 2);
    assert_eq!(report.decide.decision_count, 2);
    assert_eq!(report.decide.reused_count, 0);
    assert_eq!(report.decide.resolved_count, 2);
    assert_eq!(report.decide.invalidated_count, 0);
    assert!(report.decide.invalidations.is_empty());
    assert!(report.decide.unresolved_queue.is_empty());
    assert!(!report.resume_diagnostics.resume_attempted);
    assert_eq!(report.resume_diagnostics.status, "fresh_start");
    assert_eq!(report.materialize.inserted, 2);
    assert_eq!(report.assets.resolved_count, 2);
    assert!(report.verify.passed);
    assert!(report
        .verify
        .checks
        .iter()
        .any(|c| c.check == "decision_coverage"));

    let source_count = *source_calls.lock().expect("lock should succeed");
    let normalize_count = *normalize_calls.lock().expect("lock should succeed");
    let resolve_count = *resolve_calls.lock().expect("lock should succeed");
    assert_eq!(source_count, 1);
    assert_eq!(normalize_count, 1);
    assert_eq!(resolve_count, 2);
    assert_eq!(
        store
            .decision_journal
            .lock()
            .expect("lock should succeed")
            .len(),
        2
    );

    let checkpoints = store
        .checkpoints
        .lock()
        .expect("lock should succeed")
        .clone();
    let stages: Vec<StageName> = checkpoints.into_iter().map(|c| c.stage).collect();
    assert_eq!(
        stages,
        vec![
            StageName::Extract,
            StageName::Normalize,
            StageName::Transform,
            StageName::Decide,
            StageName::Materialize,
            StageName::Assets,
            StageName::Verify,
        ]
    );
}

#[tokio::test]
async fn run_maps_transform_failure_to_stage_error() {
    let source = MockSource {
        calls: Arc::new(Mutex::new(0u64)),
    };
    let plugin = MockPlugin {
        fail_transform: true,
        semantic_error: false,
        invalidate_decisions: false,
        normalize_calls: Arc::new(Mutex::new(0u64)),
    };
    let resolver = MockDecisionResolver {
        resolve_calls: Arc::new(Mutex::new(0u64)),
    };
    let assets = MockAssetResolver {};
    let store = InMemoryRunStore::default();

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);
    let ctx = MigrationContext::new(
        RunMetadata::new("plugin-v1", "schema-v1"),
        PipelinePolicy::default(),
    );

    let err = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect_err("run should fail at transform");

    match err {
        MigrationError::StageFailure {
            stage,
            message,
            failure_class,
            ..
        } => {
            assert_eq!(stage, "transform");
            assert!(message.contains("transform failed"));
            assert_eq!(failure_class, crate::FailureClass::RetrySafe);
        }
        other => panic!("expected stage failure, got {other:?}"),
    }

    let checkpoints = store.checkpoints.lock().expect("lock should succeed");
    assert_eq!(checkpoints.len(), 2);
    assert_eq!(checkpoints[0].stage, StageName::Extract);
    assert_eq!(checkpoints[1].stage, StageName::Normalize);
}

#[tokio::test]
async fn run_resumes_from_completed_normalize_stage() {
    let source_calls = Arc::new(Mutex::new(0u64));
    let normalize_calls = Arc::new(Mutex::new(0u64));

    let source = MockSource {
        calls: Arc::clone(&source_calls),
    };
    let plugin = MockPlugin {
        fail_transform: false,
        semantic_error: false,
        invalidate_decisions: false,
        normalize_calls: Arc::clone(&normalize_calls),
    };
    let resolver = MockDecisionResolver {
        resolve_calls: Arc::new(Mutex::new(0u64)),
    };
    let assets = MockAssetResolver {};
    let store = InMemoryRunStore::default();

    let run = RunMetadata::new("plugin-v1", "schema-v1");
    let ctx = MigrationContext::new(run.clone(), PipelinePolicy::default());

    let extract_output = crate::ExtractStageOutput {
        batches: vec![LegacyRecordBatch {
            source_table: "legacy.users".to_string(),
            records: vec![json!({ "id": 1 }), json!({ "id": 2 })],
        }],
        batch_count: 1,
        record_count: 2,
    };
    let normalize_output = crate::NormalizeStageOutput {
        batches: vec![crate::NormalizedBatch {
            records: vec![json!({ "id": 1 }), json!({ "id": 2 })],
        }],
        batch_count: 1,
        record_count: 2,
    };

    store
        .write_stage_snapshot(StageSnapshot {
            run_id: run.run_id,
            stage: StageName::Extract,
            payload: serde_json::to_value(extract_output).expect("serialize extract"),
            recorded_at: Utc::now(),
        })
        .await
        .expect("write extract snapshot");
    store
        .write_stage_snapshot(StageSnapshot {
            run_id: run.run_id,
            stage: StageName::Normalize,
            payload: serde_json::to_value(normalize_output).expect("serialize normalize"),
            recorded_at: Utc::now(),
        })
        .await
        .expect("write normalize snapshot");
    store
        .write_stage_checkpoint(StageCheckpoint {
            run_id: run.run_id,
            stage: StageName::Normalize,
            plugin_version: run.plugin_version.clone(),
            target_schema_version: run.target_schema_version.clone(),
            cursor: None,
            completed_at: Utc::now(),
        })
        .await
        .expect("write checkpoint");

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);
    let report = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect("resume run should succeed");

    assert_eq!(report.transform.record_count, 2);
    assert!(report.resume_diagnostics.resume_attempted);
    assert_eq!(report.resume_diagnostics.status, "resumed");
    assert_eq!(
        report.resume_diagnostics.resumed_from_stage,
        Some(StageName::Normalize)
    );

    let source_count = *source_calls.lock().expect("lock should succeed");
    let normalize_count = *normalize_calls.lock().expect("lock should succeed");
    assert_eq!(source_count, 0);
    assert_eq!(normalize_count, 0);
}

#[tokio::test]
async fn run_rejects_incompatible_resume_checkpoint() {
    let source_calls = Arc::new(Mutex::new(0u64));
    let source = MockSource {
        calls: Arc::clone(&source_calls),
    };
    let plugin = MockPlugin {
        fail_transform: false,
        semantic_error: false,
        invalidate_decisions: false,
        normalize_calls: Arc::new(Mutex::new(0u64)),
    };
    let resolver = MockDecisionResolver {
        resolve_calls: Arc::new(Mutex::new(0u64)),
    };
    let assets = MockAssetResolver {};
    let store = InMemoryRunStore::default();

    let run = RunMetadata::new("plugin-v1", "schema-v1");
    let ctx = MigrationContext::new(run.clone(), PipelinePolicy::default());

    store
        .write_stage_checkpoint(StageCheckpoint {
            run_id: run.run_id,
            stage: StageName::Normalize,
            plugin_version: "plugin-v0".to_string(),
            target_schema_version: run.target_schema_version.clone(),
            cursor: None,
            completed_at: Utc::now(),
        })
        .await
        .expect("write checkpoint");

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);

    let err = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect_err("run should reject incompatible resume checkpoint");

    match err {
        MigrationError::ResumeCompatibility { code, message } => {
            assert_eq!(code, "plugin_version_mismatch");
            assert!(message.contains("plugin_version mismatch"));
        }
        other => panic!("expected determinism violation, got {other:?}"),
    }

    let source_count = *source_calls.lock().expect("lock should succeed");
    assert_eq!(source_count, 0);
}

#[tokio::test]
async fn run_fails_verify_on_plugin_semantic_issue() {
    let source = MockSource {
        calls: Arc::new(Mutex::new(0u64)),
    };
    let plugin = MockPlugin {
        fail_transform: false,
        semantic_error: true,
        invalidate_decisions: false,
        normalize_calls: Arc::new(Mutex::new(0u64)),
    };
    let resolver = MockDecisionResolver {
        resolve_calls: Arc::new(Mutex::new(0u64)),
    };
    let assets = MockAssetResolver {};
    let store = InMemoryRunStore::default();

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);
    let ctx = MigrationContext::new(
        RunMetadata::new("plugin-v1", "schema-v1"),
        PipelinePolicy::default(),
    );

    let err = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect_err("run should fail in verify stage");

    match err {
        MigrationError::StageFailure { stage, message, .. } => {
            assert_eq!(stage, "verify");
            assert!(message.contains("plugin_semantic_error"));
        }
        other => panic!("expected verify stage failure, got {other:?}"),
    }
}
