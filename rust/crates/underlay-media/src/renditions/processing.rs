use underlay_blob::{BlobAdapter, BlobAdapterObjectKeyExt, BlobObjectKey};

use super::keys::parse_rendition_result_key;
use super::result::RenditionResult;
use crate::error::{MediaError, MediaResult};
use crate::image::{generate_thumbnail, ThumbnailConfig};

pub(crate) async fn read_generate_and_store<B: BlobAdapter>(
    blob_adapter: &B,
    source_key: &BlobObjectKey,
    target_key: &BlobObjectKey,
    max_dimension: u32,
    jpeg_quality: u8,
    store_label: &str,
) -> MediaResult<RenditionResult> {
    let source_bytes = blob_adapter
        .get_object_bytes(source_key)
        .await
        .map_err(|e| MediaError::storage(format!("Failed to read source: {}", e)))?;

    generate_and_store(
        blob_adapter,
        &source_bytes,
        target_key,
        max_dimension,
        jpeg_quality,
        store_label,
    )
    .await
}

pub(crate) async fn generate_and_store<B: BlobAdapter>(
    blob_adapter: &B,
    source_bytes: &[u8],
    target_key: &BlobObjectKey,
    max_dimension: u32,
    jpeg_quality: u8,
    store_label: &str,
) -> MediaResult<RenditionResult> {
    let config = ThumbnailConfig::new(max_dimension, max_dimension).with_quality(jpeg_quality);

    let generated = generate_thumbnail(source_bytes, &config)
        .map_err(|e| MediaError::image_processing(e.to_string()))?;

    let stored = blob_adapter
        .put_object_bytes(target_key, &generated.data, generated.mime_type)
        .await
        .map_err(|e| MediaError::storage(format!("Failed to store {store_label}: {e}")))?;

    Ok(RenditionResult {
        object_key: parse_rendition_result_key(&stored.key)?,
        mime_type: generated.mime_type.to_string(),
        byte_size: stored.size as i64,
        width: generated.width as i32,
        height: generated.height as i32,
    })
}
