# Underlay Patterns Catalogue

Quick reference for implementation patterns. Use this to find the right approach for common tasks.

## How to Use This Catalogue

1. **Composite Recipes** - Full system implementations. Start here for "build X" tasks.
2. **Atomic Patterns** - Individual techniques. Use when you need a specific helper.
3. **Playbook + Testing Matrix** - Use the implementation flow and minimum tests in:
   - [180-admin-workflow-playbook.md](../guides/180-admin-workflow-playbook.md)
   - [185-recipe-map-and-testing-matrix.md](../guides/185-recipe-map-and-testing-matrix.md)

---

## Composite Recipes

Complete, repeatable implementations covering backend → client → frontend.

| Recipe | Use When | Guide |
|--------|----------|-------|
| [New Project Bootstrap Prompt](./new-project-bootstrap-prompt.md) | Starting a fresh Underlay-based project and bootstrapping the skeleton | Copy/paste prompt |
| [CRUD Admin Interface](./crud-admin-interface.md) | Building complete create/read/update/delete for an entity | Baseline checklist + Dairy-scale extension phases |
| [Live Validation Endpoint](./live-validation-endpoint.md) | Real-time field validation (e.g., slug availability) | 3-phase checklist |
| [Nested Entity Management](./nested-entity-management.md) | Child entities within a parent (tabs, scoped lists) | 7-phase checklist |
| [Autonomous Admin List](./autonomous-admin-list.md) | Building reusable list surfaces with pagination, filtering, and batch actions | 7-phase checklist |
| [Reorderable Collections](./reorderable-collections.md) | Adding drag/drop reorder with scoped API persistence | 6-phase checklist |
| [Trash Lifecycle](./trash-lifecycle.md) | Implementing soft-delete restore/purge workflows | 5-phase checklist |
| [Delete Batch Cascades](./delete-batch-cascades.md) | Cascading soft deletes tracked under a single batch ID | 5-phase checklist |
| [Media Upload Pipeline](./media-upload-pipeline.md) | Direct-to-blob upload with dedup, initiate, and finalise | 5-phase checklist |
| [Relation Selector with Inline Create](./relation-selector-inline-create.md) | Selecting related entities and creating missing ones inline | 5-phase checklist |
| [Context-Preserving Navigation](./context-preserving-navigation.md) | Keeping back/navigation context across list/detail/edit flows | 5-phase checklist |
| [Synced Hierarchical Selection](./synced-hierarchical-selection.md) | Managing module/section/area style dependent selections | 5-phase checklist |
| [Admin Ops Console](./admin-ops-console.md) | Building jobs/scheduled/errors/audit operational interfaces | 5-phase checklist |
| [Nightfire Integration](./nightfire-integration.md) | Configuring strategies, renderers, and editor/render pipeline | 5-phase checklist |

### Quick Prompts

These prompts will trigger the appropriate recipe:

