use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::Utc;
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{
    AssetResolution, AssetResolver, DecisionFingerprintInput, DecisionInvalidationReason,
    DecisionJournalRecord, DecisionOutcome, DecisionProvenance, DecisionResolver,
    DecisionReusePolicy, IntegrityRunScope, LegacyRecordBatch, LegacySource, MaterializeResult,
    MigrationContext, MigrationError, MigrationOrchestrator, MigrationPlugin, PipelinePolicy,
    ResumeCheckpoint, RunMetadata, RunStore, RunSummary, SignatureEnforcementPhase,
    StageCheckpoint, StageName, StageSnapshot, UnresolvedDecisionRecord, VerificationInput,
    VerificationIssue, VerificationSeverity,
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

fn decision_fingerprint_for(input: &DecisionFingerprintInput) -> String {
    let canonical = serde_json::to_string(&input.canonical_decision_input)
        .expect("canonical json should encode");
    let raw = format!(
        "canonical_decision_input={canonical}\ndecision_type={}\nresolver_version={}\nprompt_version={}\ntarget_schema_version={}",
        input.decision_type, input.resolver_version, input.prompt_version, input.target_schema_version
    );
    let digest = Sha256::digest(raw.as_bytes());
    format!("sha256:{digest:x}")
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

#[tokio::test]
async fn run_reuses_cached_decisions_and_skips_new_journal_entries() {
    let source = MockSource {
        calls: Arc::new(Mutex::new(0u64)),
    };
    let plugin = MockPlugin {
        fail_transform: false,
        semantic_error: false,
        invalidate_decisions: false,
        normalize_calls: Arc::new(Mutex::new(0u64)),
    };
    let resolve_calls = Arc::new(Mutex::new(0u64));
    let resolver = MockDecisionResolver {
        resolve_calls: Arc::clone(&resolve_calls),
    };
    let assets = MockAssetResolver {};
    let store = InMemoryRunStore::default();

    let run = RunMetadata::new("plugin-v1", "schema-v1");
    let ctx = MigrationContext::new(run, PipelinePolicy::default());
    let cached_record = DecisionJournalRecord {
        decision_id: underlay_core::Uuid::new_v7(),
        fingerprint: decision_fingerprint_for(&DecisionFingerprintInput {
            canonical_decision_input: json!({ "id": 1 }),
            decision_type: "migration_record_resolution".to_string(),
            resolver_version: "mock-resolver-v1".to_string(),
            prompt_version: "prompt-v1".to_string(),
            target_schema_version: "schema-v1".to_string(),
        }),
        decision_type: "migration_record_resolution".to_string(),
        outcome: json!({ "id": 1 }),
        confidence: Some(0.97),
        resolver_version: "mock-resolver-v1".to_string(),
        prompt_version: "prompt-v1".to_string(),
        target_schema_version: "schema-v1".to_string(),
        created_at: Utc::now(),
        provenance: "human".to_string(),
    };
    store
        .append_decision_journal(cached_record)
        .await
        .expect("seed cached decision");

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);
    let report = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect("run should reuse cached decisions");

    assert_eq!(report.decide.reused_count, 1);
    assert_eq!(report.decide.resolved_count, 1);
    assert_eq!(report.decide.invalidated_count, 0);

    let resolve_count = *resolve_calls.lock().expect("lock should succeed");
    assert_eq!(resolve_count, 1);

    let journal_len = store
        .decision_journal
        .lock()
        .expect("lock should succeed")
        .len();
    assert_eq!(journal_len, 2);
}

#[tokio::test]
async fn run_records_invalidation_reason_when_plugin_dependency_changes() {
    let source = MockSource {
        calls: Arc::new(Mutex::new(0u64)),
    };
    let plugin = MockPlugin {
        fail_transform: false,
        semantic_error: false,
        invalidate_decisions: true,
        normalize_calls: Arc::new(Mutex::new(0u64)),
    };
    let resolver = MockDecisionResolver {
        resolve_calls: Arc::new(Mutex::new(0u64)),
    };
    let assets = MockAssetResolver {};
    let store = InMemoryRunStore::default();

    store
        .append_decision_journal(DecisionJournalRecord {
            decision_id: underlay_core::Uuid::new_v7(),
            fingerprint: decision_fingerprint_for(&DecisionFingerprintInput {
                canonical_decision_input: json!({ "id": 1 }),
                decision_type: "migration_record_resolution".to_string(),
                resolver_version: "mock-resolver-v1".to_string(),
                prompt_version: "prompt-v1".to_string(),
                target_schema_version: "schema-v1".to_string(),
            }),
            decision_type: "migration_record_resolution".to_string(),
            outcome: json!({ "id": 1 }),
            confidence: Some(0.97),
            resolver_version: "mock-resolver-v1".to_string(),
            prompt_version: "prompt-v1".to_string(),
            target_schema_version: "schema-v1".to_string(),
            created_at: Utc::now(),
            provenance: "ai".to_string(),
        })
        .await
        .expect("seed cached decision");

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);
    let ctx = MigrationContext::new(
        RunMetadata::new("plugin-v1", "schema-v1"),
        PipelinePolicy::default(),
    );
    let report = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect("run should complete");

    assert_eq!(report.decide.invalidated_count, 1);
    assert_eq!(report.decide.invalidations.len(), 1);
    assert_eq!(
        report.decide.invalidations[0].reason,
        DecisionInvalidationReason::PluginDependencyChanged
    );
}

