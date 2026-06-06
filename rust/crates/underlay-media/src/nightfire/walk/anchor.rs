use underlay_nightfire::{BlockData, NightfireMediaLocator};

use crate::domain::MediaLocatorKind;
use crate::error::{MediaError, MediaResult};

#[derive(Clone, Debug)]
pub(in crate::nightfire) struct BlockAnchor {
    block_id: Option<String>,
    pub(in crate::nightfire) rooted_data_pointer: String,
}

impl BlockAnchor {
    pub(in crate::nightfire) fn from_block(block: &BlockData, rooted_data_pointer: String) -> Self {
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

    pub(in crate::nightfire) fn locator_for(
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
