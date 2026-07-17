use crate::context::MigrationContext;
use crate::errors::MigrationResult;
use crate::plugin::{
    AssetResolver, DecisionResolver, LegacySource, MaterializeResult, MigrationPlugin,
};
use crate::run_store::RunStore;
use crate::verification::{transform_checksum, verify_stage, VerificationInput};

use super::MigrationOrchestrator;
use crate::pipeline::checkpoints::{load_stage_output, persist_stage_output};
use crate::pipeline::errors::stage_failure;
use crate::pipeline::resume::should_resume;
use crate::pipeline::types::{
    AssetsStageOutput, DecideStageOutput, ExtractStageOutput, MaterializeStageOutput,
    NormalizeStageOutput, StageName, TransformStageOutput, VerifyStageOutput,
};

impl<S, P, D, A> MigrationOrchestrator<S, P, D, A>
where
    S: LegacySource,
    P: MigrationPlugin,
    D: DecisionResolver,
    A: AssetResolver,
{
    pub(super) async fn extract_stage<R>(
        &self,
        ctx: &MigrationContext,
        run_store: &R,
        resume_from: usize,
    ) -> MigrationResult<ExtractStageOutput>
    where
        R: RunStore,
    {
        if should_resume(StageName::Extract, resume_from) {
            return load_stage_output(run_store, ctx, StageName::Extract).await;
        }

        let extracted = self
            .source
            .extract(ctx)
            .await
            .map_err(|err| stage_failure(StageName::Extract, err.to_string()))?;
        let output = ExtractStageOutput {
            batch_count: extracted.len(),
            record_count: extracted.iter().map(|batch| batch.records.len()).sum(),
            batches: extracted,
        };
        persist_stage_output(run_store, ctx, StageName::Extract, &output).await?;
        Ok(output)
    }

    pub(super) async fn normalize_stage<R>(
        &self,
        ctx: &MigrationContext,
        run_store: &R,
        resume_from: usize,
        extract_output: &ExtractStageOutput,
    ) -> MigrationResult<NormalizeStageOutput>
    where
        R: RunStore,
    {
        if should_resume(StageName::Normalize, resume_from) {
            return load_stage_output(run_store, ctx, StageName::Normalize).await;
        }

        let mut normalized = Vec::with_capacity(extract_output.batch_count);
        for batch in &extract_output.batches {
            let normalized_batch = self
                .plugin
                .normalize(ctx, batch.clone())
                .await
                .map_err(|err| stage_failure(StageName::Normalize, err.to_string()))?;
            normalized.push(normalized_batch);
        }

        let output = NormalizeStageOutput {
            batch_count: normalized.len(),
            record_count: normalized.iter().map(|batch| batch.records.len()).sum(),
            batches: normalized,
        };
        persist_stage_output(run_store, ctx, StageName::Normalize, &output).await?;
        Ok(output)
    }

    pub(super) async fn transform_stage<R>(
        &self,
        ctx: &MigrationContext,
        run_store: &R,
        resume_from: usize,
        normalize_output: &NormalizeStageOutput,
    ) -> MigrationResult<TransformStageOutput>
    where
        R: RunStore,
    {
        if should_resume(StageName::Transform, resume_from) {
            return load_stage_output(run_store, ctx, StageName::Transform).await;
        }

        let mut transformed = Vec::with_capacity(normalize_output.batch_count);
        for batch in &normalize_output.batches {
            let transformed_batch = self
                .plugin
                .transform(ctx, batch.clone())
                .await
                .map_err(|err| stage_failure(StageName::Transform, err.to_string()))?;
            transformed.push(transformed_batch);
        }

        let output = TransformStageOutput {
            batch_count: transformed.len(),
            record_count: transformed.iter().map(|batch| batch.records.len()).sum(),
            batches: transformed,
        };
        persist_stage_output(run_store, ctx, StageName::Transform, &output).await?;
        Ok(output)
    }

    pub(super) async fn materialize_stage<R>(
        &self,
        ctx: &MigrationContext,
        run_store: &R,
        resume_from: usize,
        transform_output: &TransformStageOutput,
    ) -> MigrationResult<MaterializeStageOutput>
    where
        R: RunStore,
    {
        if should_resume(StageName::Materialize, resume_from) {
            return load_stage_output(run_store, ctx, StageName::Materialize).await;
        }

        let mut materialize_totals = MaterializeResult {
            inserted: 0,
            updated: 0,
            skipped: 0,
        };
        for batch in &transform_output.batches {
            let result = self
                .plugin
                .materialize(ctx, batch.clone())
                .await
                .map_err(|err| stage_failure(StageName::Materialize, err.to_string()))?;
            materialize_totals.inserted += result.inserted;
            materialize_totals.updated += result.updated;
            materialize_totals.skipped += result.skipped;
        }

        let output = MaterializeStageOutput {
            inserted: materialize_totals.inserted,
            updated: materialize_totals.updated,
            skipped: materialize_totals.skipped,
        };
        persist_stage_output(run_store, ctx, StageName::Materialize, &output).await?;
        Ok(output)
    }

    pub(super) async fn assets_stage<R>(
        &self,
        ctx: &MigrationContext,
        run_store: &R,
        resume_from: usize,
        transform_output: &TransformStageOutput,
    ) -> MigrationResult<AssetsStageOutput>
    where
        R: RunStore,
    {
        if should_resume(StageName::Assets, resume_from) {
            return load_stage_output(run_store, ctx, StageName::Assets).await;
        }

        let mut output = AssetsStageOutput {
            resolved_count: 0,
            unresolved_count: 0,
        };
        for batch in &transform_output.batches {
            let resolution = self
                .asset_resolver
                .resolve_assets(ctx, batch)
                .await
                .map_err(|err| stage_failure(StageName::Assets, err.to_string()))?;
            output.resolved_count += resolution.resolved_count;
            output.unresolved_count += resolution.unresolved_count;
        }
        persist_stage_output(run_store, ctx, StageName::Assets, &output).await?;
        Ok(output)
    }

    #[allow(clippy::too_many_arguments)] // aggregates every prior stage's output for verification
    pub(super) async fn verify_stage_output<R>(
        &self,
        ctx: &MigrationContext,
        run_store: &R,
        resume_from: usize,
        transform_output: &TransformStageOutput,
        decide_output: &DecideStageOutput,
        materialize_output: &MaterializeStageOutput,
        assets_output: &AssetsStageOutput,
    ) -> MigrationResult<VerifyStageOutput>
    where
        R: RunStore,
    {
        if should_resume(StageName::Verify, resume_from) {
            return load_stage_output(run_store, ctx, StageName::Verify).await;
        }

        let verification_input = VerificationInput {
            transform_record_count: transform_output.record_count,
            transform_records: transform_output
                .batches
                .iter()
                .flat_map(|batch| batch.records.clone())
                .collect(),
            decision_count: decide_output.decision_count,
            unresolved_decision_count: decide_output.unresolved_count,
            decision_governance_issue_count: decide_output.governance_issues.len(),
            transform_checksum: transform_checksum(transform_output)?,
            materialize: materialize_output.clone(),
            assets: assets_output.clone(),
            rules: ctx.policy.verification_rules().to_vec(),
        };

        let verification = verify_stage(&self.plugin, ctx, &verification_input).await?;
        let output = VerifyStageOutput {
            passed: verification.passed,
            checks: verification.checks,
            issues: verification.issues,
        };
        persist_stage_output(run_store, ctx, StageName::Verify, &output).await?;
        Ok(output)
    }
}
