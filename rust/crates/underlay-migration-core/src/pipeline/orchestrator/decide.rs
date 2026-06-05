use chrono::Utc;
use serde_json::Value;

use crate::context::MigrationContext;
use crate::decision_memory::{
    effective_decision_for_fingerprint, evaluate_decision_reuse, validate_decision_journal_record,
    validate_unresolved_decision_record,
};
use crate::errors::MigrationResult;
use crate::plugin::{
    AssetResolver, DecisionFingerprintInput, DecisionOutcome, DecisionResolver,
    DecisionReusePolicy, LegacySource, MigrationPlugin,
};
use crate::run_store::{DecisionJournalRecord, RunStore, UnresolvedDecisionRecord};

use super::MigrationOrchestrator;
use crate::pipeline::checkpoints::{load_stage_output, persist_stage_output};
use crate::pipeline::decision_support::{
    build_unresolved, decision_provenance_label, low_confidence_reason, parse_provenance,
};
use crate::pipeline::errors::stage_failure;
use crate::pipeline::resume::should_resume;
use crate::pipeline::types::{
    DecideStageOutput, DecisionGovernanceIssue, DecisionInvalidationEvent, StageName,
    TransformStageOutput,
};

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
                let decision_type = "migration_record_resolution".to_string();
                let threshold = ctx.policy.ai_threshold_policy.threshold_for(&decision_type);
                let input = DecisionFingerprintInput {
                    canonical_decision_input: record.clone(),
                    decision_type: decision_type.clone(),
                    resolver_version: self.decision_resolver.resolver_version().to_string(),
                    prompt_version: prompt_version.to_string(),
                    target_schema_version: ctx.run.target_schema_version.clone(),
                };
                let canonical_decision_input = input.canonical_decision_input.clone();

                let fingerprint = self
                    .decision_resolver
                    .fingerprint(&input)
                    .await
                    .map_err(|err| stage_failure(StageName::Decide, err.to_string()))?;

                let plugin_invalidated =
                    self.plugin
                        .should_invalidate_decision(ctx, &fingerprint, reuse_policy);

                let prior_chain = run_store
                    .decisions_for_fingerprint(&fingerprint)
                    .await
                    .map_err(|err| stage_failure(StageName::Decide, err.to_string()))?;
                let mut valid_prior = Vec::new();
                for prior in prior_chain {
                    if let Err(err) = validate_decision_journal_record(&prior) {
                        governance_issues.push(DecisionGovernanceIssue {
                            artifact: "decision_journal".to_string(),
                            fingerprint: prior.fingerprint.clone(),
                            code: "decision_journal_invalid_record".to_string(),
                            message: err.to_string(),
                        });
                        continue;
                    }
                    valid_prior.push(prior);
                }

                if let Some(prior) = effective_decision_for_fingerprint(&valid_prior, &fingerprint)
                {
                    let reuse = evaluate_decision_reuse(
                        prior,
                        &fingerprint,
                        &input,
                        reuse_policy,
                        plugin_invalidated,
                    );
                    if reuse.reusable {
                        let provenance = parse_provenance(prior)?;
                        let mut decision = DecisionOutcome {
                            fingerprint: prior.fingerprint.clone(),
                            outcome: prior.outcome.clone(),
                            confidence: prior.confidence,
                            provenance,
                        };
                        if let Some(unresolved_reason) = low_confidence_reason(&decision, threshold)
                        {
                            let unresolved = build_unresolved(
                                ctx,
                                &decision,
                                &decision_type,
                                threshold,
                                unresolved_reason,
                            );
                            let unresolved_record = UnresolvedDecisionRecord {
                                unresolved_id: unresolved.unresolved_id,
                                run_id: ctx.run.run_id,
                                fingerprint: unresolved.fingerprint.clone(),
                                decision_type: unresolved.decision_type.clone(),
                                provenance: decision_provenance_label(&unresolved.provenance)
                                    .to_string(),
                                confidence: unresolved.confidence,
                                threshold: unresolved.threshold,
                                reason: unresolved.reason.clone(),
                                canonical_decision_input: canonical_decision_input.clone(),
                                created_at: Utc::now(),
                            };
                            run_store
                                .append_unresolved_decision(unresolved_record.clone())
                                .await
                                .map_err(|err| stage_failure(StageName::Decide, err.to_string()))?;
                            if let Err(err) =
                                validate_unresolved_decision_record(&unresolved_record)
                            {
                                governance_issues.push(DecisionGovernanceIssue {
                                    artifact: "unresolved_queue".to_string(),
                                    fingerprint: unresolved_record.fingerprint.clone(),
                                    code: "unresolved_queue_invalid_record".to_string(),
                                    message: err.to_string(),
                                });
                            }
                            decision.outcome = Value::Null;
                            unresolved_queue.push(unresolved);
                        }
                        decisions.push(decision);
                        reused_count += 1;
                        continue;
                    }
                    if let Some(reason) = reuse.reason {
                        invalidations.push(DecisionInvalidationEvent {
                            fingerprint: fingerprint.clone(),
                            reason,
                            decision_type: decision_type.clone(),
                        });
                        invalidated_count += 1;
                    }
                }

                let decision = self
                    .decision_resolver
                    .resolve(ctx, input, reuse_policy)
                    .await
                    .map_err(|err| stage_failure(StageName::Decide, err.to_string()))?;

                if decision.fingerprint != fingerprint {
                    return Err(stage_failure(
                        StageName::Decide,
                        format!(
                            "resolver returned fingerprint {}, expected {}",
                            decision.fingerprint, fingerprint
                        ),
                    ));
                }

                let journal_record = DecisionJournalRecord {
                    decision_id: underlay_core::Uuid::new_v7(),
                    fingerprint: fingerprint.clone(),
                    decision_type: decision_type.clone(),
                    outcome: decision.outcome.clone(),
                    confidence: decision.confidence,
                    resolver_version: self.decision_resolver.resolver_version().to_string(),
                    prompt_version: prompt_version.to_string(),
                    target_schema_version: ctx.run.target_schema_version.clone(),
                    created_at: Utc::now(),
                    provenance: decision_provenance_label(&decision.provenance).to_string(),
                };
                if let Err(err) = validate_decision_journal_record(&journal_record) {
                    governance_issues.push(DecisionGovernanceIssue {
                        artifact: "decision_journal".to_string(),
                        fingerprint: journal_record.fingerprint.clone(),
                        code: "decision_journal_invalid_record".to_string(),
                        message: err.to_string(),
                    });
                } else {
                    run_store
                        .append_decision_journal(journal_record)
                        .await
                        .map_err(|err| stage_failure(StageName::Decide, err.to_string()))?;
                }

                let mut decision = decision;
                if let Some(unresolved_reason) = low_confidence_reason(&decision, threshold) {
                    let unresolved = build_unresolved(
                        ctx,
                        &decision,
                        &decision_type,
                        threshold,
                        unresolved_reason,
                    );
                    let unresolved_record = UnresolvedDecisionRecord {
                        unresolved_id: unresolved.unresolved_id,
                        run_id: ctx.run.run_id,
                        fingerprint: unresolved.fingerprint.clone(),
                        decision_type: unresolved.decision_type.clone(),
                        provenance: decision_provenance_label(&unresolved.provenance).to_string(),
                        confidence: unresolved.confidence,
                        threshold: unresolved.threshold,
                        reason: unresolved.reason.clone(),
                        canonical_decision_input: canonical_decision_input.clone(),
                        created_at: Utc::now(),
                    };
                    run_store
                        .append_unresolved_decision(unresolved_record.clone())
                        .await
                        .map_err(|err| stage_failure(StageName::Decide, err.to_string()))?;
                    if let Err(err) = validate_unresolved_decision_record(&unresolved_record) {
                        governance_issues.push(DecisionGovernanceIssue {
                            artifact: "unresolved_queue".to_string(),
                            fingerprint: unresolved_record.fingerprint.clone(),
                            code: "unresolved_queue_invalid_record".to_string(),
                            message: err.to_string(),
                        });
                    }
                    decision.outcome = Value::Null;
                    unresolved_queue.push(unresolved);
                }

                resolved_count += 1;
                decisions.push(decision);
            }
        }

        let output = DecideStageOutput {
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
        };
        persist_stage_output(run_store, ctx, StageName::Decide, &output).await?;
        Ok(output)
    }
}
