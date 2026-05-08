# Contract: Media Library and Usage Graph

Status: Proposed shared contract
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
  `hero_01:image`, `block:cta_2/icon`
- `usage_role`: semantic role such as `primary`, `attachment`, `embedded`,
  `external`, `derived`
- `provenance_kind`: who manages the edge, such as `content_sync`,
  `legacy_migration`, `manual`, `system_generated`

The contract is intentionally not tied to a single rich-content schema. Apps
may use block JSON, plain record fields, or external/manual references as long
as they emit usage edges in this shape.

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

Locator rule:

- prefer stable block ids plus slot names
- use raw array-index paths only as fallback

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
