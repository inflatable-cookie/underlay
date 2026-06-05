use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;

use crate::{
    DecisionFingerprintInput, DecisionInvalidationReason, DecisionJournalRecord,
    DecisionProvenance, DecisionReusePolicy, MigrationContext, MigrationError,
    MigrationOrchestrator, PipelinePolicy, RunMetadata, RunStore,
};

use super::support::{
    decision_fingerprint_for, InMemoryRunStore, MockAssetResolver, MockDecisionResolver,
    MockPlugin, MockSource,
};
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

    let mut policy = PipelinePolicy {
        fail_on_unresolved_decisions: false,
        ..Default::default()
    };
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
