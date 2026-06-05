use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use underlay_blob::BlobObjectKey;
use uuid::Uuid;

use super::identifiers::{MediaId, MediaRenditionId, MediaVersionId};
use super::kinds::{MediaKind, MediaVersionState, MediaVisibility};
use super::rendition_types::RenditionType;

/// A media library item.
///
/// Media items are stable references that content can link to. Each media
/// item can have multiple versions (for replacements/updates), with one
/// designated as the "current" version.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Media {
    /// Unique identifier for this media item.
    pub id: MediaId,
    /// Type of media (image, PDF, etc.).
    pub kind: MediaKind,
    /// Access visibility level.
    pub visibility: MediaVisibility,
    /// Display title for the media.
    pub title: String,
    /// Original filename when uploaded.
    pub original_filename: Option<String>,
    /// Alternative text for accessibility.
    pub alt_text: Option<String>,
    /// The currently active version (if any).
    pub current_version_id: Option<MediaVersionId>,
    /// When the media was soft-deleted (if deleted).
    pub deleted_at: Option<DateTime<Utc>>,
    /// When the media was created.
    pub created_at: DateTime<Utc>,
    /// When the media was last updated.
    pub updated_at: DateTime<Utc>,
    /// Who created the media (optional user ID).
    pub created_by: Option<Uuid>,
}

impl Media {
    /// Check if this media item has been soft-deleted.
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// Check if this media item has a current version.
    pub fn has_version(&self) -> bool {
        self.current_version_id.is_some()
    }
}

/// Summary view of media with current version info.
///
/// This is a denormalized view that includes commonly needed data
/// from the current version, suitable for list views.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaSummary {
    /// Unique identifier for this media item.
    pub id: MediaId,
    /// Type of media (image, PDF, etc.).
    pub kind: MediaKind,
    /// Access visibility level.
    pub visibility: MediaVisibility,
    /// Display title for the media.
    pub title: String,
    /// Original filename when uploaded.
    pub original_filename: Option<String>,
    /// The currently active version (if any).
    pub current_version_id: Option<MediaVersionId>,
    /// When the media was created.
    pub created_at: DateTime<Utc>,
    /// When the media was last updated.
    pub updated_at: DateTime<Utc>,
    /// When the media was soft-deleted (if deleted).
    pub deleted_at: Option<DateTime<Utc>>,
    /// Size of current version in bytes.
    pub byte_size: Option<i64>,
    /// MIME type of current version.
    pub mime_type: Option<String>,
    /// Object key for thumbnail rendition (if available).
    pub thumbnail_object_key: Option<BlobObjectKey>,
}

impl MediaSummary {
    /// Check if this media item has been soft-deleted.
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}

/// An immutable version of a media item.
///
/// Each version represents a specific upload of content. Versions are
/// immutable once finalized - to change content, create a new version.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaVersion {
    /// Unique identifier for this version.
    pub id: MediaVersionId,
    /// The media item this version belongs to.
    pub media_id: MediaId,
    /// Current state in the upload lifecycle.
    pub state: MediaVersionState,
    /// Object key in blob storage.
    pub object_key: Option<BlobObjectKey>,
    /// MIME type of the content.
    pub mime_type: Option<String>,
    /// Size in bytes.
    pub byte_size: Option<i64>,
    /// SHA-256 hash for deduplication.
    pub sha256_hash: Option<String>,
    /// Width in pixels (for images).
    pub width: Option<i32>,
    /// Height in pixels (for images).
    pub height: Option<i32>,
    /// Storage provider name (e.g., "s3", "local").
    pub storage_provider: Option<String>,
    /// Storage bucket/container name.
    pub bucket: Option<String>,
    /// Who uploaded this version.
    pub uploaded_by: Option<Uuid>,
    /// When this version was created.
    pub created_at: DateTime<Utc>,
}

impl MediaVersion {
    /// Check if this version is ready for use.
    pub fn is_ready(&self) -> bool {
        self.state.is_ready()
    }

    /// Check if this version has completed (success or failure).
    pub fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }

    /// Check if this version has storage info (provider, bucket, key).
    pub fn has_storage_info(&self) -> bool {
        self.storage_provider.is_some() && self.bucket.is_some() && self.object_key.is_some()
    }
}

/// A derived rendition of a media version.
///
/// Renditions are automatically generated images (thumbnails, previews)
/// derived from the source version for efficient delivery.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaRendition {
    /// Unique identifier for this rendition.
    pub id: MediaRenditionId,
    /// The version this rendition was generated from.
    pub version_id: MediaVersionId,
    /// Type of rendition (thumbnail, preview, etc.).
    pub rendition_type: RenditionType,
    /// Object key in blob storage.
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
    /// Storage bucket/container name.
    pub bucket: String,
    /// When this rendition was created.
    pub created_at: DateTime<Utc>,
}

/// A record of where media is used.
///
/// Usage tracking enables finding all references to a media item and
/// preventing deletion of media that is still in use.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaUsage {
    /// Unique identifier for this usage record.
    pub id: Uuid,
    /// The media item being referenced.
    pub media_id: MediaId,
    /// Type of entity using the media (e.g., "module", "question").
    pub entity_type: String,
    /// ID of the entity using the media.
    pub entity_id: Uuid,
    /// Field name where the media is used (e.g., "cover_image", "content").
    pub field_name: String,
    /// When this usage was recorded.
    pub created_at: DateTime<Utc>,
}
