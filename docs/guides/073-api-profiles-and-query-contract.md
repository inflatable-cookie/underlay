# 073 - API Profiles and Resource Page Query Contract

This guide defines the standard API contract for Underlay-consuming apps.

It replaces projection-in-path patterns like `*-for-list` and `*-for-filter`
with canonical resource routes plus explicit query-level projection where a
resource truly needs it.

The durable contract lives in:

- [../contracts/115-admin-resource-api-shapes.md](../contracts/115-admin-resource-api-shapes.md)
- [../contracts/020-http-transport-and-server-boundary.md](../contracts/020-http-transport-and-server-boundary.md)
- [../contracts/116-canonical-collection-routes-and-query-profiles.md](../contracts/116-canonical-collection-routes-and-query-profiles.md)

## Policy Summary

- Canonical list route: `GET /v1/{scope}/{domain}/{resource}`
- Canonical detail route: `GET /v1/{scope}/{domain}/{resource}/{id}`
- Canonical child-list route: `GET /v1/{scope}/{domain}/{parent}/{parentId}/{child}`
- Use `profile` only for approved projection variants:
  - list endpoints may support `profile=list|filter|list-config`
  - detail endpoints may support `profile=details`
- Use `variant` only for named baseline list queries such as `pending`,
  `marked`, or `all`.
- Pagination/filtering/sorting are query-level concerns, never path suffixes.
- Profiles are enum-style and documented per resource. No arbitrary include strings.
- Variants are enum-style and documented per list endpoint. No arbitrary SQL,
  status, or filter pass-through.
- Root list pages and detail-tab child collections use the same paged list envelope.
- Base detail reads and write returns use the same single-record envelope.
- Resource-backed selectors and filter dropdowns should use the same canonical
  resource route with `profile=filter`.
- Utility collections like suggestions and other non-resource helpers are not
  forced onto the paged envelope unless they are actual page shells.

## Why This Standard

- Removes naming churn (`with-counts`, `/paginated`, `-for-list`, `-for-filter`)
- Keeps command discovery predictable across domains/apps
- Supports list and filter payload optimization without path sprawl
- Keeps templates from needing per-site adapter logic for page-shaped DTOs
- Keeps child tab collections on real collection routes instead of bespoke tab payloads

## Route and Profile Contract

### Lists

Use one route per resource:

- `GET /v1/admin/learning/modules?profile=list`
- `GET /v1/admin/learning/modules?profile=filter`

That same route should serve:

- page lists
- selector shells
- filter dropdowns
- other resource-backed helper reads

Profile behavior, when the resource supports it:

- `profile=list`: list-card/table projection (labels, counts, status fields needed by the list view)
- `profile=filter`: lightweight selector projection (id + label + minimal metadata)
- `profile=list-config`: list capabilities payload containing query variants
  and filter definitions for the list surface

Variant behavior, when the resource supports it:

- `variant=pending`: named server-understood baseline query
- `variant=marked`: another named baseline query
- `variant=all`: explicit "all records" baseline query when the product needs it

Variants are not filters. Filters, sorting, and pagination apply after the
active variant baseline.

Canonical paged list wire shape:

```json
{
  "data": [],
  "total": 0,
  "has_more": false
}
```

### Details

- `GET /v1/admin/learning/pathways/{pathwayId}` returns base record
- `GET /v1/admin/learning/pathways/{pathwayId}?profile=details` may return base record + approved detail enrichments

Canonical detail wire shape:

```json
{
  "data": {}
}
```

If a detail page renders a child collection in a tab, that tab should use its
own child collection route with the canonical paged list envelope. Do not embed
tab collections inside the detail payload just because they render in a tab.

## Shared Query Parameters (Lists)

All list endpoints must support:

- `page`
- `limit`
- `sort`
- resource-specific filter params through `filter[...]`

List endpoints may support:

- `variant`

Notes:

- Presence of pagination support is mandatory for page-shaped list surfaces.
- Selector/filter consumers may use the same route with smaller `limit` values
  and simply ignore `total` / `hasMore`.
- Do not add path suffixes like `/paginated` to indicate pagination.
- Use the shared sort/filter query vocabulary from the transport contract.
- Do not encode a named product view as `filter[status]=...` when the API and
  UI need a stable variant contract.

Canonical list capabilities wire shape:

