use std::collections::BTreeMap;

use crate::errors::{MigrationError, MigrationResult};
use crate::run_store::DecisionJournalRecord;

use super::models::{DecisionIndex, DecisionIndexEntry};
use super::validation::{
    validate_decision_index, validate_decision_journal_record, validate_sha256_digest,
};
use super::DECISION_INDEX_SCHEMA_V1;

pub fn parse_decision_journal_ndjson(ndjson: &str) -> MigrationResult<Vec<DecisionJournalRecord>> {
    let mut records = Vec::new();
    for (line_number, line) in ndjson.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let record = serde_json::from_str::<DecisionJournalRecord>(trimmed).map_err(|err| {
            MigrationError::Serialization(format!(
                "invalid decision_journal.ndjson at line {}: {}",
                line_number + 1,
                err
            ))
        })?;
        validate_decision_journal_record(&record)?;
        records.push(record);
    }
    Ok(records)
}

pub fn build_decision_index(
    bundle_digest: &str,
    records: &[DecisionJournalRecord],
) -> MigrationResult<DecisionIndex> {
    validate_sha256_digest(bundle_digest, "bundle_digest")?;
    let mut entries = BTreeMap::new();
    for record in records {
        validate_decision_journal_record(record)?;
        entries.insert(
            record.fingerprint.clone(),
            DecisionIndexEntry {
                bundle_digest: bundle_digest.to_string(),
                decision_id: record.decision_id,
                created_at: record.created_at,
            },
        );
    }

    let index = DecisionIndex {
        schema_version: DECISION_INDEX_SCHEMA_V1.to_string(),
        bundle_digest: bundle_digest.to_string(),
        entries,
    };
    validate_decision_index(&index)?;
    Ok(index)
}

pub fn merge_decision_indexes(indexes: &[DecisionIndex]) -> MigrationResult<DecisionIndex> {
    let mut merged_entries: BTreeMap<String, DecisionIndexEntry> = BTreeMap::new();

    for index in indexes {
        validate_decision_index(index)?;
        for (fingerprint, candidate) in &index.entries {
            let should_replace = merged_entries
                .get(fingerprint)
                .map(|existing| candidate.created_at >= existing.created_at)
                .unwrap_or(true);
            if should_replace {
                merged_entries.insert(fingerprint.clone(), candidate.clone());
            }
        }
    }

    let bundle_digest = indexes
        .last()
        .map(|index| index.bundle_digest.clone())
        .unwrap_or_else(|| {
            "sha256:0000000000000000000000000000000000000000000000000000000000000000".to_string()
        });

    let merged = DecisionIndex {
        schema_version: DECISION_INDEX_SCHEMA_V1.to_string(),
        bundle_digest,
        entries: merged_entries,
    };
    validate_decision_index(&merged)?;
    Ok(merged)
}

pub fn parse_decision_index(json: &str) -> MigrationResult<DecisionIndex> {
    let index = serde_json::from_str::<DecisionIndex>(json)
        .map_err(|err| MigrationError::Serialization(err.to_string()))?;
    validate_decision_index(&index)?;
    Ok(index)
}
