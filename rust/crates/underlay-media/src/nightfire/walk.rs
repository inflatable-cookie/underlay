use serde_json::Value;
use underlay_nightfire::{BlockData, NightfireMediaLocator, NightfireValue};

use super::{
    NightfireBlockMediaHandlerRegistry, NightfireBlockMediaUsageExtractor,
    NightfireMediaReferenceMatcher, NightfireMediaUsageExtractor, NightfireMediaVisitContext,
};
use crate::domain::{MediaContentKind, MediaLocatorKind, MediaUsageEdgeInput};
use crate::error::{MediaError, MediaResult};

#[derive(Clone, Debug)]
pub(super) struct BlockAnchor {
    block_id: Option<String>,
    pub(super) rooted_data_pointer: String,
}

impl BlockAnchor {
    pub(super) fn from_block(block: &BlockData, rooted_data_pointer: String) -> Self {
        Self {
            block_id: block
                .id
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned),
            rooted_data_pointer,
        }
    }

    pub(super) fn locator_for(
        &self,
        rooted_pointer: &str,
    ) -> MediaResult<(MediaLocatorKind, String)> {
        if let Some(block_id) = self.block_id.as_deref() {
            let relative_pointer = rooted_pointer
                .strip_prefix(&self.rooted_data_pointer)
                .unwrap_or(rooted_pointer);
            let locator =
                NightfireMediaLocator::new(block_id, relative_pointer).map_err(|err| {
                    MediaError::validation(format!("invalid Nightfire media locator: {err:?}"))
                })?;
            return Ok((MediaLocatorKind::BlockId, locator.to_locator_key()));
        }

        Ok((MediaLocatorKind::Path, rooted_pointer.to_string()))
    }
}

