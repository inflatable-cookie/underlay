//! Canonical Nightfire media-reference locators.
//!
//! These locators are designed for `media_usage.locator_key` values when a
//! media reference lives inside a Nightfire block payload. The canonical form
//! anchors on a stable block id and then uses an RFC 6901 JSON Pointer
//! relative to that block's `data` payload:
//!
//! `<block-id>#<json-pointer-relative-to-data>`
//!
//! Example:
//!
//! `hero_01#/pages/1/image_id`

use serde_json::Value;

use crate::{BlockData, NightfireValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NightfireMediaLocator {
    pub block_id: String,
    pub data_pointer: String,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum NightfireMediaLocatorError {
    #[error("locator key missing '#' separator")]
    MissingSeparator,
    #[error("locator missing block id")]
    MissingBlockId,
    #[error("invalid data pointer: {0}")]
    InvalidDataPointer(String),
}

impl NightfireMediaLocator {
    pub fn new(
        block_id: impl Into<String>,
        data_pointer: impl Into<String>,
    ) -> Result<Self, NightfireMediaLocatorError> {
        let block_id = block_id.into().trim().to_string();
        let data_pointer = data_pointer.into();

        if block_id.is_empty() {
            return Err(NightfireMediaLocatorError::MissingBlockId);
        }

        if !data_pointer.is_empty() && !data_pointer.starts_with('/') {
            return Err(NightfireMediaLocatorError::InvalidDataPointer(data_pointer));
        }

        Ok(Self {
            block_id,
            data_pointer,
        })
    }

    pub fn parse(locator_key: &str) -> Result<Self, NightfireMediaLocatorError> {
        let (block_id, data_pointer) = locator_key
            .split_once('#')
            .ok_or(NightfireMediaLocatorError::MissingSeparator)?;

        Self::new(block_id, data_pointer)
    }

    pub fn to_locator_key(&self) -> String {
        format!("{}#{}", self.block_id, self.data_pointer)
    }

    pub fn find_block<'a>(&self, value: &'a NightfireValue) -> Option<&'a BlockData> {
        value.blocks.iter().find(|block| block.id == self.block_id)
    }

    pub fn resolve_in_value<'a>(&self, value: &'a NightfireValue) -> Option<&'a Value> {
        let block_data = find_block_data_in_value(value, self.block_id.as_str())?;

        if self.data_pointer.is_empty() {
            return Some(block_data);
        }

        block_data.pointer(&self.data_pointer)
    }
}

fn find_block_data_in_value<'a>(value: &'a NightfireValue, block_id: &str) -> Option<&'a Value> {
    value
        .blocks
        .iter()
        .find_map(|block| find_block_data_in_block(block, block_id))
}

fn find_block_data_in_block<'a>(block: &'a BlockData, block_id: &str) -> Option<&'a Value> {
    if block.id == block_id {
        return Some(&block.data);
    }

    find_block_data_in_json(&block.data, block_id)
}

fn find_block_data_in_json<'a>(value: &'a Value, block_id: &str) -> Option<&'a Value> {
    match value {
        Value::Array(items) => items
            .iter()
            .find_map(|item| find_block_data_in_json(item, block_id)),
        Value::Object(map) => {
            let id_matches = map
                .get("id")
                .and_then(Value::as_str)
                .map(|id| id == block_id)
                .unwrap_or(false);

            if id_matches {
                if let Some(data) = map.get("data") {
                    return Some(data);
                }
            }

            map.values()
                .find_map(|child| find_block_data_in_json(child, block_id))
        }
        _ => None,
    }
}

#[cfg(test)]
#[path = "tests/media_locator_tests.rs"]
mod tests;
