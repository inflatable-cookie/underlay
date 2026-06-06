use std::io;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde_json::json;

use crate::{
    AssetResolution, AssetResolver, DecisionFingerprintInput, DecisionOutcome, DecisionProvenance,
    DecisionResolver, DecisionReusePolicy, LegacyRecordBatch, LegacySource, MaterializeResult,
    MigrationContext, MigrationPlugin, VerificationInput, VerificationIssue, VerificationSeverity,
};

use super::fingerprint::decision_fingerprint_for;

pub(in crate::tests) struct MockSource {
    pub(in crate::tests) calls: Arc<Mutex<u64>>,
}

#[async_trait]
impl LegacySource for MockSource {
    type Error = io::Error;

    async fn extract(
        &self,
        _ctx: &MigrationContext,
    ) -> Result<Vec<LegacyRecordBatch>, Self::Error> {
        let mut count = self
            .calls
            .lock()
            .map_err(|_| io::Error::other("lock poisoned"))?;
        *count += 1;

        Ok(vec![LegacyRecordBatch {
            source_table: "legacy.users".to_string(),
            records: vec![json!({ "id": 1 }), json!({ "id": 2 })],
        }])
    }
}

pub(in crate::tests) struct MockPlugin {
    pub(in crate::tests) fail_transform: bool,
    pub(in crate::tests) semantic_error: bool,
    pub(in crate::tests) invalidate_decisions: bool,
    pub(in crate::tests) normalize_calls: Arc<Mutex<u64>>,
}

#[async_trait]
impl MigrationPlugin for MockPlugin {
    type Error = io::Error;

    fn plugin_version(&self) -> &str {
        "mock-plugin-v1"
    }

    async fn normalize(
        &self,
        _ctx: &MigrationContext,
        batch: LegacyRecordBatch,
    ) -> Result<crate::NormalizedBatch, Self::Error> {
        let mut count = self
            .normalize_calls
            .lock()
            .map_err(|_| io::Error::other("lock poisoned"))?;
        *count += 1;

        Ok(crate::NormalizedBatch {
            records: batch.records,
        })
    }

    async fn transform(
        &self,
        _ctx: &MigrationContext,
        batch: crate::NormalizedBatch,
    ) -> Result<crate::TransformBatch, Self::Error> {
        if self.fail_transform {
            return Err(io::Error::other("transform failed"));
        }

        Ok(crate::TransformBatch {
            records: batch.records,
        })
    }

    async fn materialize(
        &self,
        _ctx: &MigrationContext,
        batch: crate::TransformBatch,
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
        if self.semantic_error {
            return Ok(vec![VerificationIssue {
                code: "plugin_semantic_error".to_string(),
                message: "plugin semantic verification failed".to_string(),
                severity: VerificationSeverity::Error,
            }]);
        }

        Ok(Vec::new())
    }

    fn should_invalidate_decision(
        &self,
        _ctx: &MigrationContext,
        _fingerprint: &str,
        _reuse_policy: DecisionReusePolicy,
    ) -> bool {
        self.invalidate_decisions
    }
}

pub(in crate::tests) struct MockDecisionResolver {
    pub(in crate::tests) resolve_calls: Arc<Mutex<u64>>,
}

#[async_trait]
impl DecisionResolver for MockDecisionResolver {
    type Error = io::Error;

    fn resolver_version(&self) -> &str {
        "mock-resolver-v1"
    }

    async fn fingerprint(
        &self,
        input: &DecisionFingerprintInput,
    ) -> crate::MigrationResult<String> {
        Ok(decision_fingerprint_for(input))
    }

    async fn resolve(
        &self,
        _ctx: &MigrationContext,
        input: DecisionFingerprintInput,
        _reuse_policy: DecisionReusePolicy,
    ) -> Result<DecisionOutcome, Self::Error> {
        let mut count = self
            .resolve_calls
            .lock()
            .map_err(|_| io::Error::other("lock poisoned"))?;
        *count += 1;

        let fingerprint = decision_fingerprint_for(&input);
        Ok(DecisionOutcome {
            fingerprint,
            outcome: input.canonical_decision_input,
            confidence: Some(0.99),
            provenance: DecisionProvenance::Ai,
        })
    }
}

pub(in crate::tests) struct MockAssetResolver {}

#[async_trait]
impl AssetResolver for MockAssetResolver {
    type Error = io::Error;

    async fn resolve_assets(
        &self,
        _ctx: &MigrationContext,
        batch: &crate::TransformBatch,
    ) -> Result<AssetResolution, Self::Error> {
        Ok(AssetResolution {
            resolved_count: batch.records.len() as u64,
            unresolved_count: 0,
        })
    }
}
