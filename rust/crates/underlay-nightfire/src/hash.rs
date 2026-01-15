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
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn hash_is_deterministic() {
        let data = json!({"text": "hello world"});
        let hash1 = compute_block_hash(&data);
        let hash2 = compute_block_hash(&data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn different_data_produces_different_hash() {
        let data1 = json!({"text": "hello"});
        let data2 = json!({"text": "world"});
        let hash1 = compute_block_hash(&data1);
        let hash2 = compute_block_hash(&data2);
        assert_ne!(hash1, hash2);
    }
}
