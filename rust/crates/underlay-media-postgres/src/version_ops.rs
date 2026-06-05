use uuid::Uuid;

use crate::postgres_rows::{MediaRow, MediaVersionRow};
use crate::SqlxMediaResultExt;
use underlay_media::{
    FinalizeUploadInput, Media, MediaError, MediaId, MediaResult, MediaVersion, MediaVersionId,
};

use super::PostgresMediaRepository;

impl PostgresMediaRepository {
    pub(super) async fn insert_version(
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
            self.config.versions_fqn()?
        );

        let row: MediaVersionRow = sqlx::query_as(&query)
            .bind(id)
            .bind(media_id.0)
            .bind(created_by)
            .fetch_one(&self.pool)
            .await
            .media_result()?;

        row.try_into()
    }

    pub(super) async fn fetch_version(
        &self,
        id: MediaVersionId,
    ) -> MediaResult<Option<MediaVersion>> {
        let query = format!(
            r#"
            SELECT id, media_id, state, object_key, mime_type, byte_size,
                   sha256, width, height, storage_provider, bucket, created_by, created_at
            FROM {}
            WHERE id = $1
            "#,
            self.config.versions_fqn()?
        );

        let row: Option<MediaVersionRow> = sqlx::query_as(&query)
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await
            .media_result()?;

        row.map(TryInto::try_into).transpose()
    }

    pub(super) async fn mark_version_ready(
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
            self.config.versions_fqn()?
        );

        let row: MediaVersionRow = sqlx::query_as(&query)
            .bind(id.0)
            .bind(input.byte_size)
            .bind(&input.mime_type)
            .bind(&input.sha256_hash)
            .bind(&input.storage_provider)
            .bind(&input.bucket)
            .bind(input.object_key.as_str())
            .bind(input.width)
            .bind(input.height)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => MediaError::version_not_found(id),
                other => MediaError::Database(other.to_string()),
            })?;

        row.try_into()
    }

    pub(super) async fn mark_version_failed(&self, id: MediaVersionId) -> MediaResult<bool> {
        let query = format!(
            r#"
            UPDATE {}
            SET state = 'failed'
            WHERE id = $1 AND state = 'uploading'
            "#,
            self.config.versions_fqn()?
        );

        let result = sqlx::query(&query)
            .bind(id.0)
            .execute(&self.pool)
            .await
            .media_result()?;

        Ok(result.rows_affected() > 0)
    }

    pub(super) async fn delete_version_row(&self, id: MediaVersionId) -> MediaResult<bool> {
        let query = format!(
            r#"
            DELETE FROM {}
            WHERE id = $1
            "#,
            self.config.versions_fqn()?
        );

        let result = sqlx::query(&query)
            .bind(id.0)
            .execute(&self.pool)
            .await
            .media_result()?;

        Ok(result.rows_affected() > 0)
    }

    pub(super) async fn fetch_versions(&self, media_id: MediaId) -> MediaResult<Vec<MediaVersion>> {
        let query = format!(
            r#"
            SELECT id, media_id, state, object_key, mime_type, byte_size,
                   sha256, width, height, storage_provider, bucket, created_by, created_at
            FROM {}
            WHERE media_id = $1
            ORDER BY created_at DESC
            "#,
            self.config.versions_fqn()?
        );

        let rows: Vec<MediaVersionRow> = sqlx::query_as(&query)
            .bind(media_id.0)
            .fetch_all(&self.pool)
            .await
            .media_result()?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub(super) async fn fetch_media_by_hash(&self, sha256: &str) -> MediaResult<Option<Media>> {
        let query = format!(
            r#"
            SELECT m.id, m.kind, m.visibility, m.title, m.original_filename, m.alt_text,
                   m.current_version_id, m.deleted_at, m.created_at, m.updated_at, m.created_by
            FROM {} m
            JOIN {} v ON v.media_id = m.id
            WHERE v.sha256 = $1 AND v.state = 'ready' AND m.deleted_at IS NULL
            LIMIT 1
            "#,
            self.config.media_fqn()?,
            self.config.versions_fqn()?
        );

        let row: Option<MediaRow> = sqlx::query_as(&query)
            .bind(sha256)
            .fetch_optional(&self.pool)
            .await
            .media_result()?;

        Ok(row.map(Into::into))
    }

    pub(super) async fn apply_current_version(
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
            self.config.media_fqn()?
        );

        sqlx::query(&query)
            .bind(media_id.0)
            .bind(version_id.0)
            .execute(&self.pool)
            .await
            .media_result()?;

        Ok(())
    }
}
