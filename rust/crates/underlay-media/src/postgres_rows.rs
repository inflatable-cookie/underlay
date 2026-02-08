//! Database row types for the PostgreSQL media repository.
//!
//! These types map database rows to domain types using sqlx's `FromRow` derive.

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::{
    Media, MediaId, MediaKind, MediaRendition, MediaRenditionId, MediaSummary, MediaUsage,
    MediaVersion, MediaVersionId, MediaVersionState, MediaVisibility, RenditionType,
};

#[derive(Debug, FromRow)]
pub(crate) struct MediaRow {
    pub id: Uuid,
    pub kind: String,
    pub visibility: String,
    pub title: String,
    pub original_filename: Option<String>,
    pub alt_text: Option<String>,
    pub current_version_id: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

impl From<MediaRow> for Media {
    fn from(row: MediaRow) -> Self {
        Self {
            id: MediaId(row.id),
            kind: row.kind.parse().unwrap_or(MediaKind::Image),
            visibility: row
                .visibility
                .parse()
                .unwrap_or(MediaVisibility::Restricted),
            title: row.title,
            original_filename: row.original_filename,
            alt_text: row.alt_text,
            current_version_id: row.current_version_id.map(MediaVersionId),
            deleted_at: row.deleted_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
            created_by: row.created_by,
        }
    }
}

#[derive(Debug, FromRow)]
pub(crate) struct MediaSummaryRow {
    pub id: Uuid,
    pub kind: String,
    pub visibility: String,
    pub title: String,
    pub original_filename: Option<String>,
    pub current_version_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub byte_size: Option<i64>,
    pub mime_type: Option<String>,
    pub thumbnail_object_key: Option<String>,
}

impl From<MediaSummaryRow> for MediaSummary {
    fn from(row: MediaSummaryRow) -> Self {
        Self {
            id: MediaId(row.id),
            kind: row.kind.parse().unwrap_or(MediaKind::Image),
            visibility: row
                .visibility
                .parse()
                .unwrap_or(MediaVisibility::Restricted),
            title: row.title,
            original_filename: row.original_filename,
            current_version_id: row.current_version_id.map(MediaVersionId),
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
            byte_size: row.byte_size,
            mime_type: row.mime_type,
            thumbnail_object_key: row.thumbnail_object_key,
        }
    }
}

#[derive(Debug, FromRow)]
pub(crate) struct MediaVersionRow {
    pub id: Uuid,
    pub media_id: Uuid,
    pub state: String,
    pub object_key: Option<String>,
    pub mime_type: Option<String>,
    pub byte_size: Option<i64>,
    pub sha256: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub storage_provider: Option<String>,
    pub bucket: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl From<MediaVersionRow> for MediaVersion {
    fn from(row: MediaVersionRow) -> Self {
        Self {
            id: MediaVersionId(row.id),
            media_id: MediaId(row.media_id),
            state: row.state.parse().unwrap_or(MediaVersionState::Uploading),
            object_key: row.object_key,
            mime_type: row.mime_type,
            byte_size: row.byte_size,
            sha256_hash: row.sha256,
            width: row.width,
            height: row.height,
            storage_provider: row.storage_provider,
            bucket: row.bucket,
            uploaded_by: row.created_by,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, FromRow)]
pub(crate) struct MediaRenditionRow {
    pub id: Uuid,
    pub media_version_id: Uuid,
    pub kind: String,
    pub object_key: String,
    pub mime_type: String,
    pub byte_size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub storage_provider: String,
    pub bucket: String,
    pub created_at: DateTime<Utc>,
}

impl From<MediaRenditionRow> for MediaRendition {
    fn from(row: MediaRenditionRow) -> Self {
        Self {
            id: MediaRenditionId(row.id),
            version_id: MediaVersionId(row.media_version_id),
            rendition_type: RenditionType::from(row.kind),
            object_key: row.object_key,
            mime_type: row.mime_type,
            byte_size: row.byte_size,
            width: row.width,
            height: row.height,
            storage_provider: row.storage_provider,
            bucket: row.bucket,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, FromRow)]
pub(crate) struct MediaUsageRow {
    pub id: Uuid,
    pub media_id: Uuid,
    pub used_by_type: String,
    pub used_by_id: Uuid,
    pub field: String,
    pub created_at: DateTime<Utc>,
}

impl From<MediaUsageRow> for MediaUsage {
    fn from(row: MediaUsageRow) -> Self {
        Self {
            id: row.id,
            media_id: MediaId(row.media_id),
            entity_type: row.used_by_type,
            entity_id: row.used_by_id,
            field_name: row.field,
            created_at: row.created_at,
        }
    }
}
