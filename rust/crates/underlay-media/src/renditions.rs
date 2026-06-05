//! Rendition generation service.
//!
//! This module provides functionality for generating thumbnails and previews
//! from source images, and managing their storage lifecycle.

use std::sync::Arc;

use crate::image::{generate_thumbnail, ThumbnailConfig};
use underlay_blob::{BlobAdapter, BlobAdapterObjectKeyExt, BlobObjectKey};

use crate::domain::{CreateRenditionInput, MediaId, MediaRendition, MediaVersionId, RenditionType};
use crate::error::{MediaError, MediaResult};
use crate::repository::MediaRepository;
use crate::storage::StorageKeyGenerator;

// ============================================================================
// Configuration
// ============================================================================

/// Configuration for rendition generation.
#[derive(Clone, Debug)]
pub struct RenditionConfig {
    /// Maximum dimension for thumbnail renditions.
    pub thumbnail_max_dimension: u32,
    /// Maximum dimension for preview renditions.
    pub preview_max_dimension: u32,
    /// JPEG quality for rendered images (1-100).
    pub jpeg_quality: u8,
    /// Whether to generate square (cropped) thumbnails.
    pub square_thumbnails: bool,
    /// Whether to generate thumbnails for images.
    pub generate_thumbnails: bool,
    /// Whether to generate previews for images.
    pub generate_previews: bool,
    /// Custom thumbnail rendition name (e.g., "thumb_128" for Farmyard compatibility).
    pub thumbnail_name: String,
    /// Custom preview rendition name.
    pub preview_name: String,
}

impl Default for RenditionConfig {
    fn default() -> Self {
        Self {
            thumbnail_max_dimension: 400,
            preview_max_dimension: 1200,
            jpeg_quality: 85,
            square_thumbnails: false,
            generate_thumbnails: true,
            generate_previews: false,
            thumbnail_name: "thumb".to_string(),
            preview_name: "preview".to_string(),
        }
    }
}

impl RenditionConfig {
    /// Create a config with only thumbnails enabled.
    pub fn thumbnails_only() -> Self {
        Self {
            generate_thumbnails: true,
            generate_previews: false,
            ..Default::default()
        }
    }

    /// Create a config with both thumbnails and previews enabled.
    pub fn with_previews() -> Self {
        Self {
            generate_thumbnails: true,
            generate_previews: true,
            ..Default::default()
        }
    }

    /// Create a farmyard-compatible config with 128x128 square thumbnails.
    ///
    /// This matches the default Farmyard configuration:
    /// - 128px square thumbnails
    /// - Quality 80
    /// - Rendition name "thumb_128"
    pub fn farmyard_compat() -> Self {
        Self {
            thumbnail_max_dimension: 128,
            jpeg_quality: 80,
            square_thumbnails: true,
            generate_thumbnails: true,
            generate_previews: false,
            thumbnail_name: "thumb_128".to_string(),
            ..Default::default()
        }
    }

    /// Set the thumbnail maximum dimension.
    pub fn thumbnail_size(mut self, max_dim: u32) -> Self {
        self.thumbnail_max_dimension = max_dim;
        self
    }

    /// Set the preview maximum dimension.
    pub fn preview_size(mut self, max_dim: u32) -> Self {
        self.preview_max_dimension = max_dim;
        self
    }

    /// Set the JPEG quality.
    pub fn quality(mut self, quality: u8) -> Self {
        self.jpeg_quality = quality.clamp(1, 100);
        self
    }

    /// Enable square (center-cropped) thumbnails.
    pub fn square(mut self) -> Self {
        self.square_thumbnails = true;
        self
    }

    /// Set custom thumbnail rendition name.
    pub fn thumbnail_name(mut self, name: impl Into<String>) -> Self {
        self.thumbnail_name = name.into();
        self
    }

    /// Set custom preview rendition name.
    pub fn preview_name(mut self, name: impl Into<String>) -> Self {
        self.preview_name = name.into();
        self
    }
}

