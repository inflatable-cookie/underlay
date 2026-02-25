//! Storage key generation for media files.
//!
//! This module provides standardized key formats for media files and their
//! renditions in blob storage. Using consistent key patterns across applications
//! makes it easier to:
//!
//! - Navigate stored files manually
//! - Implement cleanup logic
//! - Debug storage issues
//! - Migrate between storage providers
//!
//! # Default Key Structure
//!
//! ```text
//! media/{media_id}/
//!   versions/{version_id}/{filename}     # Original uploaded file
//!   renditions/{version_id}/thumb.jpg    # Generated thumbnail
//!   renditions/{version_id}/preview.jpg  # Generated preview
//! ```
//!
//! # Example
//!
//! ```rust
//! use underlay_media::storage::{StorageKeyGenerator, StorageKeyConfig};
//! use uuid::Uuid;
//!
//! let generator = StorageKeyGenerator::new(StorageKeyConfig::default());
//!
//! let media_id = Uuid::now_v7();
//! let version_id = Uuid::now_v7();
//!
//! // Generate key for an uploaded file
//! let version_key = generator.version_key(media_id, version_id, "photo.jpg");
//! // -> "media/{media_id}/versions/{version_id}/photo.jpg"
//!
//! // Generate key for a thumbnail
//! let thumb_key = generator.rendition_key(media_id, version_id, "thumb");
//! // -> "media/{media_id}/renditions/{version_id}/thumb.jpg"
//! ```

use uuid::Uuid;

use crate::domain::{MediaId, MediaVersionId, RenditionType};

// ============================================================================
// Configuration
// ============================================================================

/// Configuration for storage key generation.
#[derive(Clone, Debug)]
pub struct StorageKeyConfig {
    /// Base prefix for all media keys (e.g., "media" or "uploads").
    pub base_prefix: String,
    /// Subdirectory for version files.
    pub versions_dir: String,
    /// Subdirectory for rendition files.
    pub renditions_dir: String,
    /// Default extension for rendition files.
    pub rendition_extension: String,
}

impl Default for StorageKeyConfig {
    fn default() -> Self {
        Self {
            base_prefix: "media".to_string(),
            versions_dir: "versions".to_string(),
            renditions_dir: "renditions".to_string(),
            rendition_extension: "jpg".to_string(),
        }
    }
}

impl StorageKeyConfig {
    /// Create a new configuration with the given base prefix.
    pub fn with_prefix(prefix: impl Into<String>) -> Self {
        Self {
            base_prefix: prefix.into(),
            ..Default::default()
        }
    }

    /// Set the versions directory name.
    pub fn versions_dir(mut self, dir: impl Into<String>) -> Self {
        self.versions_dir = dir.into();
        self
    }

    /// Set the renditions directory name.
    pub fn renditions_dir(mut self, dir: impl Into<String>) -> Self {
        self.renditions_dir = dir.into();
        self
    }

    /// Set the default rendition extension.
    pub fn rendition_extension(mut self, ext: impl Into<String>) -> Self {
        self.rendition_extension = ext.into();
        self
    }
}

// ============================================================================
// Key Generator
// ============================================================================

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
    pub fn version_key(
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

    /// Generate an object key for a version file using typed IDs.
    pub fn version_key_typed(
        &self,
        media_id: MediaId,
        version_id: MediaVersionId,
        filename: &str,
    ) -> String {
        self.version_key(media_id.0, version_id.0, filename)
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
    pub fn rendition_key(
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

    /// Generate an object key for a rendition file using typed IDs.
    pub fn rendition_key_typed(
        &self,
        media_id: MediaId,
        version_id: MediaVersionId,
        rendition_name: &str,
    ) -> String {
        self.rendition_key(media_id.0, version_id.0, rendition_name)
    }

    /// Generate an object key for a rendition based on its type.
    ///
    /// Uses the rendition type to determine the rendition name:
    /// - `Thumbnail` -> "thumb"
    /// - `Preview` -> "preview"
    /// - `Custom(name)` -> the custom name
    pub fn rendition_key_for_type(
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

// ============================================================================
// Convenience Functions
// ============================================================================

/// Generate an object key for a version file using default configuration.
///
/// This is a convenience function for quick key generation without
/// creating a generator instance.
pub fn version_key(
    media_id: impl Into<Uuid>,
    version_id: impl Into<Uuid>,
    filename: &str,
) -> String {
    StorageKeyGenerator::with_defaults().version_key(media_id, version_id, filename)
}

/// Generate an object key for a rendition file using default configuration.
///
/// This is a convenience function for quick key generation without
/// creating a generator instance.
pub fn rendition_key(
    media_id: impl Into<Uuid>,
    version_id: impl Into<Uuid>,
    rendition_name: &str,
) -> String {
    StorageKeyGenerator::with_defaults().rendition_key(media_id, version_id, rendition_name)
}

/// Generate a filename for a version based on MIME type.
///
/// If `original_filename` is provided, returns it unchanged.
/// Otherwise, generates a filename like "file.{extension}" based on the content type.
pub fn version_filename(content_type: &str, original_filename: Option<&str>) -> String {
    if let Some(filename) = original_filename {
        return filename.to_string();
    }

    let extension = mime_to_extension(content_type);
    format!("file.{}", extension)
}

/// Map a MIME type to a file extension.
pub fn mime_to_extension(mime_type: &str) -> &'static str {
    match mime_type {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        "image/webp" => "webp",
        "image/svg+xml" => "svg",
        "image/avif" => "avif",
        "application/pdf" => "pdf",
        "video/mp4" => "mp4",
        "video/webm" => "webm",
        "audio/mpeg" => "mp3",
        "audio/wav" => "wav",
        "audio/ogg" => "ogg",
        "text/plain" => "txt",
        "text/html" => "html",
        "text/css" => "css",
        "application/javascript" => "js",
        "application/json" => "json",
        _ => "bin",
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
#[path = "tests/storage_tests.rs"]
mod tests;
