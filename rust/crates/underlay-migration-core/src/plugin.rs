use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::MigrationContext;
use crate::errors::MigrationResult;
use crate::verification::{VerificationInput, VerificationIssue};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DecisionReusePolicy {
    Strict,
    Compatible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecisionFingerprintInput {
    pub canonical_decision_input: Value,
    pub decision_type: String,
    pub resolver_version: String,
    pub prompt_version: String,
    pub target_schema_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DecisionProvenance {
    Rule,
    Ai,
    Human,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecisionOutcome {
    pub fingerprint: String,
    pub outcome: Value,
    pub confidence: Option<f64>,
    pub provenance: DecisionProvenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LegacyRecordBatch {
    pub source_table: String,
    pub records: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NormalizedBatch {
    pub records: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransformBatch {
    pub records: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MaterializeResult {
    pub inserted: u64,
    pub updated: u64,
    pub skipped: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssetResolution {
    pub resolved_count: u64,
    pub unresolved_count: u64,
}

#[async_trait]
pub trait LegacySource: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn extract(&self, ctx: &MigrationContext) -> Result<Vec<LegacyRecordBatch>, Self::Error>;
}

#[async_trait]
pub trait MigrationPlugin: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    fn plugin_version(&self) -> &str;

    async fn normalize(
        &self,
        ctx: &MigrationContext,
        batch: LegacyRecordBatch,
    ) -> Result<NormalizedBatch, Self::Error>;

    async fn transform(
        &self,
        ctx: &MigrationContext,
        batch: NormalizedBatch,
    ) -> Result<TransformBatch, Self::Error>;

    async fn materialize(
        &self,
        ctx: &MigrationContext,
        batch: TransformBatch,
    ) -> Result<MaterializeResult, Self::Error>;

    fn should_invalidate_decision(
        &self,
        _ctx: &MigrationContext,
        _fingerprint: &str,
        _reuse_policy: DecisionReusePolicy,
    ) -> bool {
        false
    }

    async fn verify_semantics(
        &self,
        _ctx: &MigrationContext,
        _input: &VerificationInput,
    ) -> Result<Vec<VerificationIssue>, Self::Error> {
        Ok(Vec::new())
    }
}

#[async_trait]
pub trait DecisionResolver: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    fn resolver_version(&self) -> &str;

    async fn fingerprint(&self, input: &DecisionFingerprintInput) -> MigrationResult<String>;

    async fn resolve(
        &self,
        ctx: &MigrationContext,
        input: DecisionFingerprintInput,
        reuse_policy: DecisionReusePolicy,
    ) -> Result<DecisionOutcome, Self::Error>;
}

#[async_trait]
pub trait AssetResolver: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn resolve_assets(
        &self,
        ctx: &MigrationContext,
        batch: &TransformBatch,
    ) -> Result<AssetResolution, Self::Error>;
}
