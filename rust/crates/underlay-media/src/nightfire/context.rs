use serde_json::Value;
use underlay_nightfire::BlockData;

use super::walk::normalize_relative_pointer;

pub struct NightfireMediaVisitContext<'a> {
    pub block: &'a BlockData,
    pub data_pointer: &'a str,
    pub rooted_pointer: &'a str,
}

impl<'a> NightfireMediaVisitContext<'a> {
    pub fn block_type(&self) -> &str {
        &self.block.r#type
    }

    pub fn block_id(&self) -> Option<&str> {
        let id = self.block.id.trim();
        if id.is_empty() {
            None
        } else {
            Some(id)
        }
    }

    pub fn block_data(&self) -> &Value {
        &self.block.data
    }

    pub fn resolve_relative_pointer(&self, pointer: &str) -> Option<&Value> {
        let normalized = normalize_relative_pointer(pointer);
        if normalized.is_empty() {
            return Some(self.block_data());
        }

        self.block_data().pointer(&normalized)
    }
}
