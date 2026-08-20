# Media usage vocabulary

Use these values for the shared `media_usage` contract unless you have a real
cross-app reason to extend the vocabulary.

Do not invent near-duplicates like `body_field`, `json_block`, `migration`,
`imported`, `manual_external`, or `rich_content_path` when one of the values
below already fits.

## content_kind

Recommended values:

- `record_field`
  - plain record column/reference
  - examples:
    - `thumbnail_media_id`
    - `cover_media_id`
- `structured_content`
  - media referenced inside block/JSON/rich-content values
  - examples:
    - Nightfire body blocks
    - nested summary/diagram/image content
- `external`
  - usage intentionally held outside a normal owned record field
  - examples:
    - off-site PDF distribution
    - operator-protected manual retention

## locator_kind

Recommended values:

- `field`
  - direct record-field reference
  - examples:
    - `thumbnail_media_id`
    - `cover_media_id`
- `block_id`
  - stable structured-content block identifier plus a JSON Pointer relative to
    that block's `data`
  - examples:
    - `hero_01#/imageId`
    - `gallery_02#/pages/1/imageId`
- `path`
  - deterministic JSON Pointer path when stable block ids do not exist yet
  - examples:
    - `/blocks/2/data/media_id`
    - `/blocks/4/data/pages/1/imageId`
- `external_ref`
  - stable external/manual locator outside normal record ownership
  - examples:
    - `brochure-pack-2026`
    - `staff-downloads/tax-guide`

Rule:

- prefer `block_id` over `path` when the content engine provides stable ids
- for Nightfire, `block_id` means `<block-id>#<json-pointer-relative-to-data>`
- use `path` honestly as a rollout step instead of inventing fake block ids

## usage_role

Recommended values:

- `primary`
  - main display/hero/cover usage
- `attachment`
  - downloadable or associated file usage
- `embedded`
  - media embedded inside structured content
- `external`
  - explicitly retained for use outside the normal site/content graph
- `derived`
  - separately tracked derived/media-generated usage if you truly need it

Rule:

- use `derived` sparingly
- renditions/thumbnails usually do not need their own `media_usage` rows

## provenance_kind

Recommended values:

- `content_sync`
  - managed by normal content save/update flows
- `legacy_migration`
  - created or maintained by migration replay/import lanes
- `manual`
  - operator-managed/manual protection row
- `system_generated`
  - system-owned automation outside normal content editing

Rule:

- do not collapse migration/import rows into `content_sync`
- structured-content sync must only remove rows inside its own managed
  provenance scope

## Extension rule

If you need a new value:

1. check whether one of the existing values already fits
2. if not, promote the new value into Underlay contract/docs first
3. then use it in consumer apps

The shared goal is portable semantics, not app-local string dialects.
