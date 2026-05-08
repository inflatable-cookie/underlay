# Contract: Media Library and Usage Graph

Status: active
Audience: Underlay consumers implementing media storage, structured-content
references, and migration replay

## Overview

This contract defines the reusable media boundary Underlay should own for
consumer projects:

- stable logical media assets
- immutable blob versions
- live usage edges
- storage-adapter independence
- structured-content extraction and usage sync
- migration replay bindings for portable, idempotent imports

Underlay owns the semantics and framework. Consumer apps own owner-type
registration, structured-content schemas, and project-specific migration
mapping.

## Current Implementation Boundary

This contract is active, but not every higher-level piece is fully implemented
by the shipped default backend today.

Current state:

- the richer usage-edge vocabulary, sync traits, and migration-binding types
  exist in `underlay-media`
- the shipped `PostgresMediaRepository` still primarily implements the older
  simple usage model (`MediaUsage`, `track_usage`, `sync_usages`)
- consumers should treat the generalized usage-edge and migration-binding model
  as the governing direction, but not assume every default repository/backend
  path already implements the full richer surface

Underlay should also own the recommended consumer template for the steady-state
schema shape. Apps should apply that shape through app-local migrations rather
than expecting Underlay to ship one universal migration file.

## Goals

- keep media references stable across storage-provider changes
- model media usage as a first-class graph, not inferred rendered output
- make structured-content saves and migration replays update usage edges through
  one shared sync path
- support replayable migration bundles without forcing whole-bucket rebuilds
- keep the shared surface app-agnostic

## Non-goals

- app-specific owner types, route names, or block schemas
- app-specific upload UI behavior
- hard-coding a specific rich-content engine or table layout

## Core entities

### media

Logical asset referenced by the rest of the application.

Required fields:

- `id`
- `kind`
- `visibility`
- `title`
- `original_filename`
- `current_version_id`
- audit fields
- optional soft-delete fields

### media_version

Immutable stored bytes and their storage mapping.

Required fields:

- `id`
- `media_id`
- `state`
- `byte_size`
- `mime_type`
- `sha256`
- `storage_provider`
- `bucket`
- `object_key`
- audit fields

Rules:

- each replacement creates a new `media_version`
- `object_key` is immutable per version
- `sha256` is the cross-environment byte identity

### media_rendition

Optional derived outputs for a version.

This is separate from the live usage graph. Renditions are storage derivatives,
not content ownership edges.

### media_usage

Live usage-edge table for all projects.

Required fields:

- `id`
- `media_id`
- `used_by_type`
- `used_by_id` nullable for true external/manual usage
- `owner_field` nullable for external/manual usage
- `content_kind`
- `locator_kind`
- `locator_key`
- `usage_role`
- `provenance_kind`
- `created_at`

Recommended uniqueness key:

- `media_id`
- `used_by_type`
- `used_by_id`
- `owner_field`
- `locator_kind`
- `locator_key`
- `provenance_kind`

Meaning:

- `used_by_type`: stable app-defined owner identifier, not a Rust type name
- `used_by_id`: owning record id when usage belongs to a persisted record
- `owner_field`: owning field/slot on the record, e.g. `cover_media_id`,
  `body_blocks`
- `content_kind`: broad source shape such as `record_field`, `structured_content`,
  `external`
- `locator_kind`: address type such as `field`, `block_id`, `path`,
  `external_ref`
- `locator_key`: stable in-field address such as `cover_media_id`,
  `hero_01#/imageId`, `gallery_02#/pages/1/imageId`,
  `/blocks/4/data/pages/1/imageId`
- `usage_role`: semantic role such as `primary`, `attachment`, `embedded`,
  `external`, `derived`
- `provenance_kind`: who manages the edge, such as `content_sync`,
  `legacy_migration`, `manual`, `system_generated`

The contract is intentionally not tied to a single rich-content schema. Apps
may use block JSON, plain record fields, or external/manual references as long
as they emit usage edges in this shape.

Template stance:

- this shape is the Underlay-owned steady-state template for consumer
  `media_usage` tables
- app migrations may rename/backfill from older shapes during rollout
- app migrations should converge on this shape instead of preserving older
  field-only contracts indefinitely

### migration_attachment_binding

Portable migration provenance row for replay and reuse.

Required fields:

- `source_system`
- `source_attachment_type`
- `source_attachment_id`
- `source_owner_type`
- `source_owner_id`
- `field_or_purpose`
- `sha256`
- `bundle_digest`
- `media_id`
- `media_version_id`
- `import_status`
- `imported_at`

