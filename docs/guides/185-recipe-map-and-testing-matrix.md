# 185 - Recipe Map and Testing Matrix

This guide maps each composite recipe to real Acowtancy reference files and
defines minimum testing expectations.

Ownership rule:
- treat the recipe files in `docs/patterns/` as full-stack/runtime delivery guides
- treat Poodle as the canonical visible UI composition layer for any recipe that
  builds Svelte pages, forms, lists, dialogs, detail headers, or media shells

## Recipe To Acowtancy References

| Recipe | Dairy | Cattle Grid | Farmyard |
|---|---|---|---|
| CRUD Admin Interface | `dairy/src/routes/(app)/learning/pathways/*` | `cattle-grid/src/commands/learning/modules.ts` | `farmyard/crates/api/src/routes/admin/learning/*` |
| Nested Entity Management | `dairy/src/routes/(app)/learning/modules/[moduleId]/*` | `cattle-grid/src/commands/learning/modules.ts` | `farmyard/crates/api/src/routes/admin/learning/variants/*` |
| Live Validation Endpoint | `dairy/src/lib/forms/learning/*Form.svelte` | `cattle-grid/src/commands/learning/validation.ts` | `farmyard/crates/api/src/routes/admin/learning/validation.rs` |
| Autonomous Admin List | `dairy/src/lib/lists/ModulesList.svelte` | `cattle-grid/src/commands/learning/modules.ts` | `farmyard/crates/api/src/routes/admin/learning/*listing*` |
| Reorderable Collections | `dairy/src/lib/lists/ActivitiesList.svelte` | `cattle-grid/src/commands/learning/reorder.ts` | `farmyard/crates/api/src/routes/admin/learning/*/reorder/*.rs` |
| Trash Lifecycle | `dairy/src/routes/(app)/content/trash/+page.svelte` | `cattle-grid/src/commands/content/trash.ts` | `farmyard/crates/api/src/routes/admin/content/trash/*.rs` |
| Delete Batch Cascades | `dairy/src/routes/(app)/learning/trash/+page.svelte` | `cattle-grid/src/commands/learning-commands.ts` | `farmyard/crates/db/src/learning/soft_delete/simple/cascade/*.rs` |
| Media Upload Pipeline | `dairy/src/routes/(app)/media/upload/+page.svelte` | `cattle-grid/src/commands/media-commands.ts` | `farmyard/crates/api/src/routes/admin/media/uploads/*.rs` |
| Relation Selector with Inline Create | `dairy/src/lib/forms/learning/activity-form/*-selector.svelte` | `cattle-grid/src/commands/content-commands.ts` | `farmyard/crates/api/src/routes/admin/content/*/mutations*.rs` |
| Context-Preserving Navigation | `dairy/src/lib/cards/*ActionsMenu.svelte` | `@decodelabs/underlay/client/navigation` usage | N/A |
| Synced Hierarchical Selection | `dairy/src/routes/(app)/learning/outcomes/new/+page.svelte` | `cattle-grid/src/commands/learning/modules.ts` | `farmyard/crates/api/src/routes/admin/learning/*` |
| Admin Ops Console | `dairy/src/routes/(app)/system/*` | `cattle-grid/src/commands/platform-commands.ts` | `farmyard/crates/api/src/routes/admin/{platform,infra}.rs` |
| Nightfire Integration | `dairy/src/routes/(app)/+layout.svelte` | `cattle-grid/src/commands/nightfire-commands.ts` | `farmyard/crates/api/src/routes/admin/nightfire*.rs` |

## Minimum Testing Matrix (Per Recipe)

| Layer | Minimum expectation |
|---|---|
| DB | Unit/integration test for primary query/mutation and edge case |
| API | Handler test for success + validation error + not found/forbidden path |
| Client | Command test for path/query shape and envelope parsing |
| UI | Component/page test for load state + success path + error state |

## Additional Expectations By Feature Type

| Feature type | Additional minimum tests |
|---|---|
| Reorder | Duplicate ID payload rejection, scope mismatch, conflict behavior |
| Trash | Restore then list visibility, purge irreversibility |
| Upload | Dedup hit, initiate/finalise error handling, progress state transitions |
| Relation selector | Search results, inline create success, inline create duplicate handling |
| Ops console | Filter/query synchronization, status-dependent actions |

## Testing Depth Matrix (By Change Type)

Use this to scale testing effort based on risk.

| Change type | Required depth |
|---|---|
| UI-only visual/content tweak | UI test for rendering + one interaction path |
| Endpoint behavior change | API success/error tests + client command tests + impacted UI flow test |
| Data model / migration change | DB tests + API contract tests + regression tests for listing/filtering |
| Workflow change (reorder/trash/bulk ops) | End-to-end workflow coverage across DB/API/client/UI with failure-path assertions |

## Error Taxonomy and UI Handling

Use consistent error classes and UI behaviors:

| Error class | Typical code | UI behavior |
|---|---|---|
| Validation | `validation.*` | Inline field errors and non-blocking form message |
| Conflict | `conflict.*` | Sticky alert/toast with retry or refresh guidance |
| Authentication | `auth.*` | Re-auth flow / token refresh path |
| Authorization | `forbidden.*` | Access denied state and safe navigation out |
| Not Found | `not_found.*` | Empty/not-found page with return action |
| Infrastructure | `internal.*`, `upstream.*` | Generic failure state + retry affordance + logging hook |

## Admin List Performance Guardrails

Apply to all list-heavy admin pages:

- Default server pagination with explicit page size.
- Maximum page size cap enforced by backend.
- Debounced search input (typically 200-350ms).
- Filter/sort state synced to URL query params.
- Avoid duplicate fetches on mount and navigation transitions.
- Cache key includes pagination/filter/sort context.
- Render loading/skeleton states without blocking filter controls.

## Commands

Use the narrowest relevant checks before merge:

```bash
# Rust
cargo test
cargo clippy --all-targets --all-features -- -D warnings

# TypeScript/Svelte
bun test
bun check
bun lint
```
