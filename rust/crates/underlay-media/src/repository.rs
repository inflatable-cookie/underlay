//! Repository trait for media operations.
//!
//! This trait defines the interface for media storage and retrieval.
//! Implementations can use different backends (PostgreSQL, in-memory, etc.).

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    CreateMediaInput, CreateRenditionInput, FinalizeUploadInput, ListMediaParams, Media, MediaId,
    MediaRendition, MediaRenditionId, MediaSummary, MediaUsage, MediaVersion, MediaVersionId,
    UpdateMediaInput,
};
use crate::error::MediaResult;

/// Repository trait for media operations.
///
/// This trait uses an associated error type, allowing implementations
/// to use their own error types (e.g., `sqlx::Error` for PostgreSQL).
///
/// # Example Implementation
///
/// ```rust,ignore
/// use underlay_media::{MediaRepository, MediaResult};
///
/// struct MyRepository { /* ... */ }
///
/// #[async_trait]
/// impl MediaRepository for MyRepository {
///     async fn create_media(&self, input: CreateMediaInput, created_by: Option<Uuid>) -> MediaResult<Media> {
///         // Implementation...
///     }
///     // ... other methods
/// }
/// ```
#[async_trait]
pub trait MediaRepository: Send + Sync {
    // ========================================================================
    // Media CRUD
    // ========================================================================

    /// Create a new media item.
    async fn create_media(
        &self,
        input: CreateMediaInput,
        created_by: Option<Uuid>,
    ) -> MediaResult<Media>;

    /// Get a media item by ID.
    async fn get_media(&self, id: MediaId) -> MediaResult<Option<Media>>;

    /// Update a media item's metadata.
    async fn update_media(
        &self,
        id: MediaId,
        input: UpdateMediaInput,
        updated_by: Option<Uuid>,
    ) -> MediaResult<Media>;

    /// Soft-delete a media item.
    ///
    /// This marks the media as deleted but doesn't remove it from storage.
    /// Use `purge_media` to permanently delete.
    async fn soft_delete_media(&self, id: MediaId, deleted_by: Option<Uuid>) -> MediaResult<bool>;

    /// Restore a soft-deleted media item.
    async fn restore_media(&self, id: MediaId) -> MediaResult<bool>;

    /// Permanently delete a media item and all its versions.
    ///
    /// **Warning**: This does not delete blobs from storage. Call
    /// `delete_version_blobs` for each version first if needed.
    async fn hard_delete_media(&self, id: MediaId) -> MediaResult<bool>;

    /// List media items with optional filters.
    async fn list_media(&self, params: ListMediaParams) -> MediaResult<Vec<MediaSummary>>;

    /// List soft-deleted media items (trash).
    async fn list_trash(&self) -> MediaResult<Vec<MediaSummary>>;

    // ========================================================================
    // Versions
    // ========================================================================

    /// Create a new version for a media item.
    ///
    /// The version starts in the `Uploading` state.
    async fn create_version(
        &self,
        media_id: MediaId,
        created_by: Option<Uuid>,
    ) -> MediaResult<MediaVersion>;

    /// Get a version by ID.
    async fn get_version(&self, id: MediaVersionId) -> MediaResult<Option<MediaVersion>>;

    /// Finalize a version after successful upload.
    ///
    /// This updates the version with storage info and sets state to `Ready`.
    async fn finalize_version(
        &self,
        id: MediaVersionId,
        input: FinalizeUploadInput,
    ) -> MediaResult<MediaVersion>;

    /// Mark a version as failed.
    async fn fail_version(&self, id: MediaVersionId) -> MediaResult<bool>;

    /// Delete a version.
    ///
    /// **Note**: This does not delete renditions or blobs from storage.
    async fn delete_version(&self, id: MediaVersionId) -> MediaResult<bool>;

    /// List all versions for a media item.
    async fn list_versions(&self, media_id: MediaId) -> MediaResult<Vec<MediaVersion>>;

