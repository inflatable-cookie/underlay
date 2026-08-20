use super::*;
use crate::block::BlockVersions;
use crate::registry::BlockDescriptor;
use crate::strategy::MultiConfig;
use crate::value::SchemaId;
use serde_json::json;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TestCategory {
    Text,
    Media,
}

fn make_block(type_name: &str) -> BlockData {
    BlockData {
        id: "block_1".to_string(),
        r#type: type_name.to_string(),
        version: "initial".to_string(),
        data: json!({}),
    }
}

fn test_registry() -> BlockRegistry<TestCategory> {
    let mut registry = BlockRegistry::new();
    registry.register(BlockDescriptor::new(
        "paragraph",
        "Paragraph",
        TestCategory::Text,
    ));
    registry.register(BlockDescriptor::new("image", "Image", TestCategory::Media));
    registry.register(
        BlockDescriptor::new("callout", "Callout", TestCategory::Text).with_versions(
            BlockVersions {
                current: "2",
                supported: &["1", "2"],
            },
        ),
    );
    registry
}

fn single_strategy() -> NightfireStrategy<TestCategory> {
    NightfireStrategy {
        id: SchemaId::from("test:single"),
        cardinality: StrategyCardinality::Single,
        allowed_types: vec![],
        allowed_categories: vec![TestCategory::Text],
        default_type: "paragraph".to_string(),
    }
}

#[test]
fn validates_single_block_in_blocks_array() {
    let registry = test_registry();
    let strategy = single_strategy();
    let value = NightfireValue::single("test:single", make_block("paragraph"));
    let result = validate_nightfire_value(&value, &strategy, &registry);
    assert!(result.is_ok());
}

#[test]
fn rejects_zero_blocks_for_single_strategy() {
    let registry = test_registry();
    let strategy = single_strategy();
    let value = NightfireValue::new("test:single", vec![]);
    let result = validate_nightfire_value(&value, &strategy, &registry);
    assert!(matches!(
        result,
        Err(NightfireValidationError::CardinalityMismatch {
            actual_blocks: 0,
            ..
        })
    ));
}

#[test]
fn rejects_two_blocks_for_single_strategy() {
    let registry = test_registry();
    let strategy = single_strategy();
    let value = NightfireValue::multi(
        "test:single",
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
    let strategy = single_strategy();
    let value = NightfireValue::single("test:single", make_block("image"));
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
        id: SchemaId::from("test:multi"),
        cardinality: StrategyCardinality::Multi(MultiConfig::one_or_more().with_max_blocks(3)),
        allowed_types: vec![],
        allowed_categories: vec![TestCategory::Text],
        default_type: "paragraph".to_string(),
    };

    let value = NightfireValue::multi(
        "test:multi",
        vec![make_block("paragraph"), make_block("paragraph")],
    );
    let result = validate_nightfire_value(&value, &strategy, &registry);
    assert!(result.is_ok());
}

#[test]
fn rejects_too_many_blocks() {
    let registry = test_registry();
    let strategy = NightfireStrategy {
        id: SchemaId::from("test:multi"),
        cardinality: StrategyCardinality::Multi(MultiConfig::one_or_more().with_max_blocks(2)),
        allowed_types: vec![],
        allowed_categories: vec![TestCategory::Text],
        default_type: "paragraph".to_string(),
    };

    let value = NightfireValue::multi(
        "test:multi",
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

#[test]
fn older_supported_version_resolves_to_current_implementation() {
    let registry = test_registry();
    let strategy = single_strategy();
    let value = NightfireValue::single("test:single", make_block("callout").with_version("1"));

    let resolved = resolve_nightfire_value(&value, &strategy, &registry).unwrap();
    assert_eq!(resolved.blocks.len(), 1);
    assert_eq!(resolved.blocks[0].current_version, "2");
    assert_eq!(resolved.blocks[0].descriptor.type_name, "callout");
    assert_eq!(value.blocks[0].version, "1");

    let mut coerced = value.clone();
    coerce_block_versions(&mut coerced, &registry).unwrap();
    assert_eq!(coerced.blocks[0].version, "2");
}

#[test]
fn unknown_version_fails_closed() {
    let registry = test_registry();
    let strategy = single_strategy();
    let value = NightfireValue::single("test:single", make_block("callout").with_version("9"));
    let result = validate_nightfire_value(&value, &strategy, &registry);
    assert!(matches!(
        result,
        Err(NightfireValidationError::UnknownBlockVersion {
            version,
            ..
        }) if version == "9"
    ));
}

#[test]
fn unknown_block_type_fails_closed() {
    let registry = test_registry();
    let strategy = NightfireStrategy {
        id: SchemaId::from("test:open"),
        cardinality: StrategyCardinality::Single,
        allowed_types: vec![],
        allowed_categories: vec![],
        default_type: "paragraph".to_string(),
    };
    let value = NightfireValue::single("test:open", make_block("mystery"));
    let result = validate_nightfire_value(&value, &strategy, &registry);
    assert!(matches!(
        result,
        Err(NightfireValidationError::UnknownBlockType { .. })
    ));
}
