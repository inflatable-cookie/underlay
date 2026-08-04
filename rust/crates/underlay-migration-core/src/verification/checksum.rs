use sha2::{Digest, Sha256};

use crate::errors::{MigrationError, MigrationResult};
use crate::pipeline::TransformStageOutput;

pub fn transform_checksum(transform: &TransformStageOutput) -> MigrationResult<String> {
    let payload = serde_json::to_vec(transform)
        .map_err(|err| MigrationError::Serialization(err.to_string()))?;
    let digest = Sha256::digest(payload);
    Ok(hex::encode(digest))
}
