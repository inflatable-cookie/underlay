use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::{json, Value};

use crate::{
    DecisionFingerprintInput, DecisionJournalRecord, DecisionReusePolicy, MigrationContext,
    MigrationOrchestrator, PipelinePolicy, RunMetadata, RunStore,
};

use super::support::{
    decision_fingerprint_for, InMemoryRunStore, MockAssetResolver, MockDecisionResolver,
    MockPlugin, MockSource,
};

mod governance;
mod invalidation;
mod reuse;
mod unresolved;

fn counter() -> Arc<Mutex<u64>> {
    Arc::new(Mutex::new(0))
}

fn default_context() -> MigrationContext {
    MigrationContext::new(
        RunMetadata::new("plugin-v1", "schema-v1"),
        PipelinePolicy::default(),
    )
}

struct DecisionPipelineFixture {
    resolve_calls: Arc<Mutex<u64>>,
    store: InMemoryRunStore,
    orchestrator:
        MigrationOrchestrator<MockSource, MockPlugin, MockDecisionResolver, MockAssetResolver>,
}

fn decision_pipeline_fixture(invalidate_decisions: bool) -> DecisionPipelineFixture {
    let resolve_calls = counter();
    let source = MockSource { calls: counter() };
    let plugin = MockPlugin {
        fail_transform: false,
        semantic_error: false,
        invalidate_decisions,
        normalize_calls: counter(),
    };
    let resolver = MockDecisionResolver {
        resolve_calls: Arc::clone(&resolve_calls),
    };
    let assets = MockAssetResolver {};

    DecisionPipelineFixture {
        resolve_calls,
        store: InMemoryRunStore::default(),
        orchestrator: MigrationOrchestrator::new(source, plugin, resolver, assets),
    }
}

fn decision_fingerprint_for_id(id: u64) -> String {
    decision_fingerprint_for(&DecisionFingerprintInput {
        canonical_decision_input: json!({ "id": id }),
        decision_type: "migration_record_resolution".to_string(),
        resolver_version: "mock-resolver-v1".to_string(),
        prompt_version: "prompt-v1".to_string(),
        target_schema_version: "schema-v1".to_string(),
    })
}

fn decision_record(
    fingerprint: String,
    outcome: Value,
    confidence: Option<f64>,
    provenance: &str,
    created_at: chrono::DateTime<Utc>,
) -> DecisionJournalRecord {
    DecisionJournalRecord {
        decision_id: underlay_core::Uuid::new_v7(),
        fingerprint,
        decision_type: "migration_record_resolution".to_string(),
        outcome,
        confidence,
        resolver_version: "mock-resolver-v1".to_string(),
        prompt_version: "prompt-v1".to_string(),
        target_schema_version: "schema-v1".to_string(),
        created_at,
        provenance: provenance.to_string(),
    }
}

async fn seed_decision(store: &InMemoryRunStore, record: DecisionJournalRecord) {
    store
        .append_decision_journal(record)
        .await
        .expect("seed cached decision");
}
