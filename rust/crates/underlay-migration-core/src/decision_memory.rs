mod fingerprint;
mod index;
mod models;
mod provenance;
mod reuse;
mod validation;

pub use fingerprint::{decision_fingerprint, record_fingerprint};
pub use index::{
    build_decision_index, merge_decision_indexes, parse_decision_index,
    parse_decision_journal_ndjson,
};
pub use models::{
    DecisionIndex, DecisionIndexEntry, DecisionInvalidationReason, DecisionProvenanceEvent,
    DecisionReuseEvaluation, RecordFingerprintInput,
};
pub use provenance::{effective_decision_for_fingerprint, provenance_chain_for_fingerprint};
pub use reuse::evaluate_decision_reuse;
pub use validation::{
    validate_decision_index, validate_decision_journal_record, validate_unresolved_decision_record,
};

const SHA256_PREFIX: &str = "sha256:";
const DECISION_INDEX_SCHEMA_V1: &str = "1";

#[cfg(test)]
#[path = "tests/decision_memory_tests.rs"]
mod tests;
