use underlay_blob::BlobObjectKey;

use crate::error::{MediaError, MediaResult};

pub(crate) fn parse_rendition_result_key(key: impl AsRef<str>) -> MediaResult<BlobObjectKey> {
    BlobObjectKey::parse(key)
        .map_err(|err| MediaError::storage(format!("invalid rendition object key: {err}")))
}
