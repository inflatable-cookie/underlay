//! Block data types and the Block trait.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Supported versions for a block type, plus the current implementation.
///
/// Readers resolve any supported stored version to `current`. Unknown
/// versions fail closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockVersions {
    /// Version newly created blocks use.
    pub current: &'static str,
    /// Every stored version the current implementation can coerce.
    pub supported: &'static [&'static str],
}

impl BlockVersions {
    /// Default version set used by blocks that have not evolved yet.
    pub const INITIAL: Self = Self {
        current: "initial",
        supported: &["initial"],
    };

    /// True when `version` is in the supported set.
    pub fn supports(self, version: &str) -> bool {
        self.supported.contains(&version)
    }

    /// Resolve a stored version to the current implementation.
    ///
    /// Returns `None` when the version is unknown.
    pub fn coerce(self, version: &str) -> Option<&'static str> {
        if self.supports(version) {
            Some(self.current)
        } else {
            None
        }
    }
}

/// BlockData is the raw, serialisable representation of a Nightfire block.
///
/// Wire shape: `{ id, type, version, data }`. `id` is assigned `nf_<uuid7>`
/// on export when empty. There is no envelope hash.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlockData {
    /// Stable block identifier (`nf_<uuid7>`).
    ///
    /// Empty on in-memory drafts until `ensure_block_ids` or `export`.
    #[serde(default)]
    pub id: String,

    /// Block type identifier, e.g. "paragraph", "heading", "material.selector".
    pub r#type: String,

    /// Version of this block's schema/implementation.
    pub version: String,

    /// Opaque JSON payload for this block.
    ///
    /// All type-specific fields (including any nested children/items/etc.)
    /// live inside this object.
    pub data: Value,
}

impl BlockData {
    /// Construct a block with a generated id and the initial version.
    pub fn new(r#type: impl Into<String>, data: Value) -> Self {
        Self {
            id: generate_block_id(),
            r#type: r#type.into(),
            version: BlockVersions::INITIAL.current.to_string(),
            data,
        }
    }

    /// Override the block id.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Override the stored version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// True when this block already has a non-empty id.
    pub fn has_id(&self) -> bool {
        !self.id.trim().is_empty()
    }
}

/// Generate a stable Nightfire block id.
pub fn generate_block_id() -> String {
    format!("nf_{}", Uuid::now_v7().simple())
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
    /// The first entry is the current implementation. Remaining entries
    /// stay readable via coercion.
    const VERSIONS: &'static [&'static str] = &["initial"];

    /// Active version string for this block type.
    fn active_version() -> &'static str {
        Self::VERSIONS.first().copied().unwrap_or("initial")
    }

    /// Registry version set derived from `VERSIONS`.
    fn versions() -> BlockVersions {
        BlockVersions {
            current: Self::active_version(),
            supported: Self::VERSIONS,
        }
    }

    /// Convert this block into its JSON data payload.
    fn to_data(&self) -> Value;

    /// Export this block into a `BlockData` DTO with a generated id.
    fn export(&self) -> BlockData {
        BlockData {
            id: generate_block_id(),
            r#type: Self::TYPE_NAME.to_string(),
            version: Self::active_version().to_string(),
            data: self.to_data(),
        }
    }
}

#[cfg(test)]
#[path = "tests/block_tests.rs"]
mod tests;
