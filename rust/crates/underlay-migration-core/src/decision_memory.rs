use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use underlay_core::Uuid;

use crate::errors::{MigrationError, MigrationResult};
use crate::plugin::{DecisionFingerprintInput, DecisionReusePolicy};
use crate::run_store::{DecisionJournalRecord, UnresolvedDecisionRecord};

const SHA256_PREFIX: &str = "sha256:";
const DECISION_INDEX_SCHEMA_V1: &str = "1";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecordFingerprintInput {
    pub canonical_transform_input: Value,
    pub source_identity: String,
    pub semantic_dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecisionIndexEntry {
    pub bundle_digest: String,
    pub decision_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecisionIndex {
    pub schema_version: String,
    pub bundle_digest: String,
    pub entries: BTreeMap<String, DecisionIndexEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionInvalidationReason {
    FingerprintMismatch,
    ResolverVersionMismatch,
    PromptVersionMismatch,
    TargetSchemaVersionMismatch,
    PluginDependencyChanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecisionReuseEvaluation {
    pub reusable: bool,
    pub reason: Option<DecisionInvalidationReason>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecisionProvenanceEvent {
    pub decision_id: Uuid,
    pub fingerprint: String,
    pub provenance: String,
    pub resolver_version: String,
    pub prompt_version: String,
    pub target_schema_version: String,
    pub created_at: DateTime<Utc>,
}

pub fn decision_fingerprint(input: &DecisionFingerprintInput) -> MigrationResult<String> {
    let canonical_input = canonical_json_string(&input.canonical_decision_input)?;
    let mut hasher = Sha256::new();
    hasher.update(b"canonical_decision_input=");
    hasher.update(canonical_input.as_bytes());
    hasher.update(b"\ndecision_type=");
    hasher.update(input.decision_type.as_bytes());
    hasher.update(b"\nresolver_version=");
    hasher.update(input.resolver_version.as_bytes());
    hasher.update(b"\nprompt_version=");
    hasher.update(input.prompt_version.as_bytes());
    hasher.update(b"\ntarget_schema_version=");
    hasher.update(input.target_schema_version.as_bytes());
    Ok(format!("{SHA256_PREFIX}{:x}", hasher.finalize()))
}

pub fn record_fingerprint(input: &RecordFingerprintInput) -> MigrationResult<String> {
    let canonical_transform = canonical_json_string(&input.canonical_transform_input)?;
    let semantic_dependencies = canonical_json_string(
        &serde_json::to_value(&input.semantic_dependencies)
            .map_err(|err| MigrationError::Serialization(err.to_string()))?,
    )?;

    let mut hasher = Sha256::new();
    hasher.update(b"canonical_transform_input=");
    hasher.update(canonical_transform.as_bytes());
    hasher.update(b"\nsource_identity=");
    hasher.update(input.source_identity.as_bytes());
    hasher.update(b"\nsemantic_dependencies=");
    hasher.update(semantic_dependencies.as_bytes());
    Ok(format!("{SHA256_PREFIX}{:x}", hasher.finalize()))
}

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

pub fn evaluate_decision_reuse(
    record: &DecisionJournalRecord,
    expected_fingerprint: &str,
    input: &DecisionFingerprintInput,
    reuse_policy: DecisionReusePolicy,
    plugin_invalidated: bool,
) -> DecisionReuseEvaluation {
    if plugin_invalidated {
        return DecisionReuseEvaluation {
            reusable: false,
            reason: Some(DecisionInvalidationReason::PluginDependencyChanged),
        };
    }

    if record.fingerprint != expected_fingerprint {
        return DecisionReuseEvaluation {
            reusable: false,
            reason: Some(DecisionInvalidationReason::FingerprintMismatch),
        };
    }

    if record.target_schema_version != input.target_schema_version {
        return DecisionReuseEvaluation {
            reusable: false,
            reason: Some(DecisionInvalidationReason::TargetSchemaVersionMismatch),
        };
    }

    match reuse_policy {
        DecisionReusePolicy::Strict => {
            if record.resolver_version != input.resolver_version {
                return DecisionReuseEvaluation {
                    reusable: false,
                    reason: Some(DecisionInvalidationReason::ResolverVersionMismatch),
                };
            }

            if record.prompt_version != input.prompt_version {
                return DecisionReuseEvaluation {
                    reusable: false,
                    reason: Some(DecisionInvalidationReason::PromptVersionMismatch),
                };
            }
        }
        DecisionReusePolicy::Compatible => {
            if !is_version_compatible(&record.resolver_version, &input.resolver_version) {
                return DecisionReuseEvaluation {
                    reusable: false,
                    reason: Some(DecisionInvalidationReason::ResolverVersionMismatch),
                };
            }
            if !is_version_compatible(&record.prompt_version, &input.prompt_version) {
                return DecisionReuseEvaluation {
                    reusable: false,
                    reason: Some(DecisionInvalidationReason::PromptVersionMismatch),
                };
            }
        }
    }

    DecisionReuseEvaluation {
        reusable: true,
        reason: None,
    }
}

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

fn canonical_json_string(value: &Value) -> MigrationResult<String> {
    let normalized = normalize_value(value);
    serde_json::to_string(&normalized).map_err(|err| MigrationError::Serialization(err.to_string()))
}

fn normalize_value(value: &Value) -> Value {
    match value {
        Value::Object(obj) => {
            let mut sorted = Map::new();
            let mut keys = obj.keys().collect::<Vec<_>>();
            keys.sort();
            for key in keys {
                if let Some(child) = obj.get(key) {
                    sorted.insert(key.clone(), normalize_value(child));
                }
            }
            Value::Object(sorted)
        }
        Value::Array(values) => Value::Array(values.iter().map(normalize_value).collect()),
        _ => value.clone(),
    }
}

fn validate_sha256_digest(value: &str, field: &str) -> MigrationResult<()> {
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

fn is_version_compatible(stored: &str, current: &str) -> bool {
    if stored == current {
        return true;
    }

    // Compatible mode allows semver-major matching for declared version strings.
    let stored_major = parse_major_version(stored);
    let current_major = parse_major_version(current);
    match (stored_major, current_major) {
        (Some(lhs), Some(rhs)) => lhs == rhs,
        _ => false,
    }
}

fn parse_major_version(value: &str) -> Option<u64> {
    let trimmed = value.strip_prefix('v').unwrap_or(value);
    let major = trimmed.split('.').next()?;
    major.parse::<u64>().ok()
}

fn provenance_rank(provenance: &str) -> u8 {
    match provenance {
        "human" => 3,
        "rule" => 2,
        "ai" => 1,
        _ => 0,
    }
}

#[cfg(test)]
#[path = "tests/decision_memory_tests.rs"]
mod tests;
