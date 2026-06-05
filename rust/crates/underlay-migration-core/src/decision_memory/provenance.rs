use crate::run_store::DecisionJournalRecord;

use super::models::DecisionProvenanceEvent;

pub fn effective_decision_for_fingerprint<'a>(
    records: &'a [DecisionJournalRecord],
    fingerprint: &str,
) -> Option<&'a DecisionJournalRecord> {
    records
        .iter()
        .filter(|record| record.fingerprint == fingerprint)
        .max_by_key(|record| (provenance_rank(&record.provenance), record.created_at))
}

pub fn provenance_chain_for_fingerprint(
    records: &[DecisionJournalRecord],
    fingerprint: &str,
) -> Vec<DecisionProvenanceEvent> {
    let mut chain = records
        .iter()
        .filter(|record| record.fingerprint == fingerprint)
        .map(|record| DecisionProvenanceEvent {
            decision_id: record.decision_id,
            fingerprint: record.fingerprint.clone(),
            provenance: record.provenance.clone(),
            resolver_version: record.resolver_version.clone(),
            prompt_version: record.prompt_version.clone(),
            target_schema_version: record.target_schema_version.clone(),
            created_at: record.created_at,
        })
        .collect::<Vec<_>>();
    chain.sort_by_key(|event| event.created_at);
    chain
}

fn provenance_rank(provenance: &str) -> u8 {
    match provenance {
        "human" => 3,
        "rule" => 2,
        "ai" => 1,
        _ => 0,
    }
}
