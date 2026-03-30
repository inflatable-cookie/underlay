# 077 - Media Library

This guide covers implementing a complete media library for managing uploaded files (images, PDFs, etc.) in Underlay-based applications. The media library pattern includes:

- **Backend**: Database schema, repository, and API handlers for media CRUD operations
- **Client**: TypeScript commands and types for calling the API
- **Frontend**: Admin UI with list, detail, upload, and trash views

## Quick Start

Underlay provides shared types and components to reduce boilerplate. For new implementations:

1. **Use shared types** - Import from `underlay-db` (Rust) or `@decodelabs/underlay/patterns` (TypeScript)
2. **Use shared workflow components** - Poodle `MediaPicker` for local item selectors, Poodle `MediaBrowsePanel` / `MediaUploadStatusPanel` plus media-workflow helpers for callback-driven library flows, an app-local media actions wrapper over Poodle `Menu` / `AlertDialog`, and Poodle `MediaThumbnail` for display-only previews
3. **Use the upload flow pattern** - `createMediaUploadFlow` for consistent upload state management

| Layer | Package | Exports |
|-------|---------|---------|
| Rust types | `underlay-db` | `MediaKind`, `MediaVisibility`, `MediaVersionState` |
| TypeScript types | `@decodelabs/underlay/patterns` | All types, enums, and utility functions |
| App-local media actions | local app UI | Compose `Menu`, `AlertDialog`, clipboard helpers, and media commands |
| Media workflow UI/helpers | `@poodle/svelte-composites` | `MediaPicker`, `MediaBrowsePanel`, `MediaUploadStatusPanel`, `loadMediaBrowsePage`, `mergeMediaBrowseItems`, `createResetMediaBrowseState`, `runMediaUploadWorkflow`, `uploadMediaWithKnownHash` |
| Display composites | `@poodle/svelte-composites` | `MediaThumbnail` |
| Upload primitive | `@poodle/svelte-primitives` | `FileUpload` |
| Upload pattern | `@decodelabs/underlay/patterns` | `createMediaUploadFlow` |

