use chrono::Utc;
use serde_json::Value;

use crate::context::MigrationContext;
use crate::decision_memory::{
    validate_decision_journal_record, validate_unresolved_decision_record,
};
use crate::errors::MigrationResult;
use crate::pipeline::decision_support::{
    build_unresolved, decision_provenance_label, low_confidence_reason,
};
use crate::pipeline::errors::stage_failure;
use crate::pipeline::types::{DecisionGovernanceIssue, StageName, UnresolvedDecisionItem};
use crate::plugin::DecisionOutcome;
use crate::run_store::{DecisionJournalRecord, RunStore, UnresolvedDecisionRecord};

use super::input::DecisionCandidate;

pub(super) async fn append_decision_journal<R>(
    ctx: &MigrationContext,
    run_store: &R,
    decision: &DecisionOutcome,
    candidate: &DecisionCandidate,
    fingerprint: &str,
    resolver_version: &str,
    prompt_version: &str,
    governance_issues: &mut Vec<DecisionGovernanceIssue>,
) -> MigrationResult<()>
where
    R: RunStore,
{
    let journal_record = DecisionJournalRecord {
        decision_id: underlay_core::Uuid::new_v7(),
        fingerprint: fingerprint.to_string(),
        decision_type: candidate.decision_type.clone(),
        outcome: decision.outcome.clone(),
        confidence: decision.confidence,
        resolver_version: resolver_version.to_string(),
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
        return Ok(());
    }

    run_store
        .append_decision_journal(journal_record)
        .await
        .map_err(|err| stage_failure(StageName::Decide, err.to_string()))
}

pub(super) async fn append_unresolved_if_low_confidence<R>(
    ctx: &MigrationContext,
    run_store: &R,
    decision: &mut DecisionOutcome,
    candidate: &DecisionCandidate,
    governance_issues: &mut Vec<DecisionGovernanceIssue>,
    unresolved_queue: &mut Vec<UnresolvedDecisionItem>,
) -> MigrationResult<()>
where
    R: RunStore,
{
    let Some(unresolved_reason) = low_confidence_reason(decision, candidate.threshold) else {
        return Ok(());
    };

    let unresolved = build_unresolved(
        ctx,
        decision,
        &candidate.decision_type,
        candidate.threshold,
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
        canonical_decision_input: candidate.canonical_decision_input.clone(),
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
    Ok(())
}
