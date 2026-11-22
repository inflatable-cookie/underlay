use underlay_blob::BlobAdapter;

use super::super::keys::parse_rendition_result_key;
use super::super::processing::{generate_and_store, read_source_bytes, RenditionCropMode};
use super::RenditionService;
use crate::domain::{CreateRenditionInput, MediaId, MediaRendition, MediaVersionId, RenditionType};
use crate::error::{MediaError, MediaResult};
use crate::repository::MediaRepository;

impl<B: BlobAdapter> RenditionService<B> {
    /// Generate all configured renditions for a version.
    ///
    /// This generates thumbnails and/or previews based on the configuration,
    /// storing them and recording them in the repository.
    ///
    /// **Deprecated**: Use `generate_renditions_for_version` instead, which
    /// automatically generates storage keys using the standardized format.
    pub async fn generate_version_renditions<R: MediaRepository>(
        &self,
        repo: &R,
        version_id: MediaVersionId,
        source_key: &str,
        key_prefix: &str,
    ) -> MediaResult<Vec<MediaRendition>> {
        let source_key = parse_rendition_result_key(source_key)?;
        let mut renditions = Vec::new();
        let generate_thumbnails = self.config.generate_thumbnails();
        let generate_previews = self.config.generate_previews();

        if !generate_thumbnails && !generate_previews {
            return Ok(renditions);
        }

        let thumb_key = if generate_thumbnails {
            Some(parse_rendition_result_key(format!(
                "{}/{}.jpg",
                key_prefix,
                self.config.thumbnail_name_ref()
            ))?)
        } else {
            None
        };
        let preview_key = if generate_previews {
            Some(parse_rendition_result_key(format!(
                "{}/{}.jpg",
                key_prefix,
                self.config.preview_name_ref()
            ))?)
        } else {
            None
        };

        let source_bytes = match read_source_bytes(
            self.blob_adapter.as_ref(),
            &source_key,
            self.config.max_source_bytes(),
        )
        .await
        {
            Ok(bytes) => bytes,
            Err(e) => {
                tracing::warn!(
                    version_id = %version_id,
                    error = %e,
                    "Failed to read source image for rendition generation"
                );
                return Ok(renditions);
            }
        };

        if let Some(thumb_key) = thumb_key {
            let thumb_source = source_bytes.clone();
            match generate_and_store(
                self.blob_adapter.as_ref(),
                thumb_source,
                &thumb_key,
                self.config.thumbnail_max_dimension(),
                self.config.jpeg_quality(),
                self.thumbnail_crop_mode(),
                "thumbnail",
            )
            .await
            {
                Ok(result) => {
                    let input = CreateRenditionInput {
                        rendition_type: RenditionType::Thumbnail,
                        object_key: result.object_key,
                        mime_type: result.mime_type,
                        byte_size: result.byte_size,
                        width: Some(result.width),
                        height: Some(result.height),
                        storage_provider: self.blob_adapter.name().to_string(),
                        bucket: self.blob_adapter.bucket().to_string(),
                    };
                    let rendition = repo.create_rendition(version_id, input).await?;
                    renditions.push(rendition);
                }
                Err(e) => {
                    tracing::warn!(
                        version_id = %version_id,
                        error = %e,
                        "Failed to generate thumbnail"
                    );
                }
            }
        }

        if let Some(preview_key) = preview_key {
            match generate_and_store(
                self.blob_adapter.as_ref(),
                source_bytes,
                &preview_key,
                self.config.preview_max_dimension(),
                self.config.jpeg_quality(),
                RenditionCropMode::Fit,
                "preview",
            )
            .await
            {
                Ok(result) => {
                    let input = CreateRenditionInput {
                        rendition_type: RenditionType::Preview,
                        object_key: result.object_key,
                        mime_type: result.mime_type,
                        byte_size: result.byte_size,
                        width: Some(result.width),
                        height: Some(result.height),
                        storage_provider: self.blob_adapter.name().to_string(),
                        bucket: self.blob_adapter.bucket().to_string(),
                    };
                    let rendition = repo.create_rendition(version_id, input).await?;
                    renditions.push(rendition);
                }
                Err(e) => {
                    tracing::warn!(
                        version_id = %version_id,
                        error = %e,
                        "Failed to generate preview"
                    );
                }
            }
        }

        Ok(renditions)
    }

