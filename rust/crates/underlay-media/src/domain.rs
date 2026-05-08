//! Media library domain types.
//!
//! Core types for media items, versions, renditions, and usage tracking.
//! These types are database-agnostic and can be used across different
//! storage implementations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

// Re-export enums from underlay-db for convenience
pub use underlay_db::{
    detect_media_kind_from_mime_type, MediaKind, MediaTypeParseError, MediaVersionState,
    MediaVisibility,
};

// ============================================================================
// Identifiers
// ============================================================================

/// Strongly-typed identifier for a Media item.
///
/// Media items are stable references that can have multiple versions.
/// Other entities should reference media by this ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MediaId(pub Uuid);

impl MediaId {
    /// Create a new random MediaId using UUID v7.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Create a MediaId from an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the inner UUID value.
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Default for MediaId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MediaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for MediaId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

/// Strongly-typed identifier for a MediaVersion.
///
/// Each version represents an immutable snapshot of the media content.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MediaVersionId(pub Uuid);

impl MediaVersionId {
    /// Create a new random MediaVersionId using UUID v7.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Create a MediaVersionId from an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the inner UUID value.
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Default for MediaVersionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MediaVersionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for MediaVersionId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

/// Strongly-typed identifier for a MediaRendition.
///
/// Renditions are derived blobs (thumbnails, previews) generated from versions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MediaRenditionId(pub Uuid);

impl MediaRenditionId {
    /// Create a new random MediaRenditionId using UUID v7.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Create a MediaRenditionId from an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the inner UUID value.
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Default for MediaRenditionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MediaRenditionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for MediaRenditionId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

// ============================================================================
// Rendition Type
// ============================================================================

/// Type of rendition (derived image).
///
/// Renditions are pre-generated versions of media at different sizes
/// or formats for efficient delivery.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RenditionType {
    /// Small thumbnail (typically 128-256px).
    Thumbnail,
    /// Larger preview image (typically 512-1024px).
    Preview,
    /// Custom rendition type with a specific name.
    #[serde(untagged)]
    Custom(String),
}

impl RenditionType {
    /// Get the string representation of the rendition type.
    pub fn as_str(&self) -> &str {
        match self {
            RenditionType::Thumbnail => "thumbnail",
            RenditionType::Preview => "preview",
            RenditionType::Custom(s) => s.as_str(),
        }
    }
}

impl std::fmt::Display for RenditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for RenditionType {
    fn from(s: &str) -> Self {
        match s {
            "thumbnail" => RenditionType::Thumbnail,
            "preview" => RenditionType::Preview,
            other => RenditionType::Custom(other.to_string()),
        }
    }
}

impl From<String> for RenditionType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "thumbnail" => RenditionType::Thumbnail,
            "preview" => RenditionType::Preview,
            _ => RenditionType::Custom(s),
        }
    }
}

// ============================================================================
// Domain Entities
// ============================================================================

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
    pub thumbnail_object_key: Option<String>,
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
    pub object_key: Option<String>,
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
    pub object_key: String,
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

/// Broad source shape for a media usage edge.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaContentKind {
    RecordField,
    StructuredContent,
    External,
    #[serde(untagged)]
    Custom(String),
}

impl MediaContentKind {
    pub fn as_str(&self) -> &str {
        match self {
            MediaContentKind::RecordField => "record_field",
            MediaContentKind::StructuredContent => "structured_content",
            MediaContentKind::External => "external",
            MediaContentKind::Custom(value) => value.as_str(),
        }
    }
}

/// Stable locator type for a media usage edge.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaLocatorKind {
    Field,
    /// Canonical Nightfire form: `<block-id>#<json-pointer-relative-to-block.data>`.
    BlockId,
    /// Fallback form: JSON Pointer rooted at the stored field value.
    Path,
    ExternalRef,
    #[serde(untagged)]
    Custom(String),
}

impl MediaLocatorKind {
    pub fn as_str(&self) -> &str {
        match self {
            MediaLocatorKind::Field => "field",
            MediaLocatorKind::BlockId => "block_id",
            MediaLocatorKind::Path => "path",
            MediaLocatorKind::ExternalRef => "external_ref",
            MediaLocatorKind::Custom(value) => value.as_str(),
        }
    }
}

/// Semantic role for a media usage edge.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaUsageRole {
    Primary,
    Attachment,
    Embedded,
    External,
    Derived,
    #[serde(untagged)]
    Custom(String),
}

impl MediaUsageRole {
    pub fn as_str(&self) -> &str {
        match self {
            MediaUsageRole::Primary => "primary",
            MediaUsageRole::Attachment => "attachment",
            MediaUsageRole::Embedded => "embedded",
            MediaUsageRole::External => "external",
            MediaUsageRole::Derived => "derived",
            MediaUsageRole::Custom(value) => value.as_str(),
        }
    }
}

