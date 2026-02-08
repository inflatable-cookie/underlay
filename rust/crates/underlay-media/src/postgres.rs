//! PostgreSQL implementation of the MediaRepository trait.
//!
//! This module provides a PostgreSQL-backed implementation of the media
//! repository using sqlx. It supports configurable schema and table names.

use async_trait::async_trait;
use chrono::Utc;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::{
    CreateMediaInput, CreateRenditionInput, FinalizeUploadInput, ListMediaParams, Media, MediaId,
    MediaRendition, MediaRenditionId, MediaSummary, MediaUsage, MediaVersion, MediaVersionId,
    UpdateMediaInput,
};
use crate::error::{MediaError, MediaResult};
use crate::postgres_rows::{
    MediaRenditionRow, MediaRow, MediaSummaryRow, MediaUsageRow, MediaVersionRow,
};
use crate::repository::MediaRepository;

// ============================================================================
// Configuration
// ============================================================================

/// Configuration for the PostgreSQL media repository.
#[derive(Clone, Debug)]
pub struct PostgresMediaConfig {
    /// Database schema name.
    pub schema: String,
    /// Table name for media items.
    pub media_table: String,
    /// Table name for media versions.
    pub versions_table: String,
    /// Table name for media renditions.
    pub renditions_table: String,
    /// Table name for media usages.
    pub usages_table: String,
}

impl Default for PostgresMediaConfig {
    fn default() -> Self {
        Self {
            schema: "media".to_string(),
            media_table: "media".to_string(),
            versions_table: "media_versions".to_string(),
            renditions_table: "media_renditions".to_string(),
            usages_table: "media_usages".to_string(),
        }
    }
}

impl PostgresMediaConfig {
    /// Create a new config with the given schema name.
    pub fn with_schema(schema: impl Into<String>) -> Self {
        Self {
            schema: schema.into(),
            ..Default::default()
        }
    }

    /// Get the fully qualified table name for media.
    fn media_fqn(&self) -> String {
        format!("{}.{}", self.schema, self.media_table)
    }

    /// Get the fully qualified table name for versions.
    fn versions_fqn(&self) -> String {
        format!("{}.{}", self.schema, self.versions_table)
    }

    /// Get the fully qualified table name for renditions.
    fn renditions_fqn(&self) -> String {
        format!("{}.{}", self.schema, self.renditions_table)
    }

    /// Get the fully qualified table name for usages.
    fn usages_fqn(&self) -> String {
        format!("{}.{}", self.schema, self.usages_table)
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
        let id = Uuid::now_v7();
        let query = format!(
            r#"
            INSERT INTO {} (id, kind, visibility, title, original_filename, alt_text, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, kind, visibility, title, original_filename, alt_text,
                      current_version_id, deleted_at, created_at, updated_at, created_by
            "#,
            self.config.media_fqn()
        );

        let row: MediaRow = sqlx::query_as(&query)
            .bind(id)
            .bind(input.kind.as_str())
            .bind(input.visibility.as_str())
            .bind(&input.title)
            .bind(&input.original_filename)
            .bind(&input.alt_text)
            .bind(created_by)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.into())
    }

