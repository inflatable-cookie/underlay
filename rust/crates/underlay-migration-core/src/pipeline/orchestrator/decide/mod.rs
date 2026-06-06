mod input;
mod prior;
mod write;

use serde_json::Value;

use crate::context::MigrationContext;
use crate::errors::MigrationResult;
use crate::pipeline::checkpoints::{load_stage_output, persist_stage_output};
use crate::pipeline::errors::stage_failure;
use crate::pipeline::resume::should_resume;
use crate::pipeline::types::{
    DecideStageOutput, DecisionGovernanceIssue, DecisionInvalidationEvent, StageName,
    TransformStageOutput, UnresolvedDecisionItem,
};
use crate::plugin::{
    AssetResolver, DecisionOutcome, DecisionResolver, DecisionReusePolicy, LegacySource,
    MigrationPlugin,
};
use crate::run_store::RunStore;

use super::MigrationOrchestrator;
use input::build_decision_candidate;
use prior::{evaluate_prior_decision, load_valid_prior_chain};
use write::{append_decision_journal, append_unresolved_if_low_confidence};

impl<S, P, D, A> MigrationOrchestrator<S, P, D, A>
where
    S: LegacySource,
    P: MigrationPlugin,
    D: DecisionResolver,
    A: AssetResolver,
{
    pub(super) async fn decide_stage<R>(
        &self,
        ctx: &MigrationContext,
        run_store: &R,
        resume_from: usize,
        reuse_policy: DecisionReusePolicy,
        prompt_version: &str,
        transform_output: &TransformStageOutput,
    ) -> MigrationResult<DecideStageOutput>
    where
        R: RunStore,
    {
        if should_resume(StageName::Decide, resume_from) {
            return load_stage_output(run_store, ctx, StageName::Decide).await;
        }

        let mut decisions = Vec::new();
        let mut reused_count = 0usize;
        let mut resolved_count = 0usize;
        let mut invalidated_count = 0usize;
        let mut invalidations = Vec::new();
        let mut unresolved_queue = Vec::new();
        let mut governance_issues = Vec::new();

        for batch in &transform_output.batches {
            for record in &batch.records {
                let resolver_version = self.decision_resolver.resolver_version();
                let candidate =
                    build_decision_candidate(ctx, record, resolver_version, prompt_version);

                let fingerprint = self
                    .decision_resolver
                    .fingerprint(&candidate.input)
                    .await
                    .map_err(|err| stage_failure(StageName::Decide, err.to_string()))?;

                let plugin_invalidated =
                    self.plugin
                        .should_invalidate_decision(ctx, &fingerprint, reuse_policy);
                let valid_prior =
                    load_valid_prior_chain(run_store, &fingerprint, &mut governance_issues).await?;
                let prior_reuse = evaluate_prior_decision(
                    &valid_prior,
                    &fingerprint,
                    &candidate.input,
                    reuse_policy,
                    plugin_invalidated,
                    &candidate.decision_type,
                )?;

                if let Some(mut decision) = prior_reuse.decision {
                    append_unresolved_if_low_confidence(
                        ctx,
                        run_store,
                        &mut decision,
                        &candidate,
                        &mut governance_issues,
                        &mut unresolved_queue,
                    )
                    .await?;
                    decisions.push(decision);
                    reused_count += 1;
                    continue;
                }

                if let Some(invalidation) = prior_reuse.invalidation {
                    invalidations.push(invalidation);
                    invalidated_count += 1;
                }

                let mut decision = self
                    .decision_resolver
                    .resolve(ctx, candidate.input.clone(), reuse_policy)
                    .await
                    .map_err(|err| stage_failure(StageName::Decide, err.to_string()))?;

                ensure_resolver_fingerprint_matches(&decision, &fingerprint)?;
                append_decision_journal(
                    ctx,
                    run_store,
                    &decision,
                    &candidate,
                    &fingerprint,
                    resolver_version,
                    prompt_version,
                    &mut governance_issues,
                )
                .await?;
                append_unresolved_if_low_confidence(
                    ctx,
                    run_store,
                    &mut decision,
                    &candidate,
                    &mut governance_issues,
                    &mut unresolved_queue,
                )
                .await?;

                resolved_count += 1;
                decisions.push(decision);
            }
        }

        let output = decide_stage_output(
            decisions,
            reused_count,
            resolved_count,
            invalidated_count,
            invalidations,
            unresolved_queue,
            governance_issues,
        );
        persist_stage_output(run_store, ctx, StageName::Decide, &output).await?;
        Ok(output)
    }
}

fn ensure_resolver_fingerprint_matches(
    decision: &DecisionOutcome,
    fingerprint: &str,
) -> MigrationResult<()> {
    if decision.fingerprint == fingerprint {
        return Ok(());
    }

    Err(stage_failure(
        StageName::Decide,
        format!(
            "resolver returned fingerprint {}, expected {}",
            decision.fingerprint, fingerprint
        ),
    ))
}

fn decide_stage_output(
    decisions: Vec<DecisionOutcome>,
    reused_count: usize,
    resolved_count: usize,
    invalidated_count: usize,
    invalidations: Vec<DecisionInvalidationEvent>,
    unresolved_queue: Vec<UnresolvedDecisionItem>,
    governance_issues: Vec<DecisionGovernanceIssue>,
) -> DecideStageOutput {
    DecideStageOutput {
        decision_count: decisions.len(),
        unresolved_count: decisions
            .iter()
            .filter(|decision| decision.outcome == Value::Null)
            .count(),
        reused_count,
        resolved_count,
        invalidated_count,
        invalidations,
        unresolved_queue,
        governance_issues,
        decisions,
    }
}