// ============================================================================
// Result Types
// ============================================================================

/// Result of generating a rendition.
#[derive(Debug)]
pub struct RenditionResult {
    /// Object key where the rendition was stored.
    pub object_key: BlobObjectKey,
    /// MIME type of the rendition.
    pub mime_type: String,
    /// Size in bytes.
    pub byte_size: i64,
    /// Width in pixels.
    pub width: i32,
    /// Height in pixels.
    pub height: i32,
}

// ============================================================================
// Service
// ============================================================================

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
        // Read source image
        let source_bytes = self
            .blob_adapter
            .get_object_bytes(source_key)
            .await
            .map_err(|e| MediaError::storage(format!("Failed to read source: {}", e)))?;

        // Generate thumbnail
        let config = ThumbnailConfig::new(
            self.config.thumbnail_max_dimension,
            self.config.thumbnail_max_dimension,
        )
        .with_quality(self.config.jpeg_quality);

        let thumb = generate_thumbnail(&source_bytes, &config)
            .map_err(|e| MediaError::image_processing(e.to_string()))?;

        // Store thumbnail
        let stored = self
            .blob_adapter
            .put_object_bytes(target_key, &thumb.data, thumb.mime_type)
            .await
            .map_err(|e| MediaError::storage(format!("Failed to store thumbnail: {}", e)))?;

        Ok(RenditionResult {
            object_key: parse_rendition_result_key(&stored.key)?,
            mime_type: thumb.mime_type.to_string(),
            byte_size: stored.size as i64,
            width: thumb.width as i32,
            height: thumb.height as i32,
        })
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
        // Read source image
        let source_bytes = self
            .blob_adapter
            .get_object_bytes(source_key)
            .await
            .map_err(|e| MediaError::storage(format!("Failed to read source: {}", e)))?;

        // Generate preview (larger than thumbnail)
        let config = ThumbnailConfig::new(
            self.config.preview_max_dimension,
            self.config.preview_max_dimension,
        )
        .with_quality(self.config.jpeg_quality);

        let preview = generate_thumbnail(&source_bytes, &config)
            .map_err(|e| MediaError::image_processing(e.to_string()))?;

        // Store preview
        let stored = self
            .blob_adapter
            .put_object_bytes(target_key, &preview.data, preview.mime_type)
            .await
            .map_err(|e| MediaError::storage(format!("Failed to store preview: {}", e)))?;

        Ok(RenditionResult {
            object_key: parse_rendition_result_key(&stored.key)?,
            mime_type: preview.mime_type.to_string(),
            byte_size: stored.size as i64,
            width: preview.width as i32,
            height: preview.height as i32,
        })
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
        let max_dim = match rendition_type {
            RenditionType::Thumbnail => self.config.thumbnail_max_dimension,
            RenditionType::Preview => self.config.preview_max_dimension,
            RenditionType::Custom(_) => self.config.thumbnail_max_dimension,
        };

        let config = ThumbnailConfig::new(max_dim, max_dim).with_quality(self.config.jpeg_quality);

        let result = generate_thumbnail(source_bytes, &config)
            .map_err(|e| MediaError::image_processing(e.to_string()))?;

        let stored = self
            .blob_adapter
            .put_object_bytes(target_key, &result.data, result.mime_type)
            .await
            .map_err(|e| MediaError::storage(format!("Failed to store rendition: {}", e)))?;

        Ok(RenditionResult {
            object_key: parse_rendition_result_key(&stored.key)?,
            mime_type: result.mime_type.to_string(),
            byte_size: stored.size as i64,
            width: result.width as i32,
            height: result.height as i32,
        })
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

fn parse_rendition_result_key(key: impl AsRef<str>) -> MediaResult<BlobObjectKey> {
    BlobObjectKey::parse(key)
        .map_err(|err| MediaError::storage(format!("invalid rendition object key: {err}")))
}

#[cfg(test)]
#[path = "tests/renditions_tests.rs"]
mod tests;
