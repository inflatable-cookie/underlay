use crate::DecisionProvenance;

use super::*;

#[tokio::test]
async fn run_reuses_cached_decisions_and_skips_new_journal_entries() {
    let fixture = decision_pipeline_fixture(false);
    let cached_record = decision_record(
        decision_fingerprint_for_id(1),
        json!({ "id": 1 }),
        Some(0.97),
        "human",
        Utc::now(),
    );
    seed_decision(&fixture.store, cached_record).await;

    let report = fixture
        .orchestrator
        .run(
            &default_context(),
            &fixture.store,
            DecisionReusePolicy::Strict,
            "prompt-v1",
        )
        .await
        .expect("run should reuse cached decisions");

    assert_eq!(report.decide.reused_count, 1);
    assert_eq!(report.decide.resolved_count, 1);
    assert_eq!(report.decide.invalidated_count, 0);

    let resolve_count = *fixture.resolve_calls.lock().expect("lock should succeed");
    assert_eq!(resolve_count, 1);

    let journal_len = fixture
        .store
        .decision_journal
        .lock()
        .expect("lock should succeed")
        .len();
    assert_eq!(journal_len, 2);
}

#[tokio::test]
async fn run_prefers_human_override_in_provenance_chain() {
    let fixture = decision_pipeline_fixture(false);
    let fingerprint = decision_fingerprint_for_id(1);

    seed_decision(
        &fixture.store,
        decision_record(
            fingerprint.clone(),
            json!({ "winner": "human" }),
            Some(1.0),
            "human",
            Utc::now() - chrono::Duration::seconds(60),
        ),
    )
    .await;
    seed_decision(
        &fixture.store,
        decision_record(
            fingerprint,
            json!({ "winner": "ai" }),
            Some(1.0),
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
