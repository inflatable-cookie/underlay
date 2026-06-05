use uuid::Uuid;

use crate::postgres_rows::MediaRenditionRow;
use crate::SqlxMediaResultExt;
use underlay_media::{
    CreateRenditionInput, MediaRendition, MediaRenditionId, MediaResult, MediaVersionId,
};

use super::PostgresMediaRepository;

impl PostgresMediaRepository {
    pub(super) async fn upsert_rendition(
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
            self.config.renditions_fqn()?
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
            .await
            .media_result()?;

        Ok(row.into())
    }

    pub(super) async fn fetch_rendition(
        &self,
        id: MediaRenditionId,
    ) -> MediaResult<Option<MediaRendition>> {
        let query = format!(
            r#"
            SELECT id, media_version_id, kind, object_key, mime_type, byte_size,
                   width, height, storage_provider, bucket, created_at
            FROM {}
            WHERE id = $1
            "#,
            self.config.renditions_fqn()?
        );

        let row: Option<MediaRenditionRow> = sqlx::query_as(&query)
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await
            .media_result()?;

        Ok(row.map(Into::into))
    }

    pub(super) async fn fetch_renditions(
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
            self.config.renditions_fqn()?
        );

        let rows: Vec<MediaRenditionRow> = sqlx::query_as(&query)
            .bind(version_id.0)
            .fetch_all(&self.pool)
            .await
            .media_result()?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub(super) async fn delete_rendition_rows(
        &self,
        version_id: MediaVersionId,
    ) -> MediaResult<u64> {
        let query = format!(
            r#"
            DELETE FROM {}
            WHERE media_version_id = $1
            "#,
            self.config.renditions_fqn()?
        );

        let result = sqlx::query(&query)
            .bind(version_id.0)
            .execute(&self.pool)
            .await
            .media_result()?;

        Ok(result.rows_affected())
    }
}
