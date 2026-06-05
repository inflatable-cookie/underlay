use super::*;

#[tokio::test]
async fn run_queues_low_confidence_ai_decisions_as_unresolved() {
    let fixture = decision_pipeline_fixture(false);

    let mut policy = PipelinePolicy {
        fail_on_unresolved_decisions: false,
        ..Default::default()
    };
    policy
        .ai_threshold_policy
        .decision_type_overrides
        .insert("migration_record_resolution".to_string(), 0.995);

    let ctx = MigrationContext::new(RunMetadata::new("plugin-v1", "schema-v1"), policy);
    let report = fixture
        .orchestrator
        .run(
            &ctx,
            &fixture.store,
            DecisionReusePolicy::Strict,
            "prompt-v1",
        )
        .await
        .expect("run should succeed when unresolved queue is allowed");

    assert_eq!(report.decide.decision_count, 2);
    assert_eq!(report.decide.unresolved_count, 2);
    assert_eq!(report.decide.unresolved_queue.len(), 2);
    for unresolved in &report.decide.unresolved_queue {
        assert_eq!(unresolved.reason, "low_confidence_ai");
        assert_eq!(unresolved.threshold, 0.995);
    }

    assert_eq!(fixture.store.unresolved.lock().expect("lock").len(), 2);
}
