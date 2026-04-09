# Underlay Patterns Catalogue

This catalogue is no longer the canonical home for reusable visible UI
implementation.

Use it to find:

- Poodle-first UI recipe entrypoints for visible composition
- Underlay-retained full-stack/runtime recipes
- mixed recipes where the backend/client/runtime contract still belongs in
  Underlay but the visible Svelte implementation should now be taken from
  Poodle

## How To Read This Catalogue

1. **Poodle-first UI recipes**
   Use Poodle guides first when the task is mostly about visible Svelte
   composition.
2. **Mixed Underlay recipes**
   Use these when the flow still needs real Underlay backend/client/runtime
   guidance, but follow Poodle for the visible UI layer.
3. **Underlay-retained recipes**
   Use these when the main value is backend, client, runtime, transport, or
   Nightfire integration rather than generic visible UI.

Related bridge guides:

- [180 - Admin Workflow Playbook](../guides/180-admin-workflow-playbook.md)
- [185 - Recipe Map and Testing Matrix](../guides/185-recipe-map-and-testing-matrix.md)

## Poodle-First UI Recipe Entry Points

Start with these Poodle guides when the task is mostly about page structure,
forms, lists, dialogs, media UI, or admin shell composition:

- `Form Layout And Field Recipes`
- `List And Filter Recipes`
- `Dialog And Detail Recipes`
- `Auth UI And Workflow Recipes`
- `Page Shell And Admin Recipes`
- `Media Library And Upload Recipes`
- `Admin Feature Delivery Recipes`
- `Admin App Shell Recipes`

Use the ACME apps in the separate `underlay-reference` repository as the
concrete visible implementation family.

## Mixed Recipes

These recipes still matter in Underlay, but only as full-stack or runtime
delivery guides. Their visible UI layer should now be taken from Poodle.

| Recipe | Use When | Ownership Model |
|--------|----------|-----------------|
| [CRUD Admin Interface](./crud-admin-interface.md) | Building a standard list/detail/edit admin feature | Underlay for DB/API/client/runtime order, Poodle for visible UI composition |
| [Nested Entity Management](./nested-entity-management.md) | Child entities managed within a parent route family | Underlay for parent/child contracts and navigation/runtime rules, Poodle for tabs/detail/list/dialog UI |
| [Autonomous Admin List](./autonomous-admin-list.md) | Self-contained paginated/filterable admin list surfaces | Underlay for pagination/auth/runtime wiring, Poodle for list/filter/chrome composition |
| [Reorderable Collections](./reorderable-collections.md) | Drag/drop reorder with persistence and conflict handling | Underlay for reorder workflow/controller/API, Poodle for list and mode UI |
| [Trash Lifecycle](./trash-lifecycle.md) | Soft-delete restore/purge workflows | Underlay for lifecycle semantics and commands, Poodle for trash-page composition |
| [Relation Selector with Inline Create](./relation-selector-inline-create.md) | Related-entity picking with inline create | Underlay for selector runtime/search/integration, Poodle for visible form/dialog shell |
| [Relation Selector with Drill-Down](./relation-selector-drilldown.md) | Hierarchical drill-down selection | Underlay for drill-down/search/runtime state, Poodle for visible shell composition |
| [Synced Hierarchical Selection](./synced-hierarchical-selection.md) | Module/section/area-style dependent selections | Underlay for synchronized state/runtime helpers, Poodle for visible field and layout posture |
| [Admin Ops Console](./admin-ops-console.md) | Jobs/errors/audit operational UIs | Underlay for operational data/runtime flows, Poodle for console/list/detail composition |
| [Media Upload Pipeline](./media-upload-pipeline.md) | Direct-to-blob upload with dedup/initiate/finalize | Underlay for upload lifecycle and commands, Poodle for browse/upload/picker UI |

## Underlay-Retained Recipes

These remain primarily Underlay-owned because the reusable value is not generic
visible UI.

| Recipe | Use When | Why It Stays Here |
|--------|----------|-------------------|
| [Live Validation Endpoint](./live-validation-endpoint.md) | Real-time field validation | API contract and validation result behavior are the main reusable value |
| [Context-Preserving Navigation](./context-preserving-navigation.md) | Preserving list/detail/edit context | Navigation/runtime contract is the main reusable value |
| [Delete Batch Cascades](./delete-batch-cascades.md) | Cascading soft deletes under one batch ID | DB and lifecycle semantics are the main reusable value |
| [Nightfire Integration](./nightfire-integration.md) | Wiring Nightfire strategies, renderers, and validation | Structured-content runtime and integration remain Underlay-led |

## Prompts

- "Build admin pages for X" → start with [CRUD Admin Interface](./crud-admin-interface.md) plus Poodle `Admin Feature Delivery Recipes`
- "Build nested CRUD under a parent detail route" → [Nested Entity Management](./nested-entity-management.md)
- "Build a paginated/filterable admin list" → [Autonomous Admin List](./autonomous-admin-list.md)
- "Add drag-and-drop reorder" → [Reorderable Collections](./reorderable-collections.md)
- "Add restore/purge trash flow" → [Trash Lifecycle](./trash-lifecycle.md)
- "Add inline relation create/search" → [Relation Selector with Inline Create](./relation-selector-inline-create.md)
- "Preserve back context across list/detail/edit" → [Context-Preserving Navigation](./context-preserving-navigation.md)
- "Wire Nightfire strategies and block rendering" → [Nightfire Integration](./nightfire-integration.md)

## Atomic Patterns

For lower-level techniques, use the guides directly instead of treating this
catalogue as a second UI system:

### Database Patterns

| Pattern | Description | Guide |
|---------|-------------|-------|
| ExistsCheck builder | Flexible existence checks | [050-database.md](../guides/050-database.md#existscheck-builder) |
| Composite uniqueness | slug + year, slug + parent_id | [050-database.md](../guides/050-database.md#existscheck-builder) |
| Nullable columns | `IS NOT DISTINCT FROM` for nullable | [050-database.md](../guides/050-database.md#existscheck-builder) |
| Soft-delete filtering | standard exclude/include rules | [050-database.md](../guides/050-database.md) |

### API Handler Patterns

| Pattern | Description | Guide |
|---------|-------------|-------|
| UUID path parsing | `parse_uuid_path_raw()` | [070-api-handlers.md](../guides/070-api-handlers.md#uuid-path-parameter-parsing) |
| Response helpers | `ok()`, `created()`, `list_ok()` | [070-api-handlers.md](../guides/070-api-handlers.md#response-helpers) |
| Validation results | `ValidationResult` result-body contract | [070-api-handlers.md](../guides/070-api-handlers.md#live-field-validation) |
| Pagination and query mapping | handler query conventions | [070-api-handlers.md](../guides/070-api-handlers.md#pagination) |

### Runtime, Client, and UI Ownership

| Need | Home |
|------|------|
| visible page/list/detail/dialog/media composition | Poodle guides |
| route/runtime/client orchestration | Underlay guides and runtime/client packages |
| auth workflow shells | `@decodelabs/underlay/patterns` |
| Nightfire editor/render/runtime integration | Underlay Nightfire guides and packages |

## Project Sync

To bring a project up to current patterns, use
[200-project-sync.md](../guides/200-project-sync.md).

## Next Task

Translate the remaining mixed admin recipes in batches rather than growing this
catalogue again. The next batch after the current wave should cover
`reorderable-collections` and `trash-lifecycle`, then the relation-selector and
ops/media recipes.
