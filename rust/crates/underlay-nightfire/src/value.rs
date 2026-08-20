//! Nightfire value and schema identifier types.

use serde::{Deserialize, Serialize};

use crate::block::{generate_block_id, BlockData};

/// Schema identifier for a Nightfire value.
///
/// Convention: `<namespace>:<context>/<field>`
/// e.g. `acow:content/rich_text`. Schema IDs are unversioned; version
/// lives on each block.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SchemaId(pub String);

impl SchemaId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for SchemaId {
    fn from(value: String) -> Self {
        SchemaId(value)
    }
}

impl From<&str> for SchemaId {
    fn from(value: &str) -> Self {
        SchemaId(value.to_owned())
    }
}

/// A field-level Nightfire value.
///
/// This is the shape persisted in JSONB columns and sent over the wire:
/// `{ schema, blocks: [ { id, type, version, data } ] }`.
/// Cardinality is a strategy rule, not a field shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NightfireValue {
    /// Strategy / schema identifier.
    pub schema: SchemaId,

    /// Ordered blocks. Always an array; never a sibling `block` field.
    pub blocks: Vec<BlockData>,
}

impl NightfireValue {
    /// Construct a Nightfire value with the given blocks.
    pub fn new<S: Into<SchemaId>>(schema: S, blocks: Vec<BlockData>) -> Self {
        Self {
            schema: schema.into(),
            blocks,
        }
    }

    /// Construct a value with exactly one block.
    ///
    /// Convenience only; the wire shape is still `{ schema, blocks }`.
    pub fn single<S: Into<SchemaId>>(schema: S, block: BlockData) -> Self {
        Self::new(schema, vec![block])
    }

    /// Construct a value with many blocks.
    pub fn multi<S: Into<SchemaId>>(schema: S, blocks: Vec<BlockData>) -> Self {
        Self::new(schema, blocks)
    }
}

/// Ensure every block in this Nightfire value has a stable block ID.
///
/// Returns the number of block IDs that were assigned.
pub fn ensure_block_ids(value: &mut NightfireValue) -> usize {
    value.blocks.iter_mut().map(ensure_block_id).sum()
}

fn ensure_block_id(block: &mut BlockData) -> usize {
    if block.has_id() {
        return 0;
    }

    block.id = generate_block_id();
    1
}

#[cfg(test)]
#[path = "tests/value_tests.rs"]
mod tests;
