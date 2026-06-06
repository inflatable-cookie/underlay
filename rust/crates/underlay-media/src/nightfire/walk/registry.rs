use underlay_nightfire::{BlockData, NightfireValue};

use crate::domain::{MediaContentKind, MediaUsageEdgeInput};
use crate::error::{MediaError, MediaResult};
use crate::nightfire::{
    NightfireBlockMediaHandlerRegistry, NightfireBlockMediaUsageExtractor,
    NightfireMediaVisitContext,
};

use super::anchor::BlockAnchor;
use super::nested::collect_nested_blocks;
use super::pointer::join_rooted_pointer;

impl<R> NightfireBlockMediaUsageExtractor<R>
where
    R: NightfireBlockMediaHandlerRegistry,
{
    pub(in crate::nightfire) fn walk_root_value_at(
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
