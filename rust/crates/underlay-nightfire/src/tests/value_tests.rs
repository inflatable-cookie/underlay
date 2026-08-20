use super::*;
use serde_json::json;

fn sample_block() -> BlockData {
    BlockData {
        id: "block_1".to_string(),
        r#type: "test".to_string(),
        version: "initial".to_string(),
        data: json!({"text": "hello"}),
    }
}

#[test]
fn value_always_uses_blocks_array() {
    let single = NightfireValue::single("test:schema", sample_block());
    assert_eq!(single.blocks.len(), 1);
    assert_eq!(single.blocks[0].id, "block_1");

    let multi = NightfireValue::multi("test:schema", vec![sample_block(), sample_block()]);
    assert_eq!(multi.blocks.len(), 2);
}

#[test]
fn schema_id_from_str() {
    let id: SchemaId = "test:schema".into();
    assert_eq!(id.as_str(), "test:schema");
}

#[test]
fn serializes_v2_envelope_without_block_or_hash() {
    let value = NightfireValue::single("test:schema", sample_block());
    let encoded = serde_json::to_value(&value).unwrap();
    let object = encoded.as_object().unwrap();
    assert!(object.contains_key("schema"));
    assert!(object.contains_key("blocks"));
    assert!(!object.contains_key("block"));
    assert!(!encoded["blocks"][0]
        .as_object()
        .unwrap()
        .contains_key("hash"));
}

#[test]
fn rejects_v1_block_field() {
    let result = serde_json::from_value::<NightfireValue>(json!({
        "schema": "test:schema",
        "block": {
            "id": "block_1",
            "type": "test",
            "version": "initial",
            "data": {"text": "hello"}
        }
    }));
    assert!(result.is_err());
}

#[test]
fn rejects_v1_block_and_blocks_together() {
    let result = serde_json::from_value::<NightfireValue>(json!({
        "schema": "test:schema",
        "block": {
            "id": "block_1",
            "type": "test",
            "version": "initial",
            "data": {}
        },
        "blocks": []
    }));
    assert!(result.is_err());
}

#[test]
fn rejects_missing_blocks() {
    let result = serde_json::from_value::<NightfireValue>(json!({
        "schema": "test:schema"
    }));
    assert!(result.is_err());
}

#[test]
fn ensure_block_ids_assigns_missing_ids() {
    let mut value = NightfireValue::single(
        "test:schema",
        BlockData {
            id: String::new(),
            r#type: "test".to_string(),
            version: "initial".to_string(),
            data: json!({"text": "hello"}),
        },
    );

    let assigned = ensure_block_ids(&mut value);

    assert_eq!(assigned, 1);
    assert!(value.blocks[0].id.starts_with("nf_"));
}

#[test]
fn ensure_block_ids_preserves_existing_ids_and_assigns_only_missing() {
    let mut value = NightfireValue::multi(
        "test:schema",
        vec![
            BlockData {
                id: "existing_block".to_string(),
                r#type: "test".to_string(),
                version: "initial".to_string(),
                data: json!({"text": "hello"}),
            },
            BlockData {
                id: String::new(),
                r#type: "test".to_string(),
                version: "initial".to_string(),
                data: json!({"text": "world"}),
            },
        ],
    );

    let assigned = ensure_block_ids(&mut value);

    assert_eq!(assigned, 1);
    assert_eq!(value.blocks[0].id, "existing_block");
    assert!(value.blocks[1].id.starts_with("nf_"));
}
