use sha2::{Digest, Sha256};

use crate::DecisionFingerprintInput;

pub(in crate::tests) fn decision_fingerprint_for(input: &DecisionFingerprintInput) -> String {
    let canonical = serde_json::to_string(&input.canonical_decision_input)
        .expect("canonical json should encode");
    let raw = format!(
        "canonical_decision_input={canonical}\ndecision_type={}\nresolver_version={}\nprompt_version={}\ntarget_schema_version={}",
        input.decision_type, input.resolver_version, input.prompt_version, input.target_schema_version
    );
    let digest = Sha256::digest(raw.as_bytes());
    format!("sha256:{digest:x}")
}
