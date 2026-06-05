use std::sync::Arc;

use underlay_blob::{BlobAdapter, BlobAdapterObjectKeyExt, BlobObjectKey};

use super::config::RenditionConfig;
use super::keys::parse_rendition_result_key;
use super::processing::{generate_and_store, read_generate_and_store};
use super::result::RenditionResult;
use crate::domain::{CreateRenditionInput, MediaId, MediaRendition, MediaVersionId, RenditionType};
use crate::error::{MediaError, MediaResult};
use crate::repository::MediaRepository;
use crate::storage::StorageKeyGenerator;

/// Service for generating and managing renditions.
///
/// This service handles the full lifecycle of renditions:
/// - Generating thumbnails and previews from source images
/// - Storing renditions in blob storage
/// - Recording renditions in the database
/// - Cleaning up rendition blobs when versions are deleted
///
/// # Example
///
/// ```rust,ignore
/// use underlay_media::renditions::{RenditionService, RenditionConfig};
///
/// let service = RenditionService::new(blob_adapter, RenditionConfig::default());
///
/// // Generate a thumbnail
/// let result = service.generate_thumbnail("source/key", "thumb/key").await?;
///
/// // Clean up renditions when deleting a version
/// service.delete_version_renditions(&repo, version_id).await?;
/// ```
pub struct RenditionService<B: BlobAdapter> {
    blob_adapter: Arc<B>,
    config: RenditionConfig,
    key_generator: StorageKeyGenerator,
}

impl<B: BlobAdapter> RenditionService<B> {
    /// Create a new rendition service.
    pub fn new(blob_adapter: Arc<B>, config: RenditionConfig) -> Self {
        Self {
            blob_adapter,
            config,
            key_generator: StorageKeyGenerator::with_defaults(),
        }
    }

    /// Create a new rendition service with default configuration.
    pub fn with_defaults(blob_adapter: Arc<B>) -> Self {
        Self::new(blob_adapter, RenditionConfig::default())
    }

    /// Create a new rendition service with a custom key generator.
    pub fn with_key_generator(
        blob_adapter: Arc<B>,
        config: RenditionConfig,
        key_generator: StorageKeyGenerator,
    ) -> Self {
        Self {
            blob_adapter,
            config,
            key_generator,
        }
    }

    /// Get the configuration.
    pub fn config(&self) -> &RenditionConfig {
        &self.config
    }

    /// Get the key generator.
    pub fn key_generator(&self) -> &StorageKeyGenerator {
        &self.key_generator
    }

    /// Generate a thumbnail from a source image.
    ///
    /// Reads the source image from blob storage, generates a thumbnail,
    /// and writes it to the target location.
    pub async fn generate_thumbnail(
        &self,
        source_key: &str,
        target_key: &str,
    ) -> MediaResult<RenditionResult> {
        let source_key = parse_rendition_result_key(source_key)?;
        let target_key = parse_rendition_result_key(target_key)?;
        self.generate_thumbnail_object_key(&source_key, &target_key)
            .await
    }

    /// Generate a thumbnail from typed source and target object keys.
    pub async fn generate_thumbnail_object_key(
        &self,
        source_key: &BlobObjectKey,
        target_key: &BlobObjectKey,
    ) -> MediaResult<RenditionResult> {
        read_generate_and_store(
            self.blob_adapter.as_ref(),
            source_key,
            target_key,
            self.config.thumbnail_max_dimension,
            self.config.jpeg_quality,
            "thumbnail",
        )
        .await
    }

    /// Generate a preview from a source image.
    ///
    /// Previews are larger than thumbnails, suitable for detail views.
    pub async fn generate_preview(
        &self,
        source_key: &str,
        target_key: &str,
    ) -> MediaResult<RenditionResult> {
        let source_key = parse_rendition_result_key(source_key)?;
        let target_key = parse_rendition_result_key(target_key)?;
        self.generate_preview_object_key(&source_key, &target_key)
            .await
    }

    /// Generate a preview from typed source and target object keys.
    pub async fn generate_preview_object_key(
        &self,
        source_key: &BlobObjectKey,
        target_key: &BlobObjectKey,
    ) -> MediaResult<RenditionResult> {
        read_generate_and_store(
            self.blob_adapter.as_ref(),
            source_key,
            target_key,
            self.config.preview_max_dimension,
            self.config.jpeg_quality,
            "preview",
        )
        .await
    }

