//! Block data types and the Block trait.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hash::compute_block_hash;

/// BlockData is the raw, serialisable representation of a Nightfire block.
///
/// It is the direct equivalent of the PHP implementation's Data\Block:
/// a DTO that carries the block type, version, content hash, and the
/// opaque JSON payload for that block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockData {
    /// Stable block identifier.
    ///
    /// This is optional for backward compatibility with older stored values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Block type identifier, e.g. "paragraph", "heading", "material.selector".
    pub r#type: String,

    /// Version of this block's schema/implementation.
    pub version: String,

    /// Content hash for this block's data payload.
    ///
    /// This allows higher-level domains (e.g. Assessment) to detect when
    /// the semantics of a question or activity have changed while a user
    /// is interacting with it.
    pub hash: String,

    /// Opaque JSON payload for this block.
    ///
    /// All type-specific fields (including any nested children/items/etc.)
    /// live inside this object.
    pub data: Value,
}

/// Trait implemented by typed Nightfire block structs.
///
/// This provides a thin layer over the raw `BlockData` DTO so that
/// concrete block types can define their type name, supported versions,
/// and how to serialise their data payload.
pub trait Block {
    /// Block type identifier, e.g. "paragraph", "heading", "material.selector".
    const TYPE_NAME: &'static str;

    /// Ordered list of supported versions for this block.
    ///
    /// The first entry is treated as the "active" version for newly
    /// created content.
    const VERSIONS: &'static [&'static str] = &["initial"];

    /// Active version string for this block type.
    fn active_version() -> &'static str {
        Self::VERSIONS.first().copied().unwrap_or("initial")
    }

    /// Convert this block into its JSON data payload.
    fn to_data(&self) -> Value;

    /// Export this block into a `BlockData` DTO with a computed hash.
    fn export(&self) -> BlockData {
        let data = self.to_data();

        BlockData {
            id: None,
            r#type: Self::TYPE_NAME.to_string(),
            version: Self::active_version().to_string(),
            hash: compute_block_hash(&data),
            data,
        }
    }
}

#[cfg(test)]
#[path = "tests/block_tests.rs"]
mod tests;
