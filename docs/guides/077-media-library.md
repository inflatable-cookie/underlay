# 077 - Media Library

This guide now covers the retained Underlay parts of a media-library
implementation:

- backend schema and media lifecycle
- usage-graph and structured-content sync semantics
- client/runtime helper boundaries
- upload orchestration and app-owned workflow logic

Generic media UI implementation has moved to Poodle and should now be treated
as the canonical source for:

- browse and upload shell composition
- media picker posture
- thumbnail and preview posture
- direct file-upload UI
- simple display-facing helpers used in media UI such as file-size and
  display-date labels

Use these Poodle guides for the UI layer:

- `Media Library And Upload Recipes` in the Poodle guide set
- `Media Picker Workflow Recipes` in the Poodle guide set

For detail routes specifically, use the Poodle media-detail posture:

- one `PageHeader`
- one `MetaBar`
- `Tabs variant="card"` with `historyKey="tab"`
- `Card` + `DetailSection` + `DetailItem` for the details tab
- `InlineListSection` for compact versions and usage sections under the tab
  surface
- `AlertDialog` for activate/delete version confirms, with action-specific
  titles, concise consequence copy, and `itemLabel` / `itemValue` for the
  selected version identifier

Keep the action sequencing in host code:

- open the confirm from the local row action
- perform the command in the route or host controller
- close on success and refetch or patch the detail surface locally

Use the Underlay recipe layer for the upload lifecycle and full-stack delivery
only:
- [Media Upload Pipeline](../patterns/media-upload-pipeline.md)

Authoritative shared contract:

- [050-media-library-and-usage.md](../contracts/050-media-library-and-usage.md)

## Quick Start

Underlay provides shared types and components to reduce boilerplate. For new implementations:

1. **Use shared types** - Import from `underlay-db` (Rust) or `@decodelabs/underlay/runtime/media` (TypeScript)
2. **Use Poodle for the UI layer** - Poodle `MediaPicker` for local item selectors, Poodle `MediaBrowsePanel` / `MediaUploadStatusPanel` for heavier browse/upload shells, app-local media actions over Poodle `Menu` / `AlertDialog`, and Poodle `MediaThumbnail` / `MediaPreview` for display posture
3. **Use the upload flow pattern** - the `Media Upload Pipeline` recipe for lifecycle order, and `createMediaUploadFlow` where the shared state helper still earns its place

| Layer | Package | Exports |
|-------|---------|---------|
| Rust types | `underlay-db` | `MediaKind`, `MediaVisibility`, `MediaVersionState` |
| TypeScript types | `@decodelabs/underlay/runtime/media` | All types, enums, and utility functions |
| App-local media actions | local app UI | Compose `Menu`, `AlertDialog`, clipboard helpers, and media commands |
| Media workflow UI/helpers | `@poodle/svelte` | `MediaPicker`, `MediaBrowsePanel`, `MediaUploadStatusPanel`, `loadMediaBrowsePage`, `mergeMediaBrowseItems`, `createResetMediaBrowseState`, `runMediaUploadWorkflow`, `uploadMediaWithKnownHash` |
| Display composites | `@poodle/svelte` | `MediaThumbnail` |
| Upload primitive | `@poodle/svelte` | `FileUpload` |
| Upload pattern | `@decodelabs/underlay/runtime/media` | `createMediaUploadFlow` |

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

The contract doc above is authoritative. The guide below shows the minimum
implementation shape for a consumer app.

Treat it as an Underlay-owned consumer template for the steady-state media
graph:

- Underlay owns this recommended shape
- consumer apps own their concrete migration files and rollout/backfill steps
- older consumer apps may migrate into this shape in stages, but they should
  converge on it rather than preserve field-only usage rows indefinitely

Copyable template artifact:

- [`docs/guides/code/077-media-library/media-usage-template.sql`](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/media-usage-template.sql)
- [`docs/guides/code/077-media-library/migrated-attachment-binding-template.sql`](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/migrated-attachment-binding-template.sql)
- [`docs/guides/code/077-media-library/locator-aware-rollout-recipe.md`](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/locator-aware-rollout-recipe.md)
- [`docs/guides/code/077-media-library/media-usage-vocabulary.md`](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/media-usage-vocabulary.md)
- [`docs/guides/code/077-media-library/nightfire-save-sync-resolve-recipe.md`](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/nightfire-save-sync-resolve-recipe.md)
- [`docs/guides/code/077-media-library/nightfire-block-media-handler-recipe.md`](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/nightfire-block-media-handler-recipe.md)

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

