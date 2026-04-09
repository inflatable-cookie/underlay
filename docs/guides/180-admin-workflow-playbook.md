# 180 - Admin Workflow Playbook

A practical "start here" playbook for building admin functionality with
Underlay’s retained runtime/client surfaces plus Poodle-first UI composition.

## When To Use This

Use this guide when the request is:
- "Build admin interface for X"
- "Add admin flow for X"
- "Extend admin area with list/detail/edit"

## Default Implementation Flow

1. Define scope and entity model
- Confirm entity boundaries and whether this is standalone CRUD or nested under a parent.
- Identify whether ordering, trash, or relation picking is required.
- Set expected file/module boundaries up front using `docs/guides/020-project-structure.md` (anti-god-file policy).
- Define API profiles up front (`list`, `filter`, `details`) using `docs/guides/073-api-profiles-and-query-contract.md`.

2. Choose the base recipe
- Standalone CRUD: `docs/patterns/crud-admin-interface.md`
- Nested children: `docs/patterns/nested-entity-management.md`
- Live field validation: `docs/patterns/live-validation-endpoint.md`

Recipe ownership rule:
- treat the Underlay pattern as the backend/client/runtime delivery checklist
- treat the Poodle guides as the canonical visible Svelte composition layer

3. Add common Dairy-scale extensions
- Autonomous lists: `docs/patterns/autonomous-admin-list.md`
- Reorder: `docs/patterns/reorderable-collections.md`
- Trash lifecycle: `docs/patterns/trash-lifecycle.md`
- Context navigation: `docs/patterns/context-preserving-navigation.md`

4. Build backend -> client -> UI in order
- DB/data functions, then API handlers/routes, then TS client commands/types, then Svelte pages/components.
- Keep response/error shapes consistent with `docs/guides/070-api-handlers.md`.
- For the UI implementation layer, use the Poodle guides as the canonical
  source:
  - `Admin Feature Delivery Recipes`
  - `Page Shell And Admin Recipes`
  - `List And Filter Recipes`
  - `Dialog And Detail Recipes`

Default visible mapping:
- list pages -> `ListContainer` + `FilterToolbar`
- detail pages -> `PageHeader` + `MetaBar` + `DetailSection` / `DetailItem`
- create/edit pages -> Poodle fields/actions, plus `SpaFormShell` only when its retained SPA workflow still earns its place
- destructive flows -> `AlertDialog`
- diagnostics/error inspection -> stats + `DataTable` with inline expansion by
  default, not a separate first-class detail page unless the app needs a
  permalink

5. Apply app shell/runtime requirements (SPA admin)
- Follow `docs/guides/110-admin.md` for auth runtime setup, toasts, and Nightfire strategy configuration.
- Use the ACME admin route family as the concrete implementation reference:
  - the ACME admin project list/detail/edit routes
  - the ACME admin media detail route
  - the ACME admin user detail route
  - all in the separate `underlay-reference` repository

6. Verify with minimum testing matrix
- Use the per-recipe matrix in `docs/guides/185-recipe-map-and-testing-matrix.md`.

7. Sync docs and index
- Update `docs/patterns/000-index.md` and `docs/guides/200-project-sync.md` when introducing/altering patterns.

Do not add new visible UI examples to the Underlay pattern files unless they
explain a retained runtime/workflow seam. New generic visible examples belong
in the Poodle guides instead.

## Decision Shortcuts

- If the list is large: always use pagination controller.
- If users can reorder: add explicit reorder mode and endpoint.
- If deletes should be recoverable: implement trash + restore + purge.
- If create/edit comes from multiple entry points: use navigation context helpers.
- If related entities are selected often: use relation selector with inline create.

## Exit Criteria

A feature is complete when:
- All layers are implemented (DB/API/client/UI)
- Relevant recipe checklist items are complete
- Minimum tests pass
- Docs are synced using the checklist in `docs/guides/200-project-sync.md`
