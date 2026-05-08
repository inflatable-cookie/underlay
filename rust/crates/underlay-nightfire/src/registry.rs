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

/// One consumer-owned block registration bundle.
///
/// `M` is optional extra metadata owned by a higher layer. Underlay Nightfire
/// stays generic by treating it as opaque. Shared downstream layers can use it
/// for things like media-extraction registration without forcing that concern
/// into the core protocol.
#[derive(Debug, Clone)]
pub struct BlockRegistration<C, M = ()> {
    pub descriptor: BlockDescriptor<C>,
    pub metadata: M,
}

impl<C> BlockRegistration<C, ()> {
    pub fn from_descriptor(descriptor: BlockDescriptor<C>) -> Self {
        Self {
            descriptor,
            metadata: (),
        }
    }
}

impl<C, M> BlockRegistration<C, M> {
    pub fn new(descriptor: BlockDescriptor<C>, metadata: M) -> Self {
        Self {
            descriptor,
            metadata,
        }
    }

    pub fn descriptor(&self) -> &BlockDescriptor<C> {
        &self.descriptor
    }

    pub fn metadata(&self) -> &M {
        &self.metadata
    }

    pub fn into_parts(self) -> (BlockDescriptor<C>, M) {
        (self.descriptor, self.metadata)
    }
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

    /// Register one block via a broader block-registration bundle.
    pub fn register_registration<M>(&mut self, registration: BlockRegistration<C, M>) {
        self.register(registration.descriptor);
    }

    /// Register many blocks via broader block-registration bundles.
    pub fn extend_registrations<M>(
        &mut self,
        registrations: impl IntoIterator<Item = BlockRegistration<C, M>>,
    ) {
        for registration in registrations {
            self.register_registration(registration);
        }
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
        let strategy = self.get_by_str(schema_str).ok_or_else(|| {
            NightfireValidationError::UnknownStrategy {
                schema: schema_str.to_owned(),
            }
        })?;

        validate_nightfire_value(value, strategy, block_registry)
    }
}

#[cfg(test)]
#[path = "tests/registry_tests.rs"]
mod tests;
