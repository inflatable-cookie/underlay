    use super::*;
    use crate::block::BlockData;
    use crate::strategy::StrategyCardinality;
    use serde_json::json;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestCategory {
        Text,
    }

    fn make_block(type_name: &str) -> BlockData {
        BlockData {
            r#type: type_name.to_string(),
            version: "initial".to_string(),
            hash: "abc123".to_string(),
            data: json!({}),
        }
    }

    #[test]
    fn block_registry_stores_and_retrieves() {
        let mut registry = BlockRegistry::new();

        registry.register(BlockDescriptor {
            type_name: "paragraph",
            label: "Paragraph",
            category: TestCategory::Text,
        });

        let desc = registry.get("paragraph").unwrap();
        assert_eq!(desc.type_name, "paragraph");
        assert_eq!(desc.label, "Paragraph");
        assert_eq!(desc.category, TestCategory::Text);

        assert!(registry.get("unknown").is_none());
    }

    #[test]
    fn strategy_registry_stores_and_retrieves() {
        let mut registry = StrategyRegistry::new();

        registry.register(NightfireStrategy {
            id: SchemaId::from("test:schema@1"),
            cardinality: StrategyCardinality::Single,
            allowed_types: vec![],
            allowed_categories: vec![TestCategory::Text],
            default_type: "paragraph".to_string(),
        });

        let strategy = registry.get_by_str("test:schema@1").unwrap();
        assert_eq!(strategy.id.as_str(), "test:schema@1");
        assert_eq!(strategy.default_type, "paragraph");

        assert!(registry.get_by_str("unknown").is_none());
    }

    #[test]
    fn validate_returns_unknown_strategy_for_unregistered_schema() {
        let strategy_registry: StrategyRegistry<TestCategory> = StrategyRegistry::new();
        let block_registry = BlockRegistry::new();

        let value = NightfireValue::single("unknown:schema@1", make_block("paragraph"));
        let result = strategy_registry.validate(&value, &block_registry);

        assert!(matches!(
            result,
            Err(NightfireValidationError::UnknownStrategy { .. })
        ));
    }

    #[test]
    fn validate_delegates_to_strategy_validation() {
        let mut strategy_registry = StrategyRegistry::new();
        let mut block_registry = BlockRegistry::new();

        block_registry.register(BlockDescriptor {
            type_name: "paragraph",
            label: "Paragraph",
            category: TestCategory::Text,
        });

        strategy_registry.register(NightfireStrategy {
            id: SchemaId::from("test:schema@1"),
            cardinality: StrategyCardinality::Single,
            allowed_types: vec![],
            allowed_categories: vec![TestCategory::Text],
            default_type: "paragraph".to_string(),
        });

        let value = NightfireValue::single("test:schema@1", make_block("paragraph"));
        let result = strategy_registry.validate(&value, &block_registry);
        assert!(result.is_ok());
    }