#[tokio::test]
async fn run_prefers_human_override_in_provenance_chain() {
    let source = MockSource {
        calls: Arc::new(Mutex::new(0u64)),
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

    let fingerprint = decision_fingerprint_for(&DecisionFingerprintInput {
        canonical_decision_input: json!({ "id": 1 }),
        decision_type: "migration_record_resolution".to_string(),
        resolver_version: "mock-resolver-v1".to_string(),
        prompt_version: "prompt-v1".to_string(),
        target_schema_version: "schema-v1".to_string(),
    });

    store
        .append_decision_journal(DecisionJournalRecord {
            decision_id: underlay_core::Uuid::new_v7(),
            fingerprint: fingerprint.clone(),
            decision_type: "migration_record_resolution".to_string(),
            outcome: json!({ "winner": "human" }),
            confidence: Some(1.0),
            resolver_version: "mock-resolver-v1".to_string(),
            prompt_version: "prompt-v1".to_string(),
            target_schema_version: "schema-v1".to_string(),
            created_at: Utc::now() - chrono::Duration::seconds(60),
            provenance: "human".to_string(),
        })
        .await
        .expect("seed human decision");
    store
        .append_decision_journal(DecisionJournalRecord {
            decision_id: underlay_core::Uuid::new_v7(),
            fingerprint: fingerprint.clone(),
            decision_type: "migration_record_resolution".to_string(),
            outcome: json!({ "winner": "ai" }),
            confidence: Some(1.0),
            resolver_version: "mock-resolver-v1".to_string(),
            prompt_version: "prompt-v1".to_string(),
            target_schema_version: "schema-v1".to_string(),
            created_at: Utc::now(),
            provenance: "ai".to_string(),
        })
        .await
        .expect("seed ai decision");

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);
    let ctx = MigrationContext::new(
        RunMetadata::new("plugin-v1", "schema-v1"),
        PipelinePolicy::default(),
    );
    let report = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect("run should succeed");

    assert_eq!(report.decide.reused_count, 1);
    assert_eq!(
        report.decide.decisions[0].provenance,
        DecisionProvenance::Human
    );
    assert_eq!(
        report.decide.decisions[0].outcome,
        json!({ "winner": "human" })
    );
}

