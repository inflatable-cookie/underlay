use crate::DecisionInvalidationReason;

use super::*;

#[tokio::test]
async fn run_records_invalidation_reason_when_plugin_dependency_changes() {
    let fixture = decision_pipeline_fixture(true);

    seed_decision(
        &fixture.store,
        decision_record(
            decision_fingerprint_for_id(1),
            json!({ "id": 1 }),
            Some(0.97),
            "ai",
            Utc::now(),
        ),
    )
    .await;

    let report = fixture
        .orchestrator
        .run(
            &default_context(),
            &fixture.store,
            DecisionReusePolicy::Strict,
            "prompt-v1",
        )
        .await
        .expect("run should complete");

    assert_eq!(report.decide.invalidated_count, 1);
    assert_eq!(report.decide.invalidations.len(), 1);
    assert_eq!(
        report.decide.invalidations[0].reason,
        DecisionInvalidationReason::PluginDependencyChanged
    );
}