    /// Generate a rendition from raw bytes (without reading from storage).
    ///
    /// This is useful when you already have the source bytes in memory.
    pub async fn generate_from_bytes(
        &self,
        source_bytes: &[u8],
        target_key: &str,
        rendition_type: &RenditionType,
    ) -> MediaResult<RenditionResult> {
        let target_key = parse_rendition_result_key(target_key)?;
        self.generate_from_bytes_object_key(source_bytes, &target_key, rendition_type)
            .await
    }

    /// Generate a rendition from raw bytes into a typed target object key.
    pub async fn generate_from_bytes_object_key(
        &self,
        source_bytes: &[u8],
        target_key: &BlobObjectKey,
        rendition_type: &RenditionType,
    ) -> MediaResult<RenditionResult> {
        let max_dimension = match rendition_type {
            RenditionType::Thumbnail => self.config.thumbnail_max_dimension,
            RenditionType::Preview => self.config.preview_max_dimension,
            RenditionType::Custom(_) => self.config.thumbnail_max_dimension,
        };

        generate_and_store(
            self.blob_adapter.as_ref(),
            source_bytes,
            target_key,
            max_dimension,
            self.config.jpeg_quality,
            "rendition",
        )
        .await
    }

    /// Delete all rendition blobs for a version.
    ///
    /// This fetches renditions from the repository and deletes their blobs
    /// from storage. It returns the number of blobs deleted.
    pub async fn delete_version_renditions<R: MediaRepository>(
        &self,
        repo: &R,
        version_id: MediaVersionId,
    ) -> MediaResult<u64> {
        let renditions = repo.list_renditions(version_id).await?;
        let mut deleted = 0u64;

        for rendition in renditions {
            if let Err(e) = self
                .blob_adapter
                .delete_object_key(&rendition.object_key)
                .await
            {
                tracing::warn!(
                    version_id = %version_id,
                    rendition_id = %rendition.id,
                    object_key = %rendition.object_key,
                    error = %e,
                    "Failed to delete rendition blob"
                );
            } else {
                deleted += 1;
            }
        }

        Ok(deleted)
    }

    /// Delete a single rendition blob.
    pub async fn delete_rendition_blob(&self, object_key: &str) -> MediaResult<()> {
        let object_key = parse_rendition_result_key(object_key)?;
        self.delete_rendition_blob_object_key(&object_key).await
    }

    /// Delete a single typed rendition blob.
    pub async fn delete_rendition_blob_object_key(
        &self,
        object_key: &BlobObjectKey,
    ) -> MediaResult<()> {
        self.blob_adapter
            .delete_object_key(object_key)
            .await
            .map_err(|e| MediaError::storage(format!("Failed to delete blob: {}", e)))
    }

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

        if self.config.generate_thumbnails {
            let thumb_key = parse_rendition_result_key(format!(
                "{}/{}.jpg",
                key_prefix, self.config.thumbnail_name
            ))?;
            match self
                .generate_thumbnail_object_key(&source_key, &thumb_key)
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

        if self.config.generate_previews {
            let preview_key = parse_rendition_result_key(format!(
                "{}/{}.jpg",
                key_prefix, self.config.preview_name
            ))?;
            match self
                .generate_preview_object_key(&source_key, &preview_key)
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

        if self.config.generate_thumbnails {
            let thumb_key = self
                .key_generator
                .rendition_object_key(media_id.0, version_id.0, &self.config.thumbnail_name)
                .map_err(|err| MediaError::storage(format!("invalid thumbnail key: {err}")))?;

            match self
                .generate_thumbnail_object_key(&source_key, &thumb_key)
                .await
            {
                Ok(result) => {
                    let input = CreateRenditionInput {
                        rendition_type: RenditionType::Custom(self.config.thumbnail_name.clone()),
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
                        rendition_type = %self.config.thumbnail_name,
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

        if self.config.generate_previews {
            let preview_key = self
                .key_generator
                .rendition_object_key(media_id.0, version_id.0, &self.config.preview_name)
                .map_err(|err| MediaError::storage(format!("invalid preview key: {err}")))?;

            match self
                .generate_preview_object_key(&source_key, &preview_key)
                .await
            {
                Ok(result) => {
                    let input = CreateRenditionInput {
                        rendition_type: RenditionType::Custom(self.config.preview_name.clone()),
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
                        rendition_type = %self.config.preview_name,
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

impl<B: BlobAdapter> Clone for RenditionService<B> {
    fn clone(&self) -> Self {
        Self {
            blob_adapter: Arc::clone(&self.blob_adapter),
            config: self.config.clone(),
            key_generator: self.key_generator.clone(),
        }
    }
}
