use crate::context::MigrationContext;
use crate::errors::{MigrationError, MigrationResult};
use crate::plugin::{DecisionOutcome, DecisionProvenance};
use crate::run_store::DecisionJournalRecord;

use super::types::UnresolvedDecisionItem;

pub(super) fn parse_provenance(
    record: &DecisionJournalRecord,
) -> MigrationResult<DecisionProvenance> {
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

pub(super) fn decision_provenance_label(provenance: &DecisionProvenance) -> &'static str {
    match provenance {
        DecisionProvenance::Rule => "rule",
        DecisionProvenance::Ai => "ai",
        DecisionProvenance::Human => "human",
    }
}

pub(super) fn low_confidence_reason(
    decision: &DecisionOutcome,
    threshold: f64,
) -> Option<&'static str> {
    if decision.provenance != DecisionProvenance::Ai {
        return None;
    }

    match decision.confidence {
        Some(value) if value >= threshold => None,
        Some(_) => Some("low_confidence_ai"),
        None => Some("missing_confidence_ai"),
    }
}

pub(super) fn build_unresolved(
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
