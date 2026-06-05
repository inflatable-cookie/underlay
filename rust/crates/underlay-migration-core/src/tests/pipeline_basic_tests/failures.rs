use crate::{FailureClass, MigrationError};

use super::*;

#[tokio::test]
async fn run_maps_transform_failure_to_stage_error() {
    let fixture = pipeline_fixture(true, false, false);
    let store = InMemoryRunStore::default();
    let ctx = default_context();

    let err = fixture
        .orchestrator
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
            assert_eq!(failure_class, FailureClass::RetrySafe);
        }
        other => panic!("expected stage failure, got {other:?}"),
    }

    let checkpoints = store.checkpoints.lock().expect("lock should succeed");
    assert_eq!(checkpoints.len(), 2);
    assert_eq!(checkpoints[0].stage, StageName::Extract);
    assert_eq!(checkpoints[1].stage, StageName::Normalize);
}

#[tokio::test]
async fn run_fails_verify_on_plugin_semantic_issue() {
    let fixture = pipeline_fixture(false, true, false);
    let store = InMemoryRunStore::default();
    let ctx = default_context();

    let err = fixture
        .orchestrator
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
