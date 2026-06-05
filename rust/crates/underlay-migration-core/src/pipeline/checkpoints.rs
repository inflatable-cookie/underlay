use chrono::Utc;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::context::MigrationContext;
use crate::errors::{MigrationError, MigrationResult};
use crate::run_store::{RunStore, StageCheckpoint, StageSnapshot};

use super::errors::stage_failure;
use super::types::StageName;

pub(super) async fn persist_stage_output<R, T>(
    run_store: &R,
    ctx: &MigrationContext,
    stage: StageName,
    output: &T,
) -> MigrationResult<()>
where
    R: RunStore,
    T: Serialize,
{
    let payload = serde_json::to_value(output)
        .map_err(|err| MigrationError::Serialization(err.to_string()))?;

    run_store
        .write_stage_snapshot(StageSnapshot {
            run_id: ctx.run.run_id,
            stage,
            payload,
            recorded_at: Utc::now(),
        })
        .await
        .map_err(|err| stage_failure(stage, err.to_string()))?;

    run_store
        .write_stage_checkpoint(StageCheckpoint {
            run_id: ctx.run.run_id,
            stage,
            plugin_version: ctx.run.plugin_version.clone(),
            target_schema_version: ctx.run.target_schema_version.clone(),
            cursor: None,
            completed_at: Utc::now(),
        })
        .await
        .map_err(|err| stage_failure(stage, err.to_string()))?;

    Ok(())
}

pub(super) async fn load_stage_output<R, T>(
    run_store: &R,
    ctx: &MigrationContext,
    stage: StageName,
) -> MigrationResult<T>
where
    R: RunStore,
    T: DeserializeOwned,
{
    let snapshot = run_store
        .read_stage_snapshot(ctx.run.run_id, stage)
        .await
        .map_err(|err| stage_failure(stage, err.to_string()))?;

    let Some(snapshot) = snapshot else {
        return Err(MigrationError::DeterminismViolation(format!(
            "missing required stage snapshot for resume: {}",
            stage.as_str()
        )));
    };

    serde_json::from_value(snapshot.payload).map_err(|err| {
        MigrationError::Serialization(format!(
            "stage {} snapshot decode failed: {err}",
            stage.as_str()
        ))
    })
}
