//! Nightfire value validation.

use std::hash::Hash;

use crate::block::BlockData;
use crate::registry::BlockRegistry;
use crate::strategy::{NightfireStrategy, StrategyCardinality};
use crate::value::NightfireValue;

/// Validation error when checking a `NightfireValue` against a
/// `NightfireStrategy`.
#[derive(Debug, Clone)]
pub enum NightfireValidationError {
    /// The value's shape (single vs multi and block count) does not
    /// match the strategy's cardinality.
    CardinalityMismatch {
        schema: String,
        expected: StrategyCardinality,
        actual_blocks: usize,
        is_single: bool,
    },
    /// The value references a block type that is not permitted by the
    /// strategy's `allowed_types` or `allowed_categories`.
    DisallowedBlockType { schema: String, block_type: String },
    /// The value references a block type that is unknown to the
    /// block registry.
    UnknownBlockType { schema: String, block_type: String },
    /// No strategy is registered for the given schema identifier.
    UnknownStrategy { schema: String },
}

/// Validate a `NightfireValue` against the given `NightfireStrategy`.
///
/// This enforces:
/// - Cardinality (`Single` vs `Multi` with min/max blocks).
/// - Allowed block types, derived from:
///   - `allowed_types` (explicit allow list), and/or
///   - `allowed_categories` via the provided `BlockRegistry`.
///
/// The `C` type parameter is the category enum used by the consuming
/// application.
pub fn validate_nightfire_value<C>(
    value: &NightfireValue,
    strategy: &NightfireStrategy<C>,
    block_registry: &BlockRegistry<C>,
) -> Result<(), NightfireValidationError>
where
    C: Clone + Eq + Hash,
{
    let schema_str = value.schema.as_str().to_owned();

    // Determine the effective block count and shape.
    let (is_single, block_count) = if value.block.is_some() {
        (true, 1)
    } else if let Some(ref blocks) = value.blocks {
        (false, blocks.len())
    } else {
        (false, 0)
    };

    // Cardinality checks.
    match strategy.cardinality {
        StrategyCardinality::Single => {
            if !is_single || block_count != 1 {
                return Err(NightfireValidationError::CardinalityMismatch {
                    schema: schema_str,
                    expected: StrategyCardinality::Single,
                    actual_blocks: block_count,
                    is_single,
                });
            }
        }
        StrategyCardinality::Multi(config) => {
            let min = config.min_blocks.get();
            if block_count < min {
                return Err(NightfireValidationError::CardinalityMismatch {
                    schema: schema_str,
                    expected: StrategyCardinality::Multi(config),
                    actual_blocks: block_count,
                    is_single,
                });
            }
            if let Some(max) = config.max_blocks {
                if block_count > max {
                    return Err(NightfireValidationError::CardinalityMismatch {
                        schema: schema_str,
                        expected: StrategyCardinality::Multi(config),
                        actual_blocks: block_count,
                        is_single,
                    });
                }
            }
        }
    }

    // Collect blocks for type validation.
    let blocks: Vec<&BlockData> = if let Some(ref block) = value.block {
        vec![block]
    } else if let Some(ref many) = value.blocks {
        many.iter().collect()
    } else {
        Vec::new()
    };

    // Short-circuit if there are no blocks; cardinality rules above
    // will already have rejected illegal empty payloads for
    // non-nullable strategies.
    if blocks.is_empty() {
        return Ok(());
    }

    for block in blocks {
        let block_type = block.r#type.clone();

        // Explicit allow-list wins.
        if !strategy.allowed_types.is_empty()
            && strategy.allowed_types.iter().any(|t| t == &block_type)
        {
            continue;
        }

        // Fall back to category-based allow rules.
        if !strategy.allowed_categories.is_empty() {
            let descriptor = block_registry.get(block_type.as_str()).ok_or_else(|| {
                NightfireValidationError::UnknownBlockType {
                    schema: schema_str.clone(),
                    block_type: block_type.clone(),
                }
            })?;

            if strategy
                .allowed_categories
                .iter()
                .any(|c| c == &descriptor.category)
            {
                continue;
            }

            return Err(NightfireValidationError::DisallowedBlockType {
                schema: schema_str,
                block_type,
            });
        }

        // If neither explicit types nor categories are configured,
        // treat all block types as allowed.
    }

    Ok(())
}

#[cfg(test)]
mod tests {
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
}
