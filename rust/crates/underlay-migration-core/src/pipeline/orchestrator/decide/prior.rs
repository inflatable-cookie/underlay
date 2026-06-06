use crate::decision_memory::{
    effective_decision_for_fingerprint, evaluate_decision_reuse, validate_decision_journal_record,
};
use crate::errors::MigrationResult;
use crate::pipeline::decision_support::parse_provenance;
use crate::pipeline::errors::stage_failure;
use crate::pipeline::types::{DecisionGovernanceIssue, DecisionInvalidationEvent, StageName};
use crate::plugin::{DecisionFingerprintInput, DecisionOutcome, DecisionReusePolicy};
use crate::run_store::{DecisionJournalRecord, RunStore};

pub(super) struct PriorDecisionReuse {
    pub(super) decision: Option<DecisionOutcome>,
    pub(super) invalidation: Option<DecisionInvalidationEvent>,
}

pub(super) async fn load_valid_prior_chain<R>(
    run_store: &R,
    fingerprint: &str,
    governance_issues: &mut Vec<DecisionGovernanceIssue>,
) -> MigrationResult<Vec<DecisionJournalRecord>>
where
    R: RunStore,
{
    let prior_chain = run_store
        .decisions_for_fingerprint(fingerprint)
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

    Ok(valid_prior)
}

pub(super) fn evaluate_prior_decision(
    valid_prior: &[DecisionJournalRecord],
    fingerprint: &str,
    input: &DecisionFingerprintInput,
    reuse_policy: DecisionReusePolicy,
    plugin_invalidated: bool,
    decision_type: &str,
) -> MigrationResult<PriorDecisionReuse> {
    let Some(prior) = effective_decision_for_fingerprint(valid_prior, fingerprint) else {
        return Ok(PriorDecisionReuse {
            decision: None,
            invalidation: None,
        });
    };

    let reuse =
        evaluate_decision_reuse(prior, fingerprint, input, reuse_policy, plugin_invalidated);
    if reuse.reusable {
        let provenance = parse_provenance(prior)?;
        return Ok(PriorDecisionReuse {
            decision: Some(DecisionOutcome {
                fingerprint: prior.fingerprint.clone(),
                outcome: prior.outcome.clone(),
                confidence: prior.confidence,
                provenance,
            }),
            invalidation: None,
        });
    }

    Ok(PriorDecisionReuse {
        decision: None,
        invalidation: reuse.reason.map(|reason| DecisionInvalidationEvent {
            fingerprint: fingerprint.to_string(),
            reason,
            decision_type: decision_type.to_string(),
        }),
    })
}
