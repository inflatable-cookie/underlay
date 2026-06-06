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

mod config;
mod filename;
mod generator;

use uuid::Uuid;

#[cfg(test)]
use crate::domain::RenditionType;

pub use config::StorageKeyConfig;
pub use filename::{mime_to_extension, version_filename};
pub use generator::StorageKeyGenerator;

/// Generate an object key for a version file using default configuration.
///
/// This is a convenience function for quick key generation without creating a
/// generator instance.
pub fn version_key(
    media_id: impl Into<Uuid>,
    version_id: impl Into<Uuid>,
    filename: &str,
) -> String {
    StorageKeyGenerator::with_defaults().version_key(media_id, version_id, filename)
}

/// Generate a validated blob object key for a version file using default
/// configuration.
pub fn version_object_key(
    media_id: impl Into<Uuid>,
    version_id: impl Into<Uuid>,
    filename: &str,
) -> Result<underlay_blob::BlobObjectKey, underlay_blob::BlobObjectKeyError> {
    StorageKeyGenerator::with_defaults().version_object_key(media_id, version_id, filename)
}

/// Generate an object key for a rendition file using default configuration.
///
/// This is a convenience function for quick key generation without creating a
/// generator instance.
pub fn rendition_key(
    media_id: impl Into<Uuid>,
    version_id: impl Into<Uuid>,
    rendition_name: &str,
) -> String {
    StorageKeyGenerator::with_defaults().rendition_key(media_id, version_id, rendition_name)
}

/// Generate a validated blob object key for a rendition file using default
/// configuration.
pub fn rendition_object_key(
    media_id: impl Into<Uuid>,
    version_id: impl Into<Uuid>,
    rendition_name: &str,
) -> Result<underlay_blob::BlobObjectKey, underlay_blob::BlobObjectKeyError> {
    StorageKeyGenerator::with_defaults().rendition_object_key(media_id, version_id, rendition_name)
}

#[cfg(test)]
#[path = "../tests/storage_tests.rs"]
mod tests;
