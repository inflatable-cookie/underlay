use underlay_blob::BlobObjectKey;

use super::kinds::{MediaKind, MediaVisibility};
use super::rendition_types::RenditionType;

/// Input for creating a new media item.
#[derive(Clone, Debug)]
pub struct CreateMediaInput {
    /// Type of media being created.
    pub kind: MediaKind,
    /// Initial visibility level.
    pub visibility: MediaVisibility,
    /// Display title.
    pub title: String,
    /// Original filename (if known).
    pub original_filename: Option<String>,
    /// Alternative text for accessibility.
    pub alt_text: Option<String>,
}

/// Input for updating media metadata.
#[derive(Clone, Debug)]
pub struct UpdateMediaInput {
    /// New display title.
    pub title: String,
    /// New original filename.
    pub original_filename: Option<String>,
    /// New visibility level.
    pub visibility: MediaVisibility,
    /// New alternative text.
    pub alt_text: Option<String>,
}

/// Input for finalizing an upload.
///
/// This contains the storage information to record after a successful upload.
#[derive(Clone, Debug)]
pub struct FinalizeUploadInput {
    /// Size in bytes.
    pub byte_size: i64,
    /// MIME type of the content.
    pub mime_type: String,
    /// SHA-256 hash of the content.
    pub sha256_hash: String,
    /// Storage provider name.
    pub storage_provider: String,
    /// Storage bucket name.
    pub bucket: String,
    /// Object key in storage.
    pub object_key: BlobObjectKey,
    /// Width in pixels (for images).
    pub width: Option<i32>,
    /// Height in pixels (for images).
    pub height: Option<i32>,
}

/// Input for creating a rendition.
#[derive(Clone, Debug)]
pub struct CreateRenditionInput {
    /// Type of rendition.
    pub rendition_type: RenditionType,
    /// Object key in storage.
    pub object_key: BlobObjectKey,
    /// MIME type of the rendition.
    pub mime_type: String,
    /// Size in bytes.
    pub byte_size: i64,
    /// Width in pixels.
    pub width: Option<i32>,
    /// Height in pixels.
    pub height: Option<i32>,
    /// Storage provider name.
    pub storage_provider: String,
    /// Storage bucket name.
    pub bucket: String,
}

/// Parameters for listing media items.
#[derive(Clone, Debug, Default)]
pub struct ListMediaParams {
    /// Filter by media kind.
    pub kind: Option<MediaKind>,
    /// Filter by visibility.
    pub visibility: Option<MediaVisibility>,
    /// Search query (matches title, filename).
    pub search: Option<String>,
    /// Include soft-deleted items.
    pub include_deleted: bool,
    /// Only show unused items.
    pub unused_only: bool,
    /// Maximum number of items to return.
    pub limit: Option<u32>,
    /// Cursor for pagination.
    pub cursor: Option<String>,
}