- "Build the CRUD interface for Bundles" → [CRUD Admin Interface](./crud-admin-interface.md)
- "Build a Dairy-style admin area for X" → [CRUD Admin Interface](./crud-admin-interface.md#dairy-scale-extension-checklist-required-for-complex-admin-areas)
- "Bootstrap a new Underlay project" → [New Project Bootstrap Prompt](./new-project-bootstrap-prompt.md)
- "Add live slug validation for modules" → [Live Validation Endpoint](./live-validation-endpoint.md)
- "Build the Variants tab for Modules" → [Nested Entity Management](./nested-entity-management.md)
- "Build a reusable paginated list for X" → [Autonomous Admin List](./autonomous-admin-list.md)
- "Add drag-and-drop reordering for X" → [Reorderable Collections](./reorderable-collections.md)
- "Add trash/restore/purge for X" → [Trash Lifecycle](./trash-lifecycle.md)
- "Implement cascade soft delete batches for X" → [Delete Batch Cascades](./delete-batch-cascades.md)
- "Implement media upload flow for X" → [Media Upload Pipeline](./media-upload-pipeline.md)
- "Add relation picker with inline create for X" → [Relation Selector with Inline Create](./relation-selector-inline-create.md)
- "Preserve back context across list/detail/edit" → [Context-Preserving Navigation](./context-preserving-navigation.md)
- "Build dependent selections (module/section/area)" → [Synced Hierarchical Selection](./synced-hierarchical-selection.md)
- "Build admin jobs/schedules/errors/audit pages" → [Admin Ops Console](./admin-ops-console.md)
- "Wire Nightfire strategies and custom block renderers" → [Nightfire Integration](./nightfire-integration.md)
- "Build AI routing diagnostics/cost/parity admin page" → [176-ai-runtime-routing.md](../guides/176-ai-runtime-routing.md)
- "Create admin pages for X" → [CRUD Admin Interface](./crud-admin-interface.md)
- "Add real-time validation for X field" → [Live Validation Endpoint](./live-validation-endpoint.md)
- "Manage X within Y" → [Nested Entity Management](./nested-entity-management.md)

---

## Atomic Patterns

Individual techniques referenced by the recipes above.

### Database Patterns

| Pattern | Description | Guide |
|---------|-------------|-------|
| ExistsCheck builder | Flexible existence checks | [050-database.md#existscheck-builder](../guides/050-database.md#existscheck-builder) |
| Composite uniqueness | slug + year, slug + parent_id | [050-database.md#existscheck-builder](../guides/050-database.md#existscheck-builder) |
| Nullable columns | `IS NOT DISTINCT FROM` for nullable | [050-database.md#existscheck-builder](../guides/050-database.md#existscheck-builder) |
| Tables without soft-delete | `.include_deleted()` | [050-database.md#including-deleted-records](../guides/050-database.md#including-deleted-records) |
| Docs-first schema | Document before migrate | [050-database.md#docs-first-schema-development](../guides/050-database.md#docs-first-schema-development) |
| Schema qualification | Always use `schema.table` | [050-database.md#critical-schema-qualification-in-migrations](../guides/050-database.md#critical-schema-qualification-in-migrations) |

### API Handler Patterns

| Pattern | Description | Guide |
|---------|-------------|-------|
| UUID path parsing | `parse_uuid_path_raw()` | [070-api-handlers.md#uuid-path-parameter-parsing](../guides/070-api-handlers.md#uuid-path-parameter-parsing) |
| Response helpers | `ok()`, `created()`, `list_ok()` | [070-api-handlers.md#response-helpers](../guides/070-api-handlers.md#response-helpers) |
| Error responses | `ApiError` / `ApiResult<T>` (canonical) | [070-api-handlers.md#errors](../guides/070-api-handlers.md#errors) |
| JSON naming | Canonical `snake_case` wire contracts | [071-json-naming.md](../guides/071-json-naming.md) |
| Pagination | `PaginationParams`, `Paginated<T>` | [070-api-handlers.md#pagination](../guides/070-api-handlers.md#pagination) |
| Field mapping | `FieldMapping` for sort/filter | [070-api-handlers.md#query-field-mapping](../guides/070-api-handlers.md#query-field-mapping) |

### Validation Patterns

| Pattern | Description | Guide |
|---------|-------------|-------|
| Live field validation | `ValidationResult` (200 OK) | [070-api-handlers.md#live-field-validation](../guides/070-api-handlers.md#live-field-validation) |
| UUID for validation | `parse_uuid_for_validation()` | [070-api-handlers.md#live-field-validation](../guides/070-api-handlers.md#live-field-validation) |
| Validator errors | `validation_to_app_error()` | [070-api-handlers.md#validator-crate-integration](../guides/070-api-handlers.md#validator-crate-integration) |
| Nightfire validation | `nightfire_validation_to_app_error()` | [070-api-handlers.md#nightfire-content-validation](../guides/070-api-handlers.md#nightfire-content-validation) |

### Authentication & Authorization

| Pattern | Description | Guide |
|---------|-------------|-------|
| Request context | `RequestContext`, `AuthenticatedContext` | [070-api-handlers.md#request-context](../guides/070-api-handlers.md#request-context) |
| Auth middleware | JWT validation, user extraction | [060-authentication.md](../guides/060-authentication.md) |
| Role-based access | Permission checks in handlers | [067-authorization.md](../guides/067-authorization.md) |

### Frontend Patterns

| Pattern | Description | Guide |
|---------|-------------|-------|
| Load functions | SvelteKit data loading | [100-frontend-web.md](../guides/100-frontend-web.md) |
| Form actions | SvelteKit form handling | [100-frontend-web.md](../guides/100-frontend-web.md) |
| Admin shell layout | Left nav + user menu + right context panel | [110-admin.md#app-shell-layout-nav--user-menu--context-panel](../guides/110-admin.md#app-shell-layout-nav--user-menu--context-panel) |
| DataTable | Sortable, filterable tables | Component library |
| StatusBadge | Live/draft badges | Component library |
| Tabs | Tabbed navigation | Component library |
| Field | Consistent form inputs | Component library |

### AI Runtime Patterns

| Pattern | Description | Guide |
|---------|-------------|-------|
| Provider-agnostic LLM boundary | `LlmClient`, registry, route candidates, OpenAI-compatible transport | [176-ai-runtime-routing.md](../guides/176-ai-runtime-routing.md) |
| Admin AI routing dashboard | `AiRoutingAdmin` + `AiRoutingOpsSource` for turnkey diagnostics/cost/parity ops UI | [176-ai-runtime-routing.md](../guides/176-ai-runtime-routing.md) |

### Configuration Patterns

| Pattern | Description | Guide |
|---------|-------------|-------|
| Typed config + env boundary | Keep app behavior config in typed structs; reserve `.env` for secrets/runtime-env values | [120-configuration.md](../guides/120-configuration.md) |
| Layered config loading | Defaults -> `config/default.toml` -> `config/local.toml` -> allowlisted env overrides | [120-configuration.md](../guides/120-configuration.md) |
| Config migration checklist | Repeatable per-app migration flow from env-heavy setup to typed config | [120-configuration.md](../guides/120-configuration.md) |

### Internal Development Patterns

Patterns for working on Underlay itself (not for consuming apps).

| Pattern | Description | Guide |
|---------|-------------|-------|
| Test file extraction | `#[cfg(test)] #[path = "lib_tests.rs"] mod tests;` | [041-rust-module-splitting.md](../guides/041-rust-module-splitting.md) |
| Row type extraction | `pub(crate)` types in `postgres_rows.rs` | [041-rust-module-splitting.md](../guides/041-rust-module-splitting.md) |
| Feature-gated extraction | Separate module per feature (e.g., `google.rs`, `hibp.rs`) | [041-rust-module-splitting.md](../guides/041-rust-module-splitting.md) |
| Re-export preservation | `pub use submodule::Type;` in `lib.rs` after extraction | [041-rust-module-splitting.md](../guides/041-rust-module-splitting.md) |
| File length limits | Warn >500 lines, fail >900 lines | [041-rust-module-splitting.md](../guides/041-rust-module-splitting.md) |

### Quick Prompts (Internal)

- "This file is too long" → [Module Splitting Guide](../guides/041-rust-module-splitting.md)
- "Extract tests from X" → Test file extraction pattern above
- "Split this module" → [Module Splitting Guide](../guides/041-rust-module-splitting.md)

---

## Project Sync

To bring a project up to current patterns: [200-project-sync.md](../guides/200-project-sync.md)

---

## Adding New Patterns

When you discover a new repeatable pattern:

1. **Atomic pattern**: Add to relevant guide and update this index
2. **Composite recipe**: Create new file in `docs/patterns/` and add to index
3. **Update sync guide**: Add migration steps to [200-project-sync.md](../guides/200-project-sync.md)
