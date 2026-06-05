mod decide;
mod stages;

use chrono::Utc;

use crate::context::MigrationContext;
use crate::errors::MigrationResult;
use crate::integrity::evaluate_integrity_gate;
use crate::plugin::{
    AssetResolver, DecisionResolver, DecisionReusePolicy, LegacySource, MigrationPlugin,
};
use crate::run_store::RunStore;

use super::errors::stage_failure;
use super::resume::validate_resume_compatibility;
use super::types::*;

/// Coordinates migration stage order and plugin wiring.
pub struct MigrationOrchestrator<S, P, D, A> {
    pub source: S,
    pub plugin: P,
    pub decision_resolver: D,
    pub asset_resolver: A,
}

impl<S, P, D, A> MigrationOrchestrator<S, P, D, A> {
    pub fn stage_order() -> &'static [StageName] {
        const STAGES: &[StageName] = &[
            StageName::Extract,
            StageName::Normalize,
            StageName::Transform,
            StageName::Decide,
            StageName::Materialize,
            StageName::Assets,
            StageName::Verify,
        ];
        STAGES
    }
}

impl<S, P, D, A> MigrationOrchestrator<S, P, D, A>
where
    S: LegacySource,
    P: MigrationPlugin,
    D: DecisionResolver,
    A: AssetResolver,
{
    pub fn new(source: S, plugin: P, decision_resolver: D, asset_resolver: A) -> Self {
        Self {
            source,
            plugin,
            decision_resolver,
            asset_resolver,
        }
    }

    pub async fn run<R>(
        &self,
        ctx: &MigrationContext,
        run_store: &R,
        reuse_policy: DecisionReusePolicy,
        prompt_version: &str,
    ) -> MigrationResult<PipelineRunReport>
    where
        R: RunStore,
    {
        let started_at = Utc::now();
        let resume_plan = validate_resume_compatibility(ctx, run_store).await?;
        let resume_from = resume_plan.resume_from;

        let extract = self.extract_stage(ctx, run_store, resume_from).await?;
        let normalize = self
            .normalize_stage(ctx, run_store, resume_from, &extract)
            .await?;
        let transform = self
            .transform_stage(ctx, run_store, resume_from, &normalize)
            .await?;
        let decide = self
            .decide_stage(
                ctx,
                run_store,
                resume_from,
                reuse_policy,
                prompt_version,
                &transform,
            )
            .await?;

        let integrity_gate =
            evaluate_integrity_gate(&ctx.policy.integrity_policy, &ctx.policy.integrity_evidence);
        if !integrity_gate.passed {
            let message = integrity_gate
                .blockers
                .iter()
                .map(|blocker| format!("{}: {}", blocker.code, blocker.message))
                .collect::<Vec<_>>()
                .join("; ");
            return Err(stage_failure(
                StageName::Materialize,
                format!("integrity gate failed: {message}"),
            ));
        }

        let materialize = self
            .materialize_stage(ctx, run_store, resume_from, &transform)
            .await?;
        let assets = self
            .assets_stage(ctx, run_store, resume_from, &transform)
            .await?;
        let verify = self
            .verify_stage_output(
                ctx,
                run_store,
                resume_from,
                &transform,
                &decide,
                &materialize,
                &assets,
            )
            .await?;

        if !verify.passed {
            return Err(stage_failure(
                StageName::Verify,
                verify
                    .issues
                    .iter()
                    .map(|issue| format!("{}: {}", issue.code, issue.message))
                    .collect::<Vec<_>>()
                    .join("; "),
            ));
        }

        Ok(PipelineRunReport {
            run_id: ctx.run.run_id,
            started_at,
            finished_at: Utc::now(),
            extract,
            normalize,
            transform,
            decide,
            integrity_gate,
            materialize,
            assets,
            verify,
            resume_diagnostics: resume_plan.diagnostics,
        })
    }
}
