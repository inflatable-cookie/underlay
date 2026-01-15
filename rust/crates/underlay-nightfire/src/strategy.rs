//! Nightfire strategy types for describing content shape.

use std::num::NonZeroUsize;

use crate::value::SchemaId;

/// Cardinality for a Nightfire strategy.
///
/// Strategies only describe non-null values. An "empty" Nightfire
/// payload should be normalised to `null` at the field boundary,
/// so `min_blocks` is always at least 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategyCardinality {
    /// Exactly one block.
    Single,
    /// Multiple blocks within a min/max range.
    Multi(MultiConfig),
}

/// Multi-block configuration for a strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MultiConfig {
    pub min_blocks: NonZeroUsize,
    pub max_blocks: Option<usize>,
}

/// Strategy describing how a Nightfire value is shaped and which
/// blocks it may contain for a given schema / strategy ID.
///
/// The `C` type parameter is the category enum used by the consuming
/// application (e.g. `BlockCategory` in Acowtancy).
#[derive(Debug, Clone)]
pub struct NightfireStrategy<C> {
    /// Schema identifier for this strategy.
    pub id: SchemaId,

    /// Cardinality constraint (single vs multi).
    pub cardinality: StrategyCardinality,

    /// Explicitly allowed block types by name.
    pub allowed_types: Vec<String>,

    /// Allowed block categories (resolved via registry).
    pub allowed_categories: Vec<C>,

    /// Default block type to use when creating new content.
    pub default_type: String,
}
