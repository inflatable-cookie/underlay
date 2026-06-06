//! Storage key generation for media version and rendition objects.

mod config;
mod filename;
mod generator;

use uuid::Uuid;

#[cfg(test)]
use crate::domain::RenditionType;

pub use config::StorageKeyConfig;
pub use filename::{mime_to_extension, version_filename};
pub use generator::StorageKeyGenerator;

/// Generate a validated blob object key for a version file using default
/// configuration.
pub fn version_object_key(
    media_id: impl Into<Uuid>,
    version_id: impl Into<Uuid>,
    filename: &str,
) -> Result<underlay_blob::BlobObjectKey, underlay_blob::BlobObjectKeyError> {
    StorageKeyGenerator::with_defaults().version_object_key(media_id, version_id, filename)
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
