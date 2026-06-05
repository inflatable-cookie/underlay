use chrono::Utc;
use serde_json::Value;

use crate::context::MigrationContext;
use crate::decision_memory::{
    effective_decision_for_fingerprint, evaluate_decision_reuse, validate_decision_journal_record,
    validate_unresolved_decision_record,
};
use crate::errors::MigrationResult;
use crate::integrity::evaluate_integrity_gate;
use crate::plugin::{
    AssetResolver, DecisionFingerprintInput, DecisionOutcome, DecisionResolver,
    DecisionReusePolicy, LegacySource, MaterializeResult, MigrationPlugin,
};
use crate::run_store::{DecisionJournalRecord, RunStore, UnresolvedDecisionRecord};
use crate::verification::{transform_checksum, verify_stage, VerificationInput};

use super::checkpoints::{load_stage_output, persist_stage_output};
use super::decision_support::{
    build_unresolved, decision_provenance_label, low_confidence_reason, parse_provenance,
};
use super::errors::stage_failure;
use super::resume::{should_resume, validate_resume_compatibility};
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

        let extract_output = if should_resume(StageName::Extract, resume_from) {
            load_stage_output(run_store, ctx, StageName::Extract).await?
        } else {
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
            output
        };

        let normalize_output = if should_resume(StageName::Normalize, resume_from) {
            load_stage_output(run_store, ctx, StageName::Normalize).await?
        } else {
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
            output
        };

        let transform_output = if should_resume(StageName::Transform, resume_from) {
            load_stage_output(run_store, ctx, StageName::Transform).await?
        } else {
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
            output
        };

        let decide_output = if should_resume(StageName::Decide, resume_from) {
            load_stage_output(run_store, ctx, StageName::Decide).await?
        } else {
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

                    if let Some(prior) =
                        effective_decision_for_fingerprint(&valid_prior, &fingerprint)
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
                            if let Some(unresolved_reason) =
                                low_confidence_reason(&decision, threshold)
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
                                    .map_err(|err| {
                                        stage_failure(StageName::Decide, err.to_string())
                                    })?;
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
                    .filter(|d| d.outcome == Value::Null)
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
            output
        };

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

        let materialize_output = if should_resume(StageName::Materialize, resume_from) {
            load_stage_output(run_store, ctx, StageName::Materialize).await?
        } else {
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
            output
        };

        let assets_output = if should_resume(StageName::Assets, resume_from) {
            load_stage_output(run_store, ctx, StageName::Assets).await?
        } else {
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
            output
        };

        let verify_output = if should_resume(StageName::Verify, resume_from) {
            load_stage_output(run_store, ctx, StageName::Verify).await?
        } else {
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
                transform_checksum: transform_checksum(&transform_output)?,
                materialize: materialize_output.clone(),
                assets: assets_output.clone(),
                rules: ctx.policy.verification_rules.clone(),
            };

            let verification = verify_stage(&self.plugin, ctx, &verification_input).await?;
            let output = VerifyStageOutput {
                passed: verification.passed,
                checks: verification.checks,
                issues: verification.issues,
            };
            persist_stage_output(run_store, ctx, StageName::Verify, &output).await?;
            output
        };

        if !verify_output.passed {
            return Err(stage_failure(
                StageName::Verify,
                verify_output
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
            extract: extract_output,
            normalize: normalize_output,
            transform: transform_output,
            decide: decide_output,
            integrity_gate,
            materialize: materialize_output,
            assets: assets_output,
            verify: verify_output,
            resume_diagnostics: resume_plan.diagnostics,
        })
    }
}
