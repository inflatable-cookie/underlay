//! Block and strategy registries.

use std::collections::HashMap;
use std::hash::Hash;

use crate::strategy::NightfireStrategy;
use crate::validation::{validate_nightfire_value, NightfireValidationError};
use crate::value::{NightfireValue, SchemaId};

/// Descriptor for a concrete block type.
///
/// The `C` type parameter is the category enum used by the consuming
/// application.
#[derive(Debug, Clone)]
pub struct BlockDescriptor<C> {
    /// Block type identifier, e.g. "paragraph", "markdown".
    pub type_name: &'static str,

    /// Human-readable label for this block type.
    pub label: &'static str,

    /// Category this block belongs to.
    pub category: C,
}

/// Registry of known Nightfire block descriptors.
///
/// This allows strategies and APIs to resolve human-friendly labels
/// and categories for block types.
///
/// The `C` type parameter is the category enum used by the consuming
/// application.
#[derive(Debug, Clone)]
pub struct BlockRegistry<C> {
    blocks: HashMap<&'static str, BlockDescriptor<C>>,
}

impl<C> Default for BlockRegistry<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C> BlockRegistry<C> {
    /// Create a new empty block registry.
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    /// Register a block descriptor.
    pub fn register(&mut self, descriptor: BlockDescriptor<C>) {
        self.blocks.insert(descriptor.type_name, descriptor);
    }

    /// Look up a block descriptor by type name.
    pub fn get(&self, type_name: &str) -> Option<&BlockDescriptor<C>> {
        self.blocks.get(type_name)
    }

    /// Iterate over all registered block descriptors.
    pub fn all(&self) -> impl Iterator<Item = &BlockDescriptor<C>> {
        self.blocks.values()
    }
}

/// Registry of known Nightfire strategies.
///
/// This allows validation and APIs to resolve strategies from schema
/// identifiers.
///
/// The `C` type parameter is the category enum used by the consuming
/// application.
#[derive(Debug, Clone)]
pub struct StrategyRegistry<C> {
    strategies: HashMap<String, NightfireStrategy<C>>,
}

impl<C> Default for StrategyRegistry<C>
where
    C: Clone + Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C> StrategyRegistry<C>
where
    C: Clone + Eq + Hash,
{
    /// Create a new empty strategy registry.
    pub fn new() -> Self {
        Self {
            strategies: HashMap::new(),
        }
    }

    /// Register a strategy.
    pub fn register(&mut self, strategy: NightfireStrategy<C>) {
        let id_str = strategy.id.as_str().to_owned();
        self.strategies.insert(id_str, strategy);
    }

    /// Look up a strategy by schema identifier.
    pub fn get(&self, schema: &SchemaId) -> Option<&NightfireStrategy<C>> {
        self.strategies.get(schema.as_str())
    }

    /// Look up a strategy by schema identifier string.
    pub fn get_by_str(&self, schema: &str) -> Option<&NightfireStrategy<C>> {
        self.strategies.get(schema)
    }

    /// Iterate over all registered strategies.
    pub fn all(&self) -> impl Iterator<Item = &NightfireStrategy<C>> {
        self.strategies.values()
    }

    /// Validate a `NightfireValue` by looking up its schema in this registry.
    ///
    /// Returns `Err(NightfireValidationError::UnknownStrategy)` if no strategy
    /// is registered for the value's schema identifier.
    pub fn validate(
        &self,
        value: &NightfireValue,
        block_registry: &BlockRegistry<C>,
    ) -> Result<(), NightfireValidationError> {
        let schema_str = value.schema.as_str();
        let strategy =
            self.get_by_str(schema_str)
                .ok_or_else(|| NightfireValidationError::UnknownStrategy {
                    schema: schema_str.to_owned(),
                })?;

        validate_nightfire_value(value, strategy, block_registry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::BlockData;
    use crate::strategy::StrategyCardinality;
    use serde_json::json;

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
}