```json
{
  "data": {
    "default_variant_id": "pending",
    "variants": [
      { "id": "pending", "label": "Pending", "count": 42, "is_default": true },
      { "id": "marked", "label": "Marked", "count": 310 },
      { "id": "void", "label": "Void", "count": 6 },
      { "id": "all", "label": "All", "count": 358 }
    ],
    "filters": [
      { "id": "marker_id", "type": "select", "label": "Marker", "variants": ["pending", "marked"] },
      { "id": "search", "type": "search", "label": "Search" },
      { "id": "sort", "type": "sort", "label": "Sort" }
    ]
  }
}
```

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
2. Parse `variant` with a typed per-endpoint enum when the endpoint supports
   named base queries.
3. Dispatch to profile-specific query builders/repositories.
4. Apply the selected variant baseline before filters, sorting, and pagination.
5. Keep DTOs profile-specific but under one canonical route.
6. Keep detail payloads focused on the record plus any tiny summary/count
   enrichments that genuinely belong to the main detail view.
7. Keep child tab collections on canonical child-list routes.
8. Keep the paged list envelope consistent with the transport and API-shape
   contracts.

## TypeScript Client Pattern

1. Keep canonical command names (`listModulesAdmin`, `getPathwayAdmin`).
2. Accept typed `profile` and endpoint-specific `variant` in command params.
3. Reuse shared pagination/query param helpers.
4. Normalize `has_more` to `hasMore` only at the client boundary if desired.
5. Avoid per-profile path constants; only query params should vary.
6. Prefer one canonical route family per resource. Separate exported wrappers
   like `*ForList*` / `*ForFilter*` are acceptable when they are thin typed
   wrappers over that same route family.

## Frontend Pattern (Dairy/Cream)

1. Root lists and tab child collections both use the canonical paged list shape.
2. Lazy filter dropdowns should use `profile=filter` on the same canonical list route.
3. Named list views should use `variant`, not hidden filter defaults.
4. CRUD detail pages load one detail DTO; tab collections load their own child-list routes.
5. If tabs show badges, prefer main-detail summary fields over count-only side routes.
6. Selector callers should unwrap `response.data` locally rather than requiring
   helper-specific list routes.

Reference loader handshake:

- [code/073-api-profiles-and-query-contract/entity-list-page-paged-loader.ts](./code/073-api-profiles-and-query-contract/entity-list-page-paged-loader.ts)
- [code/073-api-profiles-and-query-contract/entity-detail-tab-paged-list.ts](./code/073-api-profiles-and-query-contract/entity-detail-tab-paged-list.ts)
- use `toPagedListResult(...)` from `@decodelabs/underlay/templates` as the
  canonical client-to-template bridge

## Migration Sequence

1. Add profile support to canonical resource routes in API.
2. Add typed variant support to list endpoints that have named baseline views.
3. Normalize root list and child-tab list endpoints onto the canonical paged list envelope.
4. Migrate client commands to canonical routes and typed profile / variant
   params where needed.
5. Migrate frontend callsites and remove legacy route usage.
6. Remove deprecated projection path variants.
7. Collapse duplicated route families first. Only collapse wrapper commands
   when doing so actually improves clarity rather than weakening typing.
8. Run endpoint naming, pagination, and wasteful-calls sweeps.

## Verification Checklist

- No routes include `/paginated`, `with-counts`, `-for-list`, `-for-filter`
- Root lists and child-tab lists return `data + total + has_more`
- Detail endpoints return `{ data: record }`
- Utility/helper collections still using `{ data: [] }` are intentionally
  bounded, non-resource helpers rather than alternate resource list routes
- List endpoints accept and honor `profile=list|filter` only when needed
- List endpoints with named base views accept and honor typed `variant` values
- Selector/filter callers for the same resource family use the canonical route
  and query/profile variation rather than helper-specific paths
- Badge-bearing detail views keep counts on the main detail response rather
  than count-only side routes
- Shared list pagination/filter/sort params work consistently across domains

## Related Docs

- [070-api-handlers.md](./070-api-handlers.md)
- [080-typescript-client.md](./080-typescript-client.md)
- [093-pagination.md](./093-pagination.md)
- [097-autonomous-list-components.md](./097-autonomous-list-components.md)
- [100-frontend-web.md](./100-frontend-web.md)
- [021-wasteful-endpoint-calls-sweep.md](../sweeps/021-wasteful-endpoint-calls-sweep.md)
- [022-api-endpoint-naming-convention-sweep.md](../sweeps/022-api-endpoint-naming-convention-sweep.md)
