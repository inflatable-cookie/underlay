//! Nightfire value validation and two-lookup resolution.

use std::hash::Hash;

use crate::block::BlockData;
use crate::registry::{BlockDescriptor, BlockRegistry, StrategyRegistry};
use crate::strategy::{NightfireStrategy, StrategyCardinality};
use crate::value::NightfireValue;

/// Validation error when checking a `NightfireValue` against a
/// `NightfireStrategy`.
#[derive(Debug, Clone, thiserror::Error)]
pub enum NightfireValidationError {
    /// The value's block count does not match the strategy's cardinality.
    #[error(
        "cardinality mismatch for schema {schema}: expected {expected:?}, got {actual_blocks} blocks"
    )]
    CardinalityMismatch {
        schema: String,
        expected: StrategyCardinality,
        actual_blocks: usize,
    },
    /// The value references a block type that is not permitted by the
    /// strategy's `allowed_types` or `allowed_categories`.
    #[error("disallowed block type {block_type} for schema {schema}")]
    DisallowedBlockType { schema: String, block_type: String },
    /// The value references a block type that is unknown to the
    /// block registry.
    #[error("unknown block type {block_type} for schema {schema}")]
    UnknownBlockType { schema: String, block_type: String },
    /// The stored block version is not in the registry's supported set.
    #[error("unknown block version {version} for type {block_type} in schema {schema}")]
    UnknownBlockVersion {
        schema: String,
        block_type: String,
        version: String,
    },
    /// No strategy is registered for the given schema identifier.
    #[error("no strategy registered for schema {schema}")]
    UnknownStrategy { schema: String },
}

/// A stored block resolved to its current implementation.
#[derive(Debug, Clone, Copy)]
pub struct ResolvedBlock<'a, C> {
    pub block: &'a BlockData,
    pub descriptor: &'a BlockDescriptor<C>,
    pub current_version: &'static str,
}

/// A Nightfire value after the two-lookup resolution rule:
/// 1. strategy → cardinality + allowed types
/// 2. block type+version → current implementation
#[derive(Debug, Clone)]
pub struct ResolvedNightfireValue<'a, C> {
    pub schema: &'a str,
    pub strategy: &'a NightfireStrategy<C>,
    pub blocks: Vec<ResolvedBlock<'a, C>>,
}

/// Resolve a stored block type+version to the current implementation.
///
/// Unknown types and unknown versions fail closed.
pub fn resolve_block_implementation<'a, C>(
    block_type: &str,
    version: &str,
    block_registry: &'a BlockRegistry<C>,
) -> Result<(&'a BlockDescriptor<C>, &'static str), NightfireValidationError> {
    resolve_block_implementation_for_schema(block_type, version, "", block_registry)
}

fn resolve_block_implementation_for_schema<'a, C>(
    block_type: &str,
    version: &str,
    schema: &str,
    block_registry: &'a BlockRegistry<C>,
) -> Result<(&'a BlockDescriptor<C>, &'static str), NightfireValidationError> {
    let descriptor = block_registry.get(block_type).ok_or_else(|| {
        NightfireValidationError::UnknownBlockType {
            schema: schema.to_owned(),
            block_type: block_type.to_owned(),
        }
    })?;

    let current = descriptor.versions.coerce(version).ok_or_else(|| {
        NightfireValidationError::UnknownBlockVersion {
            schema: schema.to_owned(),
            block_type: block_type.to_owned(),
            version: version.to_owned(),
        }
    })?;

    Ok((descriptor, current))
}

/// Coerce every supported older block version onto the current
/// implementation version. Unknown versions fail closed.
pub fn coerce_block_versions<C>(
    value: &mut NightfireValue,
    block_registry: &BlockRegistry<C>,
) -> Result<(), NightfireValidationError> {
    let schema = value.schema.as_str().to_owned();
    for block in &mut value.blocks {
        let (_descriptor, current) = resolve_block_implementation_for_schema(
            &block.r#type,
            &block.version,
            &schema,
            block_registry,
        )?;
        block.version = current.to_string();
    }
    Ok(())
}

