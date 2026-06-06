use uuid::Uuid;

use crate::domain::{MediaId, MediaVersionId, RenditionType};

use super::StorageKeyConfig;

/// Generates standardized storage keys for media files.
///
/// This struct provides methods to generate consistent object keys for:
/// - Version files (the original uploaded content)
/// - Rendition files (thumbnails, previews, etc.)
///
/// # Key Format
///
/// The default format produces keys like:
/// - Version: `{prefix}/{media_id}/{versions_dir}/{version_id}/{filename}`
/// - Rendition: `{prefix}/{media_id}/{renditions_dir}/{version_id}/{rendition_name}.{ext}`
#[derive(Clone, Debug)]
pub struct StorageKeyGenerator {
    config: StorageKeyConfig,
}

impl StorageKeyGenerator {
    /// Create a new storage key generator with the given configuration.
    pub fn new(config: StorageKeyConfig) -> Self {
        Self { config }
    }

    /// Create a new storage key generator with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(StorageKeyConfig::default())
    }

    /// Get the configuration.
    pub fn config(&self) -> &StorageKeyConfig {
        &self.config
    }

    /// Generate an object key for a version file.
    ///
    /// # Arguments
    ///
    /// * `media_id` - The media item ID
    /// * `version_id` - The version ID
    /// * `filename` - The original filename (with extension)
    ///
    /// # Returns
    ///
    /// A key like `media/{media_id}/versions/{version_id}/{filename}`
    fn version_key(
        &self,
        media_id: impl Into<Uuid>,
        version_id: impl Into<Uuid>,
        filename: &str,
    ) -> String {
        format!(
            "{}/{}/{}/{}/{}",
            self.config.base_prefix,
            media_id.into(),
            self.config.versions_dir,
            version_id.into(),
            filename
        )
    }

    /// Generate a validated blob object key for a version file.
    pub fn version_object_key(
        &self,
        media_id: impl Into<Uuid>,
        version_id: impl Into<Uuid>,
        filename: &str,
    ) -> Result<underlay_blob::BlobObjectKey, underlay_blob::BlobObjectKeyError> {
        underlay_blob::BlobObjectKey::parse(self.version_key(media_id, version_id, filename))
    }

    /// Generate a validated blob object key for a version file using typed IDs.
    pub fn version_object_key_typed(
        &self,
        media_id: MediaId,
        version_id: MediaVersionId,
        filename: &str,
    ) -> Result<underlay_blob::BlobObjectKey, underlay_blob::BlobObjectKeyError> {
        self.version_object_key(media_id.0, version_id.0, filename)
    }

    /// Generate an object key for a rendition file.
    ///
    /// # Arguments
    ///
    /// * `media_id` - The media item ID
    /// * `version_id` - The version ID
    /// * `rendition_name` - The rendition name (e.g., "thumb", "preview", "thumb_128")
    ///
    /// # Returns
    ///
    /// A key like `media/{media_id}/renditions/{version_id}/{rendition_name}.jpg`
    fn rendition_key(
        &self,
        media_id: impl Into<Uuid>,
        version_id: impl Into<Uuid>,
        rendition_name: &str,
    ) -> String {
        format!(
            "{}/{}/{}/{}/{}.{}",
            self.config.base_prefix,
            media_id.into(),
            self.config.renditions_dir,
            version_id.into(),
            rendition_name,
            self.config.rendition_extension
        )
    }

    /// Generate a validated blob object key for a rendition file.
    pub fn rendition_object_key(
        &self,
        media_id: impl Into<Uuid>,
        version_id: impl Into<Uuid>,
        rendition_name: &str,
    ) -> Result<underlay_blob::BlobObjectKey, underlay_blob::BlobObjectKeyError> {
        underlay_blob::BlobObjectKey::parse(self.rendition_key(
            media_id,
            version_id,
            rendition_name,
        ))
    }

    /// Generate a validated blob object key for a rendition file using typed IDs.
    pub fn rendition_object_key_typed(
        &self,
        media_id: MediaId,
        version_id: MediaVersionId,
        rendition_name: &str,
    ) -> Result<underlay_blob::BlobObjectKey, underlay_blob::BlobObjectKeyError> {
        self.rendition_object_key(media_id.0, version_id.0, rendition_name)
    }

    /// Generate an object key for a rendition based on its type.
    ///
    /// Uses the rendition type to determine the rendition name:
    /// - `Thumbnail` -> "thumb"
    /// - `Preview` -> "preview"
    /// - `Custom(name)` -> the custom name
    fn rendition_key_for_type(
        &self,
        media_id: impl Into<Uuid>,
        version_id: impl Into<Uuid>,
        rendition_type: &RenditionType,
    ) -> String {
        let name = match rendition_type {
            RenditionType::Thumbnail => "thumb",
            RenditionType::Preview => "preview",
            RenditionType::Custom(name) => name.as_str(),
        };
        self.rendition_key(media_id, version_id, name)
    }

    /// Generate a validated blob object key for a rendition based on its type.
    pub fn rendition_object_key_for_type(
        &self,
        media_id: impl Into<Uuid>,
        version_id: impl Into<Uuid>,
        rendition_type: &RenditionType,
    ) -> Result<underlay_blob::BlobObjectKey, underlay_blob::BlobObjectKeyError> {
        underlay_blob::BlobObjectKey::parse(self.rendition_key_for_type(
            media_id,
            version_id,
            rendition_type,
        ))
    }

    /// Generate the key prefix for all files of a media item.
    ///
    /// Useful for listing or deleting all files for a media item.
    pub fn media_prefix(&self, media_id: impl Into<Uuid>) -> String {
        format!("{}/{}/", self.config.base_prefix, media_id.into())
    }

    /// Generate the key prefix for all versions of a media item.
    pub fn versions_prefix(&self, media_id: impl Into<Uuid>) -> String {
        format!(
            "{}/{}/{}/",
            self.config.base_prefix,
            media_id.into(),
            self.config.versions_dir
        )
    }

    /// Generate the key prefix for all renditions of a media item.
    pub fn renditions_prefix(&self, media_id: impl Into<Uuid>) -> String {
        format!(
            "{}/{}/{}/",
            self.config.base_prefix,
            media_id.into(),
            self.config.renditions_dir
        )
    }

    /// Generate the key prefix for all renditions of a specific version.
    pub fn version_renditions_prefix(
        &self,
        media_id: impl Into<Uuid>,
        version_id: impl Into<Uuid>,
    ) -> String {
        format!(
            "{}/{}/{}/{}/",
            self.config.base_prefix,
            media_id.into(),
            self.config.renditions_dir,
            version_id.into()
        )
    }
}

impl Default for StorageKeyGenerator {
    fn default() -> Self {
        Self::with_defaults()
    }
}
