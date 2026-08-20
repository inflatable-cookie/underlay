use serde_json::Value;
use underlay_nightfire::BlockData;

use crate::domain::{MediaContentKind, MediaUsageEdgeInput};
use crate::error::MediaResult;
use crate::nightfire::{
    NightfireMediaReferenceMatcher, NightfireMediaUsageExtractor, NightfireMediaVisitContext,
};

use super::anchor::BlockAnchor;
use super::nested::as_nested_block;
use super::pointer::push_pointer_segment;

impl<M> NightfireMediaUsageExtractor<M>
where
    M: NightfireMediaReferenceMatcher,
{
    pub(in crate::nightfire) fn walk_block(
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
                    let nested_anchor = if nested_block.has_id() {
                        BlockAnchor::from_block(&nested_block, nested_data_pointer.clone())
                    } else {
                        anchor.clone()
                    };

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