See [Shared Underlay Components](#shared-underlay-components) for detailed usage. The sections below cover implementing the backend and custom frontend if needed.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         Media Library Architecture                        │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                           │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────────┐  │
│  │   Admin UI      │    │   API Server    │    │   Blob Storage      │  │
│  │   (SvelteKit)   │◄──►│   (Axum/Rust)   │◄──►│   (S3/Local)        │  │
│  └─────────────────┘    └─────────────────┘    └─────────────────────┘  │
│         │                       │                                        │
│         │                       │                                        │
│         ▼                       ▼                                        │
│  ┌─────────────────┐    ┌─────────────────┐                             │
│  │  TS API Client  │    │   PostgreSQL    │                             │
│  │  (cattle-grid)  │    │   (media tables)│                             │
│  └─────────────────┘    └─────────────────┘                             │
│                                                                           │
└─────────────────────────────────────────────────────────────────────────┘
```

### Upload Flow

The upload process uses **direct-to-blob** uploads with pre-signed URLs:

1. **Client hashes file** → SHA-256 computed client-side for deduplication
2. **Client checks for duplicates** → API checks if hash exists
3. **Client creates media record** → API creates `media` row
4. **Client initiates upload** → API creates `media_version` row, returns pre-signed URL
5. **Client uploads to blob storage** → Direct PUT to S3/local storage
6. **Client finalizes upload** → API marks version as ready, sets as current

```
┌──────────┐       ┌──────────┐       ┌──────────┐       ┌──────────┐
│  Client  │       │   API    │       │ Database │       │   Blob   │
└────┬─────┘       └────┬─────┘       └────┬─────┘       └────┬─────┘
     │                  │                  │                  │
     │ 1. Compute hash  │                  │                  │
     │◄────────────────►│                  │                  │
     │                  │                  │                  │
     │ 2. Check duplicate (sha256)        │                  │
     │─────────────────►│                  │                  │
     │                  │  SELECT          │                  │
     │                  │─────────────────►│                  │
     │  {exists, media} │                  │                  │
     │◄─────────────────│                  │                  │
     │                  │                  │                  │
     │ 3. Create media  │                  │                  │
     │─────────────────►│  INSERT media    │                  │
     │                  │─────────────────►│                  │
     │  {media}         │                  │                  │
     │◄─────────────────│                  │                  │
     │                  │                  │                  │
     │ 4. Initiate upload                 │                  │
     │─────────────────►│ INSERT version   │                  │
     │                  │─────────────────►│                  │
     │                  │ Generate presigned URL             │
     │                  │─────────────────────────────────────►
     │  {versionId, uploadPlan}           │                  │
     │◄─────────────────│                  │                  │
     │                  │                  │                  │
     │ 5. Upload file directly            │                  │
     │───────────────────────────────────────────────────────►│
     │                  │                  │                  │
     │ 6. Finalize upload                 │                  │
     │─────────────────►│ UPDATE version   │                  │
     │                  │─────────────────►│                  │
     │  {media}         │                  │                  │
     │◄─────────────────│                  │                  │
```

## Database Schema

### Tables

The media library uses three tables in the `media` schema:

```sql
-- Media items (images, PDFs, etc.)
CREATE TABLE media.media (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    kind TEXT NOT NULL,                    -- 'image', 'pdf'
    visibility TEXT NOT NULL DEFAULT 'public', -- 'public', 'restricted'
    title TEXT,
    original_filename TEXT,
    current_version_id UUID,               -- FK to media_version
    usage_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ                 -- Soft delete
);

-- Version history for each media item
CREATE TABLE media.media_version (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    media_id UUID NOT NULL REFERENCES media.media(id) ON DELETE CASCADE,
    state TEXT NOT NULL DEFAULT 'uploading', -- 'uploading', 'ready', 'failed', 'purging'
    object_key TEXT,                       -- Storage path
    sha256 TEXT,                           -- Content hash for deduplication
    byte_size BIGINT,
    mime_type TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Track where media is used (for reference counting)
CREATE TABLE media.media_usage (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    media_id UUID NOT NULL REFERENCES media.media(id) ON DELETE CASCADE,
    used_by_type TEXT NOT NULL,            -- 'qa_item', 'document', etc.
    used_by_id UUID NOT NULL,              -- ID of the referencing entity
    field TEXT,                            -- Which field references it
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(media_id, used_by_type, used_by_id, field)
);

-- Indexes
CREATE INDEX idx_media_deleted_at ON media.media(deleted_at);
CREATE INDEX idx_media_kind ON media.media(kind);
CREATE INDEX idx_media_version_media_id ON media.media_version(media_id);
CREATE INDEX idx_media_version_sha256 ON media.media_version(sha256);
CREATE INDEX idx_media_usage_media_id ON media.media_usage(media_id);
CREATE INDEX idx_media_usage_used_by ON media.media_usage(used_by_type, used_by_id);

-- Foreign key for current version (added after both tables exist)
ALTER TABLE media.media
    ADD CONSTRAINT fk_media_current_version
    FOREIGN KEY (current_version_id) REFERENCES media.media_version(id);
```

### Enums

Media enums are provided by `underlay-db` and should be re-exported by consuming apps:

```rust
// Re-export from underlay-db in your domain layer
pub use underlay_db::{MediaKind, MediaVisibility, MediaVersionState};
```

The enums serialize to lowercase strings (`"image"`, `"pdf"`, `"public"`, `"restricted"`, etc.) matching the TypeScript definitions.

**Available enums:**

| Enum | Values | Description |
|------|--------|-------------|
| `MediaKind` | `Image`, `Pdf` | Type of media file |
| `MediaVisibility` | `Public`, `Restricted` | Access level |
| `MediaVersionState` | `Uploading`, `Ready`, `Failed`, `Purging` | Upload lifecycle state |

**Utility methods on each enum:**

```rust
use underlay_db::{MediaKind, MediaVisibility, MediaVersionState};

// String conversion
let kind = MediaKind::Image;
assert_eq!(kind.as_str(), "image");
assert_eq!(kind.label(), "Image");
assert_eq!(kind.to_string(), "image");

// Parsing from string (FromStr trait)
let kind: MediaKind = "image".parse().unwrap();
let visibility: MediaVisibility = "public".parse().unwrap();

// Detect kind from MIME type
use underlay_db::detect_media_kind_from_mime_type;
assert_eq!(detect_media_kind_from_mime_type("image/jpeg"), Some(MediaKind::Image));
assert_eq!(detect_media_kind_from_mime_type("application/pdf"), Some(MediaKind::Pdf));

// Check version state
let state = MediaVersionState::Ready;
assert!(state.is_ready());
assert!(state.is_terminal());
```

## Backend Implementation

### Repository Layer

```rust
// crates/db/src/media.rs

use sqlx::PgPool;
use underlay_db::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct MediaRow {
    pub id: Uuid,
    pub kind: String,
    pub visibility: String,
    pub title: Option<String>,
    pub original_filename: Option<String>,
    pub current_version_id: Option<Uuid>,
    pub usage_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct MediaVersionRow {
    pub id: Uuid,
    pub media_id: Uuid,
    pub state: String,
    pub object_key: Option<String>,
    pub sha256: Option<String>,
    pub byte_size: Option<i64>,
    pub mime_type: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct MediaUsageRow {
    pub id: Uuid,
    pub media_id: Uuid,
    pub used_by_type: String,
    pub used_by_id: Uuid,
    pub field: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// List all media (excluding soft-deleted)
pub async fn list_media(pool: &PgPool) -> Result<Vec<MediaRow>, sqlx::Error> {
    sqlx::query_as::<_, MediaRow>(
        r#"
        SELECT m.*, v.byte_size, v.mime_type
        FROM media.media m
        LEFT JOIN media.media_version v ON v.id = m.current_version_id
        WHERE m.deleted_at IS NULL
        ORDER BY m.created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
}

/// Get media by ID with current version info
pub async fn get_media(pool: &PgPool, id: Uuid) -> Result<Option<MediaRow>, sqlx::Error> {
    sqlx::query_as::<_, MediaRow>(
        "SELECT * FROM media.media WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

/// Create a new media record
pub async fn create_media(
    pool: &PgPool,
    kind: &str,
    visibility: &str,
    title: Option<&str>,
    original_filename: Option<&str>,
) -> Result<MediaRow, sqlx::Error> {
    sqlx::query_as::<_, MediaRow>(
        r#"
        INSERT INTO media.media (kind, visibility, title, original_filename)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#
    )
    .bind(kind)
    .bind(visibility)
    .bind(title)
    .bind(original_filename)
    .fetch_one(pool)
    .await
}

/// Check for duplicate by SHA-256 hash
pub async fn find_by_sha256(pool: &PgPool, sha256: &str) -> Result<Option<MediaRow>, sqlx::Error> {
    sqlx::query_as::<_, MediaRow>(
        r#"
        SELECT m.*
        FROM media.media m
        JOIN media.media_version v ON v.media_id = m.id
        WHERE v.sha256 = $1 AND v.state = 'ready' AND m.deleted_at IS NULL
        LIMIT 1
        "#
    )
    .bind(sha256)
    .fetch_optional(pool)
    .await
}

/// Create a new version record
pub async fn create_version(
    pool: &PgPool,
    media_id: Uuid,
    object_key: &str,
    sha256: &str,
    byte_size: i64,
    mime_type: &str,
) -> Result<MediaVersionRow, sqlx::Error> {
    sqlx::query_as::<_, MediaVersionRow>(
        r#"
        INSERT INTO media.media_version (media_id, object_key, sha256, byte_size, mime_type, state)
        VALUES ($1, $2, $3, $4, $5, 'uploading')
        RETURNING *
        "#
    )
    .bind(media_id)
    .bind(object_key)
    .bind(sha256)
    .bind(byte_size)
    .bind(mime_type)
    .fetch_one(pool)
    .await
}

/// Finalize upload - mark version as ready and set as current
pub async fn finalize_version(
    pool: &PgPool,
    media_id: Uuid,
    version_id: Uuid,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    // Mark version as ready
    sqlx::query(
        "UPDATE media.media_version SET state = 'ready' WHERE id = $1"
    )
    .bind(version_id)
    .execute(&mut *tx)
    .await?;

    // Set as current version
    sqlx::query(
        "UPDATE media.media SET current_version_id = $1, updated_at = NOW() WHERE id = $2"
    )
    .bind(version_id)
    .bind(media_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await
}

/// Soft delete media
pub async fn soft_delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE media.media SET deleted_at = NOW(), updated_at = NOW() WHERE id = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Restore soft-deleted media
pub async fn restore(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE media.media SET deleted_at = NULL, updated_at = NOW() WHERE id = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

/// List soft-deleted media (trash)
pub async fn list_trash(pool: &PgPool) -> Result<Vec<MediaRow>, sqlx::Error> {
    sqlx::query_as::<_, MediaRow>(
        r#"
        SELECT * FROM media.media
        WHERE deleted_at IS NOT NULL
        ORDER BY deleted_at DESC
        "#
    )
    .fetch_all(pool)
    .await
}
```

### API Handlers

```rust
// crates/api/src/routes/admin/media.rs

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use underlay_db::Uuid;

use crate::{AppState, Result};

// DTOs
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSummary {
    pub id: Uuid,
    pub kind: String,
    pub visibility: String,
    pub title: Option<String>,
    pub original_filename: Option<String>,
    pub byte_size: Option<i64>,
    pub usage_count: i32,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaDetail {
    pub id: Uuid,
    pub kind: String,
    pub visibility: String,
    pub title: Option<String>,
    pub original_filename: Option<String>,
    pub current_version_id: Option<Uuid>,
    pub current_version: Option<MediaVersion>,
    pub usage_count: i32,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaVersion {
    pub id: Uuid,
    pub state: String,
    pub object_key: Option<String>,
    pub sha256: Option<String>,
    pub byte_size: Option<i64>,
    pub mime_type: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMediaRequest {
    pub kind: String,
    pub visibility: String,
    pub title: Option<String>,
    pub original_filename: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiateUploadRequest {
    pub content_type: String,
    pub content_length: i64,
    pub sha256: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiateUploadResponse {
    pub version_id: Uuid,
    pub upload_plan: UploadPlan,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadPlan {
    pub upload_url: String,
    pub method: String,
    pub headers: std::collections::HashMap<String, String>,
    pub expires_at: String,
    pub max_bytes: i64,
    pub allowed_content_types: Vec<String>,
}

// Handlers
pub async fn list_media(State(state): State<AppState>) -> Result<Json<Vec<MediaSummary>>> {
    let media = db::media::list_media(&state.pool).await?;
    let summaries = media.into_iter().map(|m| MediaSummary {
        id: m.id,
        kind: m.kind,
        visibility: m.visibility,
        title: m.title,
        original_filename: m.original_filename,
        byte_size: m.byte_size,
        usage_count: m.usage_count,
        created_at: m.created_at.to_rfc3339(),
        deleted_at: m.deleted_at.map(|d| d.to_rfc3339()),
    }).collect();
    Ok(Json(summaries))
}

pub async fn get_media(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MediaDetail>> {
    let media = db::media::get_media(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Media not found"))?;

    let current_version = if let Some(version_id) = media.current_version_id {
        db::media::get_version(&state.pool, version_id).await?.map(|v| MediaVersion {
            id: v.id,
            state: v.state,
            object_key: v.object_key,
            sha256: v.sha256,
            byte_size: v.byte_size,
            mime_type: v.mime_type,
            created_at: v.created_at.to_rfc3339(),
        })
    } else {
        None
    };

    Ok(Json(MediaDetail {
        id: media.id,
        kind: media.kind,
        visibility: media.visibility,
        title: media.title,
        original_filename: media.original_filename,
        current_version_id: media.current_version_id,
        current_version,
        usage_count: media.usage_count,
        created_at: media.created_at.to_rfc3339(),
        updated_at: media.updated_at.to_rfc3339(),
        deleted_at: media.deleted_at.map(|d| d.to_rfc3339()),
    }))
}

pub async fn create_media(
    State(state): State<AppState>,
    Json(req): Json<CreateMediaRequest>,
) -> Result<Json<MediaDetail>> {
    let media = db::media::create_media(
        &state.pool,
        &req.kind,
        &req.visibility,
        req.title.as_deref(),
        req.original_filename.as_deref(),
    ).await?;

    // Return as MediaDetail
    // ...
}

pub async fn initiate_upload(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<InitiateUploadRequest>,
) -> Result<Json<InitiateUploadResponse>> {
    // Generate object key
    let object_key = format!("media/{}/{}", id, uuid::Uuid::new_v4());

    // Create version record
    let version = db::media::create_version(
        &state.pool,
        id,
        &object_key,
        &req.sha256,
        req.content_length,
        &req.content_type,
    ).await?;

    // Generate pre-signed upload URL
    let upload_plan = state.blob_adapter.generate_upload_plan(
        &object_key,
        &req.content_type,
        req.content_length,
    ).await?;

    Ok(Json(InitiateUploadResponse {
        version_id: version.id,
        upload_plan,
    }))
}

pub async fn finalize_upload(
    State(state): State<AppState>,
    Path((media_id, version_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<MediaDetail>> {
    db::media::finalize_version(&state.pool, media_id, version_id).await?;
    get_media(State(state), Path(media_id)).await
}
```

### Router Setup

```rust
// crates/api/src/routes/admin/router.rs

use axum::{
    routing::{get, post, put, delete},
    Router,
};

pub fn admin_media_routes() -> Router<AppState> {
    Router::new()
        .route("/media", get(media::list_media).post(media::create_media))
        .route("/media/trash", get(media::list_trash))
        .route("/media/check-duplicate", post(media::check_duplicate))
        .route("/media/:id", get(media::get_media).patch(media::update_media))
        .route("/media/:id/soft-delete", post(media::soft_delete))
        .route("/media/:id/restore", post(media::restore))
        .route("/media/:id/purge", delete(media::purge))
        .route("/media/:id/upload", post(media::initiate_upload))
        .route("/media/:id/upload/:versionId/finalize", post(media::finalize_upload))
        .route("/media/:id/versions", get(media::list_versions))
        .route("/media/:id/usages", get(media::list_usages))
}
```

## TypeScript Client

### Types

```typescript
// src/types/media-types.ts

export interface MediaSummary {
  id: string;
  kind: string;
  visibility: string;
  title: string | null;
  originalFilename: string | null;
  byteSize: number | null;
  usageCount: number;
  createdAt: string;
  deletedAt: string | null;
}

export interface MediaDetail {
  id: string;
  kind: string;
  visibility: string;
  title: string | null;
  originalFilename: string | null;
  currentVersionId: string | null;
  currentVersion: MediaVersion | null;
  usageCount: number;
  createdAt: string;
  updatedAt: string;
  deletedAt: string | null;
}

export interface MediaVersion {
  id: string;
  state: string;
  objectKey: string | null;
  sha256: string | null;
  byteSize: number | null;
  mimeType: string | null;
  createdAt: string;
}

export interface MediaUsage {
  id: string;
  mediaId: string;
  usedByType: string;
  usedById: string;
  field: string | null;
  createdAt: string;
}

export interface CreateMediaRequest {
  kind: string;
  visibility: string;
  title?: string | null;
  originalFilename?: string | null;
}

export interface InitiateUploadRequest {
  contentType: string;
  contentLength: number;
  sha256: string;
}

export interface InitiateUploadResponse {
  versionId: string;
  uploadPlan: UploadPlan;
}

export interface UploadPlan {
  uploadUrl: string;
  method: string;
  headers: Record<string, string>;
  expiresAt: string;
  maxBytes: number;
  allowedContentTypes: string[];
}

export interface DuplicateCheckResult {
  exists: boolean;
  media: MediaSummary | null;
}

// Enums
export const MediaKind = {
  Image: "image",
  Pdf: "pdf",
} as const;

export const MediaVisibility = {
  Public: "public",
  Restricted: "restricted",
} as const;

export const MediaVersionState = {
  Uploading: "uploading",
  Ready: "ready",
  Failed: "failed",
  Purging: "purging",
} as const;
```

### Commands

```typescript
// src/commands/media-commands.ts

import type {
  MediaSummary,
  MediaDetail,
  MediaVersion,
  MediaUsage,
  CreateMediaRequest,
  InitiateUploadRequest,
  InitiateUploadResponse,
  DuplicateCheckResult,
} from "../types/media-types";

const BASE = "/admin/media";

export const mediaCommands = {
  // List all media
  async listMedia(
    fetchFn: typeof fetch,
    token: string
  ): Promise<MediaSummary[]> {
    const res = await fetchFn(`${BASE}`, {
      headers: { Authorization: `Bearer ${token}` },
    });
    if (!res.ok) throw new Error("Failed to list media");
    return res.json();
  },

  // Get media by ID
  async getMedia(
    id: string,
    fetchFn: typeof fetch,
    token: string
  ): Promise<MediaDetail> {
    const res = await fetchFn(`${BASE}/${id}`, {
      headers: { Authorization: `Bearer ${token}` },
    });
    if (!res.ok) throw new Error("Failed to get media");
    return res.json();
  },

  // Create new media
  async createMedia(
    data: CreateMediaRequest,
    fetchFn: typeof fetch,
    token: string
  ): Promise<MediaDetail> {
    const res = await fetchFn(`${BASE}`, {
      method: "POST",
      headers: {
        Authorization: `Bearer ${token}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
    if (!res.ok) throw new Error("Failed to create media");
    return res.json();
  },

  // Check for duplicate by hash
  async checkDuplicate(
    data: { sha256: string },
    fetchFn: typeof fetch,
    token: string
  ): Promise<DuplicateCheckResult> {
    const res = await fetchFn(`${BASE}/check-duplicate`, {
      method: "POST",
      headers: {
        Authorization: `Bearer ${token}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
    if (!res.ok) throw new Error("Failed to check duplicate");
    return res.json();
  },

  // Initiate upload
  async initiateUpload(
    mediaId: string,
    data: InitiateUploadRequest,
    fetchFn: typeof fetch,
    token: string
  ): Promise<InitiateUploadResponse> {
    const res = await fetchFn(`${BASE}/${mediaId}/upload`, {
      method: "POST",
      headers: {
        Authorization: `Bearer ${token}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
    if (!res.ok) throw new Error("Failed to initiate upload");
    return res.json();
  },

  // Finalize upload
  async finaliseUpload(
    mediaId: string,
    versionId: string,
    data: { sha256: string },
    fetchFn: typeof fetch,
    token: string
  ): Promise<MediaDetail> {
    const res = await fetchFn(`${BASE}/${mediaId}/upload/${versionId}/finalize`, {
      method: "POST",
      headers: {
        Authorization: `Bearer ${token}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
    if (!res.ok) throw new Error("Failed to finalize upload");
    return res.json();
  },

  // List versions
  async listVersions(
    mediaId: string,
    fetchFn: typeof fetch,
    token: string
  ): Promise<MediaVersion[]> {
    const res = await fetchFn(`${BASE}/${mediaId}/versions`, {
      headers: { Authorization: `Bearer ${token}` },
    });
    if (!res.ok) throw new Error("Failed to list versions");
    return res.json();
  },

  // List usages
  async listUsages(
    mediaId: string,
    fetchFn: typeof fetch,
    token: string
  ): Promise<MediaUsage[]> {
    const res = await fetchFn(`${BASE}/${mediaId}/usages`, {
      headers: { Authorization: `Bearer ${token}` },
    });
    if (!res.ok) throw new Error("Failed to list usages");
    return res.json();
  },

  // Soft delete
  async softDeleteMedia(
    id: string,
    fetchFn: typeof fetch,
    token: string
  ): Promise<void> {
    const res = await fetchFn(`${BASE}/${id}/soft-delete`, {
      method: "POST",
      headers: { Authorization: `Bearer ${token}` },
    });
    if (!res.ok) throw new Error("Failed to soft delete media");
  },

  // Restore from trash
  async restoreMedia(
    id: string,
    fetchFn: typeof fetch,
    token: string
  ): Promise<void> {
    const res = await fetchFn(`${BASE}/${id}/restore`, {
      method: "POST",
      headers: { Authorization: `Bearer ${token}` },
    });
    if (!res.ok) throw new Error("Failed to restore media");
  },

  // Permanent delete
  async purgeMedia(
    id: string,
    fetchFn: typeof fetch,
    token: string
  ): Promise<void> {
    const res = await fetchFn(`${BASE}/${id}/purge`, {
      method: "DELETE",
      headers: { Authorization: `Bearer ${token}` },
    });
    if (!res.ok) throw new Error("Failed to purge media");
  },

  // List trash
  async listMediaTrash(
    fetchFn: typeof fetch,
    token: string
  ): Promise<MediaSummary[]> {
    const res = await fetchFn(`${BASE}/trash`, {
      headers: { Authorization: `Bearer ${token}` },
    });
    if (!res.ok) throw new Error("Failed to list trash");
    return res.json();
  },
};
```

## Frontend Implementation

### Navigation

Add media section to admin navigation:

```svelte
<!-- src/lib/ui/AdminNavList.svelte -->
<NavGroup label="Media">
  <NavLink href="/media" icon={Image}>Library</NavLink>
  <NavLink href="/media/upload" icon={Upload}>Upload</NavLink>
  <NavLink href="/media/trash" icon={Trash2}>Trash</NavLink>
</NavGroup>
```

### List Page

```svelte
<!-- src/routes/(app)/media/+page.svelte -->
<script lang="ts">
  import { useAuthenticatedData } from "@decodelabs/underlay/runtime";
  import { PageHeader as PoodlePageHeader, PageLoading } from "@poodle/svelte-composites";
  import { Grid, ListCard, Pill } from "@poodle/svelte-primitives";
  import { mediaCommands, type MediaSummary, MediaKind } from "@my-client";
  import { auth, authLoading, currentUser } from "$lib/stores/auth";
  import Image from "lucide-svelte/icons/image";
  import FileText from "lucide-svelte/icons/file-text";

  const pageData = useAuthenticatedData<MediaSummary[]>(
    async (fetchFn, token) => mediaCommands.listMedia(fetchFn, token),
    { getToken: () => auth.getToken() }
  );

  $effect(() => {
    pageData.tryFetch($authLoading, $currentUser);
  });

  function getMediaIcon(kind: string) {
    return kind === MediaKind.Image ? Image : FileText;
  }

  function getKindAccent(kind: string): string {
    return kind === MediaKind.Image ? "#22c55e" : "#ef4444";
  }
</script>

<PoodlePageHeader title="Media Library">
  <a href="/media/upload" class="button-primary">Upload Media</a>
</PoodlePageHeader>

{#if pageData.loading}
  <PageLoading presentation="inline" message="Loading media..." />
{:else if pageData.data}
  <Grid columns="repeat(auto-fit, minmax(min(22.5rem, 100%), 1fr))" gap="lg">
    {#each pageData.data as item}
      {@const Icon = getMediaIcon(item.kind)}
      <ListCard href={`/media/${item.id}`} title={item.title || item.originalFilename || "Untitled"}>
        <svelte:fragment slot="leading">
          <Icon size={30} />
        </svelte:fragment>
        <svelte:fragment slot="trailing">
          <Pill accent={getKindAccent(item.kind)}>{item.kind}</Pill>
        </svelte:fragment>
      </ListCard>
    {/each}
  </Grid>
{/if}
```

### Upload Page

The upload page uses Underlay's blob upload utilities:

```svelte
<!-- src/routes/(app)/media/upload/+page.svelte -->
<script lang="ts">
  import { goto } from "$app/navigation";
  import { useToasts } from "@decodelabs/underlay/runtime/feedback";
  import {
    uploadToBlob,
    computeFileHash,
    validateFile,
    formatFileSize,
    ALLOWED_MEDIA_TYPES,
    type UploadPlan
  } from "@decodelabs/underlay/runtime/media";
  import { mediaCommands, MediaKind, MediaVisibility } from "@my-client";
  import { auth } from "$lib/stores/auth";
  import { PageHeader as PoodlePageHeader } from "@poodle/svelte-composites";
  import { Button, Field, TextInput, Select } from "@poodle/svelte-primitives";

  const toastStore = useToasts();

  // Form state
  let title = $state("");
  let visibility = $state(MediaVisibility.Public);
  let selectedFile = $state<File | null>(null);
  let fileError = $state<string | null>(null);

  // Upload state
  let uploadStep = $state<"select" | "checking" | "duplicate" | "uploading" | "complete" | "error">("select");
  let uploadProgress = $state(0);
  let fileHash = $state<string | null>(null);
  let duplicateMedia = $state<any>(null);
  let createdMediaId = $state<string | null>(null);

  function validateAndSetFile(file: File) {
    fileError = null;
    selectedFile = null;

    const result = validateFile(file, 25 * 1024 * 1024);
    if (!result.valid) {
      fileError = result.error ?? "Invalid file";
      return;
    }

    selectedFile = file;
    if (!title) {
      title = file.name.replace(/\.[^/.]+$/, "");
    }
  }

  function getMediaKind(file: File): string {
    if (file.type.startsWith("image/")) return MediaKind.Image;
    if (file.type === "application/pdf") return MediaKind.Pdf;
    return MediaKind.Image;
  }

  async function startUpload() {
    if (!selectedFile) return;

    const token = auth.getToken();
    if (!token) {
      toastStore.push({ variant: "error", message: "Not authenticated" });
      return;
    }

    uploadStep = "checking";

    try {
      // Step 1: Compute hash
      fileHash = await computeFileHash(selectedFile);

      // Step 2: Check for duplicates
      const duplicateCheck = await mediaCommands.checkDuplicate(
        { sha256: fileHash },
        fetch,
        token
      );

      if (duplicateCheck.exists && duplicateCheck.media) {
        duplicateMedia = duplicateCheck.media;
        uploadStep = "duplicate";
        return;
      }

      // No duplicate - proceed
      await proceedWithUpload(token);
    } catch (e) {
      uploadStep = "error";
    }
  }

  async function proceedWithUpload(token: string) {
    if (!selectedFile || !fileHash) return;

    uploadStep = "uploading";
    uploadProgress = 0;

    try {
      // Step 3: Create media item
      const media = await mediaCommands.createMedia(
        {
          kind: getMediaKind(selectedFile),
          visibility,
          title: title || null,
          originalFilename: selectedFile.name
        },
        fetch,
        token
      );

      createdMediaId = media.id;

      // Step 4: Initiate upload
      const uploadResponse = await mediaCommands.initiateUpload(
        media.id,
        {
          contentType: selectedFile.type,
          contentLength: selectedFile.size,
          sha256: fileHash
        },
        fetch,
        token
      );

      // Step 5: Upload to blob storage
      const plan: UploadPlan = {
        uploadUrl: uploadResponse.uploadPlan.uploadUrl,
        method: uploadResponse.uploadPlan.method,
        requiredHeaders: uploadResponse.uploadPlan.headers,
        expiresAt: uploadResponse.uploadPlan.expiresAt,
        maxBytes: uploadResponse.uploadPlan.maxBytes || 25 * 1024 * 1024,
        allowedContentTypes: uploadResponse.uploadPlan.allowedContentTypes || [],
        objectKey: ""
      };

      await uploadToBlob(plan, selectedFile, {
        onProgress: (progress) => {
          uploadProgress = progress.percent;
        }
      });

      // Step 6: Finalize
      await mediaCommands.finaliseUpload(
        media.id,
        uploadResponse.versionId,
        { sha256: fileHash },
        fetch,
        token
      );

      uploadStep = "complete";
      toastStore.push({ variant: "success", message: "Media uploaded successfully" });
    } catch (e) {
      uploadStep = "error";
    }
  }
</script>

<PageHeader title="Upload Media" backHref="/media" backLabel="Back to media" />

<div class="upload-container">
  {#if uploadStep === "select"}
    <div
      class="dropzone"
      ondrop={(e) => { e.preventDefault(); validateAndSetFile(e.dataTransfer?.files?.[0]!); }}
      ondragover={(e) => e.preventDefault()}
    >
      {#if selectedFile}
        <p>{selectedFile.name} ({formatFileSize(selectedFile.size)})</p>
        <Button variant="subtle" onclick={() => { selectedFile = null; }}>Remove</Button>
      {:else}
        <p>Drag and drop a file here, or click to browse</p>
        <input type="file" accept={ALLOWED_MEDIA_TYPES.join(",")} onchange={(e) => validateAndSetFile(e.target.files?.[0]!)} />
      {/if}
    </div>

    <Field label="Title (optional)">
      <TextInput bind:value={title} placeholder="Enter a title" />
    </Field>

    <Field label="Visibility">
      <Select bind:value={visibility}>
        <option value={MediaVisibility.Public}>Public</option>
        <option value={MediaVisibility.Restricted}>Restricted</option>
      </Select>
    </Field>

    <Button variant="primary" disabled={!selectedFile} onclick={startUpload}>
      Upload
    </Button>

  {:else if uploadStep === "uploading"}
    <div class="progress">
      <p>Uploading... {uploadProgress}%</p>
      <div class="progress-bar" style="width: {uploadProgress}%"></div>
    </div>

  {:else if uploadStep === "complete"}
    <p>Upload complete!</p>
    <Button onclick={() => goto(`/media/${createdMediaId}`)}>View Media</Button>
  {/if}
</div>
```

### Detail Page with Tabs

```svelte
<!-- src/routes/(app)/media/[id]/+page.svelte -->
<script lang="ts">
  import { page } from "$app/stores";
  import {
    getBackButtonInfo,
    useAuthenticatedData
  } from "@decodelabs/underlay/runtime";
  import { PageLoading } from "@poodle/svelte-composites";
  import { Card, Code, MetaBar, MetaItem, PageHeader, Pill, Tabs, TimeAgo, type TabItem } from "@poodle/svelte-primitives";
  import { MediaActionsMenu } from "$lib/menus";
  import { mediaCommands, MediaKind, MediaVisibility, MediaVersionState } from "@my-client";
  import { auth, authLoading, currentUser } from "$lib/stores/auth";

  const mediaId = $derived($page.params.id!);

  const pageData = useAuthenticatedData(
    async (fetchFn, token) => {
      const [media, versions, usages] = await Promise.all([
        mediaCommands.getMedia(mediaId, fetchFn, token),
        mediaCommands.listVersions(mediaId, fetchFn, token),
        mediaCommands.listUsages(mediaId, fetchFn, token)
      ]);
      return { media, versions, usages };
    },
    { getToken: () => auth.getToken() }
  );

  $effect(() => {
    pageData.tryFetch($authLoading, $currentUser);
  });

  const media = $derived(pageData.data?.media);
  const versions = $derived(pageData.data?.versions ?? []);
  const usages = $derived(pageData.data?.usages ?? []);

  let activeTab = $state("details");
  const usageCount = $derived(usages.length);
  const tabItems = $derived<TabItem[]>([
    { value: "details", label: "Details" },
    { value: "usage", label: "Usage", count: usageCount }
  ]);
  const backInfo = getBackButtonInfo("Back to media", "/media");

  function formatFileSize(bytes: number | null): string {
    if (!bytes) return "—";
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }
</script>

{#if pageData.loading}
  <PageLoading presentation="inline" message="Loading media..." />
{:else if media}
  <PageHeader
    title={media.title || media.originalFilename || "Untitled"}
    backHref={backInfo.href}
    backLabel={backInfo.label}
    bannerMessage={media.deletedAt ? "This media has been soft-deleted." : undefined}
  >
    <p>
      <strong>ID:</strong> <code>{media.id}</code>
      <Pill accent={media.kind === MediaKind.Image ? "#22c55e" : "#ef4444"}>
        {media.kind}
      </Pill>
      <Pill accent={media.visibility === MediaVisibility.Restricted ? "#f59e0b" : "#3b82f6"}>
        {media.visibility}
      </Pill>
    </p>

    {#snippet actions()}
      <MediaActionsMenu
        {media}
        onSoftDeleteSuccess={() => pageData.refetch()}
        onRestoreSuccess={() => pageData.refetch()}
      />
    {/snippet}
  </PageHeader>

  <Tabs bind:value={activeTab} items={tabItems} variant="card" size="sm" historyKey="tab" ariaLabel="Media sections" let:activeValue>
    {#if activeValue === "details"}
      <div class="underlay-details-content">
        <DetailsCard>
          <DetailsSection legend="File Details">
            <DetailsItem label="Original Filename" value={media.originalFilename} />
            {#if media.currentVersion}
              <DetailsItem label="File Size" value={formatFileSize(media.currentVersion.byteSize)} />
              <DetailsItem label="MIME Type" value={media.currentVersion.mimeType} />
            {/if}
            <DetailsItem label="Usage Count" value={String(media.usageCount)} />
          </DetailsSection>

          <DetailsSection legend="Timestamps">
            <DetailsItem label="Created">
              <TimeAgo datetime={media.createdAt} />
            </DetailsItem>
            <DetailsItem label="Last Updated">
              <TimeAgo datetime={media.updatedAt} />
            </DetailsItem>
          </DetailsSection>
        </DetailsCard>

        <Card>Caller-owned versions list here</Card>
      </div>
    {/if}

    {#if activeValue === "usage"}
      <div class="underlay-details-content">
        {#if usages.length === 0}
          <p>This media is not used anywhere yet.</p>
        {:else}
          <Card>Caller-owned usages list here</Card>
        {/if}
      </div>
    {/if}
  </Tabs>
{/if}
```

The current media-detail direction is direct Poodle `Card` composition with
caller-owned rows rather than a retained Underlay inline-list wrapper pair.

### Actions Menu

```svelte
<!-- src/lib/menus/MediaActionsMenu.svelte -->
<script lang="ts">
  import { useToasts } from "@decodelabs/underlay/runtime/feedback";
  import { AlertDialog } from "@poodle/svelte-primitives";
  import { mediaCommands, type MediaDetail } from "@my-client";
  import { auth } from "$lib/stores/auth";
  import Trash2 from "lucide-svelte/icons/trash-2";
  import RotateCcw from "lucide-svelte/icons/rotate-ccw";
  import AlertTriangle from "lucide-svelte/icons/alert-triangle";

  interface Props {
    media: MediaDetail;
    onSoftDeleteSuccess?: () => void;
    onRestoreSuccess?: () => void;
  }

  let { media, onSoftDeleteSuccess, onRestoreSuccess }: Props = $props();

  const toastStore = useToasts();

  let softDeleteOpen = $state(false);
  let restoreOpen = $state(false);
  let purgeOpen = $state(false);

  async function confirmSoftDelete() {
    const token = auth.getToken();
    if (!token) return;

    await mediaCommands.softDeleteMedia(media.id, fetch, token);
    softDeleteOpen = false;
    toastStore.push({ variant: "success", message: "Media moved to trash" });
    onSoftDeleteSuccess?.();
  }

  async function confirmRestore() {
    const token = auth.getToken();
    if (!token) return;

    await mediaCommands.restoreMedia(media.id, fetch, token);
    restoreOpen = false;
    toastStore.push({ variant: "success", message: "Media restored" });
    onRestoreSuccess?.();
  }

  async function confirmPurge() {
    const token = auth.getToken();
    if (!token) return;

    await mediaCommands.purgeMedia(media.id, fetch, token);
    purgeOpen = false;
    toastStore.push({ variant: "success", message: "Media permanently deleted" });
  }
</script>

<AlertDialog
  bind:open={softDeleteOpen}
  title="Move to trash?"
  description="This media will be moved to trash. You can restore it later."
  confirmLabel="Move to trash"
  onConfirm={confirmSoftDelete}
  onCancel={() => { softDeleteOpen = false; }}
  tone="danger"
/>

<AlertDialog
  bind:open={restoreOpen}
  title="Restore media?"
  description="This will restore the media back to the library."
  confirmLabel="Restore"
  onConfirm={confirmRestore}
  onCancel={() => { restoreOpen = false; }}
  tone="warning"
/>

<AlertDialog
  bind:open={purgeOpen}
  title="Permanently delete?"
  description="This will permanently delete the media and all versions. This cannot be undone."
  confirmLabel="Delete permanently"
  onConfirm={confirmPurge}
  onCancel={() => { purgeOpen = false; }}
  tone="danger"
/>

<Menu items={menuItems} triggerAriaLabel="Media actions">
  {#if media.deletedAt}
    <button onclick={() => { restoreOpen = true; }}>
      <RotateCcw size={14} /> Restore
    </button>
    <button class="danger" onclick={() => { purgeOpen = true; }}>
      <AlertTriangle size={14} /> Delete Permanently
    </button>
  {:else}
    <button onclick={() => { softDeleteOpen = true; }}>
      <Trash2 size={14} /> Move to Trash
    </button>
  {/if}
</Menu>
```

## Shared Underlay Components

Underlay provides reusable components and patterns for media library implementations. These significantly reduce boilerplate code when building media features.

### Shared Types

All media types are exported from `@decodelabs/underlay/patterns`:

```typescript
import {
  // Enums
  MediaKind,
  MediaVisibility,
  MediaVersionState,

  // Types
  type MediaSummary,
  type MediaDetail,
  type MediaVersion,
  type MediaRendition,
  type MediaUsage,

  // Request/Response DTOs
  type CreateMediaRequest,
  type UpdateMediaRequest,
  type CheckDuplicateRequest,
  type CheckDuplicateResponse,
  type InitiateUploadRequest,
  type InitiateUploadResponse,
  type FinaliseUploadRequest,
  type FinaliseUploadResponse,
  type MediaListQuery,

  // Utility functions
  getMediaKindLabel,
  getMediaKindAccent,
  getMediaVisibilityLabel,
  getMediaVisibilityAccent,
  getMediaVersionStateLabel,
  getMediaVersionStateAccent,
  detectMediaKindFromMimeType,
  isMediaDeleted,
  getMediaDisplayName,
} from "@decodelabs/underlay/patterns";
```

These types match the API contracts, so your TypeScript client can use them directly. Consuming apps typically re-export these from their API client package for convenience.

### MediaPicker Component

Underlay `MediaPicker` is retired. Use the Poodle media surfaces directly:

- Poodle `MediaPicker` for lightweight local-item selection
- Poodle `MediaBrowsePanel` and `MediaUploadStatusPanel` for callback-driven media-library UI
- Poodle media-workflow helpers for paginated browse, duplicate detection, and upload orchestration

```svelte
<script lang="ts">
  import { MediaThumbnail } from "@poodle/svelte-composites";
  import type { MediaKind } from "@decodelabs/underlay/patterns";

  export let thumbnailUrl: string | null = null;
  export let title = "Media thumbnail";
  export let kind: MediaKind = "image";

  function toPoodleMediaKind(kind: MediaKind): "image" | "audio" | "video" | "document" | "embed" {
    if (kind === "image") return "image";
    if (kind === "audio") return "audio";
    if (kind === "video") return "video";
    return "document";
  }
</script>

<MediaThumbnail
  kind={toPoodleMediaKind(kind)}
  presentation="compact"
  aspectRatio="square"
  ariaLabel={title}
>
  {#if thumbnailUrl}
    <img src={thumbnailUrl} alt={title} class="media-thumbnail-image" />
  {/if}
</MediaThumbnail>
```

A callback-driven media-library picker is now caller-owned composition:

```svelte
<script lang="ts">
  import {
    Dialog,
    FileUpload,
    Tabs,
    type FileUploadItem,
    type TabItem,
  } from "@poodle/svelte-primitives";
  import {
    MediaBrowsePanel,
    MediaUploadStatusPanel,
    createResetMediaBrowseState,
    loadMediaBrowsePage,
    mergeMediaBrowseItems,
    runMediaUploadWorkflow,
    uploadMediaWithKnownHash,
    type MediaPickerItem,
    type MediaUploadDisplayStep,
  } from "@poodle/svelte-composites";
  import { mediaCommands, type MediaSummary } from "@my-client";
  import { auth } from "$lib/stores/auth";

  let pickerOpen = $state(false);
  let selectedMedia = $state<MediaSummary | null>(null);
  let activeTab = $state("browse");
  let uploadFiles = $state<FileUploadItem[]>([]);
  let uploadStep = $state<MediaUploadDisplayStep>("select");
  let uploadProgress = $state(0);
  let uploadError = $state<string | null>(null);
  let duplicateMedia = $state<MediaSummary | null>(null);
  let createdMedia = $state<MediaSummary | null>(null);

  let browseItems = $state<MediaSummary[]>([]);
  let browseNextCursor = $state<string | null>(null);
  let browseHasMore = $state(false);

  async function listMediaPaginated(params?: PaginationParams) {
    const token = auth.getToken();
    if (!token) throw new Error("Not authenticated");
    return mediaCommands.listMediaPaginated(fetch, token, params);
  }

  async function checkDuplicate(sha256: string) {
    const token = auth.getToken();
    if (!token) throw new Error("Not authenticated");
    return mediaCommands.checkDuplicate({ sha256 }, fetch, token);
  }

  async function createMedia(request: CreateMediaRequest) {
    const token = auth.getToken();
    if (!token) throw new Error("Not authenticated");
    return mediaCommands.createMedia(request, fetch, token);
  }

  async function initiateUpload(mediaId: string, request: InitiateUploadRequest) {
    const token = auth.getToken();
    if (!token) throw new Error("Not authenticated");
    return mediaCommands.initiateUpload(mediaId, request, fetch, token);
  }

  async function finaliseUpload(mediaId: string, versionId: string, request: FinaliseUploadRequest) {
    const token = auth.getToken();
    if (!token) throw new Error("Not authenticated");
    return mediaCommands.finaliseUpload(mediaId, versionId, request, fetch, token);
  }

  function handleSelect(mediaId: string, media: MediaSummary) {
    selectedMedia = media;
    pickerOpen = false;
  }

  function toPickerItem(media: MediaSummary): MediaPickerItem {
    return {
      id: media.id,
      label: media.title ?? media.originalFilename ?? "Untitled",
      thumbnailUrl: media.thumbnailUrl,
      kind: media.kind === "image" ? "image" : "document",
    };
  }

  async function loadInitialBrowse() {
    const page = await loadMediaBrowsePage({ listPage: listMediaPaginated });
    browseItems = page.items;
    browseNextCursor = page.nextCursor;
    browseHasMore = page.hasMore;
  }

  async function loadMoreBrowse() {
    if (!browseNextCursor) return;
    const page = await loadMediaBrowsePage({
      listPage: listMediaPaginated,
      cursor: browseNextCursor,
    });
    browseItems = mergeMediaBrowseItems(browseItems, page.items, browseNextCursor);
    browseNextCursor = page.nextCursor;
    browseHasMore = page.hasMore;
  }

  async function startUpload() {
    const file = uploadFiles[0]?.file;
    if (!file) return;

    const result = await runMediaUploadWorkflow({
      file,
      maxFileSize: 25 * 1024 * 1024,
      checkDuplicate: async (sha256) => {
        const duplicate = await checkDuplicate(sha256);
        return { exists: duplicate.exists, item: duplicate.media ?? null };
      },
      createRecord: createMedia,
      buildCreateRequest: (nextFile) => ({
        kind: nextFile.type.startsWith("image/") ? "image" : "document",
        visibility: "public",
        title: nextFile.name.replace(/\.[^/.]+$/, ""),
        originalFilename: nextFile.name,
      }),
      initiateUpload: (media, request) => initiateUpload(media.id, request),
      buildInitiateRequest: (nextFile, fileHash) => ({
        contentType: nextFile.type,
        contentLength: nextFile.size,
        sha256: fileHash,
      }),
      finaliseUpload: (media, versionId, request) =>
        finaliseUpload(media.id, versionId, request),
      buildFinaliseRequest: (nextFile, fileHash) => ({
        sha256: fileHash,
        contentType: nextFile.type,
      }),
      toCreatedItem: (result) => result.media,
      onStep: (step) => (uploadStep = step),
      onProgress: (percent) => (uploadProgress = percent),
    });

    if (result.kind === "duplicate") {
      duplicateMedia = result.existingItem;
      return;
    }

    createdMedia = result.createdItem;
  }
</script>

<button onclick={() => pickerOpen = true}>Select Media</button>

<Dialog bind:open={pickerOpen} title="Select an image">
  <Tabs
    value={activeTab}
    items={[
      { value: "browse", label: "Browse" },
      { value: "upload", label: "Upload" },
    ] satisfies TabItem[]}
    on:valueChange={(event) => (activeTab = event.detail.value)}
  />

  {#if activeTab === "browse"}
    <MediaBrowsePanel
      items={browseItems.map(toPickerItem)}
      hasMore={browseHasMore}
      on:loadMore={loadMoreBrowse}
      on:select={(event) => {
        const media = browseItems.find((item) => item.id === event.detail.item.id);
        if (media) handleSelect(media.id, media);
      }}
    />
  {:else if uploadStep === "select"}
    <FileUpload bind:files={uploadFiles} showPreview={false} />
    <button onclick={startUpload}>Upload</button>
  {:else}
    <MediaUploadStatusPanel
      {uploadStep}
      {uploadProgress}
      {uploadError}
      duplicateLabel={duplicateMedia?.title ?? duplicateMedia?.originalFilename ?? null}
      on:selectDuplicate={() => duplicateMedia && handleSelect(duplicateMedia.id, duplicateMedia)}
      on:selectUploaded={() => createdMedia && handleSelect(createdMedia.id, createdMedia)}
      on:uploadAnyway={async () => {
        const file = uploadFiles[0]?.file;
        if (!file) return;
        const result = await uploadMediaWithKnownHash({
          file,
          fileHash: "",
          maxFileSize: 25 * 1024 * 1024,
          createRecord: createMedia,
          buildCreateRequest: (nextFile) => ({
            kind: nextFile.type.startsWith("image/") ? "image" : "document",
            visibility: "public",
            title: nextFile.name.replace(/\.[^/.]+$/, ""),
            originalFilename: nextFile.name,
          }),
          initiateUpload: (media, request) => initiateUpload(media.id, request),
          buildInitiateRequest: (nextFile, fileHash) => ({
            contentType: nextFile.type,
            contentLength: nextFile.size,
            sha256: fileHash,
          }),
          finaliseUpload: (media, versionId, request) =>
            finaliseUpload(media.id, versionId, request),
          buildFinaliseRequest: (nextFile, fileHash) => ({
            sha256: fileHash,
            contentType: nextFile.type,
          }),
          toCreatedItem: (result) => result.media,
        });
        createdMedia = result;
      }}
      on:clearUpload={() => {
        uploadFiles = [];
        uploadStep = "select";
        duplicateMedia = null;
        createdMedia = null;
        uploadError = null;
      }}
    />
  {/if}
</Dialog>
```

For app convenience, it is still reasonable to make a local wrapper that
pre-binds your media commands and auth wiring. That wrapper should be
app-owned now, not exported from Underlay.

**Boundary note:**

- Use Poodle `MediaPicker` when you already have a local `items` array and only
  need lightweight browse/search/upload-tab composition.
- Use Poodle `MediaBrowsePanel` and `MediaUploadStatusPanel` when you need the
  reusable browse-grid or upload-status UI.
- Use the Poodle media-workflow helpers when the host still owns paginated
  browse loading, duplicate detection, create/initiate/finalise callbacks, and
  media-library upload orchestration.

### MediaActionsMenu Wrapper

Keep media actions local to the consuming app. Build the wrapper directly from
Poodle `Menu` and `AlertDialog`, plus local media-command wiring and
Underlay clipboard/toast helpers:

```svelte
<script lang="ts">
  import { AlertDialog, Button, Menu } from "@poodle/svelte-primitives";
  import { copyToClipboard } from "@decodelabs/underlay/runtime/feedback";
  import { useToasts } from "@decodelabs/underlay/runtime/feedback";
  import { mediaCommands, type MediaDetail } from "@my-client";
  import { auth } from "$lib/stores/auth";

  interface Props {
    media: MediaDetail;
  }

  let { media }: Props = $props();

  const toastStore = useToasts();
  let softDeleteOpen = $state(false);

  async function softDelete(mediaId: string) {
    const token = auth.getToken();
    if (!token) throw new Error("Not authenticated");
    await mediaCommands.softDeleteMedia(mediaId, fetch, token);
  }

  async function restore(mediaId: string) {
    const token = auth.getToken();
    if (!token) throw new Error("Not authenticated");
    await mediaCommands.restoreMedia(mediaId, fetch, token);
  }

  async function purge(mediaId: string) {
    const token = auth.getToken();
    if (!token) throw new Error("Not authenticated");
    await mediaCommands.purgeMedia(mediaId, fetch, token);
  }

  async function handleAction(value: string) {
    if (value === "copy-id") {
      await copyToClipboard(toastStore, media.id, "Copied ID", "Failed to copy ID");
    } else if (value === "replace") {
      goto(`/media/upload?replace=${media.id}`);
    } else if (value === "soft-delete") {
      softDeleteOpen = true;
    }
  }

  const items = [
    { value: "replace", label: "Replace file" },
    { value: "soft-delete", label: "Soft delete", tone: "danger" as const },
    { value: "separator-copy", label: "", kind: "separator" as const },
    { value: "copy-id", label: "Copy ID" }
  ];
</script>

<Menu items={items} ariaLabel="Media actions" triggerAriaLabel="Media actions" on:action={(event) => void handleAction(event.detail.value)}>
  <Button slot="trigger" variant="secondary">Actions</Button>
</Menu>

<AlertDialog
  bind:open={softDeleteOpen}
  title="Soft delete media?"
  description="Soft deleting will hide this media from listings. You can restore it later from trash."
  confirmLabel="Soft delete"
  tone="danger"
  onConfirm={() => softDelete(media.id)}
  onCancel={() => (softDeleteOpen = false)}
/>
```

The menu automatically shows appropriate actions based on the media's deleted state.
It is the retained Underlay media-operation shell: copy, edit, replace-file,
soft-delete, restore, and purge workflow orchestration stay here, while generic
modal, button, upload, thumbnail, and status chrome should resolve through
Poodle.

### Upload Flow Pattern

For building custom upload pages, use the `createMediaUploadFlow` state machine:

```typescript
import { createMediaUploadFlow, type MediaUploadFlowController } from "@decodelabs/underlay/runtime/media";
import { mediaCommands } from "@my-client";
import { auth } from "$lib/stores/auth";

const uploadFlow = createMediaUploadFlow({
  // API callbacks
  checkDuplicate: async (sha256) => {
    const token = auth.getToken()!;
    return mediaCommands.checkDuplicate({ sha256 }, fetch, token);
  },
  createMedia: async (request) => {
    const token = auth.getToken()!;
    return mediaCommands.createMedia(request, fetch, token);
  },
  initiateUpload: async (mediaId, request) => {
    const token = auth.getToken()!;
    return mediaCommands.initiateUpload(mediaId, request, fetch, token);
  },
  finaliseUpload: async (mediaId, versionId, request) => {
    const token = auth.getToken()!;
    return mediaCommands.finaliseUpload(mediaId, versionId, request, fetch, token);
  },

  // Lifecycle callbacks
  onComplete: (media) => {
    toasts.push({ variant: "success", message: "Upload complete!" });
    goto(`/media/${media.id}`);
  },
  onError: (error) => {
    toasts.push({ variant: "error", message: error.message });
  },

  // Options
  maxFileSize: 25 * 1024 * 1024,
  defaultVisibility: MediaVisibility.Public,
});
```

**State Machine Steps:**

| Step | Description |
|------|-------------|
| `select` | Initial state, waiting for file selection |
| `checking` | Computing hash, checking for duplicates |
| `duplicate` | Duplicate found, user chooses action |
| `uploading` | Uploading to blob storage |
| `finalising` | Finalizing upload on server |
| `complete` | Upload successful |
| `error` | Error occurred |

**Controller Interface:**

```typescript
interface MediaUploadFlowController {
  // Reactive state
  readonly step: MediaUploadStep;
  readonly file: File | null;
  readonly fileError: string | null;
  readonly fileHash: string | null;
  readonly progress: number;        // 0-100
  readonly error: string | null;
  readonly duplicateMedia: MediaSummary | null;
  readonly createdMedia: MediaDetail | null;

  // Computed
  readonly canUpload: boolean;
  readonly isUploading: boolean;

  // Actions
  setFile: (file: File) => void;
  clearFile: () => void;
  startUpload: (metadata?: Partial<CreateMediaRequest>) => Promise<void>;
  proceedWithUpload: (metadata?: Partial<CreateMediaRequest>) => Promise<void>;
  useDuplicate: () => void;
  reset: () => void;
}
```

**Using in a component:**

```svelte
<script lang="ts">
  const uploadFlow = createMediaUploadFlow({ /* ... */ });
</script>

{#if uploadFlow.step === "select"}
  <FileDropzone onfile={(f) => uploadFlow.setFile(f)} />
  {#if uploadFlow.file}
    <Button onclick={() => uploadFlow.startUpload()}>Upload</Button>
  {/if}

{:else if uploadFlow.step === "checking"}
  <Spinner /> Checking for duplicates...

{:else if uploadFlow.step === "duplicate"}
  <p>This file already exists: {uploadFlow.duplicateMedia?.title}</p>
  <Button onclick={() => uploadFlow.useDuplicate()}>Use existing</Button>
  <Button onclick={() => uploadFlow.proceedWithUpload()}>Upload anyway</Button>

{:else if uploadFlow.step === "uploading"}
  <Progress value={uploadFlow.progress} ariaLabel="Upload progress" />

{:else if uploadFlow.step === "complete"}
  <p>Upload complete!</p>
  <Button onclick={() => uploadFlow.reset()}>Upload another</Button>

{:else if uploadFlow.step === "error"}
  <p>Error: {uploadFlow.error}</p>
  <Button onclick={() => uploadFlow.reset()}>Try again</Button>
{/if}
```

**Replace file flow:**

For adding a new version to existing media, pass `existingMediaId`:

```typescript
const replaceFlow = createMediaUploadFlow({
  // ... callbacks
  existingMediaId: media.id,
  existingVersionHashes: media.versions?.map(v => v.sha256).filter(Boolean) ?? [],
});
```

This skips `createMedia` and prevents uploading the same file that's already a version.

### Rust Types (underlay-db)

For Rust backends, media types are exported from the `underlay-db` crate:

```rust
use underlay_db::{
    // Enums
    MediaKind,
    MediaVisibility,
    MediaVersionState,

    // Utility function
    detect_media_kind_from_mime_type,

    // Error type for parsing
    MediaTypeParseError,
};
```

**Re-exporting in your domain layer:**

Consuming apps should re-export the types for use throughout the codebase:

```rust
// crates/domain/src/media/mod.rs

// Re-export media enums from underlay-db
pub use underlay_db::{MediaKind, MediaVersionState, MediaVisibility};

// Your domain-specific types
pub struct MediaId(pub Uuid);
pub struct MediaVersionId(pub Uuid);

pub struct Media {
    pub id: MediaId,
    pub kind: MediaKind,          // Using underlay-db type
    pub visibility: MediaVisibility,
    pub title: String,
    // ...
}
```

**Converting from database rows:**

```rust
impl From<MediaRow> for Media {
    fn from(row: MediaRow) -> Self {
        Self {
            id: MediaId(row.id),
            // Parse string from DB into enum using FromStr
            kind: row.kind.parse().unwrap_or(MediaKind::Image),
            visibility: row.visibility.parse().unwrap_or(MediaVisibility::Public),
            title: row.title,
            // ...
        }
    }
}
```

**Serialization:**

The enums serialize to lowercase strings matching the TypeScript definitions:

```rust
use serde_json;

let kind = MediaKind::Image;
assert_eq!(serde_json::to_string(&kind).unwrap(), "\"image\"");

let visibility = MediaVisibility::Restricted;
assert_eq!(serde_json::to_string(&visibility).unwrap(), "\"restricted\"");
```

## Blob Upload Utilities

Underlay provides client-side utilities for blob uploads in `@decodelabs/underlay/patterns`:

### Available Functions

| Function | Description |
|----------|-------------|
| `uploadToBlob(plan, file, options)` | Upload file directly to blob storage |
| `computeFileHash(file)` | Compute SHA-256 hash for deduplication |
| `validateFile(file, maxBytes)` | Validate file type and size |
| `validateFileType(file, allowedTypes)` | Check if file type is allowed |
| `validateFileSize(file, maxBytes)` | Check if file size is within limit |
| `formatFileSize(bytes)` | Format bytes to human-readable string |
| `getFileTypeDescription(mimeType)` | Get friendly file type name |
| `isVideoFile(file)` | Check if file is a video (to reject) |

### Types

| Type | Description |
|------|-------------|
| `UploadPlan` | Pre-signed URL and constraints from server |
| `UploadProgress` | Progress info (loaded, total, percent) |
| `UploadResult` | Result after successful upload |
| `UploadOptions` | Options for upload (onProgress, signal) |
| `BlobUploadError` | Error with code and status |

### Constants

```typescript
import {
  ALLOWED_IMAGE_TYPES,  // ['image/jpeg', 'image/png', ...]
  ALLOWED_PDF_TYPES,    // ['application/pdf']
  ALLOWED_MEDIA_TYPES,  // Combined image + PDF
  REJECTED_VIDEO_TYPES, // Video types to reject
} from "@decodelabs/underlay/patterns";
```

## Best Practices

### Deduplication

Always check for duplicates before uploading:

```typescript
const hash = await computeFileHash(file);
const { exists, media } = await mediaCommands.checkDuplicate({ sha256: hash }, fetch, token);

if (exists) {
  // Offer to use existing media or upload as new
}
```

### Progress Tracking

Use the onProgress callback for user feedback:

```typescript
await uploadToBlob(plan, file, {
  onProgress: (progress) => {
    uploadProgress = progress.percent;
  }
});
```

### Error Handling

Handle upload errors gracefully:

```typescript
try {
  await uploadToBlob(plan, file, options);
} catch (e) {
  if (e instanceof BlobUploadError) {
    switch (e.code) {
      case "FILE_TOO_LARGE":
        showError("File exceeds size limit");
        break;
      case "UPLOAD_EXPIRED":
        showError("Upload URL expired, please try again");
        break;
      case "NETWORK_ERROR":
        showError("Network error, check your connection");
        break;
      // ...
    }
  }
}
```

### Soft Delete Pattern

Always use soft delete by default, with permanent delete requiring confirmation:

1. **Soft delete**: Moves to trash, can be restored
2. **Restore**: Brings back from trash
3. **Purge**: Permanent deletion, requires double confirmation

### Usage Tracking

Track where media is used to prevent orphaned files:

```rust
// When a content item references media
db::media::create_usage(&pool, media_id, "qa_item", qa_item_id, "image").await?;

// When reference is removed
db::media::remove_usage(&pool, media_id, "qa_item", qa_item_id, "image").await?;
```

## Next Steps

- See [076-nightfire.md](./076-nightfire.md) for integrating media with block-based content
- See [055-background-jobs.md](./055-background-jobs.md) for implementing media processing jobs (thumbnails, optimization)