/// Resolve a stored value with the two-lookup rule.
pub fn resolve_nightfire_value<'a, C>(
    value: &'a NightfireValue,
    strategy: &'a NightfireStrategy<C>,
    block_registry: &'a BlockRegistry<C>,
) -> Result<ResolvedNightfireValue<'a, C>, NightfireValidationError>
where
    C: Clone + Eq + Hash,
{
    validate_cardinality(value, strategy)?;

    let schema_str = value.schema.as_str();
    let mut resolved_blocks = Vec::with_capacity(value.blocks.len());

    for block in &value.blocks {
        check_block_allowed(schema_str, block, strategy, block_registry)?;
        let (descriptor, current_version) = resolve_block_implementation_for_schema(
            &block.r#type,
            &block.version,
            schema_str,
            block_registry,
        )?;
        resolved_blocks.push(ResolvedBlock {
            block,
            descriptor,
            current_version,
        });
    }

    Ok(ResolvedNightfireValue {
        schema: schema_str,
        strategy,
        blocks: resolved_blocks,
    })
}

/// Validate a `NightfireValue` against the given `NightfireStrategy`.
///
/// This enforces:
/// - Cardinality (`Single` means `blocks.len() == 1`; `Multi` uses min/max).
/// - Allowed block types, derived from:
///   - `allowed_types` (explicit allow list), and/or
///   - `allowed_categories` via the provided `BlockRegistry`.
/// - Block type registration and supported versions (unknown version fails
///   closed).
pub fn validate_nightfire_value<C>(
    value: &NightfireValue,
    strategy: &NightfireStrategy<C>,
    block_registry: &BlockRegistry<C>,
) -> Result<(), NightfireValidationError>
where
    C: Clone + Eq + Hash,
{
    resolve_nightfire_value(value, strategy, block_registry).map(|_| ())
}

/// Resolve and validate a value by looking up its schema in the strategy
/// registry, then resolving each block type+version.
pub fn resolve_nightfire_value_by_schema<'a, C>(
    value: &'a NightfireValue,
    strategy_registry: &'a StrategyRegistry<C>,
    block_registry: &'a BlockRegistry<C>,
) -> Result<ResolvedNightfireValue<'a, C>, NightfireValidationError>
where
    C: Clone + Eq + Hash,
{
    let schema_str = value.schema.as_str();
    let strategy = strategy_registry.get_by_str(schema_str).ok_or_else(|| {
        NightfireValidationError::UnknownStrategy {
            schema: schema_str.to_owned(),
        }
    })?;

    resolve_nightfire_value(value, strategy, block_registry)
}

fn validate_cardinality<C>(
    value: &NightfireValue,
    strategy: &NightfireStrategy<C>,
) -> Result<(), NightfireValidationError> {
    let schema_str = value.schema.as_str().to_owned();
    let block_count = value.blocks.len();

    match strategy.cardinality {
        StrategyCardinality::Single => {
            if block_count != 1 {
                return Err(NightfireValidationError::CardinalityMismatch {
                    schema: schema_str,
                    expected: StrategyCardinality::Single,
                    actual_blocks: block_count,
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
                });
            }
            if let Some(max) = config.max_blocks() {
                if block_count > max {
                    return Err(NightfireValidationError::CardinalityMismatch {
                        schema: schema_str,
                        expected: StrategyCardinality::Multi(config),
                        actual_blocks: block_count,
                    });
                }
            }
        }
    }

    Ok(())
}

fn check_block_allowed<C>(
    schema_str: &str,
    block: &BlockData,
    strategy: &NightfireStrategy<C>,
    block_registry: &BlockRegistry<C>,
) -> Result<(), NightfireValidationError>
where
    C: Clone + Eq + Hash,
{
    let block_type = block.r#type.clone();

    if !strategy.allowed_types.is_empty() && strategy.allowed_types.iter().any(|t| t == &block_type)
    {
        return Ok(());
    }

    if !strategy.allowed_categories.is_empty() {
        let descriptor = block_registry.get(block_type.as_str()).ok_or_else(|| {
            NightfireValidationError::UnknownBlockType {
                schema: schema_str.to_owned(),
                block_type: block_type.clone(),
            }
        })?;

        if strategy
            .allowed_categories
            .iter()
            .any(|c| c == &descriptor.category)
        {
            return Ok(());
        }

        return Err(NightfireValidationError::DisallowedBlockType {
            schema: schema_str.to_owned(),
            block_type,
        });
    }

    Ok(())
}

#[cfg(test)]
#[path = "tests/validation_tests.rs"]
mod tests;
