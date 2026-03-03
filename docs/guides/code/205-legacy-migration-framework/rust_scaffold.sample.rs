use async_trait::async_trait;
use serde_json::Value;
use underlay_migration_core::{
    AssetResolution, AssetResolver, DecisionFingerprintInput, DecisionOutcome, DecisionResolver,
    DecisionReusePolicy, LegacyRecordBatch, LegacySource, MaterializeResult, MigrationContext,
    MigrationError, MigrationPlugin, NormalizedBatch, TransformBatch, VerificationInput,
    VerificationIssue,
};

pub struct AppLegacySource;
pub struct AppMigrationPlugin;
pub struct AppDecisionResolver;
pub struct AppAssetResolver;

#[async_trait]
impl LegacySource for AppLegacySource {
    type Error = MigrationError;

    async fn extract(
        &self,
        _ctx: &MigrationContext,
    ) -> Result<Vec<LegacyRecordBatch>, Self::Error> {
        Ok(vec![LegacyRecordBatch {
            source_table: "legacy.users".to_string(),
            records: vec![serde_json::json!({"id": 1, "email": "user@example.com"})],
        }])
    }
}

#[async_trait]
impl MigrationPlugin for AppMigrationPlugin {
    type Error = MigrationError;

    fn plugin_version(&self) -> &str {
        "app-plugin-v1"
    }

    async fn normalize(
        &self,
        _ctx: &MigrationContext,
        batch: LegacyRecordBatch,
    ) -> Result<NormalizedBatch, Self::Error> {
        Ok(NormalizedBatch {
            records: batch.records,
        })
    }

    async fn transform(
        &self,
        _ctx: &MigrationContext,
        batch: NormalizedBatch,
    ) -> Result<TransformBatch, Self::Error> {
        Ok(TransformBatch {
            records: batch.records,
        })
    }

    async fn materialize(
        &self,
        _ctx: &MigrationContext,
        batch: TransformBatch,
    ) -> Result<MaterializeResult, Self::Error> {
        Ok(MaterializeResult {
            inserted: batch.records.len() as u64,
            updated: 0,
            skipped: 0,
        })
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
impl DecisionResolver for AppDecisionResolver {
    type Error = MigrationError;

    fn resolver_version(&self) -> &str {
        "resolver-v1"
    }

    async fn fingerprint(&self, input: &DecisionFingerprintInput) -> Result<String, MigrationError> {
        underlay_migration_core::decision_fingerprint(input)
            .map_err(|err| MigrationError::Plugin(err.to_string()))
    }

    async fn resolve(
        &self,
        _ctx: &MigrationContext,
        input: DecisionFingerprintInput,
        _reuse_policy: DecisionReusePolicy,
    ) -> Result<DecisionOutcome, Self::Error> {
        let fingerprint = underlay_migration_core::decision_fingerprint(&input)
            .map_err(|err| MigrationError::Plugin(err.to_string()))?;
        Ok(DecisionOutcome {
            fingerprint,
            outcome: Value::Object(Default::default()),
            confidence: Some(0.95),
            provenance: underlay_migration_core::DecisionProvenance::Ai,
        })
    }
}

#[async_trait]
impl AssetResolver for AppAssetResolver {
    type Error = MigrationError;

    async fn resolve_assets(
        &self,
        _ctx: &MigrationContext,
        _batch: &TransformBatch,
    ) -> Result<AssetResolution, Self::Error> {
        Ok(AssetResolution {
            resolved_count: 0,
            unresolved_count: 0,
        })
    }
}
