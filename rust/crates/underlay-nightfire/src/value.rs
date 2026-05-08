//! Nightfire value and schema identifier types.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::block::BlockData;

/// Schema identifier for a Nightfire value.
///
/// Convention: `<namespace>:<context>/<field>@<version>`
/// e.g. `acow:learning/activity.material@1`.
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
/// This is the shape persisted in JSONB columns and sent over the wire.
/// It may represent either a single block or multiple blocks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NightfireValue {
    /// Strategy / schema identifier.
    pub schema: SchemaId,

    /// Single-block content, when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<BlockData>,

    /// Multi-block content, when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<BlockData>>,
}

impl NightfireValue {
    /// Construct a single-block Nightfire value.
    pub fn single<S: Into<SchemaId>>(schema: S, block: BlockData) -> Self {
        Self {
            schema: schema.into(),
            block: Some(block),
            blocks: None,
        }
    }

    /// Construct a multi-block Nightfire value.
    pub fn multi<S: Into<SchemaId>>(schema: S, blocks: Vec<BlockData>) -> Self {
        Self {
            schema: schema.into(),
            block: None,
            blocks: Some(blocks),
        }
    }

    /// Returns true if this value encodes a single block.
    pub fn is_single(&self) -> bool {
        self.block.is_some()
    }

    /// Returns true if this value encodes multiple blocks.
    pub fn is_multi(&self) -> bool {
        self.blocks.is_some()
    }
}

/// Ensure every block in this Nightfire value has a stable block ID.
///
/// Returns the number of block IDs that were assigned.
pub fn ensure_block_ids(value: &mut NightfireValue) -> usize {
    let mut assigned = 0usize;

    if let Some(block) = value.block.as_mut() {
        assigned += ensure_block_id(block);
    }

    if let Some(blocks) = value.blocks.as_mut() {
        for block in blocks {
            assigned += ensure_block_id(block);
        }
    }

    assigned
}

fn ensure_block_id(block: &mut BlockData) -> usize {
    if block
        .id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .is_some()
    {
        return 0;
    }

    block.id = Some(generate_block_id());
    1
}

fn generate_block_id() -> String {
    format!("nf_{}", Uuid::now_v7().simple())
}

#[cfg(test)]
#[path = "tests/value_tests.rs"]
mod tests;
