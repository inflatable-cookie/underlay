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
    strict_determinism: bool,
    fail_on_unresolved_decisions: bool,
    ai_threshold_policy: AiThresholdPolicy,
    #[serde(default)]
    verification_rules: Vec<VerificationRule>,
    integrity_policy: IntegrityPolicy,
    integrity_evidence: IntegrityEvidence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiThresholdPolicy {
    default_confidence_threshold: f64,
    decision_type_overrides: HashMap<String, f64>,
}

impl AiThresholdPolicy {
    pub fn new(default_confidence_threshold: f64) -> Self {
        Self {
            default_confidence_threshold,
            decision_type_overrides: HashMap::new(),
        }
    }

    pub fn with_default_confidence_threshold(mut self, threshold: f64) -> Self {
        self.default_confidence_threshold = threshold;
        self
    }

    pub fn with_decision_type_override(
        mut self,
        decision_type: impl Into<String>,
        threshold: f64,
    ) -> Self {
        self.decision_type_overrides
            .insert(decision_type.into(), threshold);
        self
    }

    pub fn threshold_for(&self, decision_type: &str) -> f64 {
        self.decision_type_overrides
            .get(decision_type)
            .copied()
            .unwrap_or(self.default_confidence_threshold)
    }

    pub fn default_confidence_threshold(&self) -> f64 {
        self.default_confidence_threshold
    }

    pub fn decision_type_overrides(&self) -> &HashMap<String, f64> {
        &self.decision_type_overrides
    }
}

impl Default for AiThresholdPolicy {
    fn default() -> Self {
        Self::new(0.92)
    }
}

impl Default for PipelinePolicy {
    fn default() -> Self {
        Self {
            strict_determinism: true,
            fail_on_unresolved_decisions: true,
            ai_threshold_policy: AiThresholdPolicy::default(),
            verification_rules: Vec::new(),
            integrity_policy: IntegrityPolicy::default(),
            integrity_evidence: IntegrityEvidence::default(),
        }
    }
}

impl PipelinePolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_strict_determinism(mut self, strict_determinism: bool) -> Self {
        self.strict_determinism = strict_determinism;
        self
    }

    pub fn with_fail_on_unresolved_decisions(mut self, fail: bool) -> Self {
        self.fail_on_unresolved_decisions = fail;
        self
    }

    pub fn with_ai_threshold_policy(mut self, policy: AiThresholdPolicy) -> Self {
        self.ai_threshold_policy = policy;
        self
    }

    pub fn with_verification_rules(mut self, rules: Vec<VerificationRule>) -> Self {
        self.verification_rules = rules;
        self
    }

    pub fn with_integrity_policy(mut self, policy: IntegrityPolicy) -> Self {
        self.integrity_policy = policy;
        self
    }

    pub fn with_integrity_evidence(mut self, evidence: IntegrityEvidence) -> Self {
        self.integrity_evidence = evidence;
        self
    }

    pub fn strict_determinism(&self) -> bool {
        self.strict_determinism
    }

    pub fn fail_on_unresolved_decisions(&self) -> bool {
        self.fail_on_unresolved_decisions
    }

    pub fn ai_threshold_policy(&self) -> &AiThresholdPolicy {
        &self.ai_threshold_policy
    }

    pub fn verification_rules(&self) -> &[VerificationRule] {
        &self.verification_rules
    }

    pub fn integrity_policy(&self) -> &IntegrityPolicy {
        &self.integrity_policy
    }

    pub fn integrity_evidence(&self) -> &IntegrityEvidence {
        &self.integrity_evidence
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