    /// Generate all configured renditions for a version using standardized keys.
    ///
    /// This is the recommended method for generating renditions. It uses the
    /// built-in key generator to create storage keys in the standard format:
    /// `media/{media_id}/renditions/{version_id}/{rendition_name}.jpg`
    ///
    /// # Arguments
    ///
    /// * `repo` - The media repository for storing rendition records
    /// * `media_id` - The media item ID (needed for key generation)
    /// * `version_id` - The version ID
    /// * `source_key` - The object key of the source image
    ///
    /// # Returns
    ///
    /// A list of created renditions, or an empty list if generation failed.
    pub async fn generate_renditions_for_version<R: MediaRepository>(
        &self,
        repo: &R,
        media_id: MediaId,
        version_id: MediaVersionId,
        source_key: &str,
    ) -> MediaResult<Vec<MediaRendition>> {
        let source_key = parse_rendition_result_key(source_key)?;
        let mut renditions = Vec::new();
        let generate_thumbnails = self.config.generate_thumbnails();
        let generate_previews = self.config.generate_previews();

        if !generate_thumbnails && !generate_previews {
            return Ok(renditions);
        }

        let thumb_key = if generate_thumbnails {
            Some(
                self.key_generator
                    .rendition_object_key(
                        media_id.0,
                        version_id.0,
                        self.config.thumbnail_name_ref(),
                    )
                    .map_err(|err| MediaError::storage(format!("invalid thumbnail key: {err}")))?,
            )
        } else {
            None
        };
        let preview_key = if generate_previews {
            Some(
                self.key_generator
                    .rendition_object_key(media_id.0, version_id.0, self.config.preview_name_ref())
                    .map_err(|err| MediaError::storage(format!("invalid preview key: {err}")))?,
            )
        } else {
            None
        };

        let source_bytes = match read_source_bytes(
            self.blob_adapter.as_ref(),
            &source_key,
            self.config.max_source_bytes(),
        )
        .await
        {
            Ok(bytes) => bytes,
            Err(e) => {
                tracing::warn!(
                    version_id = %version_id,
                    error = %e,
                    "Failed to read source image for rendition generation"
                );
                return Ok(renditions);
            }
        };

        if let Some(thumb_key) = thumb_key {
            let thumb_source = source_bytes.clone();
            match generate_and_store(
                self.blob_adapter.as_ref(),
                thumb_source,
                &thumb_key,
                self.config.thumbnail_max_dimension(),
                self.config.jpeg_quality(),
                self.thumbnail_crop_mode(),
                "thumbnail",
            )
            .await
            {
                Ok(result) => {
                    let input = CreateRenditionInput {
                        rendition_type: RenditionType::Custom(
                            self.config.thumbnail_name_ref().to_string(),
                        ),
                        object_key: result.object_key,
                        mime_type: result.mime_type,
                        byte_size: result.byte_size,
                        width: Some(result.width),
                        height: Some(result.height),
                        storage_provider: self.blob_adapter.name().to_string(),
                        bucket: self.blob_adapter.bucket().to_string(),
                    };
                    let rendition = repo.create_rendition(version_id, input).await?;
                    renditions.push(rendition);
                    tracing::info!(
                        version_id = %version_id,
                        rendition_type = %self.config.thumbnail_name_ref(),
                        "Generated thumbnail"
                    );
                }
                Err(e) => {
                    tracing::warn!(
                        version_id = %version_id,
                        error = %e,
                        "Failed to generate thumbnail"
                    );
                }
            }
        }

        if let Some(preview_key) = preview_key {
            match generate_and_store(
                self.blob_adapter.as_ref(),
                source_bytes,
                &preview_key,
                self.config.preview_max_dimension(),
                self.config.jpeg_quality(),
                RenditionCropMode::Fit,
                "preview",
            )
            .await
            {
                Ok(result) => {
                    let input = CreateRenditionInput {
                        rendition_type: RenditionType::Custom(
                            self.config.preview_name_ref().to_string(),
                        ),
                        object_key: result.object_key,
                        mime_type: result.mime_type,
                        byte_size: result.byte_size,
                        width: Some(result.width),
                        height: Some(result.height),
                        storage_provider: self.blob_adapter.name().to_string(),
                        bucket: self.blob_adapter.bucket().to_string(),
                    };
                    let rendition = repo.create_rendition(version_id, input).await?;
                    renditions.push(rendition);
                    tracing::info!(
                        version_id = %version_id,
                        rendition_type = %self.config.preview_name_ref(),
                        "Generated preview"
                    );
                }
                Err(e) => {
                    tracing::warn!(
                        version_id = %version_id,
                        error = %e,
                        "Failed to generate preview"
                    );
                }
            }
        }

        Ok(renditions)
    }
}