impl<M> NightfireMediaUsageExtractor<M>
where
    M: NightfireMediaReferenceMatcher,
{
    pub(super) fn walk_block(
        &self,
        block: &BlockData,
        anchor: BlockAnchor,
        data_pointer: &str,
        value: &Value,
        edges: &mut Vec<MediaUsageEdgeInput>,
    ) -> MediaResult<()> {
        let relative_pointer = data_pointer
            .strip_prefix(&anchor.rooted_data_pointer)
            .unwrap_or(data_pointer);

        let context = NightfireMediaVisitContext {
            block,
            data_pointer: relative_pointer,
            rooted_pointer: data_pointer,
        };

        if let Some(media_ref) = self.matcher.match_media_reference(&context, value)? {
            let (locator_kind, locator_key) = anchor.locator_for(data_pointer)?;
            edges.push(MediaUsageEdgeInput {
                media_id: media_ref.media_id,
                used_by_type: self.used_by_type.clone(),
                used_by_id: self.used_by_id,
                owner_field: Some(self.owner_field.clone()),
                content_kind: MediaContentKind::StructuredContent,
                locator_kind,
                locator_key,
                usage_role: media_ref.usage_role,
                provenance_kind: self.provenance_kind.clone(),
            });
        }

        match value {
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    let pointer = push_pointer_segment(data_pointer, &index.to_string());
                    self.walk_block(block, anchor.clone(), &pointer, item, edges)?;
                }
            }
            Value::Object(map) => {
                if let Some(nested_block) = as_nested_block(value) {
                    let nested_data_pointer = push_pointer_segment(data_pointer, "data");
                    let nested_anchor = nested_block
                        .id
                        .as_deref()
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                        .map(|_| {
                            BlockAnchor::from_block(&nested_block, nested_data_pointer.clone())
                        })
                        .unwrap_or_else(|| anchor.clone());

                    self.walk_block(
                        &nested_block,
                        nested_anchor,
                        &nested_data_pointer,
                        &nested_block.data,
                        edges,
                    )?;
                    return Ok(());
                }

                for (key, child) in map {
                    let pointer = push_pointer_segment(data_pointer, key);
                    self.walk_block(block, anchor.clone(), &pointer, child, edges)?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

impl<R> NightfireBlockMediaUsageExtractor<R>
where
    R: NightfireBlockMediaHandlerRegistry,
{
    pub(super) fn walk_root_value_at(
        &self,
        value: &NightfireValue,
        root_pointer: &str,
        fallback_anchor: Option<&BlockAnchor>,
        edges: &mut Vec<MediaUsageEdgeInput>,
    ) -> MediaResult<()> {
        if let Some(block) = value.block.as_ref() {
            let rooted_pointer = join_rooted_pointer(root_pointer, "/block/data");
            let anchor = block
                .id
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|_| BlockAnchor::from_block(block, rooted_pointer.clone()))
                .or_else(|| fallback_anchor.cloned())
                .unwrap_or_else(|| BlockAnchor::from_block(block, rooted_pointer.clone()));
            self.walk_block(block, anchor, &rooted_pointer, edges)?;
        }

        if let Some(blocks) = value.blocks.as_ref() {
            for (index, block) in blocks.iter().enumerate() {
                let rooted_pointer =
                    join_rooted_pointer(root_pointer, &format!("/blocks/{index}/data"));
                let anchor = block
                    .id
                    .as_deref()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(|_| BlockAnchor::from_block(block, rooted_pointer.clone()))
                    .or_else(|| fallback_anchor.cloned())
                    .unwrap_or_else(|| BlockAnchor::from_block(block, rooted_pointer.clone()));
                self.walk_block(block, anchor, &rooted_pointer, edges)?;
            }
        }

        Ok(())
    }

    fn walk_block(
        &self,
        block: &BlockData,
        anchor: BlockAnchor,
        rooted_data_pointer: &str,
        edges: &mut Vec<MediaUsageEdgeInput>,
    ) -> MediaResult<()> {
        let context = NightfireMediaVisitContext {
            block,
            data_pointer: "",
            rooted_pointer: rooted_data_pointer,
        };

        if let Some(handler) = self.registry.handler_for(&block.r#type) {
            for media_ref in handler.extract_media_references(&context)? {
                let rooted_pointer =
                    join_rooted_pointer(rooted_data_pointer, &media_ref.data_pointer);
                let (locator_kind, locator_key) = anchor.locator_for(&rooted_pointer)?;
                edges.push(MediaUsageEdgeInput {
                    media_id: media_ref.media_id,
                    used_by_type: self.used_by_type.clone(),
                    used_by_id: self.used_by_id,
                    owner_field: Some(self.owner_field.clone()),
                    content_kind: MediaContentKind::StructuredContent,
                    locator_kind,
                    locator_key,
                    usage_role: media_ref.usage_role,
                    provenance_kind: self.provenance_kind.clone(),
                });
            }

            for nested in handler.nested_nightfire_values(&context)? {
                let nested_root_pointer =
                    join_rooted_pointer(rooted_data_pointer, &nested.data_pointer);
                let nested_value = context
                    .resolve_relative_pointer(&nested.data_pointer)
                    .ok_or_else(|| {
                        MediaError::validation(format!(
                            "nested Nightfire pointer not found in block {}: {}",
                            block.r#type, nested.data_pointer
                        ))
                    })?
                    .clone();
                let nested_value =
                    serde_json::from_value::<NightfireValue>(nested_value).map_err(|err| {
                        MediaError::validation(format!(
                            "invalid nested Nightfire value in block {} at {}: {err}",
                            block.r#type, nested.data_pointer
                        ))
                    })?;
                self.walk_root_value_at(&nested_value, &nested_root_pointer, Some(&anchor), edges)?;
            }
        }

        for (nested_pointer, nested_block) in collect_nested_blocks(&block.data, "")? {
            let rooted_data_pointer =
                join_rooted_pointer(&anchor.rooted_data_pointer, &nested_pointer);
            let nested_anchor = nested_block
                .id
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|_| BlockAnchor::from_block(&nested_block, rooted_data_pointer.clone()))
                .unwrap_or_else(|| anchor.clone());

            self.walk_block(&nested_block, nested_anchor, &rooted_data_pointer, edges)?;
        }

        Ok(())
    }
}

fn as_nested_block(value: &Value) -> Option<BlockData> {
    serde_json::from_value::<BlockData>(value.clone()).ok()
}

fn as_nested_nightfire_value(value: &Value) -> Option<NightfireValue> {
    serde_json::from_value::<NightfireValue>(value.clone()).ok()
}

fn push_pointer_segment(pointer: &str, segment: &str) -> String {
    let escaped = segment.replace('~', "~0").replace('/', "~1");
    format!("{pointer}/{escaped}")
}

pub(super) fn normalize_relative_pointer(pointer: &str) -> String {
    let trimmed = pointer.trim();
    if trimmed.is_empty() || trimmed == "/" {
        String::new()
    } else if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{trimmed}")
    }
}

fn join_rooted_pointer(rooted_pointer: &str, relative_pointer: &str) -> String {
    let normalized = normalize_relative_pointer(relative_pointer);
    if normalized.is_empty() {
        rooted_pointer.to_string()
    } else {
        format!("{rooted_pointer}{normalized}")
    }
}

fn collect_nested_blocks(value: &Value, pointer: &str) -> MediaResult<Vec<(String, BlockData)>> {
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
