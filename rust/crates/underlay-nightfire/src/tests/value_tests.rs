    use super::*;
    use serde_json::json;

    fn sample_block() -> BlockData {
        BlockData {
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