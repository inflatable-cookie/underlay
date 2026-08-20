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
        id: "block_1".to_string(),
        r#type: type_name.to_string(),
        version: "initial".to_string(),
        data: json!({}),
    }
}

#[test]
fn block_registry_stores_and_retrieves() {
    let mut registry = BlockRegistry::new();

    registry.register(BlockDescriptor::new(
        "paragraph",
        "Paragraph",
        TestCategory::Text,
    ));

    let desc = registry.get("paragraph").unwrap();
    assert_eq!(desc.type_name, "paragraph");
    assert_eq!(desc.label, "Paragraph");
    assert_eq!(desc.category, TestCategory::Text);
    assert_eq!(desc.versions.current, "initial");

    assert!(registry.get("unknown").is_none());
}

#[test]
fn block_registry_accepts_registration_bundles() {
    let mut registry = BlockRegistry::new();

    registry.register_registration(BlockRegistration::new(
        BlockDescriptor::new("hero", "Hero", TestCategory::Text),
        "media-handler-present",
    ));

    let desc = registry.get("hero").unwrap();
    assert_eq!(desc.type_name, "hero");
    assert_eq!(desc.label, "Hero");
}

#[test]
fn strategy_registry_stores_and_retrieves() {
    let mut registry = StrategyRegistry::new();

    registry.register(NightfireStrategy {
        id: SchemaId::from("test:schema"),
        cardinality: StrategyCardinality::Single,
        allowed_types: vec![],
        allowed_categories: vec![TestCategory::Text],
        default_type: "paragraph".to_string(),
    });

    let strategy = registry.get_by_str("test:schema").unwrap();
    assert_eq!(strategy.id.as_str(), "test:schema");
    assert_eq!(strategy.default_type, "paragraph");

    assert!(registry.get_by_str("unknown").is_none());
}

#[test]
fn validate_returns_unknown_strategy_for_unregistered_schema() {
    let strategy_registry: StrategyRegistry<TestCategory> = StrategyRegistry::new();
    let block_registry = BlockRegistry::new();

    let value = NightfireValue::single("unknown:schema", make_block("paragraph"));
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

    block_registry.register(BlockDescriptor::new(
        "paragraph",
        "Paragraph",
        TestCategory::Text,
    ));

    strategy_registry.register(NightfireStrategy {
        id: SchemaId::from("test:schema"),
        cardinality: StrategyCardinality::Single,
        allowed_types: vec![],
        allowed_categories: vec![TestCategory::Text],
        default_type: "paragraph".to_string(),
    });

    let value = NightfireValue::single("test:schema", make_block("paragraph"));
    let result = strategy_registry.validate(&value, &block_registry);
    assert!(result.is_ok());
}
