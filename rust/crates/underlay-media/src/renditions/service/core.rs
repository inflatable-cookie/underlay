use std::sync::Arc;

use underlay_blob::BlobAdapter;

use super::super::config::RenditionConfig;
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
    pub(super) blob_adapter: Arc<B>,
    pub(super) config: RenditionConfig,
    pub(super) key_generator: StorageKeyGenerator,
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
