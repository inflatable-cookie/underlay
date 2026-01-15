//! Nightfire value and schema identifier types.

use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample_block() -> BlockData {
        BlockData {
            r#type: "test".to_string(),
            version: "initial".to_string(),
            hash: "abc123".to_string(),
            data: json!({"text": "hello"}),
        }
    }

    #[test]
    fn single_value_has_block() {
        let value = NightfireValue::single("test:schema@1", sample_block());
        assert!(value.is_single());
        assert!(!value.is_multi());
    }

    #[test]
    fn multi_value_has_blocks() {
        let value = NightfireValue::multi("test:schema@1", vec![sample_block()]);
        assert!(!value.is_single());
        assert!(value.is_multi());
    }

    #[test]
    fn schema_id_from_str() {
        let id: SchemaId = "test:schema@1".into();
        assert_eq!(id.as_str(), "test:schema@1");
    }
}
