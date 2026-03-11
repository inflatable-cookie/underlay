use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::integrity::{IntegrityEvidence, IntegrityPolicy};
use crate::verification_rules::VerificationRule;

use underlay_core::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunMetadata {
    pub run_id: Uuid,
    pub bundle_id: Option<Uuid>,
    pub plugin_version: String,
    pub target_schema_version: String,
    pub created_at: DateTime<Utc>,
}

impl RunMetadata {
    pub fn new(
        plugin_version: impl Into<String>,
        target_schema_version: impl Into<String>,
    ) -> Self {
        Self {
            run_id: Uuid::new_v7(),
            bundle_id: None,
            plugin_version: plugin_version.into(),
            target_schema_version: target_schema_version.into(),
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelinePolicy {
    pub strict_determinism: bool,
    pub fail_on_unresolved_decisions: bool,
    pub ai_threshold_policy: AiThresholdPolicy,
    #[serde(default)]
    pub verification_rules: Vec<VerificationRule>,
    pub integrity_policy: IntegrityPolicy,
    pub integrity_evidence: IntegrityEvidence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiThresholdPolicy {
    pub default_confidence_threshold: f64,
    pub decision_type_overrides: HashMap<String, f64>,
}

impl AiThresholdPolicy {
    pub fn threshold_for(&self, decision_type: &str) -> f64 {
        self.decision_type_overrides
            .get(decision_type)
            .copied()
            .unwrap_or(self.default_confidence_threshold)
    }
}

impl Default for PipelinePolicy {
    fn default() -> Self {
        Self {
            strict_determinism: true,
            fail_on_unresolved_decisions: true,
            ai_threshold_policy: AiThresholdPolicy {
                default_confidence_threshold: 0.92,
                decision_type_overrides: HashMap::new(),
            },
            verification_rules: Vec::new(),
            integrity_policy: IntegrityPolicy::default(),
            integrity_evidence: IntegrityEvidence::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StageMetadata {
    pub stage_name: String,
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MigrationContext {
    pub run: RunMetadata,
    pub policy: PipelinePolicy,
}

impl MigrationContext {
    pub fn new(run: RunMetadata, policy: PipelinePolicy) -> Self {
        Self { run, policy }
    }
}
