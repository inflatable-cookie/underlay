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

    #[test]
    fn export_creates_block_data() {
        let block = TestBlock {
            text: "hello".to_string(),
        };
        let data = block.export();

        assert_eq!(data.r#type, "test");
        assert_eq!(data.version, "initial");
        assert!(!data.hash.is_empty());
        assert_eq!(data.data, json!({"text": "hello"}));
    }