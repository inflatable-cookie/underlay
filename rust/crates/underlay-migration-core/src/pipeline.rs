use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::MigrationContext;
use crate::decision_memory::{
    effective_decision_for_fingerprint, evaluate_decision_reuse, validate_decision_journal_record,
    validate_unresolved_decision_record, DecisionInvalidationReason,
};
use crate::errors::{FailureClass, MigrationError, MigrationResult};
use crate::integrity::{evaluate_integrity_gate, IntegrityGateResult};
use crate::plugin::{
    AssetResolver, DecisionFingerprintInput, DecisionOutcome, DecisionProvenance, DecisionResolver,
    DecisionReusePolicy, LegacyRecordBatch, LegacySource, MaterializeResult, MigrationPlugin,
    NormalizedBatch, TransformBatch,
};
use crate::run_store::{
    DecisionJournalRecord, RunStore, StageCheckpoint, StageSnapshot, UnresolvedDecisionRecord,
};
use crate::verification::{
    transform_checksum, verify_stage, VerificationCheckResult, VerificationInput, VerificationIssue,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum StageName {
    Extract,
    Normalize,
    Transform,
    Decide,
    Materialize,
    Assets,
    Verify,
}

impl StageName {
    pub fn as_str(self) -> &'static str {
        match self {
            StageName::Extract => "extract",
            StageName::Normalize => "normalize",
            StageName::Transform => "transform",
            StageName::Decide => "decide",
            StageName::Materialize => "materialize",
            StageName::Assets => "assets",
            StageName::Verify => "verify",
        }
    }

    fn index(self) -> usize {
        match self {
            StageName::Extract => 0,
            StageName::Normalize => 1,
            StageName::Transform => 2,
            StageName::Decide => 3,
            StageName::Materialize => 4,
            StageName::Assets => 5,
            StageName::Verify => 6,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractStageOutput {
    pub batches: Vec<LegacyRecordBatch>,
    pub batch_count: usize,
    pub record_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NormalizeStageOutput {
    pub batches: Vec<NormalizedBatch>,
    pub batch_count: usize,
    pub record_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransformStageOutput {
    pub batches: Vec<TransformBatch>,
    pub batch_count: usize,
    pub record_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecideStageOutput {
    pub decisions: Vec<DecisionOutcome>,
    pub decision_count: usize,
    pub unresolved_count: usize,
    #[serde(default)]
    pub reused_count: usize,
    #[serde(default)]
    pub resolved_count: usize,
    #[serde(default)]
    pub invalidated_count: usize,
    #[serde(default)]
    pub invalidations: Vec<DecisionInvalidationEvent>,
    #[serde(default)]
    pub unresolved_queue: Vec<UnresolvedDecisionItem>,
    #[serde(default)]
    pub governance_issues: Vec<DecisionGovernanceIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecisionInvalidationEvent {
    pub fingerprint: String,
    pub reason: DecisionInvalidationReason,
    pub decision_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnresolvedDecisionItem {
    pub unresolved_id: underlay_core::Uuid,
    pub fingerprint: String,
    pub decision_type: String,
    pub confidence: Option<f64>,
    pub threshold: f64,
    pub reason: String,
    pub provenance: DecisionProvenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecisionGovernanceIssue {
    pub artifact: String,
    pub fingerprint: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MaterializeStageOutput {
    pub inserted: u64,
    pub updated: u64,
    pub skipped: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssetsStageOutput {
    pub resolved_count: u64,
    pub unresolved_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyStageOutput {
    pub passed: bool,
    pub checks: Vec<VerificationCheckResult>,
    pub issues: Vec<VerificationIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelineRunReport {
    pub run_id: underlay_core::Uuid,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub extract: ExtractStageOutput,
    pub normalize: NormalizeStageOutput,
    pub transform: TransformStageOutput,
    pub decide: DecideStageOutput,
    #[serde(default)]
    pub integrity_gate: IntegrityGateResult,
    pub materialize: MaterializeStageOutput,
    pub assets: AssetsStageOutput,
    pub verify: VerifyStageOutput,
    #[serde(default)]
    pub resume_diagnostics: ResumeDiagnostics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResumeDiagnostics {
    pub resume_attempted: bool,
    pub resumed_from_stage: Option<StageName>,
    pub status: String,
    pub reason: Option<String>,
}

impl Default for ResumeDiagnostics {
    fn default() -> Self {
        Self {
            resume_attempted: false,
            resumed_from_stage: None,
            status: "fresh_start".to_string(),
            reason: None,
        }
    }
}

#[derive(Debug, Clone)]
struct ResumePlan {
    resume_from: usize,
    diagnostics: ResumeDiagnostics,
}

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
        let resume_plan = self.validate_resume_compatibility(ctx, run_store).await?;
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

    async fn validate_resume_compatibility<R>(
        &self,
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
}

fn should_resume(stage: StageName, resume_from: usize) -> bool {
    stage.index() < resume_from
}

fn stage_failure(stage: StageName, message: String) -> MigrationError {
    let (failure_class, recovery_hint) = classify_stage_failure(stage, &message);
    MigrationError::StageFailure {
        stage: stage.as_str().to_string(),
        message,
        failure_class,
        recovery_hint,
    }
}

fn classify_stage_failure(stage: StageName, message: &str) -> (FailureClass, String) {
    if message.contains("digest mismatch")
        || message.contains("integrity")
        || message.contains("governance")
    {
        return (
            FailureClass::NonRetrySafe,
            "inspect integrity/governance inputs, correct data, then rerun from a clean checkpoint"
                .to_string(),
        );
    }

    match stage {
        StageName::Extract | StageName::Normalize | StageName::Transform | StageName::Assets => (
            FailureClass::RetrySafe,
            "retry from the last successful checkpoint after validating source connectivity".to_string(),
        ),
        StageName::Decide => (
            FailureClass::RetrySafe,
            "retry after confirming decision resolver/AI dependencies; unresolved items can be resumed"
                .to_string(),
        ),
        StageName::Materialize => (
            FailureClass::NonRetrySafe,
            "validate partial writes and run targeted cleanup or restore before retrying materialization"
                .to_string(),
        ),
        StageName::Verify => (
            FailureClass::NonRetrySafe,
            "address verification failures before promotion; do not blindly retry".to_string(),
        ),
    }
}

fn parse_provenance(record: &DecisionJournalRecord) -> MigrationResult<DecisionProvenance> {
    match record.provenance.as_str() {
        "rule" => Ok(DecisionProvenance::Rule),
        "ai" => Ok(DecisionProvenance::Ai),
        "human" => Ok(DecisionProvenance::Human),
        other => Err(MigrationError::DeterminismViolation(format!(
            "unsupported decision provenance in journal: {}",
            other
        ))),
    }
}

fn decision_provenance_label(provenance: &DecisionProvenance) -> &'static str {
    match provenance {
        DecisionProvenance::Rule => "rule",
        DecisionProvenance::Ai => "ai",
        DecisionProvenance::Human => "human",
    }
}

fn low_confidence_reason(decision: &DecisionOutcome, threshold: f64) -> Option<&'static str> {
    if decision.provenance != DecisionProvenance::Ai {
        return None;
    }

    match decision.confidence {
        Some(value) if value >= threshold => None,
        Some(_) => Some("low_confidence_ai"),
        None => Some("missing_confidence_ai"),
    }
}

fn build_unresolved(
    ctx: &MigrationContext,
    decision: &DecisionOutcome,
    decision_type: &str,
    threshold: f64,
    reason: &str,
) -> UnresolvedDecisionItem {
    let mut hasher = sha2::Sha256::new();
    use sha2::Digest as _;
    hasher.update(ctx.run.run_id.to_string().as_bytes());
    hasher.update(b":");
    hasher.update(decision.fingerprint.as_bytes());
    hasher.update(b":");
    hasher.update(decision_type.as_bytes());
    let seed = hasher.finalize();
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&seed[..16]);
    bytes[6] = (bytes[6] & 0x0f) | 0x70;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    UnresolvedDecisionItem {
        unresolved_id: underlay_core::Uuid(underlay_core::RawUuid::from_bytes(bytes)),
        fingerprint: decision.fingerprint.clone(),
        decision_type: decision_type.to_string(),
        confidence: decision.confidence,
        threshold,
        reason: reason.to_string(),
        provenance: decision.provenance.clone(),
    }
}

async fn persist_stage_output<R, T>(
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

async fn load_stage_output<R, T>(
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
