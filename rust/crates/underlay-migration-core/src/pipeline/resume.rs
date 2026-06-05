use crate::context::MigrationContext;
use crate::errors::{MigrationError, MigrationResult};
use crate::run_store::RunStore;

use super::types::{ResumeDiagnostics, StageName};

#[derive(Debug, Clone)]
pub(super) struct ResumePlan {
    pub(super) resume_from: usize,
    pub(super) diagnostics: ResumeDiagnostics,
}

pub(super) async fn validate_resume_compatibility<R>(
    ctx: &MigrationContext,
    run_store: &R,
) -> MigrationResult<ResumePlan>
where
    R: RunStore,
{
    let checkpoint = run_store
        .latest_resume_checkpoint(ctx.run.run_id)
        .await
        .map_err(|err| MigrationError::RunStore(err.to_string()))?;

    if let Some(checkpoint) = checkpoint {
        if checkpoint.plugin_version != ctx.run.plugin_version {
            return Err(MigrationError::ResumeCompatibility {
                code: "plugin_version_mismatch".to_string(),
                message: format!(
                    "resume checkpoint plugin_version mismatch: expected {}, found {}",
                    ctx.run.plugin_version, checkpoint.plugin_version
                ),
            });
        }

        if checkpoint.target_schema_version != ctx.run.target_schema_version {
            return Err(MigrationError::ResumeCompatibility {
                code: "target_schema_version_mismatch".to_string(),
                message: format!(
                    "resume checkpoint target_schema_version mismatch: expected {}, found {}",
                    ctx.run.target_schema_version, checkpoint.target_schema_version
                ),
            });
        }

        return Ok(ResumePlan {
            resume_from: checkpoint.last_completed_stage.index() + 1,
            diagnostics: ResumeDiagnostics {
                resume_attempted: true,
                resumed_from_stage: Some(checkpoint.last_completed_stage),
                status: "resumed".to_string(),
                reason: None,
            },
        });
    }

    Ok(ResumePlan {
        resume_from: 0,
        diagnostics: ResumeDiagnostics::default(),
    })
}

pub(super) fn should_resume(stage: StageName, resume_from: usize) -> bool {
    stage.index() < resume_from
}
