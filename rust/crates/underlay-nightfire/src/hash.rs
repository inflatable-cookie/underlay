//! Content hashing for Nightfire blocks.

use serde_json::Value;

/// Compute a stable content hash for a block's data payload.
///
/// The hash is based on the JSON representation of `data` so that
/// semantically identical payloads (from Nightfire's perspective)
/// produce the same hash.
pub fn compute_block_hash(data: &Value) -> String {
    let json = serde_json::to_vec(data).unwrap_or_default();
    let digest = blake3::hash(&json);
    digest.to_hex().to_string()
}

#[cfg(test)]
#[path = "tests/hash_tests.rs"]
mod tests;