-- Track live media usage edges
CREATE TABLE media.media_usage (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    media_id UUID NOT NULL REFERENCES media.media(id) ON DELETE CASCADE,
    used_by_type TEXT NOT NULL,            -- app-defined owner type
    used_by_id UUID,                       -- nullable for manual/external usage
    owner_field TEXT,                      -- e.g. 'cover_media_id', 'body_blocks'
    content_kind TEXT NOT NULL,            -- 'record_field' | 'structured_content' | 'external'
    locator_kind TEXT NOT NULL,            -- 'field' | 'block_id' | 'path' | 'external_ref'
    locator_key TEXT NOT NULL,             -- stable in-field address
    usage_role TEXT NOT NULL,              -- 'primary' | 'attachment' | 'embedded' | 'external'
    provenance_kind TEXT NOT NULL,         -- 'content_sync' | 'legacy_migration' | 'manual'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(media_id, used_by_type, used_by_id, owner_field, locator_kind, locator_key, provenance_kind)
);

-- Replay-safe migration provenance
CREATE TABLE migration.migrated_attachment_binding (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_system TEXT NOT NULL,
    source_attachment_type TEXT NOT NULL,
    source_attachment_id TEXT NOT NULL,
    source_owner_type TEXT NOT NULL,
    source_owner_id TEXT NOT NULL,
    field_or_purpose TEXT NOT NULL,
    sha256 TEXT NOT NULL,
    bundle_digest TEXT NOT NULL,
    media_id UUID NOT NULL REFERENCES media.media(id) ON DELETE CASCADE,
    media_version_id UUID NOT NULL REFERENCES media.media_version(id) ON DELETE CASCADE,
    import_status TEXT NOT NULL,
    imported_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_media_deleted_at ON media.media(deleted_at);
CREATE INDEX idx_media_kind ON media.media(kind);
CREATE INDEX idx_media_version_media_id ON media.media_version(media_id);
CREATE INDEX idx_media_version_sha256 ON media.media_version(sha256);
CREATE INDEX idx_media_usage_media_id ON media.media_usage(media_id);
CREATE INDEX idx_media_usage_used_by_scope ON media.media_usage(used_by_type, used_by_id, provenance_kind);
CREATE INDEX idx_media_usage_owner_field ON media.media_usage(used_by_type, used_by_id, owner_field, provenance_kind);

-- Foreign key for current version (added after both tables exist)
ALTER TABLE media.media
    ADD CONSTRAINT fk_media_current_version
    FOREIGN KEY (current_version_id) REFERENCES media.media_version(id);
```

Rollout rule for structured content:

- if your structured-content engine does not yet provide stable block ids,
  start with `locator_kind = 'path'`
- once stable block ids exist, upgrade extractors to emit
  `locator_kind = 'block_id'` where possible
- that upgrade should change extractor output, not the shared table contract

Canonical Nightfire locator format:

- `locator_kind = 'block_id'` uses
  `<block-id>#<json-pointer-relative-to-block.data>`
- `locator_kind = 'path'` uses a JSON Pointer rooted at the stored Nightfire
  value

Examples:

- `hero_01#/imageId`
- `gallery_02#/pages/1/imageId`
- fallback path: `/blocks/4/data/pages/1/imageId`

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

The older `field`-only usage helpers are too narrow for structured-content and
manual/external usage. Prefer a full usage-edge model plus one shared sync
path.

Recommended shared surfaces:

- `MediaUsageEdge`
- `MediaUsageEdgeKey`
- `MigratedAttachmentBinding`
- `MediaUsageRepository`
- `MigrationAttachmentBindingRepository`
- `StructuredContentMediaExtractor`
- `NightfireBlockMediaUsageExtractor` with the `underlay-media` `nightfire`
  feature
- `NightfireBlockMediaHandler`
- `NightfireBlockMediaRegistration`
- `NightfireBlockMediaHandlerRegistry` / `NightfireBlockMediaHandlerMap`
- `NightfireMediaUsageExtractor`
  - `NightfireMediaReferenceMatcher` / `NightfireFieldNameMatcher`
  remain available as a compatibility path for older field-name matcher setups
- `resolve_nightfire_media_usage(...)` for the inverse lookup from stored
  `media_usage` rows back into the current Nightfire JSON
- `sync_media_usages_for_record(...)`
- `audit_media_usages_for_record(...)`

Suggested Nightfire starting point:

```rust
use underlay_media::{
    MediaId, MediaUsageProvenanceKind, MediaUsageRole,
    NightfireBlockMediaHandler, NightfireBlockMediaHandlerMap,
    NightfireBlockMediaReference, NightfireBlockMediaUsageExtractor,
    NightfireBlockMediaRegistration, NightfireMediaVisitContext,
};
use underlay_nightfire::{BlockDescriptor, BlockRegistration};

struct HeroBlockHandler;

impl NightfireBlockMediaHandler for HeroBlockHandler {
    fn extract_media_references(
        &self,
        context: &NightfireMediaVisitContext<'_>,
    ) -> underlay_media::MediaResult<Vec<NightfireBlockMediaReference>> {
        let Some(media_id) = context
            .resolve_relative_pointer("/imageId")
            .and_then(|value| value.as_str())
            .and_then(|value| uuid::Uuid::parse_str(value).ok())
            .map(MediaId::from_uuid)
        else {
            return Ok(Vec::new());
        };

        Ok(vec![NightfireBlockMediaReference::new(
            media_id,
            MediaUsageRole::Embedded,
            "/imageId",
        )])
    }
}

fn hero_media_registration() -> NightfireBlockMediaRegistration {
    NightfireBlockMediaRegistration::new("hero", HeroBlockHandler)
}

fn hero_block_registration() -> BlockRegistration<MyBlockCategory, NightfireBlockMediaRegistration> {
    BlockRegistration::new(
        BlockDescriptor {
            type_name: "hero",
            label: "Hero",
            category: MyBlockCategory::Content,
        },
        hero_media_registration(),
    )
}

let registry = NightfireBlockMediaHandlerMap::from_block_registrations([
    hero_block_registration(),
]);

let extractor = NightfireBlockMediaUsageExtractor::new(
    "lesson",
    Some(lesson_id),
    "body_blocks",
    MediaUsageProvenanceKind::ContentSync,
    registry,
);
```

If you already have a usage-sync repository, the shared extractor can also do
the full extract-plus-sync step directly:

```rust
let report = extractor
    .extract_and_sync(&media_repo, &nightfire_value)
    .await?;
```

For a fuller recipe, including nested Nightfire child values inside blocks, use:

- [`docs/guides/code/077-media-library/nightfire-block-media-handler-recipe.md`](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/nightfire-block-media-handler-recipe.md)
- and pair it with the broader block-module assembly guide:
  [`docs/guides/code/076-nightfire/nightfire-block-module-pattern.md`](/Users/tom/Dev/projects/underlay/docs/guides/code/076-nightfire/nightfire-block-module-pattern.md)

Later, when an audit or detail UI needs to follow a stored locator back into
the structured content, use the shared resolver instead of reimplementing the
lookup:

```rust
use underlay_media::{resolve_nightfire_media_usage, MediaLocatorKind};

let current_value = resolve_nightfire_media_usage(
    &nightfire_value,
    &MediaLocatorKind::BlockId,
    "gallery_02#/pages/1/imageId",
);
```

If you want one short copyable reference for the whole Nightfire lifecycle,
use:

- [`docs/guides/code/077-media-library/nightfire-save-sync-resolve-recipe.md`](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/nightfire-save-sync-resolve-recipe.md)
- pair it with the TS save-boundary guidance in
  [`docs/guides/076-nightfire.md`](/Users/tom/Dev/projects/underlay/docs/guides/076-nightfire.md),
  especially `writePreparedNightfireToFormData(...)` and the rule that inner
  Nightfire block keys stay verbatim on the wire
- and the API ingest recipe in
  [`docs/guides/070-api-handlers.md`](/Users/tom/Dev/projects/underlay/docs/guides/070-api-handlers.md),
  which shows the server-side order: ensure ids -> persist exact JSON ->
  extract and sync

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

The visible media-library UI is no longer taught here in detail.

Use these as the canonical implementation references instead:

- `Media Library And Upload Recipes` in the Poodle guide set
- `Media Picker Workflow Recipes` in the Poodle guide set
- the ACME admin media route family in the separate `underlay-reference`
  repository

What still belongs in Underlay:

- media schema and backend lifecycle
- media client/runtime helper boundaries
- upload orchestration helpers such as `createMediaUploadFlow`
- package-boundary decisions for retained helpers and shared types

## Shared Underlay Components

Underlay provides reusable components and patterns for media library implementations. These significantly reduce boilerplate code when building media features.

### Shared Types

All media types are exported from `@decodelabs/underlay/runtime/media`:

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
} from "@decodelabs/underlay/runtime/media";
```

These types match the API contracts, so your TypeScript client can use them directly. Consuming apps typically re-export these from their API client package for convenience.

### MediaPicker Component

Underlay `MediaPicker` is retired.

Keep the new split explicit:

- use the Poodle media guides for visible picker/browse/upload composition
- keep any callback-driven, auth-aware media-library wrapper app-local
- keep retained Underlay guidance here focused on helper/runtime boundaries and
  shared types

Use these references instead of the old embedded examples:

- `Media Picker Workflow Recipes` in the Poodle guide set
- `Media Library And Upload Recipes` in the Poodle guide set
- the ACME admin media upload and detail routes in the separate
  `underlay-reference` repository

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

### Upload Pipeline Factory

When multiple apps share the same upload wrapper shape and only differ in
generated media command bindings, use `createMediaUploadPipeline()` from
`@decodelabs/underlay/runtime/media` instead of hand-wrapping
`createMediaAndUpload()`, `replaceMediaUpload()`, and `checkMediaDuplicateFile()`
in each app.

```typescript
import {
  DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE,
  createMediaUploadPipeline
} from "@decodelabs/underlay/runtime/media";
import { detectMediaKindFromMimeType, mediaCommands } from "@my-client";

