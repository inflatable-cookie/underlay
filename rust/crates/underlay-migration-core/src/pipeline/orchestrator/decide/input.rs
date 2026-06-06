use serde_json::Value;

use crate::context::MigrationContext;
use crate::plugin::DecisionFingerprintInput;

const DECISION_TYPE: &str = "migration_record_resolution";

pub(super) struct DecisionCandidate {
    pub(super) decision_type: String,
    pub(super) threshold: f64,
    pub(super) input: DecisionFingerprintInput,
    pub(super) canonical_decision_input: Value,
}

pub(super) fn build_decision_candidate(
    ctx: &MigrationContext,
    record: &Value,
    resolver_version: &str,
    prompt_version: &str,
) -> DecisionCandidate {
    let decision_type = DECISION_TYPE.to_string();
    let threshold = ctx
        .policy
        .ai_threshold_policy()
        .threshold_for(&decision_type);
    let input = DecisionFingerprintInput {
        canonical_decision_input: record.clone(),
        decision_type: decision_type.clone(),
        resolver_version: resolver_version.to_string(),
        prompt_version: prompt_version.to_string(),
        target_schema_version: ctx.run.target_schema_version.clone(),
    };

    DecisionCandidate {
        decision_type,
        threshold,
        canonical_decision_input: input.canonical_decision_input.clone(),
        input,
    }
}
