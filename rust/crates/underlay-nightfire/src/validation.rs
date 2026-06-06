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
    /// The value shape is invalid before strategy validation runs.
    ///
    /// Nightfire values must encode either a single block or multiple blocks,
    /// never both and never neither.
    InvalidValueShape {
        schema: String,
        has_block: bool,
        has_blocks: bool,
    },
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
    let has_block = value.block.is_some();
    let has_blocks = value.blocks.is_some();

    if has_block == has_blocks {
        return Err(NightfireValidationError::InvalidValueShape {
            schema: schema_str,
            has_block,
            has_blocks,
        });
    }

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
            let min = config.min_blocks().get();
            if block_count < min {
                return Err(NightfireValidationError::CardinalityMismatch {
                    schema: schema_str,
                    expected: StrategyCardinality::Multi(config),
                    actual_blocks: block_count,
                    is_single,
                });
            }
            if let Some(max) = config.max_blocks() {
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
#[path = "tests/validation_tests.rs"]
mod tests;