/// Source-of-truth lane responsible for a usage edge.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaUsageProvenanceKind {
    ContentSync,
    LegacyMigration,
    Manual,
    SystemGenerated,
    #[serde(untagged)]
    Custom(String),
}

impl MediaUsageProvenanceKind {
    pub fn as_str(&self) -> &str {
        match self {
            MediaUsageProvenanceKind::ContentSync => "content_sync",
            MediaUsageProvenanceKind::LegacyMigration => "legacy_migration",
            MediaUsageProvenanceKind::Manual => "manual",
            MediaUsageProvenanceKind::SystemGenerated => "system_generated",
            MediaUsageProvenanceKind::Custom(value) => value.as_str(),
        }
    }
}

/// Natural key for a media usage edge inside one managed scope.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MediaUsageEdgeKey {
    pub media_id: MediaId,
    pub used_by_type: String,
    pub used_by_id: Option<Uuid>,
    pub owner_field: Option<String>,
    pub locator_kind: MediaLocatorKind,
    pub locator_key: String,
    pub provenance_kind: MediaUsageProvenanceKind,
}

/// Desired or persisted media usage edge without persistence metadata.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaUsageEdgeInput {
    pub media_id: MediaId,
    pub used_by_type: String,
    pub used_by_id: Option<Uuid>,
    pub owner_field: Option<String>,
    pub content_kind: MediaContentKind,
    pub locator_kind: MediaLocatorKind,
    pub locator_key: String,
    pub usage_role: MediaUsageRole,
    pub provenance_kind: MediaUsageProvenanceKind,
}

impl MediaUsageEdgeInput {
    pub fn key(&self) -> MediaUsageEdgeKey {
        MediaUsageEdgeKey {
            media_id: self.media_id,
            used_by_type: self.used_by_type.clone(),
            used_by_id: self.used_by_id,
            owner_field: self.owner_field.clone(),
            locator_kind: self.locator_kind.clone(),
            locator_key: self.locator_key.clone(),
            provenance_kind: self.provenance_kind.clone(),
        }
    }
}

/// Persisted media usage edge.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaUsageEdge {
    pub id: Uuid,
    pub media_id: MediaId,
    pub used_by_type: String,
    pub used_by_id: Option<Uuid>,
    pub owner_field: Option<String>,
    pub content_kind: MediaContentKind,
    pub locator_kind: MediaLocatorKind,
    pub locator_key: String,
    pub usage_role: MediaUsageRole,
    pub provenance_kind: MediaUsageProvenanceKind,
    pub created_at: DateTime<Utc>,
}

impl MediaUsageEdge {
    pub fn key(&self) -> MediaUsageEdgeKey {
        MediaUsageEdgeKey {
            media_id: self.media_id,
            used_by_type: self.used_by_type.clone(),
            used_by_id: self.used_by_id,
            owner_field: self.owner_field.clone(),
            locator_kind: self.locator_kind.clone(),
            locator_key: self.locator_key.clone(),
            provenance_kind: self.provenance_kind.clone(),
        }
    }
}

/// Identity for a migrated source attachment.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MigratedAttachmentIdentity {
    pub source_system: String,
    pub source_attachment_type: String,
    pub source_attachment_id: String,
    pub source_owner_type: String,
    pub source_owner_id: String,
    pub field_or_purpose: String,
}

/// Persisted replay-safe source attachment binding.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigratedAttachmentBinding {
    pub id: Uuid,
    pub identity: MigratedAttachmentIdentity,
    pub sha256: String,
    pub bundle_digest: String,
    pub media_id: MediaId,
    pub media_version_id: MediaVersionId,
    pub import_status: String,
    pub imported_at: DateTime<Utc>,
}

/// Input for storing or updating a migrated attachment binding.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigratedAttachmentBindingInput {
    pub identity: MigratedAttachmentIdentity,
    pub sha256: String,
    pub bundle_digest: String,
    pub media_id: MediaId,
    pub media_version_id: MediaVersionId,
    pub import_status: String,
}

/// A managed field payload that should be scanned for media references.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaUsageFieldPayload {
    pub owner_field: String,
    pub content_kind: MediaContentKind,
    pub value: Value,
}

// ============================================================================
// Input Types
// ============================================================================

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
    pub object_key: String,
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
    pub object_key: String,
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

// ============================================================================
// Query Parameters
// ============================================================================

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

#[cfg(test)]
#[path = "tests/domain_tests.rs"]
mod tests;
