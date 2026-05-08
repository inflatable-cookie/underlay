# Locator-aware media usage rollout recipe

Use this when a consumer app still has a field-only `media_usage` table and
needs to converge on Underlay's locator-aware contract.

## Goal

Move from:

- one coarse row per `owner_field`

To:

- one exact usage edge per reference
- explicit `content_kind`
- explicit `locator_kind`
- explicit `locator_key`
- explicit `usage_role`
- explicit `provenance_kind`

## Recommended order

1. Expand the schema first.

- rename `field` to `owner_field`
- add `content_kind`
- add `locator_kind`
- add `locator_key`
- add `usage_role`
- add `provenance_kind`
- backfill old rows as:
  - `content_kind = 'record_field'`
  - `locator_kind = 'field'`
  - `locator_key = owner_field`
  - `usage_role = 'primary'` or the closest known role
  - `provenance_kind = 'content_sync'` unless the row is known migration/manual state

2. Update repository and DTO surfaces.

- persist/read the full edge shape
- stop flattening structured content back to `owner_field` only
- expose the richer shape through admin/API contracts

3. Switch write-time sync to exact edges.

- plain field references still emit `locator_kind = 'field'`
- structured content emits:
  - `locator_kind = 'path'` first if block ids do not exist yet
  - `locator_kind = 'block_id'` later when stable ids exist

4. Switch audits to compare exact edge signatures.

- compare full usage edges inside the managed provenance scope
- report missing/stale exact locators, not only media-id set differences

5. Move migration replay rows onto explicit provenance.

- legacy replay/import rows should use `provenance_kind = 'legacy_migration'`
- do not let migration-created rows blend into `content_sync`

## Guardrails

- remove stale rows only inside the managed owner/provenance scope
- do not auto-remove manual or external rows during content sync
- if block ids do not exist yet, prefer honest `path` locators over fake block ids
- upgrading from `path` to `block_id` later should change extractor output, not the shared schema contract

## Locator choice

Use:

- `field`
  - plain record-field reference
- `block_id`
  - structured content with stable block ids
- `path`
  - structured content without stable block ids yet

Do not:

- emit fake block ids for content that only has array-position truth today
- keep coarse field-only usage rows once exact structured locators are
  available

End-to-end Nightfire recipe:

- [nightfire-save-sync-resolve-recipe.md](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/nightfire-save-sync-resolve-recipe.md)

## Template artifacts

- [media-usage-template.sql](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/media-usage-template.sql)
- [migrated-attachment-binding-template.sql](/Users/tom/Dev/projects/underlay/docs/guides/code/077-media-library/migrated-attachment-binding-template.sql)
