use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use crate::errors::{MigrationError, MigrationResult};
use crate::plugin::DecisionFingerprintInput;

use super::models::RecordFingerprintInput;
use super::SHA256_PREFIX;

pub fn decision_fingerprint(input: &DecisionFingerprintInput) -> MigrationResult<String> {
    let canonical_input = canonical_json_string(&input.canonical_decision_input)?;
    let mut hasher = Sha256::new();
    hasher.update(b"canonical_decision_input=");
    hasher.update(canonical_input.as_bytes());
    hasher.update(b"\ndecision_type=");
    hasher.update(input.decision_type.as_bytes());
    hasher.update(b"\nresolver_version=");
    hasher.update(input.resolver_version.as_bytes());
    hasher.update(b"\nprompt_version=");
    hasher.update(input.prompt_version.as_bytes());
    hasher.update(b"\ntarget_schema_version=");
    hasher.update(input.target_schema_version.as_bytes());
    Ok(format!("{SHA256_PREFIX}{:x}", hasher.finalize()))
}

pub fn record_fingerprint(input: &RecordFingerprintInput) -> MigrationResult<String> {
    let canonical_transform = canonical_json_string(&input.canonical_transform_input)?;
    let semantic_dependencies = canonical_json_string(
        &serde_json::to_value(&input.semantic_dependencies)
            .map_err(|err| MigrationError::Serialization(err.to_string()))?,
    )?;

    let mut hasher = Sha256::new();
    hasher.update(b"canonical_transform_input=");
    hasher.update(canonical_transform.as_bytes());
    hasher.update(b"\nsource_identity=");
    hasher.update(input.source_identity.as_bytes());
    hasher.update(b"\nsemantic_dependencies=");
    hasher.update(semantic_dependencies.as_bytes());
    Ok(format!("{SHA256_PREFIX}{:x}", hasher.finalize()))
}

fn canonical_json_string(value: &Value) -> MigrationResult<String> {
    let normalized = normalize_value(value);
    serde_json::to_string(&normalized).map_err(|err| MigrationError::Serialization(err.to_string()))
}

fn normalize_value(value: &Value) -> Value {
    match value {
        Value::Object(obj) => {
            let mut sorted = Map::new();
            let mut keys = obj.keys().collect::<Vec<_>>();
            keys.sort();
            for key in keys {
                if let Some(child) = obj.get(key) {
                    sorted.insert(key.clone(), normalize_value(child));
                }
            }
            Value::Object(sorted)
        }
        Value::Array(values) => Value::Array(values.iter().map(normalize_value).collect()),
        _ => value.clone(),
    }
}