    async fn get_media(&self, id: MediaId) -> MediaResult<Option<Media>> {
        let query = format!(
            r#"
            SELECT id, kind, visibility, title, original_filename, alt_text,
                   current_version_id, deleted_at, created_at, updated_at, created_by
            FROM {}
            WHERE id = $1
            "#,
            self.config.media_fqn()
        );

        let row: Option<MediaRow> = sqlx::query_as(&query)
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Into::into))
    }

    async fn update_media(
        &self,
        id: MediaId,
        input: UpdateMediaInput,
        _updated_by: Option<Uuid>,
    ) -> MediaResult<Media> {
        let query = format!(
            r#"
            UPDATE {}
            SET title = $2, original_filename = $3, visibility = $4, alt_text = $5,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, kind, visibility, title, original_filename, alt_text,
                      current_version_id, deleted_at, created_at, updated_at, created_by
            "#,
            self.config.media_fqn()
        );

        let row: MediaRow = sqlx::query_as(&query)
            .bind(id.0)
            .bind(&input.title)
            .bind(&input.original_filename)
            .bind(input.visibility.as_str())
            .bind(&input.alt_text)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => MediaError::not_found(id),
                other => MediaError::Database(other.to_string()),
            })?;

        Ok(row.into())
    }

    async fn soft_delete_media(&self, id: MediaId, _deleted_by: Option<Uuid>) -> MediaResult<bool> {
        let query = format!(
            r#"
            UPDATE {}
            SET deleted_at = NOW(), updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            self.config.media_fqn()
        );

        let result = sqlx::query(&query).bind(id.0).execute(&self.pool).await?;

        Ok(result.rows_affected() > 0)
    }

    async fn restore_media(&self, id: MediaId) -> MediaResult<bool> {
        let query = format!(
            r#"
            UPDATE {}
            SET deleted_at = NULL, updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NOT NULL
            "#,
            self.config.media_fqn()
        );

        let result = sqlx::query(&query).bind(id.0).execute(&self.pool).await?;

        Ok(result.rows_affected() > 0)
    }

    async fn hard_delete_media(&self, id: MediaId) -> MediaResult<bool> {
        let query = format!(
            r#"
            DELETE FROM {}
            WHERE id = $1
            "#,
            self.config.media_fqn()
        );

        let result = sqlx::query(&query).bind(id.0).execute(&self.pool).await?;

        Ok(result.rows_affected() > 0)
    }

    async fn list_media(&self, params: ListMediaParams) -> MediaResult<Vec<MediaSummary>> {
        // Build dynamic query based on params
        let mut conditions = vec![];
        let mut bind_index = 1;

        if !params.include_deleted {
            conditions.push("m.deleted_at IS NULL".to_string());
        }

        if params.kind.is_some() {
            conditions.push(format!("m.kind = ${}", bind_index));
            bind_index += 1;
        }

        if params.visibility.is_some() {
            conditions.push(format!("m.visibility = ${}", bind_index));
            bind_index += 1;
        }

        if params.search.is_some() {
            conditions.push(format!(
                "(m.title ILIKE ${} OR m.original_filename ILIKE ${})",
                bind_index, bind_index
            ));
            #[allow(unused_assignments)]
            {
                bind_index += 1;
            }
        }

        if params.unused_only {
            conditions.push(format!(
                "NOT EXISTS (SELECT 1 FROM {} u WHERE u.media_id = m.id)",
                self.config.usages_fqn()
            ));
        }

        let where_clause = if conditions.is_empty() {
            "1=1".to_string()
        } else {
            conditions.join(" AND ")
        };

        let limit_clause = params
            .limit
            .map(|l| format!("LIMIT {}", l))
            .unwrap_or_default();

        let query = format!(
            r#"
            SELECT m.id, m.kind, m.visibility, m.title, m.original_filename,
                   m.current_version_id, m.created_at, m.updated_at, m.deleted_at,
                   v.byte_size, v.mime_type,
                   r.object_key as thumbnail_object_key
            FROM {} m
            LEFT JOIN {} v ON v.id = m.current_version_id
            LEFT JOIN {} r ON r.media_version_id = v.id AND r.kind = 'thumbnail'
            WHERE {}
            ORDER BY m.updated_at DESC, m.id DESC
            {}
            "#,
            self.config.media_fqn(),
            self.config.versions_fqn(),
            self.config.renditions_fqn(),
            where_clause,
            limit_clause
        );

        let mut query_builder = sqlx::query_as::<_, MediaSummaryRow>(&query);

        if let Some(kind) = &params.kind {
            query_builder = query_builder.bind(kind.as_str());
        }

        if let Some(visibility) = &params.visibility {
            query_builder = query_builder.bind(visibility.as_str());
        }

        if let Some(search) = &params.search {
            let search_pattern = format!("%{}%", search);
            query_builder = query_builder.bind(search_pattern);
        }

        let rows: Vec<MediaSummaryRow> = query_builder.fetch_all(&self.pool).await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_trash(&self) -> MediaResult<Vec<MediaSummary>> {
        let query = format!(
            r#"
            SELECT m.id, m.kind, m.visibility, m.title, m.original_filename,
                   m.current_version_id, m.created_at, m.updated_at, m.deleted_at,
                   v.byte_size, v.mime_type,
                   r.object_key as thumbnail_object_key
            FROM {} m
            LEFT JOIN {} v ON v.id = m.current_version_id
            LEFT JOIN {} r ON r.media_version_id = v.id AND r.kind = 'thumbnail'
            WHERE m.deleted_at IS NOT NULL
            ORDER BY m.deleted_at DESC, m.id DESC
            "#,
            self.config.media_fqn(),
            self.config.versions_fqn(),
            self.config.renditions_fqn()
        );

        let rows: Vec<MediaSummaryRow> = sqlx::query_as(&query).fetch_all(&self.pool).await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    // ========================================================================
    // Versions
    // ========================================================================

    async fn create_version(
        &self,
        media_id: MediaId,
        created_by: Option<Uuid>,
    ) -> MediaResult<MediaVersion> {
        let id = Uuid::now_v7();
        let query = format!(
            r#"
            INSERT INTO {} (id, media_id, state, created_by)
            VALUES ($1, $2, 'uploading', $3)
            RETURNING id, media_id, state, object_key, mime_type, byte_size,
                      sha256, width, height, storage_provider, bucket, created_by, created_at
            "#,
            self.config.versions_fqn()
        );

        let row: MediaVersionRow = sqlx::query_as(&query)
            .bind(id)
            .bind(media_id.0)
            .bind(created_by)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.into())
    }

    async fn get_version(&self, id: MediaVersionId) -> MediaResult<Option<MediaVersion>> {
        let query = format!(
            r#"
            SELECT id, media_id, state, object_key, mime_type, byte_size,
                   sha256, width, height, storage_provider, bucket, created_by, created_at
            FROM {}
            WHERE id = $1
            "#,
            self.config.versions_fqn()
        );

        let row: Option<MediaVersionRow> = sqlx::query_as(&query)
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Into::into))
    }

    async fn finalize_version(
        &self,
        id: MediaVersionId,
        input: FinalizeUploadInput,
    ) -> MediaResult<MediaVersion> {
        let query = format!(
            r#"
            UPDATE {}
            SET state = 'ready',
                byte_size = $2,
                mime_type = $3,
                sha256 = $4,
                storage_provider = $5,
                bucket = $6,
                object_key = $7,
                width = $8,
                height = $9
            WHERE id = $1
            RETURNING id, media_id, state, object_key, mime_type, byte_size,
                      sha256, width, height, storage_provider, bucket, created_by, created_at
            "#,
            self.config.versions_fqn()
        );

        let row: MediaVersionRow = sqlx::query_as(&query)
            .bind(id.0)
            .bind(input.byte_size)
            .bind(&input.mime_type)
            .bind(&input.sha256_hash)
            .bind(&input.storage_provider)
            .bind(&input.bucket)
            .bind(&input.object_key)
            .bind(input.width)
            .bind(input.height)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => MediaError::version_not_found(id),
                other => MediaError::Database(other.to_string()),
            })?;

        Ok(row.into())
    }

    async fn fail_version(&self, id: MediaVersionId) -> MediaResult<bool> {
        let query = format!(
            r#"
            UPDATE {}
            SET state = 'failed'
            WHERE id = $1 AND state = 'uploading'
            "#,
            self.config.versions_fqn()
        );

        let result = sqlx::query(&query).bind(id.0).execute(&self.pool).await?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete_version(&self, id: MediaVersionId) -> MediaResult<bool> {
        let query = format!(
            r#"
            DELETE FROM {}
            WHERE id = $1
            "#,
            self.config.versions_fqn()
        );

        let result = sqlx::query(&query).bind(id.0).execute(&self.pool).await?;

        Ok(result.rows_affected() > 0)
    }

    async fn list_versions(&self, media_id: MediaId) -> MediaResult<Vec<MediaVersion>> {
        let query = format!(
            r#"
            SELECT id, media_id, state, object_key, mime_type, byte_size,
                   sha256, width, height, storage_provider, bucket, created_by, created_at
            FROM {}
            WHERE media_id = $1
            ORDER BY created_at DESC
            "#,
            self.config.versions_fqn()
        );

        let rows: Vec<MediaVersionRow> = sqlx::query_as(&query)
            .bind(media_id.0)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_by_hash(&self, sha256: &str) -> MediaResult<Option<Media>> {
        let query = format!(
            r#"
            SELECT m.id, m.kind, m.visibility, m.title, m.original_filename, m.alt_text,
                   m.current_version_id, m.deleted_at, m.created_at, m.updated_at, m.created_by
            FROM {} m
            JOIN {} v ON v.media_id = m.id
            WHERE v.sha256 = $1 AND v.state = 'ready' AND m.deleted_at IS NULL
            LIMIT 1
            "#,
            self.config.media_fqn(),
            self.config.versions_fqn()
        );

        let row: Option<MediaRow> = sqlx::query_as(&query)
            .bind(sha256)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Into::into))
    }

    async fn set_current_version(
        &self,
        media_id: MediaId,
        version_id: MediaVersionId,
    ) -> MediaResult<()> {
        let query = format!(
            r#"
            UPDATE {}
            SET current_version_id = $2, updated_at = NOW()
            WHERE id = $1
            "#,
            self.config.media_fqn()
        );

        sqlx::query(&query)
            .bind(media_id.0)
            .bind(version_id.0)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // ========================================================================
    // Renditions
    // ========================================================================

    async fn create_rendition(
        &self,
        version_id: MediaVersionId,
        input: CreateRenditionInput,
    ) -> MediaResult<MediaRendition> {
        let id = Uuid::now_v7();
        let query = format!(
            r#"
            INSERT INTO {} (id, media_version_id, kind, object_key, mime_type, byte_size,
                           width, height, storage_provider, bucket)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (media_version_id, kind) DO UPDATE
            SET object_key = EXCLUDED.object_key,
                mime_type = EXCLUDED.mime_type,
                byte_size = EXCLUDED.byte_size,
                width = EXCLUDED.width,
                height = EXCLUDED.height
            RETURNING id, media_version_id, kind, object_key, mime_type, byte_size,
                      width, height, storage_provider, bucket, created_at
            "#,
            self.config.renditions_fqn()
        );

        let row: MediaRenditionRow = sqlx::query_as(&query)
            .bind(id)
            .bind(version_id.0)
            .bind(input.rendition_type.as_str())
            .bind(&input.object_key)
            .bind(&input.mime_type)
            .bind(input.byte_size)
            .bind(input.width)
            .bind(input.height)
            .bind(&input.storage_provider)
            .bind(&input.bucket)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.into())
    }

    async fn get_rendition(&self, id: MediaRenditionId) -> MediaResult<Option<MediaRendition>> {
        let query = format!(
            r#"
            SELECT id, media_version_id, kind, object_key, mime_type, byte_size,
                   width, height, storage_provider, bucket, created_at
            FROM {}
            WHERE id = $1
            "#,
            self.config.renditions_fqn()
        );

        let row: Option<MediaRenditionRow> = sqlx::query_as(&query)
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Into::into))
    }

    async fn list_renditions(
        &self,
        version_id: MediaVersionId,
    ) -> MediaResult<Vec<MediaRendition>> {
        let query = format!(
            r#"
            SELECT id, media_version_id, kind, object_key, mime_type, byte_size,
                   width, height, storage_provider, bucket, created_at
            FROM {}
            WHERE media_version_id = $1
            ORDER BY kind
            "#,
            self.config.renditions_fqn()
        );

        let rows: Vec<MediaRenditionRow> = sqlx::query_as(&query)
            .bind(version_id.0)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn delete_renditions(&self, version_id: MediaVersionId) -> MediaResult<u64> {
        let query = format!(
            r#"
            DELETE FROM {}
            WHERE media_version_id = $1
            "#,
            self.config.renditions_fqn()
        );

        let result = sqlx::query(&query)
            .bind(version_id.0)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    // ========================================================================
    // Usage tracking
    // ========================================================================

    async fn track_usage(&self, usage: &MediaUsage) -> MediaResult<()> {
        let query = format!(
            r#"
            INSERT INTO {} (id, media_id, used_by_type, used_by_id, field)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (media_id, used_by_type, used_by_id, field) DO NOTHING
            "#,
            self.config.usages_fqn()
        );

        sqlx::query(&query)
            .bind(usage.id)
            .bind(usage.media_id.0)
            .bind(&usage.entity_type)
            .bind(usage.entity_id)
            .bind(&usage.field_name)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn remove_usage(
        &self,
        media_id: MediaId,
        entity_type: &str,
        entity_id: Uuid,
        field_name: &str,
    ) -> MediaResult<bool> {
        let query = format!(
            r#"
            DELETE FROM {}
            WHERE media_id = $1 AND used_by_type = $2 AND used_by_id = $3 AND field = $4
            "#,
            self.config.usages_fqn()
        );

        let result = sqlx::query(&query)
            .bind(media_id.0)
            .bind(entity_type)
            .bind(entity_id)
            .bind(field_name)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn list_usages(&self, media_id: MediaId) -> MediaResult<Vec<MediaUsage>> {
        let query = format!(
            r#"
            SELECT id, media_id, used_by_type, used_by_id, field, created_at
            FROM {}
            WHERE media_id = $1
            ORDER BY created_at DESC
            "#,
            self.config.usages_fqn()
        );

        let rows: Vec<MediaUsageRow> = sqlx::query_as(&query)
            .bind(media_id.0)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn is_media_used(&self, media_id: MediaId) -> MediaResult<bool> {
        let query = format!(
            r#"
            SELECT EXISTS(SELECT 1 FROM {} WHERE media_id = $1) as used
            "#,
            self.config.usages_fqn()
        );

        let row = sqlx::query(&query)
            .bind(media_id.0)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.get::<bool, _>("used"))
    }

    async fn get_usage_count(&self, media_id: MediaId) -> MediaResult<i64> {
        let query = format!(
            r#"
            SELECT COUNT(*) as count FROM {} WHERE media_id = $1
            "#,
            self.config.usages_fqn()
        );

        let row = sqlx::query(&query)
            .bind(media_id.0)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.get::<i64, _>("count"))
    }

    async fn sync_usages(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        field_name: &str,
        media_ids: &[MediaId],
    ) -> MediaResult<()> {
        use std::collections::HashSet;

        // Get current usages for this entity+field
        let query = format!(
            r#"
            SELECT id, media_id, used_by_type, used_by_id, field, created_at
            FROM {}
            WHERE used_by_type = $1 AND used_by_id = $2 AND field = $3
            "#,
            self.config.usages_fqn()
        );

        let current_usages: Vec<MediaUsageRow> = sqlx::query_as(&query)
            .bind(entity_type)
            .bind(entity_id)
            .bind(field_name)
            .fetch_all(&self.pool)
            .await?;

        // Build sets for comparison
        let current_ids: HashSet<Uuid> = current_usages.iter().map(|u| u.media_id).collect();
        let new_ids: HashSet<Uuid> = media_ids.iter().map(|id| id.0).collect();

        // Add usages for new IDs
        for media_uuid in new_ids.difference(&current_ids) {
            let usage = MediaUsage {
                id: Uuid::now_v7(),
                media_id: MediaId(*media_uuid),
                entity_type: entity_type.to_string(),
                entity_id,
                field_name: field_name.to_string(),
                created_at: Utc::now(),
            };
            // Ignore errors for individual adds (media might not exist)
            let _ = self.track_usage(&usage).await;
        }

        // Remove usages for IDs no longer present
        for media_uuid in current_ids.difference(&new_ids) {
            let _ = self
                .remove_usage(MediaId(*media_uuid), entity_type, entity_id, field_name)
                .await;
        }

        Ok(())
    }

    async fn batch_soft_delete_media(
        &self,
        ids: &[MediaId],
        _deleted_by: Option<Uuid>,
    ) -> MediaResult<i64> {
        if ids.is_empty() {
            return Ok(0);
        }

        let raw_ids: Vec<Uuid> = ids.iter().map(|id| id.0).collect();

        let query = format!(
            r#"
            UPDATE {}
            SET deleted_at = NOW(), updated_at = NOW()
            WHERE id = ANY($1) AND deleted_at IS NULL
            "#,
            self.config.media_fqn()
        );

        let result = sqlx::query(&query)
            .bind(&raw_ids)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() as i64)
    }

    async fn list_unused_media(&self) -> MediaResult<Vec<Media>> {
        let query = format!(
            r#"
            SELECT m.id, m.kind, m.visibility, m.title, m.original_filename, m.alt_text,
                   m.current_version_id, m.deleted_at, m.created_at, m.updated_at, m.created_by
            FROM {} m
            WHERE m.deleted_at IS NULL
              AND NOT EXISTS (SELECT 1 FROM {} u WHERE u.media_id = m.id)
            ORDER BY m.created_at DESC
            "#,
            self.config.media_fqn(),
            self.config.usages_fqn()
        );

        let rows: Vec<MediaRow> = sqlx::query_as(&query).fetch_all(&self.pool).await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }
}