#[tokio::test]
async fn run_surfaces_governance_issue_for_invalid_cached_record() {
    let source = MockSource {
        calls: Arc::new(Mutex::new(0u64)),
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
    let fingerprint = decision_fingerprint_for(&DecisionFingerprintInput {
        canonical_decision_input: json!({ "id": 1 }),
        decision_type: "migration_record_resolution".to_string(),
        resolver_version: "mock-resolver-v1".to_string(),
        prompt_version: "prompt-v1".to_string(),
        target_schema_version: "schema-v1".to_string(),
    });

    store
        .append_decision_journal(DecisionJournalRecord {
            decision_id: underlay_core::Uuid::new_v7(),
            fingerprint,
            decision_type: "migration_record_resolution".to_string(),
            outcome: json!({ "id": 1 }),
            confidence: Some(0.99),
            resolver_version: "mock-resolver-v1".to_string(),
            prompt_version: "prompt-v1".to_string(),
            target_schema_version: "schema-v1".to_string(),
            created_at: Utc::now(),
            provenance: "robot".to_string(),
        })
        .await
        .expect("seed invalid decision");

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);
    let ctx = MigrationContext::new(
        RunMetadata::new("plugin-v1", "schema-v1"),
        PipelinePolicy::default(),
    );

    let err = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect_err("run should fail at verify on governance issue");

    match err {
        MigrationError::StageFailure { stage, message, .. } => {
            assert_eq!(stage, "verify");
            assert!(message.contains("decision_governance_integrity"));
        }
        other => panic!("expected verify failure, got {other:?}"),
    }
}

#[tokio::test]
async fn run_queues_low_confidence_ai_decisions_as_unresolved() {
    let source = MockSource {
        calls: Arc::new(Mutex::new(0u64)),
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

    let mut policy = PipelinePolicy::default();
    policy.fail_on_unresolved_decisions = false;
    policy
        .ai_threshold_policy
        .decision_type_overrides
        .insert("migration_record_resolution".to_string(), 0.995);

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);
    let ctx = MigrationContext::new(RunMetadata::new("plugin-v1", "schema-v1"), policy);

    let report = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect("run should succeed when unresolved queue is allowed");

    assert_eq!(report.decide.decision_count, 2);
    assert_eq!(report.decide.unresolved_count, 2);
    assert_eq!(report.decide.unresolved_queue.len(), 2);
    for unresolved in &report.decide.unresolved_queue {
        assert_eq!(unresolved.reason, "low_confidence_ai");
        assert_eq!(unresolved.threshold, 0.995);
    }

    assert_eq!(store.unresolved.lock().expect("lock").len(), 2);
}

#[tokio::test]
async fn run_fails_pre_apply_when_integrity_gate_blocked() {
    let source = MockSource {
        calls: Arc::new(Mutex::new(0u64)),
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

    let mut policy = PipelinePolicy::default();
    policy.integrity_evidence.digest_verified = false;

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);
    let ctx = MigrationContext::new(RunMetadata::new("plugin-v1", "schema-v1"), policy);
    let err = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect_err("run should fail before materialization");

    match err {
        MigrationError::StageFailure {
            stage,
            failure_class,
            message,
            ..
        } => {
            assert_eq!(stage, "materialize");
            assert_eq!(failure_class, crate::FailureClass::NonRetrySafe);
            assert!(message.contains("integrity gate failed"));
        }
        other => panic!("expected stage failure, got {other:?}"),
    }
}

#[tokio::test]
async fn run_fails_pre_apply_when_signature_rollout_enforces_preprod() {
    let source = MockSource {
        calls: Arc::new(Mutex::new(0u64)),
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

    let mut policy = PipelinePolicy::default();
    policy.integrity_policy.require_signature_verification = false;
    policy.integrity_policy.signature_enforcement_phase = SignatureEnforcementPhase::EnforcePreprod;
    policy.integrity_policy.run_scope = IntegrityRunScope::PreProduction;
    policy.integrity_evidence.signature_verified = None;

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);
    let ctx = MigrationContext::new(RunMetadata::new("plugin-v1", "schema-v1"), policy);
    let err = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect_err("run should fail when preprod signature enforcement is active");

    match err {
        MigrationError::StageFailure { stage, message, .. } => {
            assert_eq!(stage, "materialize");
            assert!(message.contains("signature_verification_required"));
        }
        other => panic!("expected stage failure, got {other:?}"),
    }
}

