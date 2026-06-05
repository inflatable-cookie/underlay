use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use underlay_core::Uuid;

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