Purpose:

- binds legacy attachment identity to stable target media ids
- lets reruns reuse unchanged media instead of uploading again
- separates migration provenance from live `media_usage`

## Storage contract

Content and app tables reference `media_id`, never raw URLs.

Blob adapters must support:

- initiate upload
- finalise upload
- public download URL resolution
- signed download URL resolution
- idempotent delete

Recommended immutable object-key shape:

- `media/{media_id}/versions/{version_id}/original/{safe_filename}`

The stable contract is:

- `media_id`
- `media_version_id`
- `sha256`
- `storage_provider + bucket + object_key`

Not the internal identity of any one S3-compatible backend.

## Usage sync contract

Projects must not manage `media_usage` with scattered app code.

Underlay should provide a shared usage-sync service that:

1. accepts desired usage edges for an owner record or external/manual target
2. loads existing managed edges for the same owner scope
3. performs set-diff reconciliation
4. inserts missing edges
5. retains unchanged edges
6. removes stale managed edges

Recommended service shape:

- `extract_media_usages_from_value(content_kind, owner_field, value)`
- `sync_media_usages_for_record(used_by_type, used_by_id, field_payloads, provenance_kind)`
- `sync_manual_media_usages(...)`

Deletion guardrail:

- sync may remove only rows inside the managed owner/provenance scope
- manual or external rows must not be removed by structured-content sync

Recommended rollout rule:

- if a consumer app still has only coarse field-level usage rows, it should
  migrate first to this locator-aware schema shape and then switch save-time
  sync and audit logic to emit exact usage edges

## Structured-content extraction contract

Underlay should provide generic interfaces for structured-content traversal, not
project-specific block logic.

Recommended extension points:

- `StructuredContentMediaExtractor`
- `StructuredContentWalker`
- `MediaUsageAuditSource`

Expected model:

- one generic recursive walker
- small extractor functions per block/node type
- recursive traversal for nested lists/containers
- emitted usage edges with stable locators

Integration rule for shared-walker work:

- keep `StructuredContentWalker` as the stable seam
- format-specific implementations like Nightfire may own temporary traversal
  code, but a later shared JSON walker should plug in underneath that seam
- that refactor must not change `MediaUsageEdgeInput`, locator semantics, or
  the `sync_media_usages_for_record(...)` contract

Current shared Nightfire base:

- `underlay-media` now ships a `nightfire` feature with:
  - `NightfireBlockMediaUsageExtractor`
  - `NightfireBlockMediaHandler`
  - `NightfireBlockMediaRegistration`
  - `NightfireBlockMediaHandlerRegistry`
  - `NightfireBlockMediaHandlerMap`
  - `resolve_nightfire_media_usage(...)`
- that shared base owns the generic traversal and canonical locator emission
- consumer apps should implement block-specific media handlers alongside their
  block definitions, export one registration per block module, and assemble the
  shared walker from those registrations
- `NightfireMediaUsageExtractor`
  - `NightfireMediaReferenceMatcher`
  - `NightfireFieldNameMatcher`
  remain as a compatibility seam for older field-name-matching consumers

Shared lifecycle:

1. persist a `NightfireValue` with stable block ids
2. use `NightfireBlockMediaUsageExtractor` to emit `MediaUsageEdgeInput`
3. reconcile those edges with `sync_media_usages_for_record(...)` or
   `extract_and_sync(...)`
4. later resolve a stored `locator_kind + locator_key` pair back into the
   current Nightfire JSON with `resolve_nightfire_media_usage(...)`

Handler rule:

- the generic walker owns traversal
- block handlers own semantic extraction for one block payload type
- handlers may declare nested Nightfire child values when a block embeds inner
  Nightfire documents
- when a nested child Nightfire document has no local block ids yet, locator
  fallback should inherit the nearest stable outer anchor instead of inventing
  a fake child root
- consumer apps should test handler behavior beside the block definition rather
  than burying media extraction in API-route JSON heuristics

Locator rule:

- for plain record fields, use `locator_kind = field` and set `locator_key` to
  the field name
- for Nightfire or other block content with stable block ids, use
  `locator_kind = block_id` and encode the locator as
  `<block-id>#<json-pointer-relative-to-block.data>`
- for Nightfire or other block content that does not yet have a stable block
  id anchor, use `locator_kind = path` and encode the locator as a JSON Pointer
  rooted at the stored value
- use raw array-index paths only inside the JSON Pointer fallback when there is
  no stable block id anchor available

Nightfire examples:

- `hero_01#/imageId`
- `gallery_02#/pages/1/imageId`
- fallback path: `/blocks/4/data/pages/1/imageId`

