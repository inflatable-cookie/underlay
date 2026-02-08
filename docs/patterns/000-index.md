# Underlay Patterns Catalogue

Quick reference for implementation patterns. Use this to find the right approach for common tasks.

## How to Use This Catalogue

1. **Composite Recipes** - Full system implementations. Start here for "build X" tasks.
2. **Atomic Patterns** - Individual techniques. Use when you need a specific helper.

---

## Composite Recipes

Complete, repeatable implementations covering backend → client → frontend.

| Recipe | Use When | Guide |
|--------|----------|-------|
| [New Project Bootstrap Prompt](./new-project-bootstrap-prompt.md) | Starting a fresh Underlay-based project and bootstrapping the skeleton | Copy/paste prompt |
| [CRUD Admin Interface](./crud-admin-interface.md) | Building complete create/read/update/delete for an entity | 8-phase checklist |
| [Live Validation Endpoint](./live-validation-endpoint.md) | Real-time field validation (e.g., slug availability) | 3-phase checklist |
| [Nested Entity Management](./nested-entity-management.md) | Child entities within a parent (tabs, scoped lists) | 7-phase checklist |

### Quick Prompts

These prompts will trigger the appropriate recipe:

- "Build the CRUD interface for Bundles" → [CRUD Admin Interface](./crud-admin-interface.md)
- "Bootstrap a new Underlay project" → [New Project Bootstrap Prompt](./new-project-bootstrap-prompt.md)
- "Add live slug validation for modules" → [Live Validation Endpoint](./live-validation-endpoint.md)
- "Build the Variants tab for Modules" → [Nested Entity Management](./nested-entity-management.md)
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
| Load functions | SvelteKit data loading | [110-sveltekit-frontend.md](../guides/110-sveltekit-frontend.md) |
| Form actions | SvelteKit form handling | [110-sveltekit-frontend.md](../guides/110-sveltekit-frontend.md) |
| Admin shell layout | Left nav + user menu + right context panel | [110-admin.md#app-shell-layout-nav--user-menu--context-panel](../guides/110-admin.md#app-shell-layout-nav--user-menu--context-panel) |
| DataTable | Sortable, filterable tables | Component library |
| StatusPill | Live/draft badges | Component library |
| Tabs | Tabbed navigation | Component library |
| FormField | Consistent form inputs | Component library |

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
