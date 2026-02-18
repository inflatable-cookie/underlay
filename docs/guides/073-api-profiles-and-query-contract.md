# 073 - API Profiles and Unified Query Contract

This guide defines the standard API contract for Underlay-consuming apps.

It replaces projection-in-path patterns like `*-for-list` and `*-for-filter` with canonical resource routes plus explicit `profile` query params.

## Policy Summary

- Canonical list route: `GET /v1/{scope}/{domain}/{resource}`
- Canonical detail route: `GET /v1/{scope}/{domain}/{resource}/{id}`
- Use `profile` to select approved payload shapes:
  - list endpoints: `profile=list|filter`
  - detail endpoints: `profile=details` (optional; default is base record)
- Pagination/filtering/sorting are query-level concerns, never path suffixes.
- Profiles are enum-style and documented per resource. No arbitrary include strings.

## Why This Standard

- Removes naming churn (`with-counts`, `/paginated`, `-for-list`, `-for-filter`)
- Keeps command discovery predictable across domains/apps
- Supports list and filter payload optimization without path sprawl
- Enforces single-fetch detail badge counts via `profile=details`

## Route and Profile Contract

### Lists

Use one route per resource:

- `GET /v1/admin/learning/modules?profile=list`
- `GET /v1/admin/learning/modules?profile=filter`

Profile behavior:

- `profile=list`: list-card/table projection (labels, counts, status fields needed by the list view)
- `profile=filter`: lightweight selector projection (id + label + minimal metadata)

### Details

- `GET /v1/admin/learning/pathways/{pathwayId}` returns base record
- `GET /v1/admin/learning/pathways/{pathwayId}?profile=details` returns base record + defined detail enrichments (for example tab badge counts)

`profile=details` is the required pattern for any CRUD detail page that displays tab count badges.

## Shared Query Parameters (Lists)

All list endpoints must support:

- `limit`
- `cursor`
- `direction`
- `includeTotal`
- `sort`
- resource-specific filter params (for example `moduleId`, `pathwayId`, `query`, `isLive`)

Notes:

- Presence of pagination support is mandatory; callers can request a large limit when appropriate.
- Do not add path suffixes like `/paginated` to indicate pagination.

## Naming Rules

Allowed:

- `/v1/admin/{domain}/{resource}`
- `/v1/admin/{domain}/{resource}/{id}`
- explicit action subpaths for non-CRUD operations (`/reorder`, `/soft-delete`, `/restore`, etc.)

Disallowed:

- `/paginated`
- `with-counts`, `with-joins`, `flat`
- `-for-list`, `-for-filter` (post-migration target)

## Backend Implementation Pattern (Rust)

1. Parse `profile` with typed enums (`ListProfile`, `DetailProfile`).
2. Dispatch to profile-specific query builders/repositories.
3. Keep DTOs profile-specific but under one canonical route.
4. Ensure `profile=details` includes tab badge subqueries for all badge-bearing tabs on that detail view.
5. Keep list pagination envelope consistent with the pagination guide.

## TypeScript Client Pattern

1. Keep canonical command names (`listModulesAdmin`, `getPathwayAdmin`).
2. Accept typed `profile` in command params.
3. Reuse shared pagination/query param helpers.
4. Avoid per-profile path constants; only query params should vary.

## Frontend Pattern (Dairy/Cream)

1. Root/tab lists call the same resource list command, with explicit `profile=list`.
2. Lazy filter dropdowns call list command with `profile=filter`.
3. CRUD detail pages with badge tabs call detail command with `profile=details`.
4. Never do supplementary count-only fetches for tab badges.

## Migration Sequence

1. Add profile support to canonical resource routes in API.
2. Migrate client commands to profile params on canonical routes.
3. Migrate frontend callsites and remove legacy route usage.
4. Remove deprecated projection path variants.
5. Run endpoint naming, pagination, and wasteful-calls sweeps.

## Verification Checklist

- No routes include `/paginated`, `with-counts`, `-for-list`, `-for-filter`
- List endpoints accept and honor `profile=list|filter`
- Detail endpoints with badge tabs support `profile=details`
- Frontend detail views render badge counts from the main detail response
- Shared list pagination/filter/sort params work consistently across domains

## Related Docs

- [070-api-handlers.md](./070-api-handlers.md)
- [080-typescript-client.md](./080-typescript-client.md)
- [093-pagination.md](./093-pagination.md)
- [097-autonomous-list-components.md](./097-autonomous-list-components.md)
- [100-frontend-web.md](./100-frontend-web.md)
- [021-wasteful-endpoint-calls-sweep.md](../sweeps/021-wasteful-endpoint-calls-sweep.md)
- [022-api-endpoint-naming-convention-sweep.md](../sweeps/022-api-endpoint-naming-convention-sweep.md)
