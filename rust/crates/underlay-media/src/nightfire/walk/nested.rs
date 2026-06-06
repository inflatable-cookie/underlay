use serde_json::Value;
use underlay_nightfire::{BlockData, NightfireValue};

use crate::error::MediaResult;

use super::pointer::push_pointer_segment;

pub(super) fn as_nested_block(value: &Value) -> Option<BlockData> {
    serde_json::from_value::<BlockData>(value.clone()).ok()
}

fn as_nested_nightfire_value(value: &Value) -> Option<NightfireValue> {
    serde_json::from_value::<NightfireValue>(value.clone()).ok()
}

pub(super) fn collect_nested_blocks(
    value: &Value,
    pointer: &str,
) -> MediaResult<Vec<(String, BlockData)>> {
    let mut nested = Vec::new();
    collect_nested_blocks_into(value, pointer, &mut nested)?;
    Ok(nested)
}

fn collect_nested_blocks_into(
    value: &Value,
    pointer: &str,
    nested: &mut Vec<(String, BlockData)>,
) -> MediaResult<()> {
    match value {
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                let child_pointer = push_pointer_segment(pointer, &index.to_string());
                collect_nested_blocks_into(item, &child_pointer, nested)?;
            }
        }
        Value::Object(map) => {
            if let Some(block) = as_nested_block(value) {
                nested.push((pointer.to_string(), block));
                return Ok(());
            }

            if as_nested_nightfire_value(value).is_some() {
                return Ok(());
            }

            for (key, child) in map {
                let child_pointer = push_pointer_segment(pointer, key);
                collect_nested_blocks_into(child, &child_pointer, nested)?;
            }
        }
        _ => {}
    }

    Ok(())
}
