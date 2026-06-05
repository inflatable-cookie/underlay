use uuid::Uuid;

use crate::postgres_rows::{MediaRow, MediaSummaryRow};
use crate::SqlxMediaResultExt;
use underlay_media::{
    CreateMediaInput, ListMediaParams, Media, MediaId, MediaSummary, UpdateMediaInput,
};
use underlay_media::{MediaError, MediaResult};

use super::{list_query, PostgresMediaRepository};

impl PostgresMediaRepository {
    pub(super) async fn insert_media(
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
            self.config.media_fqn()?
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
            .await
            .media_result()?;

        Ok(row.into())
    }

    pub(super) async fn fetch_media(&self, id: MediaId) -> MediaResult<Option<Media>> {
        let query = format!(
            r#"
            SELECT id, kind, visibility, title, original_filename, alt_text,
                   current_version_id, deleted_at, created_at, updated_at, created_by
            FROM {}
            WHERE id = $1
            "#,
            self.config.media_fqn()?
        );

        let row: Option<MediaRow> = sqlx::query_as(&query)
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await
            .media_result()?;

        Ok(row.map(Into::into))
    }

    pub(super) async fn apply_media_update(
        &self,
        id: MediaId,
        input: UpdateMediaInput,
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
            self.config.media_fqn()?
        );

        let row: MediaRow = sqlx::query_as(&query)
            .bind(id.0)
            .bind(&input.title)
            .bind(&input.original_filename)
            .bind(input.visibility.as_str())
            .bind(&input.alt_text)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => MediaError::not_found(id),
                other => MediaError::Database(other.to_string()),
            })?;

        Ok(row.into())
    }

    pub(super) async fn mark_media_deleted(&self, id: MediaId) -> MediaResult<bool> {
        let query = format!(
            r#"
            UPDATE {}
            SET deleted_at = NOW(), updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            self.config.media_fqn()?
        );

        let result = sqlx::query(&query)
            .bind(id.0)
            .execute(&self.pool)
            .await
            .media_result()?;

        Ok(result.rows_affected() > 0)
    }

    pub(super) async fn restore_deleted_media(&self, id: MediaId) -> MediaResult<bool> {
        let query = format!(
            r#"
            UPDATE {}
            SET deleted_at = NULL, updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NOT NULL
            "#,
            self.config.media_fqn()?
        );

        let result = sqlx::query(&query)
            .bind(id.0)
            .execute(&self.pool)
            .await
            .media_result()?;

        Ok(result.rows_affected() > 0)
    }

    pub(super) async fn delete_media_row(&self, id: MediaId) -> MediaResult<bool> {
        let query = format!(
            r#"
            DELETE FROM {}
            WHERE id = $1
            "#,
            self.config.media_fqn()?
        );

        let result = sqlx::query(&query)
            .bind(id.0)
            .execute(&self.pool)
            .await
            .media_result()?;

        Ok(result.rows_affected() > 0)
    }

    pub(super) async fn fetch_media_list(
        &self,
        params: ListMediaParams,
    ) -> MediaResult<Vec<MediaSummary>> {
        let query = list_query::build_list_media_query(&self.config, &params)?;

        let mut query_builder = sqlx::query_as::<_, MediaSummaryRow>(&query);

        if let Some(kind) = &params.kind {
            query_builder = query_builder.bind(kind.as_str());
        }

        if let Some(visibility) = &params.visibility {
            query_builder = query_builder.bind(visibility.as_str());
        }

        if let Some(search) = &params.search {
            let search_pattern = format!("%{search}%");
            query_builder = query_builder.bind(search_pattern);
        }

        let rows: Vec<MediaSummaryRow> =
            query_builder.fetch_all(&self.pool).await.media_result()?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub(super) async fn fetch_trash(&self) -> MediaResult<Vec<MediaSummary>> {
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
            self.config.media_fqn()?,
            self.config.versions_fqn()?,
            self.config.renditions_fqn()?
        );

        let rows: Vec<MediaSummaryRow> = sqlx::query_as(&query)
            .fetch_all(&self.pool)
            .await
            .media_result()?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    pub(super) async fn batch_mark_media_deleted(&self, ids: &[MediaId]) -> MediaResult<i64> {
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
            self.config.media_fqn()?
        );

        let result = sqlx::query(&query)
            .bind(&raw_ids)
            .execute(&self.pool)
            .await
            .media_result()?;

        Ok(result.rows_affected() as i64)
    }

    pub(super) async fn fetch_unused_media(&self) -> MediaResult<Vec<Media>> {
        let query = format!(
            r#"
            SELECT m.id, m.kind, m.visibility, m.title, m.original_filename, m.alt_text,
                   m.current_version_id, m.deleted_at, m.created_at, m.updated_at, m.created_by
            FROM {} m
            WHERE m.deleted_at IS NULL
              AND NOT EXISTS (SELECT 1 FROM {} u WHERE u.media_id = m.id)
            ORDER BY m.created_at DESC
            "#,
            self.config.media_fqn()?,
            self.config.usages_fqn()?
        );

        let rows: Vec<MediaRow> = sqlx::query_as(&query)
            .fetch_all(&self.pool)
            .await
            .media_result()?;

        Ok(rows.into_iter().map(Into::into).collect())
    }
}
