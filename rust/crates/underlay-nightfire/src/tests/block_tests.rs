use super::*;
use serde_json::json;

struct TestBlock {
    text: String,
}

impl Block for TestBlock {
    const TYPE_NAME: &'static str = "test";

    fn to_data(&self) -> Value {
        json!({"text": self.text})
    }
}

struct VersionedBlock;

impl Block for VersionedBlock {
    const TYPE_NAME: &'static str = "versioned";
    const VERSIONS: &'static [&'static str] = &["2", "1"];

    fn to_data(&self) -> Value {
        json!({})
    }
}

#[test]
fn export_creates_block_data_without_hash() {
    let block = TestBlock {
        text: "hello".to_string(),
    };
    let data = block.export();

    assert!(data.id.starts_with("nf_"));
    assert_eq!(data.r#type, "test");
    assert_eq!(data.version, "initial");
    assert_eq!(data.data, json!({"text": "hello"}));
}

#[test]
fn export_omits_hash_on_the_wire() {
    let data = TestBlock {
        text: "hello".to_string(),
    }
    .export();
    let value = serde_json::to_value(&data).unwrap();
    let object = value.as_object().unwrap();
    assert!(!object.contains_key("hash"));
    assert!(object.contains_key("id"));
    assert!(object.contains_key("type"));
    assert!(object.contains_key("version"));
    assert!(object.contains_key("data"));
}

#[test]
fn rejects_hash_on_block_data() {
    let result = serde_json::from_value::<BlockData>(json!({
        "id": "nf_1",
        "type": "test",
        "version": "initial",
        "hash": "abc123",
        "data": {}
    }));
    assert!(result.is_err());
}

#[test]
fn versions_uses_first_entry_as_current() {
    let versions = VersionedBlock::versions();
    assert_eq!(versions.current, "2");
    assert_eq!(versions.supported, &["2", "1"]);
    assert_eq!(versions.coerce("1"), Some("2"));
    assert_eq!(versions.coerce("9"), None);
}