export const MAX_FILE_SIZE = DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE;

const pipeline = createMediaUploadPipeline({
  detectKind: detectMediaKindFromMimeType,
  createMedia: (request, context: { fetchFn: typeof fetch; accessToken: string }) =>
    mediaCommands.createMedia(request, context.fetchFn, context.accessToken),
  initiateUpload: (mediaId, request, context: { fetchFn: typeof fetch; accessToken: string }) =>
    mediaCommands.initiateUpload(mediaId, request, context.fetchFn, context.accessToken),
  finaliseUpload: (
    mediaId,
    versionId,
    request,
    context: { fetchFn: typeof fetch; accessToken: string }
  ) => mediaCommands.finaliseUpload(mediaId, versionId, request, context.fetchFn, context.accessToken),
  checkDuplicate: (request, context: { fetchFn: typeof fetch; accessToken: string }) =>
    mediaCommands.checkDuplicate(request, context.fetchFn, context.accessToken),
  includeHashInInitiate: true
});

export const createAndUpload = pipeline.createAndUpload;
export const replaceUpload = pipeline.replaceUpload;

export function checkDuplicate(file: File, fetchFn: typeof fetch, accessToken: string) {
  return pipeline.checkDuplicate(file, { fetchFn, accessToken });
}
```

Use raw `createMediaAndUpload()` and `replaceMediaUpload()` directly only when
the app genuinely needs a different wrapper shape.

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

Underlay provides client-side utilities for blob uploads in `@decodelabs/underlay/runtime/media`:

### Available Functions

| Function | Description |
|----------|-------------|
| `uploadToBlob(plan, file, options)` | Upload file directly to blob storage |
| `computeFileHash(file)` | Compute SHA-256 hash for deduplication |
| `validateFile(file, maxBytes)` | Validate file type and size |
| `validateFileType(file, allowedTypes)` | Check if file type is allowed |
| `validateFileSize(file, maxBytes)` | Check if file size is within limit |
| `formatFileSize(bytes)` | Format bytes to human-readable string for UI display; use Poodle `@poodle/svelte` |
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
} from "@decodelabs/underlay/runtime/media";
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
