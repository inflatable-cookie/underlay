use super::*;
use serde_json::json;

fn sample_block() -> BlockData {
    BlockData {
        id: Some("block_1".to_string()),
        r#type: "test".to_string(),
        version: "initial".to_string(),
        hash: "abc123".to_string(),
        data: json!({"text": "hello"}),
    }
}

#[test]
fn single_value_has_block() {
    let value = NightfireValue::single("test:schema@1", sample_block());
    assert!(value.is_single());
    assert!(!value.is_multi());
}

#[test]
fn multi_value_has_blocks() {
    let value = NightfireValue::multi("test:schema@1", vec![sample_block()]);
    assert!(!value.is_single());
    assert!(value.is_multi());
}

#[test]
fn schema_id_from_str() {
    let id: SchemaId = "test:schema@1".into();
    assert_eq!(id.as_str(), "test:schema@1");
}

#[test]
fn block_id_round_trips_in_value() {
    let value = NightfireValue::single("test:schema@1", sample_block());
    let block = value.block.expect("single block");
    assert_eq!(block.id.as_deref(), Some("block_1"));
}

#[test]
fn ensure_block_ids_assigns_missing_single_block_id() {
    let mut value = NightfireValue::single(
        "test:schema@1",
        BlockData {
            id: None,
            r#type: "test".to_string(),
            version: "initial".to_string(),
            hash: "abc123".to_string(),
            data: json!({"text": "hello"}),
        },
    );

    let assigned = ensure_block_ids(&mut value);
    let block = value.block.expect("single block");

    assert_eq!(assigned, 1);
    assert!(block.id.as_deref().is_some_and(|id| id.starts_with("nf_")));
}

#[test]
fn ensure_block_ids_preserves_existing_ids_and_assigns_only_missing_multi_blocks() {
    let mut value = NightfireValue::multi(
        "test:schema@1",
        vec![
            BlockData {
                id: Some("existing_block".to_string()),
                r#type: "test".to_string(),
                version: "initial".to_string(),
                hash: "abc123".to_string(),
                data: json!({"text": "hello"}),
            },
            BlockData {
                id: None,
                r#type: "test".to_string(),
                version: "initial".to_string(),
                hash: "def456".to_string(),
                data: json!({"text": "world"}),
            },
        ],
    );

    let assigned = ensure_block_ids(&mut value);
    let blocks = value.blocks.expect("multi blocks");

    assert_eq!(assigned, 1);
    assert_eq!(blocks[0].id.as_deref(), Some("existing_block"));
    assert!(blocks[1]
        .id
        .as_deref()
        .is_some_and(|id| id.starts_with("nf_")));
}
