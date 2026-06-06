use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use crate::{
    DecisionJournalRecord, ResumeCheckpoint, RunStore, RunSummary, StageCheckpoint, StageName,
    StageSnapshot, UnresolvedDecisionRecord,
};

#[derive(Clone, Default)]
pub(in crate::tests) struct InMemoryRunStore {
    pub(in crate::tests) checkpoints: Arc<Mutex<Vec<StageCheckpoint>>>,
    pub(in crate::tests) snapshots:
        Arc<Mutex<HashMap<(underlay_core::Uuid, StageName), StageSnapshot>>>,
    pub(in crate::tests) decision_journal: Arc<Mutex<Vec<DecisionJournalRecord>>>,
    pub(in crate::tests) unresolved: Arc<Mutex<Vec<UnresolvedDecisionRecord>>>,
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