#[tokio::test]
async fn run_fails_pre_apply_when_signature_evidence_is_incomplete() {
    let source = MockSource {
        calls: Arc::new(Mutex::new(0u64)),
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

    let mut policy = PipelinePolicy::default();
    policy.integrity_policy.require_signature_verification = true;
    policy.integrity_evidence.signature_verified = Some(true);
    policy.integrity_evidence.signature_verified_at = None;
    policy.integrity_evidence.signer_identity = Some("".to_string());
    policy.integrity_evidence.signature_key_id = Some("".to_string());

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);
    let ctx = MigrationContext::new(RunMetadata::new("plugin-v1", "schema-v1"), policy);
    let err = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect_err("run should fail when signature evidence is incomplete");

    match err {
        MigrationError::StageFailure { stage, message, .. } => {
            assert_eq!(stage, "materialize");
            assert!(message.contains("signature_evidence_incomplete"));
        }
        other => panic!("expected stage failure, got {other:?}"),
    }
}

struct MockSource {
    calls: Arc<Mutex<u64>>,
}

#[async_trait]
impl LegacySource for MockSource {
    type Error = io::Error;

    async fn extract(
        &self,
        _ctx: &MigrationContext,
    ) -> Result<Vec<LegacyRecordBatch>, Self::Error> {
        let mut count = self
            .calls
            .lock()
            .map_err(|_| io::Error::other("lock poisoned"))?;
        *count += 1;

        Ok(vec![LegacyRecordBatch {
            source_table: "legacy.users".to_string(),
            records: vec![json!({ "id": 1 }), json!({ "id": 2 })],
        }])
    }
}

struct MockPlugin {
    fail_transform: bool,
    semantic_error: bool,
    invalidate_decisions: bool,
    normalize_calls: Arc<Mutex<u64>>,
}

#[async_trait]
impl MigrationPlugin for MockPlugin {
    type Error = io::Error;

    fn plugin_version(&self) -> &str {
        "mock-plugin-v1"
    }

    async fn normalize(
        &self,
        _ctx: &MigrationContext,
        batch: LegacyRecordBatch,
    ) -> Result<crate::NormalizedBatch, Self::Error> {
        let mut count = self
            .normalize_calls
            .lock()
            .map_err(|_| io::Error::other("lock poisoned"))?;
        *count += 1;

        Ok(crate::NormalizedBatch {
            records: batch.records,
        })
    }

    async fn transform(
        &self,
        _ctx: &MigrationContext,
        batch: crate::NormalizedBatch,
    ) -> Result<crate::TransformBatch, Self::Error> {
        if self.fail_transform {
            return Err(io::Error::other("transform failed"));
        }

        Ok(crate::TransformBatch {
            records: batch.records,
        })
    }

    async fn materialize(
        &self,
        _ctx: &MigrationContext,
        batch: crate::TransformBatch,
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
        if self.semantic_error {
            return Ok(vec![VerificationIssue {
                code: "plugin_semantic_error".to_string(),
                message: "plugin semantic verification failed".to_string(),
                severity: VerificationSeverity::Error,
            }]);
        }

        Ok(Vec::new())
    }

    fn should_invalidate_decision(
        &self,
        _ctx: &MigrationContext,
        _fingerprint: &str,
        _reuse_policy: DecisionReusePolicy,
    ) -> bool {
        self.invalidate_decisions
    }
}

struct MockDecisionResolver {
    resolve_calls: Arc<Mutex<u64>>,
}

#[async_trait]
impl DecisionResolver for MockDecisionResolver {
    type Error = io::Error;

    fn resolver_version(&self) -> &str {
        "mock-resolver-v1"
    }

    async fn fingerprint(
        &self,
        input: &DecisionFingerprintInput,
    ) -> crate::MigrationResult<String> {
        Ok(decision_fingerprint_for(input))
    }

    async fn resolve(
        &self,
        _ctx: &MigrationContext,
        input: DecisionFingerprintInput,
        _reuse_policy: DecisionReusePolicy,
    ) -> Result<DecisionOutcome, Self::Error> {
        let mut count = self
            .resolve_calls
            .lock()
            .map_err(|_| io::Error::other("lock poisoned"))?;
        *count += 1;

        let fingerprint = decision_fingerprint_for(&input);
        Ok(DecisionOutcome {
            fingerprint,
            outcome: input.canonical_decision_input,
            confidence: Some(0.99),
            provenance: DecisionProvenance::Ai,
        })
    }
}

struct MockAssetResolver {}

