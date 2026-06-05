use std::collections::HashSet;

use chrono::Utc;
use sqlx::Row;
use uuid::Uuid;

use crate::postgres_rows::MediaUsageRow;
use crate::SqlxMediaResultExt;
use underlay_media::{MediaId, MediaResult, MediaUsage};

use super::PostgresMediaRepository;

impl PostgresMediaRepository {
    pub(super) async fn insert_usage(&self, usage: &MediaUsage) -> MediaResult<()> {
        let query = format!(
            r#"
            INSERT INTO {} (id, media_id, used_by_type, used_by_id, field)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (media_id, used_by_type, used_by_id, field) DO NOTHING
            "#,
            self.config.usages_fqn()?
        );

        sqlx::query(&query)
            .bind(usage.id)
            .bind(usage.media_id.0)
            .bind(&usage.entity_type)
            .bind(usage.entity_id)
            .bind(&usage.field_name)
            .execute(&self.pool)
            .await
            .media_result()?;

        Ok(())
    }

    pub(super) async fn delete_usage(
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
            self.config.usages_fqn()?
        );

        let result = sqlx::query(&query)
            .bind(media_id.0)
            .bind(entity_type)
            .bind(entity_id)
            .bind(field_name)
            .execute(&self.pool)
            .await
            .media_result()?;

        Ok(result.rows_affected() > 0)
    }

    pub(super) async fn fetch_usages(&self, media_id: MediaId) -> MediaResult<Vec<MediaUsage>> {
        let query = format!(
            r#"
            SELECT id, media_id, used_by_type, used_by_id, field, created_at
            FROM {}
            WHERE media_id = $1
            ORDER BY created_at DESC
            "#,
            self.config.usages_fqn()?
        );

        let rows: Vec<MediaUsageRow> = sqlx::query_as(&query)
            .bind(media_id.0)
            .fetch_all(&self.pool)
            .await
            .media_result()?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub(super) async fn has_media_usage(&self, media_id: MediaId) -> MediaResult<bool> {
        let query = format!(
            r#"
            SELECT EXISTS(SELECT 1 FROM {} WHERE media_id = $1) as used
            "#,
            self.config.usages_fqn()?
        );

        let row = sqlx::query(&query)
            .bind(media_id.0)
            .fetch_one(&self.pool)
            .await
            .media_result()?;

        Ok(row.get::<bool, _>("used"))
    }

    pub(super) async fn count_media_usage(&self, media_id: MediaId) -> MediaResult<i64> {
        let query = format!(
            r#"
            SELECT COUNT(*) as count FROM {} WHERE media_id = $1
            "#,
            self.config.usages_fqn()?
        );

        let row = sqlx::query(&query)
            .bind(media_id.0)
            .fetch_one(&self.pool)
            .await
            .media_result()?;

        Ok(row.get::<i64, _>("count"))
    }

    pub(super) async fn replace_usages(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        field_name: &str,
        media_ids: &[MediaId],
    ) -> MediaResult<()> {
        let current_usages = self
            .fetch_usages_for_entity_field(entity_type, entity_id, field_name)
            .await?;

        let current_ids: HashSet<Uuid> = current_usages.iter().map(|u| u.media_id).collect();
        let new_ids: HashSet<Uuid> = media_ids.iter().map(|id| id.0).collect();

        for media_uuid in new_ids.difference(&current_ids) {
            let usage = MediaUsage {
                id: Uuid::now_v7(),
                media_id: MediaId(*media_uuid),
                entity_type: entity_type.to_string(),
                entity_id,
                field_name: field_name.to_string(),
                created_at: Utc::now(),
            };
            let _ = self.insert_usage(&usage).await;
        }

        for media_uuid in current_ids.difference(&new_ids) {
            let _ = self
                .delete_usage(MediaId(*media_uuid), entity_type, entity_id, field_name)
                .await;
        }

        Ok(())
    }

    async fn fetch_usages_for_entity_field(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        field_name: &str,
    ) -> MediaResult<Vec<MediaUsageRow>> {
        let query = format!(
            r#"
            SELECT id, media_id, used_by_type, used_by_id, field, created_at
            FROM {}
            WHERE used_by_type = $1 AND used_by_id = $2 AND field = $3
            "#,
            self.config.usages_fqn()?
        );

        Ok(sqlx::query_as(&query)
            .bind(entity_type)
            .bind(entity_id)
            .bind(field_name)
            .fetch_all(&self.pool)
            .await
            .media_result()?)
    }
}
