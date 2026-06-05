use chrono::Utc;
use serde_json::json;

use crate::{
    LegacyRecordBatch, MigrationError, NormalizeStageOutput, RunStore, StageCheckpoint,
    StageSnapshot,
};

use super::*;

#[tokio::test]
async fn run_resumes_from_completed_normalize_stage() {
    let fixture = pipeline_fixture(false, false, false);
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
    let normalize_output = NormalizeStageOutput {
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

    let report = fixture
        .orchestrator
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

    assert_eq!(counter_value(&fixture.source_calls), 0);
    assert_eq!(counter_value(&fixture.normalize_calls), 0);
}

#[tokio::test]
async fn run_rejects_incompatible_resume_checkpoint() {
    let fixture = pipeline_fixture(false, false, false);
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

    let err = fixture
        .orchestrator
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

    assert_eq!(counter_value(&fixture.source_calls), 0);
}