Resolution rule:

- resolve `block_id` locators by finding the matching block id in the
  Nightfire value, then applying the JSON Pointer against that block's `data`
  payload
- resolve `path` locators by applying the rooted JSON Pointer directly against
  the stored Nightfire value
- if a nested child block later has its own stable id, re-anchor on that child
  block id instead of preserving a longer ancestor-relative pointer

Practical rollout rule:

- if the structured-content engine does not yet provide stable block ids,
  consumers should emit `locator_kind = path` first
- once stable block ids exist, consumers should upgrade the extractor to emit
  `locator_kind = block_id` where possible
- that upgrade should not change the higher-level `media_usage` contract, only
  the consumer extractor implementation and the specific `locator_key` strings

## Consumer schema template

Underlay should treat the following as the canonical consumer template shape
for the media graph. Apps own their concrete migration files, ordering, and
backfills.

This is a template contract, not a universal migration artifact.

```sql
CREATE TABLE media.media_usage (
    id uuid PRIMARY KEY,
    media_id uuid NOT NULL REFERENCES media.media(id) ON DELETE CASCADE,
    used_by_type text NOT NULL,
    used_by_id uuid,
    owner_field text,
    content_kind text NOT NULL,
    locator_kind text NOT NULL,
    locator_key text NOT NULL,
    usage_role text NOT NULL,
    provenance_kind text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX media_usage_unique_edge
    ON media.media_usage (
        media_id,
        used_by_type,
        used_by_id,
        owner_field,
        locator_kind,
        locator_key,
        provenance_kind
    );

CREATE INDEX media_usage_used_by_scope_idx
    ON media.media_usage (used_by_type, used_by_id, provenance_kind);

CREATE INDEX media_usage_owner_field_idx
    ON media.media_usage (used_by_type, used_by_id, owner_field, provenance_kind);

CREATE INDEX media_usage_media_id_idx
    ON media.media_usage (media_id);
```

Apps may add local indexes for local query patterns, but this is the minimum
recommended template shape Underlay should document and keep stable.

Copyable artifact:

- [`docs/guides/code/077-media-library/media-usage-template.sql`](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/media-usage-template.sql)
- [`docs/guides/code/077-media-library/migrated-attachment-binding-template.sql`](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/migrated-attachment-binding-template.sql)
- [`docs/guides/code/077-media-library/locator-aware-rollout-recipe.md`](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/locator-aware-rollout-recipe.md)
- [`docs/guides/code/077-media-library/media-usage-vocabulary.md`](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/media-usage-vocabulary.md)
- [`docs/guides/code/077-media-library/nightfire-save-sync-resolve-recipe.md`](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/nightfire-save-sync-resolve-recipe.md)
- [`docs/guides/code/077-media-library/media-usage-vocabulary.md`](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/media-usage-vocabulary.md)

## Migration replay contract

Portable migration bundles should include:

- transformed target payloads
- staged media bytes
- attachment manifest rows
- decision/provenance artifacts

Each attachment manifest row should preserve:

- legacy attachment identity
- source owner identity
- target owner identity or deterministic target owner allocation key
- field/purpose
- `sha256`
- `target_media_id`
- `target_media_version_id`
- staged bundle asset path
- desired usage-edge metadata

Replay rules:

- import bytes from staged bundle assets, not mutable legacy URLs
- check existing `migration_attachment_binding` by identity plus `sha256`
  before uploading
- same identity + same hash: reuse existing target media mapping
- same identity + different hash: create a new version under the stable
  `media_id` when replacement semantics apply
- new identity: create new `media` + `media_version`

This makes bundles portable while keeping target imports idempotent.

## Audit and reconciliation

Underlay should provide audit/reconciliation scaffolding for:

1. forward audit
   - re-extract desired usages from source records
   - compare against `media_usage`
2. reverse audit
   - find usage rows pointing at missing owners or missing media
   - find orphan candidates with no active usage edges
   - find invalid locators or duplicate edges

Report-first is the default posture. Auto-repair can be layered on later.

## Consumer responsibilities

Consumer apps must provide:

- owner-type strings
- owner-field registration
- structured-content schemas and block/node extractors
- project-specific migration source identity mapping
- app-level admin and storage policy

Consumer apps must not:

- bypass the shared usage-sync surface on save/import flows
- store raw storage URLs in content records
- collapse migration provenance into live usage rows

## Extraction map

This contract is a direct candidate for shared extraction from Acowtancy into
Underlay. App-specific block schemas remain consumer-owned.