    /// Find a media item by content hash.
    ///
    /// This is used for deduplication - if content with the same hash
    /// already exists, we can reuse it.
    async fn find_by_hash(&self, sha256: &str) -> MediaResult<Option<Media>>;

    /// Set the current version for a media item.
    async fn set_current_version(
        &self,
        media_id: MediaId,
        version_id: MediaVersionId,
    ) -> MediaResult<()>;

    // ========================================================================
    // Renditions
    // ========================================================================

    /// Create a rendition for a version.
    async fn create_rendition(
        &self,
        version_id: MediaVersionId,
        input: CreateRenditionInput,
    ) -> MediaResult<MediaRendition>;

    /// Get a rendition by ID.
    async fn get_rendition(&self, id: MediaRenditionId) -> MediaResult<Option<MediaRendition>>;

    /// List all renditions for a version.
    async fn list_renditions(&self, version_id: MediaVersionId)
        -> MediaResult<Vec<MediaRendition>>;

    /// Delete all renditions for a version.
    ///
    /// Returns the number of renditions deleted.
    async fn delete_renditions(&self, version_id: MediaVersionId) -> MediaResult<u64>;

    // ========================================================================
    // Usage tracking
    // ========================================================================

    /// Track that a media item is used by an entity.
    async fn track_usage(&self, usage: &MediaUsage) -> MediaResult<()>;

    /// Remove a usage record.
    async fn remove_usage(
        &self,
        media_id: MediaId,
        entity_type: &str,
        entity_id: Uuid,
        field_name: &str,
    ) -> MediaResult<bool>;

    /// List all usages for a media item.
    async fn list_usages(&self, media_id: MediaId) -> MediaResult<Vec<MediaUsage>>;

    /// Check if a media item is used by any entity.
    async fn is_media_used(&self, media_id: MediaId) -> MediaResult<bool>;

    /// Get the usage count for a media item.
    async fn get_usage_count(&self, media_id: MediaId) -> MediaResult<i64>;

    /// Sync usages for a content field.
    ///
    /// This compares current usages for a (entity_type, entity_id, field_name)
    /// against the provided media_ids, adding new usages and removing stale ones.
    async fn sync_usages(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        field_name: &str,
        media_ids: &[MediaId],
    ) -> MediaResult<()>;

    // ========================================================================
    // Batch operations
    // ========================================================================

    /// Batch soft-delete multiple media items.
    ///
    /// Returns the count of items actually deleted.
    async fn batch_soft_delete_media(
        &self,
        ids: &[MediaId],
        deleted_by: Option<Uuid>,
    ) -> MediaResult<i64> {
        let mut count = 0i64;
        for id in ids {
            if self.soft_delete_media(*id, deleted_by).await? {
                count += 1;
            }
        }
        Ok(count)
    }

    /// List unused media items.
    ///
    /// Returns media items that have no usage records.
    async fn list_unused_media(&self) -> MediaResult<Vec<Media>>;
}

/// Extension trait for repository operations that span multiple concerns.
///
/// These methods have default implementations that compose basic repository
/// operations but can be overridden for optimization.
#[async_trait]
pub trait MediaRepositoryExt: MediaRepository {
    /// Finalize a version and set it as the current version atomically.
    ///
    /// Default implementation calls `finalize_version` then `set_current_version`.
    async fn finalize_and_set_current(
        &self,
        version_id: MediaVersionId,
        input: FinalizeUploadInput,
    ) -> MediaResult<MediaVersion> {
        let version = self.finalize_version(version_id, input).await?;
        self.set_current_version(version.media_id, version.id)
            .await?;
        Ok(version)
    }
}

// Blanket implementation for all MediaRepository implementations
impl<T: MediaRepository> MediaRepositoryExt for T {}

#[cfg(test)]
#[path = "tests/repository_tests.rs"]
mod tests;
