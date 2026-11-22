use underlay_blob::{BlobAdapter, BlobAdapterObjectKeyExt, BlobObjectKey};

use super::keys::parse_rendition_result_key;
use super::result::RenditionResult;
use crate::error::{MediaError, MediaResult};
use crate::image::{generate_square_thumbnail, generate_thumbnail, ThumbnailConfig};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RenditionCropMode {
    Fit,
    Square,
}

pub(crate) async fn read_source_bytes<B: BlobAdapter>(
    blob_adapter: &B,
    source_key: &BlobObjectKey,
    max_source_bytes: u64,
) -> MediaResult<Vec<u8>> {
    blob_adapter
        .get_bytes_bounded(source_key.as_str(), max_source_bytes)
        .await
        .map_err(|e| MediaError::storage(format!("Failed to read source: {}", e)))
}

pub(crate) async fn read_generate_and_store<B: BlobAdapter>(
    blob_adapter: &B,
    source_key: &BlobObjectKey,
    target_key: &BlobObjectKey,
    max_dimension: u32,
    jpeg_quality: u8,
    max_source_bytes: u64,
    crop_mode: RenditionCropMode,
    store_label: &str,
) -> MediaResult<RenditionResult> {
    let source_bytes = read_source_bytes(blob_adapter, source_key, max_source_bytes).await?;

    generate_and_store(
        blob_adapter,
        source_bytes,
        target_key,
        max_dimension,
        jpeg_quality,
        crop_mode,
        store_label,
    )
    .await
}

pub(crate) async fn generate_and_store<B: BlobAdapter>(
    blob_adapter: &B,
    source_bytes: Vec<u8>,
    target_key: &BlobObjectKey,
    max_dimension: u32,
    jpeg_quality: u8,
    crop_mode: RenditionCropMode,
    store_label: &str,
) -> MediaResult<RenditionResult> {
    let generated = tokio::task::spawn_blocking(move || match crop_mode {
        RenditionCropMode::Fit => {
            let config =
                ThumbnailConfig::new(max_dimension, max_dimension).with_quality(jpeg_quality);
            generate_thumbnail(&source_bytes, &config)
        }
        RenditionCropMode::Square => {
            generate_square_thumbnail(&source_bytes, max_dimension.max(1), jpeg_quality)
        }
    })
    .await
    .map_err(|e| MediaError::image_processing(format!("Thumbnail task failed: {e}")))?
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
