use crate::MigrationError;

use super::*;

#[tokio::test]
async fn run_surfaces_governance_issue_for_invalid_cached_record() {
    let fixture = decision_pipeline_fixture(false);

    seed_decision(
        &fixture.store,
        decision_record(
            decision_fingerprint_for_id(1),
            json!({ "id": 1 }),
            Some(0.99),
            "robot",
            Utc::now(),
        ),
    )
    .await;

    let err = fixture
        .orchestrator
        .run(
            &default_context(),
            &fixture.store,
            DecisionReusePolicy::Strict,
            "prompt-v1",
        )
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
