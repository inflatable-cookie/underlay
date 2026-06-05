use std::sync::{Arc, Mutex};

use crate::{
    DecisionReusePolicy, MigrationContext, MigrationOrchestrator, PipelinePolicy, RunMetadata,
    StageName,
};

use super::support::{
    InMemoryRunStore, MockAssetResolver, MockDecisionResolver, MockPlugin, MockSource,
};

mod failures;
mod full_run;
mod resume;

fn counter() -> Arc<Mutex<u64>> {
    Arc::new(Mutex::new(0))
}

fn counter_value(counter: &Arc<Mutex<u64>>) -> u64 {
    *counter.lock().expect("lock should succeed")
}

fn default_context() -> MigrationContext {
    MigrationContext::new(
        RunMetadata::new("plugin-v1", "schema-v1"),
        PipelinePolicy::default(),
    )
}

struct PipelineFixture {
    source_calls: Arc<Mutex<u64>>,
    normalize_calls: Arc<Mutex<u64>>,
    resolve_calls: Arc<Mutex<u64>>,
    orchestrator:
        MigrationOrchestrator<MockSource, MockPlugin, MockDecisionResolver, MockAssetResolver>,
}

fn pipeline_fixture(
    fail_transform: bool,
    semantic_error: bool,
    invalidate_decisions: bool,
) -> PipelineFixture {
    let source_calls = counter();
    let normalize_calls = counter();
    let resolve_calls = counter();

    let source = MockSource {
        calls: Arc::clone(&source_calls),
    };
    let plugin = MockPlugin {
        fail_transform,
        semantic_error,
        invalidate_decisions,
        normalize_calls: Arc::clone(&normalize_calls),
    };
    let resolver = MockDecisionResolver {
        resolve_calls: Arc::clone(&resolve_calls),
    };
    let assets = MockAssetResolver {};

    PipelineFixture {
        source_calls,
        normalize_calls,
        resolve_calls,
        orchestrator: MigrationOrchestrator::new(source, plugin, resolver, assets),
    }
}

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
