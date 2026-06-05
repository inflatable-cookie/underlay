use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{
    AssetResolution, AssetResolver, DecisionFingerprintInput, DecisionJournalRecord,
    DecisionOutcome, DecisionProvenance, DecisionResolver, DecisionReusePolicy, LegacyRecordBatch,
    LegacySource, MaterializeResult, MigrationContext, MigrationPlugin, ResumeCheckpoint, RunStore,
    RunSummary, StageCheckpoint, StageName, StageSnapshot, UnresolvedDecisionRecord,
    VerificationInput, VerificationIssue, VerificationSeverity,
};

pub(super) fn decision_fingerprint_for(input: &DecisionFingerprintInput) -> String {
    let canonical = serde_json::to_string(&input.canonical_decision_input)
        .expect("canonical json should encode");
    let raw = format!(
        "canonical_decision_input={canonical}\ndecision_type={}\nresolver_version={}\nprompt_version={}\ntarget_schema_version={}",
        input.decision_type, input.resolver_version, input.prompt_version, input.target_schema_version
    );
    let digest = Sha256::digest(raw.as_bytes());
    format!("sha256:{digest:x}")
}
pub(super) struct MockSource {
    pub(super) calls: Arc<Mutex<u64>>,
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

pub(super) struct MockPlugin {
    pub(super) fail_transform: bool,
    pub(super) semantic_error: bool,
    pub(super) invalidate_decisions: bool,
    pub(super) normalize_calls: Arc<Mutex<u64>>,
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

pub(super) struct MockDecisionResolver {
    pub(super) resolve_calls: Arc<Mutex<u64>>,
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

pub(super) struct MockAssetResolver {}

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

#[derive(Clone, Default)]
pub(super) struct InMemoryRunStore {
    pub(super) checkpoints: Arc<Mutex<Vec<StageCheckpoint>>>,
    pub(super) snapshots: Arc<Mutex<HashMap<(underlay_core::Uuid, StageName), StageSnapshot>>>,
    pub(super) decision_journal: Arc<Mutex<Vec<DecisionJournalRecord>>>,
    pub(super) unresolved: Arc<Mutex<Vec<UnresolvedDecisionRecord>>>,
}

#[async_trait]
impl RunStore for InMemoryRunStore {
    type Error = io::Error;

    async fn write_stage_checkpoint(&self, checkpoint: StageCheckpoint) -> Result<(), Self::Error> {
        self.checkpoints
            .lock()
            .map_err(|_| io::Error::other("poisoned checkpoint lock"))?
            .push(checkpoint);
        Ok(())
    }

    async fn latest_resume_checkpoint(
        &self,
        run_id: underlay_core::Uuid,
    ) -> Result<Option<ResumeCheckpoint>, Self::Error> {
        let checkpoints = self
            .checkpoints
            .lock()
            .map_err(|_| io::Error::other("poisoned checkpoint lock"))?;

        let latest = checkpoints
            .iter()
            .rev()
            .find(|c| c.run_id == run_id)
            .cloned();
        Ok(latest.map(|checkpoint| ResumeCheckpoint {
            run_id,
            last_completed_stage: checkpoint.stage,
            plugin_version: checkpoint.plugin_version,
            target_schema_version: checkpoint.target_schema_version,
            cursor: checkpoint.cursor,
        }))
    }

    async fn append_decision_journal(
        &self,
        record: DecisionJournalRecord,
    ) -> Result<(), Self::Error> {
        self.decision_journal
            .lock()
            .map_err(|_| io::Error::other("poisoned decision journal lock"))?
            .push(record);
        Ok(())
    }

    async fn write_summary(&self, _summary: RunSummary) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn latest_decision(
        &self,
        fingerprint: &str,
    ) -> Result<Option<DecisionJournalRecord>, Self::Error> {
        let latest = self
            .decision_journal
            .lock()
            .map_err(|_| io::Error::other("poisoned decision journal lock"))?
            .iter()
            .rev()
            .find(|entry| entry.fingerprint == fingerprint)
            .cloned();
        Ok(latest)
    }

    async fn decisions_for_fingerprint(
        &self,
        fingerprint: &str,
    ) -> Result<Vec<DecisionJournalRecord>, Self::Error> {
        let records = self
            .decision_journal
            .lock()
            .map_err(|_| io::Error::other("poisoned decision journal lock"))?
            .iter()
            .filter(|entry| entry.fingerprint == fingerprint)
            .cloned()
            .collect::<Vec<_>>();
        Ok(records)
    }

    async fn append_unresolved_decision(
        &self,
        record: UnresolvedDecisionRecord,
    ) -> Result<(), Self::Error> {
        self.unresolved
            .lock()
            .map_err(|_| io::Error::other("poisoned unresolved lock"))?
            .push(record);
        Ok(())
    }

    async fn write_stage_snapshot(&self, snapshot: StageSnapshot) -> Result<(), Self::Error> {
        self.snapshots
            .lock()
            .map_err(|_| io::Error::other("poisoned snapshot lock"))?
            .insert((snapshot.run_id, snapshot.stage), snapshot);
        Ok(())
    }

    async fn read_stage_snapshot(
        &self,
        run_id: underlay_core::Uuid,
        stage: StageName,
    ) -> Result<Option<StageSnapshot>, Self::Error> {
        Ok(self
            .snapshots
            .lock()
            .map_err(|_| io::Error::other("poisoned snapshot lock"))?
            .get(&(run_id, stage))
            .cloned())
    }
}
