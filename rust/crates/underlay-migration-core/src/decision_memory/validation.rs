use crate::errors::{MigrationError, MigrationResult};
use crate::run_store::{DecisionJournalRecord, UnresolvedDecisionRecord};

use super::models::DecisionIndex;
use super::{DECISION_INDEX_SCHEMA_V1, SHA256_PREFIX};

pub fn validate_decision_journal_record(record: &DecisionJournalRecord) -> MigrationResult<()> {
    if record.decision_type.trim().is_empty() {
        return Err(MigrationError::DeterminismViolation(
            "decision_journal record missing decision_type".to_string(),
        ));
    }

    validate_sha256_digest(&record.fingerprint, "fingerprint")?;

    if let Some(confidence) = record.confidence {
        if !(0.0..=1.0).contains(&confidence) {
            return Err(MigrationError::DeterminismViolation(format!(
                "decision_journal confidence out of range: {}",
                confidence
            )));
        }
    }

    match record.provenance.as_str() {
        "rule" | "ai" | "human" => Ok(()),
        other => Err(MigrationError::DeterminismViolation(format!(
            "decision_journal provenance must be rule|ai|human, found {}",
            other
        ))),
    }
}

pub fn validate_unresolved_decision_record(
    record: &UnresolvedDecisionRecord,
) -> MigrationResult<()> {
    validate_sha256_digest(&record.fingerprint, "fingerprint")?;
    if record.decision_type.trim().is_empty() {
        return Err(MigrationError::DeterminismViolation(
            "unresolved decision missing decision_type".to_string(),
        ));
    }

    if !(0.0..=1.0).contains(&record.threshold) {
        return Err(MigrationError::DeterminismViolation(format!(
            "unresolved decision threshold out of range: {}",
            record.threshold
        )));
    }

    if let Some(confidence) = record.confidence {
        if !(0.0..=1.0).contains(&confidence) {
            return Err(MigrationError::DeterminismViolation(format!(
                "unresolved decision confidence out of range: {}",
                confidence
            )));
        }
    }

    if record.reason.trim().is_empty() {
        return Err(MigrationError::DeterminismViolation(
            "unresolved decision reason must not be empty".to_string(),
        ));
    }

    match record.provenance.as_str() {
        "rule" | "ai" | "human" => Ok(()),
        other => Err(MigrationError::DeterminismViolation(format!(
            "unresolved decision provenance must be rule|ai|human, found {}",
            other
        ))),
    }
}

pub fn validate_decision_index(index: &DecisionIndex) -> MigrationResult<()> {
    if index.schema_version != DECISION_INDEX_SCHEMA_V1 {
        return Err(MigrationError::DeterminismViolation(format!(
            "unsupported decision_index schema_version {}",
            index.schema_version
        )));
    }

    validate_sha256_digest(&index.bundle_digest, "bundle_digest")?;

    for (fingerprint, entry) in &index.entries {
        validate_sha256_digest(fingerprint, "entry fingerprint")?;
        validate_sha256_digest(&entry.bundle_digest, "entry bundle_digest")?;
    }

    Ok(())
}

pub(super) fn validate_sha256_digest(value: &str, field: &str) -> MigrationResult<()> {
    if !value.starts_with(SHA256_PREFIX) || value.len() != SHA256_PREFIX.len() + 64 {
        return Err(MigrationError::DeterminismViolation(format!(
            "{} must be sha256:<64-hex>",
            field
        )));
    }
    if !value[SHA256_PREFIX.len()..]
        .bytes()
        .all(|ch| ch.is_ascii_hexdigit())
    {
        return Err(MigrationError::DeterminismViolation(format!(
            "{} must be sha256:<64-hex>",
            field
        )));
    }
    Ok(())
}
