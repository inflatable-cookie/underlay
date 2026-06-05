use super::*;

#[tokio::test]
async fn run_executes_all_stages_and_returns_report() {
    let fixture = pipeline_fixture(false, false, false);
    let store = InMemoryRunStore::default();
    let ctx = default_context();

    let report = fixture
        .orchestrator
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

    assert_eq!(counter_value(&fixture.source_calls), 1);
    assert_eq!(counter_value(&fixture.normalize_calls), 1);
    assert_eq!(counter_value(&fixture.resolve_calls), 2);
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
