use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use underlay_core::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BundleManifest {
    pub bundle_id: Uuid,
    pub bundle_version: String,
    pub source_system: String,
    pub target_schema_version: String,
    pub created_at: DateTime<Utc>,
    pub tables: Vec<SourceTableManifest>,
    pub assets: Vec<AssetManifestItem>,
    pub stages: Vec<StageManifest>,
    pub decision_policy: DecisionPolicyConfig,
    pub replay_contract: ReplayContract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SourceTableManifest {
    pub source_table: String,
    pub row_count: u64,
    pub chunk_count: u64,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssetManifestItem {
    pub source_uri: String,
    pub sha256: String,
    pub byte_size: u64,
    pub mime_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StageManifest {
    pub name: String,
    pub depends_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecisionPolicyConfig {
    pub reuse_policy: String,
    pub default_confidence_threshold: f64,
    pub prompt_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplayContract {
    pub require_digest_pinning: bool,
    pub required_env: Vec<String>,
    pub compatibility_notes: Option<String>,
}
