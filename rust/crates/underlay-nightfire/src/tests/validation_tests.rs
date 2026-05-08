use super::*;
use crate::registry::BlockDescriptor;
use crate::strategy::MultiConfig;
use crate::value::SchemaId;
use serde_json::json;
use std::num::NonZeroUsize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TestCategory {
    Text,
    Media,
}

fn make_block(type_name: &str) -> BlockData {
    BlockData {
        id: None,
        r#type: type_name.to_string(),
        version: "initial".to_string(),
        hash: "abc123".to_string(),
        data: json!({}),
    }
}

fn test_registry() -> BlockRegistry<TestCategory> {
    let mut registry = BlockRegistry::new();
    registry.register(BlockDescriptor {
        type_name: "paragraph",
        label: "Paragraph",
        category: TestCategory::Text,
    });
    registry.register(BlockDescriptor {
        type_name: "image",
        label: "Image",
        category: TestCategory::Media,
    });
    registry
}

#[test]
fn rejects_value_with_both_block_and_blocks() {
    let registry = test_registry();
    let strategy = NightfireStrategy {
        id: SchemaId::from("test:single@1"),
        cardinality: StrategyCardinality::Single,
        allowed_types: vec![],
        allowed_categories: vec![TestCategory::Text],
        default_type: "paragraph".to_string(),
    };

    let value = NightfireValue {
        schema: SchemaId::from("test:single@1"),
        block: Some(make_block("paragraph")),
        blocks: Some(vec![make_block("paragraph")]),
    };
    let result = validate_nightfire_value(&value, &strategy, &registry);
    assert!(matches!(
        result,
        Err(NightfireValidationError::InvalidValueShape {
            has_block: true,
            has_blocks: true,
            ..
        })
    ));
}

#[test]
fn rejects_value_with_neither_block_nor_blocks() {
    let registry = test_registry();
    let strategy = NightfireStrategy {
        id: SchemaId::from("test:single@1"),
        cardinality: StrategyCardinality::Single,
        allowed_types: vec![],
        allowed_categories: vec![TestCategory::Text],
        default_type: "paragraph".to_string(),
    };

    let value = NightfireValue {
        schema: SchemaId::from("test:single@1"),
        block: None,
        blocks: None,
    };
    let result = validate_nightfire_value(&value, &strategy, &registry);
    assert!(matches!(
        result,
        Err(NightfireValidationError::InvalidValueShape {
            has_block: false,
            has_blocks: false,
            ..
        })
    ));
}

#[test]
fn validates_single_block() {
    let registry = test_registry();
    let strategy = NightfireStrategy {
        id: SchemaId::from("test:single@1"),
        cardinality: StrategyCardinality::Single,
        allowed_types: vec![],
        allowed_categories: vec![TestCategory::Text],
        default_type: "paragraph".to_string(),
    };

    let value = NightfireValue::single("test:single@1", make_block("paragraph"));
    let result = validate_nightfire_value(&value, &strategy, &registry);
    assert!(result.is_ok());
}

#[test]
fn rejects_multi_for_single_strategy() {
    let registry = test_registry();
    let strategy = NightfireStrategy {
        id: SchemaId::from("test:single@1"),
        cardinality: StrategyCardinality::Single,
        allowed_types: vec![],
        allowed_categories: vec![TestCategory::Text],
        default_type: "paragraph".to_string(),
    };

    let value = NightfireValue::multi(
        "test:single@1",
        vec![make_block("paragraph"), make_block("paragraph")],
    );
    let result = validate_nightfire_value(&value, &strategy, &registry);
    assert!(matches!(
        result,
        Err(NightfireValidationError::CardinalityMismatch { .. })
    ));
}

#[test]
fn rejects_disallowed_category() {
    let registry = test_registry();
    let strategy = NightfireStrategy {
        id: SchemaId::from("test:text@1"),
        cardinality: StrategyCardinality::Single,
        allowed_types: vec![],
        allowed_categories: vec![TestCategory::Text],
        default_type: "paragraph".to_string(),
    };

    // Image is Media category, not Text
    let value = NightfireValue::single("test:text@1", make_block("image"));
    let result = validate_nightfire_value(&value, &strategy, &registry);
    assert!(matches!(
        result,
        Err(NightfireValidationError::DisallowedBlockType { .. })
    ));
}

#[test]
fn validates_multi_block_within_range() {
    let registry = test_registry();
    let strategy = NightfireStrategy {
        id: SchemaId::from("test:multi@1"),
        cardinality: StrategyCardinality::Multi(MultiConfig {
            min_blocks: NonZeroUsize::new(1).unwrap(),
            max_blocks: Some(3),
        }),
        allowed_types: vec![],
        allowed_categories: vec![TestCategory::Text],
        default_type: "paragraph".to_string(),
    };

    let value = NightfireValue::multi(
        "test:multi@1",
        vec![make_block("paragraph"), make_block("paragraph")],
    );
    let result = validate_nightfire_value(&value, &strategy, &registry);
    assert!(result.is_ok());
}

#[test]
fn rejects_too_many_blocks() {
    let registry = test_registry();
    let strategy = NightfireStrategy {
        id: SchemaId::from("test:multi@1"),
        cardinality: StrategyCardinality::Multi(MultiConfig {
            min_blocks: NonZeroUsize::new(1).unwrap(),
            max_blocks: Some(2),
        }),
        allowed_types: vec![],
        allowed_categories: vec![TestCategory::Text],
        default_type: "paragraph".to_string(),
    };

    let value = NightfireValue::multi(
        "test:multi@1",
        vec![
            make_block("paragraph"),
            make_block("paragraph"),
            make_block("paragraph"),
        ],
    );
    let result = validate_nightfire_value(&value, &strategy, &registry);
    assert!(matches!(
        result,
        Err(NightfireValidationError::CardinalityMismatch { .. })
    ));
}
