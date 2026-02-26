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
