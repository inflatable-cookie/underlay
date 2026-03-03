use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use underlay_core::Uuid;

use crate::pipeline::StageName;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StageCheckpoint {
    pub run_id: Uuid,
    pub stage: StageName,
    pub plugin_version: String,
    pub target_schema_version: String,
    pub cursor: Option<String>,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResumeCheckpoint {
    pub run_id: Uuid,
    pub last_completed_stage: StageName,
    pub plugin_version: String,
    pub target_schema_version: String,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StageSnapshot {
    pub run_id: Uuid,
    pub stage: StageName,
    pub payload: Value,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecisionJournalRecord {
    pub decision_id: Uuid,
    pub fingerprint: String,
    pub decision_type: String,
    pub outcome: serde_json::Value,
    pub confidence: Option<f64>,
    pub resolver_version: String,
    pub prompt_version: String,
    #[serde(default)]
    pub target_schema_version: String,
    pub created_at: DateTime<Utc>,
    pub provenance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnresolvedDecisionRecord {
    pub unresolved_id: Uuid,
    pub run_id: Uuid,
    pub fingerprint: String,
    pub decision_type: String,
    pub provenance: String,
    pub confidence: Option<f64>,
    pub threshold: f64,
    pub reason: String,
    pub canonical_decision_input: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunSummary {
    pub run_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub inserted: u64,
    pub updated: u64,
    pub skipped: u64,
    pub decision_count: u64,
}

#[async_trait]
pub trait RunStore: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn write_stage_checkpoint(&self, checkpoint: StageCheckpoint) -> Result<(), Self::Error>;

    async fn latest_resume_checkpoint(
        &self,
        run_id: Uuid,
    ) -> Result<Option<ResumeCheckpoint>, Self::Error>;

    async fn append_decision_journal(
        &self,
        record: DecisionJournalRecord,
    ) -> Result<(), Self::Error>;

    async fn write_summary(&self, summary: RunSummary) -> Result<(), Self::Error>;

    async fn latest_decision(
        &self,
        fingerprint: &str,
    ) -> Result<Option<DecisionJournalRecord>, Self::Error>;

    async fn decisions_for_fingerprint(
        &self,
        fingerprint: &str,
    ) -> Result<Vec<DecisionJournalRecord>, Self::Error> {
        Ok(self
            .latest_decision(fingerprint)
            .await?
            .into_iter()
            .collect())
    }

    async fn append_unresolved_decision(
        &self,
        record: UnresolvedDecisionRecord,
    ) -> Result<(), Self::Error>;

    async fn write_stage_snapshot(&self, snapshot: StageSnapshot) -> Result<(), Self::Error>;

    async fn read_stage_snapshot(
        &self,
        run_id: Uuid,
        stage: StageName,
    ) -> Result<Option<StageSnapshot>, Self::Error>;
}
