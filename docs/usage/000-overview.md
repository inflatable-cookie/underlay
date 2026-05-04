# Underlay Usage Documentation

This is the user-facing documentation for building applications with Underlay.

## Reading Order

For **backend developers**:
1. [Backend Overview](./backend/000-overview.md)
2. [Rust Backend](./backend/040-rust-backend.md)
3. [Database & Migrations](./backend/050-database.md)
4. [API Handlers](./backend/070-api-handlers.md)
5. [Authentication](./backend/060-authentication.md)

For **frontend developers**:
1. [Frontend Overview](./frontend/000-overview.md)
2. [SvelteKit Setup](./frontend/100-sveltekit-setup.md)
3. [Routing](./frontend/110-routing.md)

For **template system** (admin UIs):
1. [Template System Overview](./templates/000-template-system-overview.md)
2. [Entity List Page](./templates/entity-list-page.md)
3. [Entity Detail Page](./templates/entity-detail-page.md)
4. [Entity Form Page](./templates/entity-form-page.md)

For **runtime helpers**:
1. [Runtime Overview](./runtime/000-overview.md)
2. [Client & Navigation](./runtime/080-client.md)
3. [Auth & Data](./runtime/095-navigation.md)

## Quick Reference

### Template System

The template system provides higher-order Svelte components for common admin page shapes:

```svelte
import { EntityListPage, EntityDetailPage } from "@decodelabs/underlay/templates";
```

**Three-level composition:**

- **Level 1 — Page Shells:** `EntityListPage`, `EntityDetailPage`, `EntityFormPage`
- **Level 2 — Sections:** `EntityList`, `EntityDetail`, `EntityForm` (reusable in tabs, dialogs)
- **Level 3 — Primitives:** Poodle components (`PageHeader`, `DataTable`, `DetailSection`, etc.)

### Developer Skill

Use `/underlay-template` to look up template documentation:

```
/underlay-template list      → entity-list-page docs
/underlay-template detail    → entity-detail-page docs
/underlay-template form      → entity-form-page docs
/underlay-template overview  → template system overview
```

## Deprecated

The old `docs/guides/` directory is deprecated. Content has been migrated here.
