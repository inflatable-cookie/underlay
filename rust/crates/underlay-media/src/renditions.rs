//! Rendition generation service.
//!
//! This module provides functionality for generating thumbnails and previews
//! from source images, and managing their storage lifecycle.

use std::sync::Arc;

use underlay_blob::BlobAdapter;
use underlay_image::{generate_thumbnail, ThumbnailConfig};

use crate::domain::{CreateRenditionInput, MediaRendition, MediaVersionId, RenditionType};
use crate::error::{MediaError, MediaResult};
use crate::repository::MediaRepository;

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
    /// Whether to generate thumbnails for images.
    pub generate_thumbnails: bool,
    /// Whether to generate previews for images.
    pub generate_previews: bool,
}

impl Default for RenditionConfig {
    fn default() -> Self {
        Self {
            thumbnail_max_dimension: 400,
            preview_max_dimension: 1200,
            jpeg_quality: 85,
            generate_thumbnails: true,
            generate_previews: false,
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
}

// ============================================================================
// Result Types
// ============================================================================

/// Result of generating a rendition.
#[derive(Debug)]
pub struct RenditionResult {
    /// Object key where the rendition was stored.
    pub object_key: String,
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
}

impl<B: BlobAdapter> RenditionService<B> {
    /// Create a new rendition service.
    pub fn new(blob_adapter: Arc<B>, config: RenditionConfig) -> Self {
        Self {
            blob_adapter,
            config,
        }
    }

    /// Create a new rendition service with default configuration.
    pub fn with_defaults(blob_adapter: Arc<B>) -> Self {
        Self::new(blob_adapter, RenditionConfig::default())
    }

    /// Get the configuration.
    pub fn config(&self) -> &RenditionConfig {
        &self.config
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
        // Read source image
        let source_bytes = self
            .blob_adapter
            .get_bytes(source_key)
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
            .put_bytes(target_key, &thumb.data, thumb.mime_type)
            .await
            .map_err(|e| MediaError::storage(format!("Failed to store thumbnail: {}", e)))?;

        Ok(RenditionResult {
            object_key: stored.key,
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
        // Read source image
        let source_bytes = self
            .blob_adapter
            .get_bytes(source_key)
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
            .put_bytes(target_key, &preview.data, preview.mime_type)
            .await
            .map_err(|e| MediaError::storage(format!("Failed to store preview: {}", e)))?;

        Ok(RenditionResult {
            object_key: stored.key,
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
            .put_bytes(target_key, &result.data, result.mime_type)
            .await
            .map_err(|e| MediaError::storage(format!("Failed to store rendition: {}", e)))?;

        Ok(RenditionResult {
            object_key: stored.key,
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
            if let Err(e) = self.blob_adapter.delete(&rendition.object_key).await {
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
        self.blob_adapter
            .delete(object_key)
            .await
            .map_err(|e| MediaError::storage(format!("Failed to delete blob: {}", e)))
    }

    /// Generate all configured renditions for a version.
    ///
    /// This generates thumbnails and/or previews based on the configuration,
    /// storing them and recording them in the repository.
    pub async fn generate_version_renditions<R: MediaRepository>(
        &self,
        repo: &R,
        version_id: MediaVersionId,
        source_key: &str,
        key_prefix: &str,
    ) -> MediaResult<Vec<MediaRendition>> {
        let mut renditions = Vec::new();

        if self.config.generate_thumbnails {
            let thumb_key = format!("{}/thumb.jpg", key_prefix);
            match self.generate_thumbnail(source_key, &thumb_key).await {
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
            let preview_key = format!("{}/preview.jpg", key_prefix);
            match self.generate_preview(source_key, &preview_key).await {
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
}

impl<B: BlobAdapter> Clone for RenditionService<B> {
    fn clone(&self) -> Self {
        Self {
            blob_adapter: Arc::clone(&self.blob_adapter),
            config: self.config.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rendition_config_defaults() {
        let config = RenditionConfig::default();
        assert_eq!(config.thumbnail_max_dimension, 400);
        assert_eq!(config.preview_max_dimension, 1200);
        assert_eq!(config.jpeg_quality, 85);
        assert!(config.generate_thumbnails);
        assert!(!config.generate_previews);
    }

    #[test]
    fn test_rendition_config_builder() {
        let config = RenditionConfig::with_previews()
            .thumbnail_size(256)
            .preview_size(1024)
            .quality(90);

        assert_eq!(config.thumbnail_max_dimension, 256);
        assert_eq!(config.preview_max_dimension, 1024);
        assert_eq!(config.jpeg_quality, 90);
        assert!(config.generate_thumbnails);
        assert!(config.generate_previews);
    }

    #[test]
    fn test_quality_clamping() {
        let config = RenditionConfig::default().quality(150);
        assert_eq!(config.jpeg_quality, 100);

        let config = RenditionConfig::default().quality(0);
        assert_eq!(config.jpeg_quality, 1);
    }
}