#[async_trait]
impl AssetResolver for MockAssetResolver {
    type Error = io::Error;

    async fn resolve_assets(
        &self,
        _ctx: &MigrationContext,
        batch: &crate::TransformBatch,
    ) -> Result<AssetResolution, Self::Error> {
        Ok(AssetResolution {
            resolved_count: batch.records.len() as u64,
            unresolved_count: 0,
        })
    }
}

#[derive(Clone, Default)]
struct InMemoryRunStore {
    checkpoints: Arc<Mutex<Vec<StageCheckpoint>>>,
    snapshots: Arc<Mutex<HashMap<(underlay_core::Uuid, StageName), StageSnapshot>>>,
    decision_journal: Arc<Mutex<Vec<DecisionJournalRecord>>>,
    unresolved: Arc<Mutex<Vec<UnresolvedDecisionRecord>>>,
}

#[async_trait]
impl RunStore for InMemoryRunStore {
    type Error = io::Error;

    async fn write_stage_checkpoint(&self, checkpoint: StageCheckpoint) -> Result<(), Self::Error> {
        self.checkpoints
            .lock()
            .map_err(|_| io::Error::other("poisoned checkpoint lock"))?
            .push(checkpoint);
        Ok(())
    }

    async fn latest_resume_checkpoint(
        &self,
        run_id: underlay_core::Uuid,
    ) -> Result<Option<ResumeCheckpoint>, Self::Error> {
        let checkpoints = self
            .checkpoints
            .lock()
            .map_err(|_| io::Error::other("poisoned checkpoint lock"))?;

        let latest = checkpoints
            .iter()
            .rev()
            .find(|c| c.run_id == run_id)
            .cloned();
        Ok(latest.map(|checkpoint| ResumeCheckpoint {
            run_id,
            last_completed_stage: checkpoint.stage,
            plugin_version: checkpoint.plugin_version,
            target_schema_version: checkpoint.target_schema_version,
            cursor: checkpoint.cursor,
        }))
    }

    async fn append_decision_journal(
        &self,
        record: DecisionJournalRecord,
    ) -> Result<(), Self::Error> {
        self.decision_journal
            .lock()
            .map_err(|_| io::Error::other("poisoned decision journal lock"))?
            .push(record);
        Ok(())
    }

    async fn write_summary(&self, _summary: RunSummary) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn latest_decision(
        &self,
        fingerprint: &str,
    ) -> Result<Option<DecisionJournalRecord>, Self::Error> {
        let latest = self
            .decision_journal
            .lock()
            .map_err(|_| io::Error::other("poisoned decision journal lock"))?
            .iter()
            .rev()
            .find(|entry| entry.fingerprint == fingerprint)
            .cloned();
        Ok(latest)
    }

    async fn decisions_for_fingerprint(
        &self,
        fingerprint: &str,
    ) -> Result<Vec<DecisionJournalRecord>, Self::Error> {
        let records = self
            .decision_journal
            .lock()
            .map_err(|_| io::Error::other("poisoned decision journal lock"))?
            .iter()
            .filter(|entry| entry.fingerprint == fingerprint)
            .cloned()
            .collect::<Vec<_>>();
        Ok(records)
    }

    async fn append_unresolved_decision(
        &self,
        record: UnresolvedDecisionRecord,
    ) -> Result<(), Self::Error> {
        self.unresolved
            .lock()
            .map_err(|_| io::Error::other("poisoned unresolved lock"))?
            .push(record);
        Ok(())
    }

    async fn write_stage_snapshot(&self, snapshot: StageSnapshot) -> Result<(), Self::Error> {
        self.snapshots
            .lock()
            .map_err(|_| io::Error::other("poisoned snapshot lock"))?
            .insert((snapshot.run_id, snapshot.stage), snapshot);
        Ok(())
    }

    async fn read_stage_snapshot(
        &self,
        run_id: underlay_core::Uuid,
        stage: StageName,
    ) -> Result<Option<StageSnapshot>, Self::Error> {
        Ok(self
            .snapshots
            .lock()
            .map_err(|_| io::Error::other("poisoned snapshot lock"))?
            .get(&(run_id, stage))
            .cloned())
    }
}
