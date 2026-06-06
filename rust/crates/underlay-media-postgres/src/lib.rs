//! PostgreSQL implementation of the MediaRepository trait.
//!
//! This module provides a PostgreSQL-backed implementation of the media
//! repository using sqlx. It supports configurable schema and table names.

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use underlay_media::MediaResult;
use underlay_media::{
    CreateMediaInput, CreateRenditionInput, FinalizeUploadInput, ListMediaParams, Media, MediaId,
    MediaRendition, MediaRenditionId, MediaRepository, MediaSummary, MediaUsage,
    MediaUsageRepository, MediaVersion, MediaVersionId, UpdateMediaInput,
};

mod list_query;
mod media_ops;
mod postgres_rows;
mod rendition_ops;
mod tables;
mod usage_ops;
mod version_ops;

pub use tables::PostgresMediaConfig;

pub(crate) trait SqlxMediaResultExt<T> {
    fn media_result(self) -> MediaResult<T>;
}

impl<T> SqlxMediaResultExt<T> for Result<T, sqlx::Error> {
    fn media_result(self) -> MediaResult<T> {
        self.map_err(|err| underlay_media::MediaError::database(err.to_string()))
    }
}

// ============================================================================
// Repository Implementation
// ============================================================================

/// PostgreSQL implementation of the MediaRepository trait.
#[derive(Clone)]
pub struct PostgresMediaRepository {
    pool: PgPool,
    config: PostgresMediaConfig,
}

impl PostgresMediaRepository {
    /// Create a new repository with the given connection pool.
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            config: PostgresMediaConfig::default(),
        }
    }

    /// Create a new repository with custom configuration.
    pub fn with_config(pool: PgPool, config: PostgresMediaConfig) -> Self {
        Self { pool, config }
    }

    /// Get the underlying connection pool.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Get the configuration.
    pub fn config(&self) -> &PostgresMediaConfig {
        &self.config
    }
}

#[async_trait]
impl MediaRepository for PostgresMediaRepository {
    // ========================================================================
    // Media CRUD
    // ========================================================================

    async fn create_media(
        &self,
        input: CreateMediaInput,
        created_by: Option<Uuid>,
    ) -> MediaResult<Media> {
        self.insert_media(input, created_by).await
    }

    async fn get_media(&self, id: MediaId) -> MediaResult<Option<Media>> {
        self.fetch_media(id).await
    }

    async fn update_media(
        &self,
        id: MediaId,
        input: UpdateMediaInput,
        _updated_by: Option<Uuid>,
    ) -> MediaResult<Media> {
        self.apply_media_update(id, input).await
    }

    async fn soft_delete_media(&self, id: MediaId, _deleted_by: Option<Uuid>) -> MediaResult<bool> {
        self.mark_media_deleted(id).await
    }

    async fn restore_media(&self, id: MediaId) -> MediaResult<bool> {
        self.restore_deleted_media(id).await
    }

    async fn hard_delete_media(&self, id: MediaId) -> MediaResult<bool> {
        self.delete_media_row(id).await
    }

    async fn list_media(&self, params: ListMediaParams) -> MediaResult<Vec<MediaSummary>> {
        self.fetch_media_list(params).await
    }

    async fn list_trash(&self) -> MediaResult<Vec<MediaSummary>> {
        self.fetch_trash().await
    }

    // ========================================================================
    // Versions
    // ========================================================================

    async fn create_version(
        &self,
        media_id: MediaId,
        created_by: Option<Uuid>,
    ) -> MediaResult<MediaVersion> {
        self.insert_version(media_id, created_by).await
    }

    async fn get_version(&self, id: MediaVersionId) -> MediaResult<Option<MediaVersion>> {
        self.fetch_version(id).await
    }

    async fn finalize_version(
        &self,
        id: MediaVersionId,
        input: FinalizeUploadInput,
    ) -> MediaResult<MediaVersion> {
        self.mark_version_ready(id, input).await
    }

    async fn fail_version(&self, id: MediaVersionId) -> MediaResult<bool> {
        self.mark_version_failed(id).await
    }

    async fn delete_version(&self, id: MediaVersionId) -> MediaResult<bool> {
        self.delete_version_row(id).await
    }

    async fn list_versions(&self, media_id: MediaId) -> MediaResult<Vec<MediaVersion>> {
        self.fetch_versions(media_id).await
    }

    async fn find_by_hash(&self, sha256: &str) -> MediaResult<Option<Media>> {
        self.fetch_media_by_hash(sha256).await
    }

    async fn set_current_version(
        &self,
        media_id: MediaId,
        version_id: MediaVersionId,
    ) -> MediaResult<()> {
        self.apply_current_version(media_id, version_id).await
    }

    // ========================================================================
    // Renditions
    // ========================================================================

    async fn create_rendition(
        &self,
        version_id: MediaVersionId,
        input: CreateRenditionInput,
    ) -> MediaResult<MediaRendition> {
        self.upsert_rendition(version_id, input).await
    }

    async fn get_rendition(&self, id: MediaRenditionId) -> MediaResult<Option<MediaRendition>> {
        self.fetch_rendition(id).await
    }

    async fn list_renditions(
        &self,
        version_id: MediaVersionId,
    ) -> MediaResult<Vec<MediaRendition>> {
        self.fetch_renditions(version_id).await
    }

    async fn delete_renditions(&self, version_id: MediaVersionId) -> MediaResult<u64> {
        self.delete_rendition_rows(version_id).await
    }

    async fn batch_soft_delete_media(
        &self,
        ids: &[MediaId],
        _deleted_by: Option<Uuid>,
    ) -> MediaResult<i64> {
        self.batch_mark_media_deleted(ids).await
    }

    async fn list_unused_media(&self) -> MediaResult<Vec<Media>> {
        self.fetch_unused_media().await
    }
}

#[async_trait]
impl MediaUsageRepository for PostgresMediaRepository {
    async fn track_usage(&self, usage: &MediaUsage) -> MediaResult<()> {
        self.insert_usage(usage).await
    }

    async fn remove_usage(
        &self,
        media_id: MediaId,
        entity_type: &str,
        entity_id: Uuid,
        field_name: &str,
    ) -> MediaResult<bool> {
        self.delete_usage(media_id, entity_type, entity_id, field_name)
            .await
    }

    async fn list_usages(&self, media_id: MediaId) -> MediaResult<Vec<MediaUsage>> {
        self.fetch_usages(media_id).await
    }

    async fn is_media_used(&self, media_id: MediaId) -> MediaResult<bool> {
        self.has_media_usage(media_id).await
    }

    async fn get_usage_count(&self, media_id: MediaId) -> MediaResult<i64> {
        self.count_media_usage(media_id).await
    }

    async fn sync_usages(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        field_name: &str,
        media_ids: &[MediaId],
    ) -> MediaResult<()> {
        self.replace_usages(entity_type, entity_id, field_name, media_ids)
            .await
    }
}
