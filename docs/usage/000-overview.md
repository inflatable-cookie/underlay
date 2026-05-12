# Underlay Usage Documentation

This is the user-facing documentation for building applications with Underlay.

## Reading Order

For **backend developers**:
1. [Architecture Overview](../guides/000-overview.md)
2. [Rust Backend](../guides/040-rust-backend.md)
3. [Database & Migrations](../guides/050-database.md)
4. [API Handlers](../guides/070-api-handlers.md)
5. [Authentication](../guides/060-authentication.md)

For **frontend developers**:
1. [Frontend Web](../guides/100-frontend-web.md)
2. [Admin Frontend](../guides/110-admin.md)
3. [Admin/Front Separation](../guides/072-admin-front-separation.md)

For **template system** (admin UIs):
1. [Template System Overview](./templates/000-template-system-overview.md)
2. [Entity List Page](./templates/entity-list-page.md)
3. [Entity Detail Page](./templates/entity-detail-page.md)
4. [Entity Form Page](./templates/entity-form-page.md)

For **migration and state operations**:
1. [Migration State Layout And Effigy](./migration/000-state-layout-and-effigy.md)

For **runtime helpers**:
1. [TypeScript Client](../guides/080-typescript-client.md)
2. [Navigation Context](../guides/095-navigation-context.md)
3. [Session Management](../guides/065-session-management.md)

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

### Developer Skills

Skills are installable via the `npx skills` CLI. After cloning the Underlay repo,
install the build skill locally:

```bash
npx skills add ./underlay --skill underlay-build
```

Or install from the remote repository:

```bash
npx skills add inflatable-cookie/underlay --skill underlay-build
```

Use `/underlay-template` to look up template documentation:

```
/underlay-template list      → entity-list-page docs
/underlay-template detail    → entity-detail-page docs
/underlay-template form      → entity-form-page docs
/underlay-template overview  → template system overview
```

Use `/underlay-build` to stay on-contract when building or maintaining
Underlay-based applications:

```
/underlay-build admin-list   → list page templates + guardrails
/underlay-build admin-detail → detail page templates + guardrails
/underlay-build admin-form   → form page templates + guardrails
/underlay-build admin-api    → admin API shapes and query contracts
/underlay-build bootstrap    → new project bootstrap workflow
/underlay-build contract     → contract index by system area
/underlay-build patterns     → pattern catalog
/underlay-build check        → audit an app/page for contract compliance
/underlay-build upgrade      → upgrade compatibility guidance
```

## Deprecated

The old `docs/guides/` directory is deprecated. Content has been migrated here.
