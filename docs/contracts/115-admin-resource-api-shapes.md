# Contract: Admin Resource API Shapes

Status: active
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`, `110-admin-template-system.md`

## Purpose

Define the shared API shape contract Underlay owns for admin resource pages that
use the template system:

- root list pages
- detail pages
- child collections rendered inside detail tabs
- optional list/detail profile variants when a resource needs multiple approved
  projections

This contract does not replace the lower transport contract in `020`, and it
does not reopen app-specific route trees. It fixes the higher-level resource
page shapes that `EntityListPage`, `EntityDetailPage`, and child tab lists
expect across consuming apps.

## Sources of Truth

Primary shared sources:

- [`docs/contracts/020-http-transport-and-server-boundary.md`](/Users/tom/Dev/projects/underlay/docs/contracts/020-http-transport-and-server-boundary.md)
- [`docs/contracts/110-admin-template-system.md`](/Users/tom/Dev/projects/underlay/docs/contracts/110-admin-template-system.md)
- [`ts/src/templates/template.types.ts`](/Users/tom/Dev/projects/underlay/ts/src/templates/template.types.ts)
- [`ts/src/client/types.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/types.ts)
- [`ts/src/templates/EntityList.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityList.svelte)
- [`ts/src/templates/EntityListPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityListPage.svelte)
- [`ts/src/templates/EntityDetailPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityDetailPage.svelte)
- [`docs/guides/073-api-profiles-and-query-contract.md`](/Users/tom/Dev/projects/underlay/docs/guides/073-api-profiles-and-query-contract.md)

Reference consumer evidence:

- [`underlay-reference/acme-client/src/types/common-types.ts`](/Users/tom/Dev/projects/underlay-reference/acme-client/src/types/common-types.ts)
- [`underlay-reference/acme-client/src/commands/admin/project-commands.ts`](/Users/tom/Dev/projects/underlay-reference/acme-client/src/commands/admin/project-commands.ts)
- [`underlay-reference/acme-client/src/commands/admin/task-commands.ts`](/Users/tom/Dev/projects/underlay-reference/acme-client/src/commands/admin/task-commands.ts)
- [`underlay-reference/acme-client/src/commands/media-commands.ts`](/Users/tom/Dev/projects/underlay-reference/acme-client/src/commands/media-commands.ts)
- [`underlay-reference/acme-admin/src/lib/lists/ProjectsListPage.svelte`](/Users/tom/Dev/projects/underlay-reference/acme-admin/src/lib/lists/ProjectsListPage.svelte)
- [`underlay-reference/acme-admin/src/lib/lists/TasksListPage.svelte`](/Users/tom/Dev/projects/underlay-reference/acme-admin/src/lib/lists/TasksListPage.svelte)
- [`underlay-reference/acme-admin/src/routes/(app)/projects/[projectId]/+page.svelte`](</Users/tom/Dev/projects/underlay-reference/acme-admin/src/routes/(app)/projects/[projectId]/+page.svelte>)
- [`underlay-reference/acme-admin/src/routes/(app)/projects/[projectId]/tasks/[taskId]/+page.svelte`](</Users/tom/Dev/projects/underlay-reference/acme-admin/src/routes/(app)/projects/[projectId]/tasks/[taskId]/+page.svelte>)

If these diverge, the shared template contract plus the reference consumer shape
win. Older guides must catch up.

## Contract Goal

Underlay should give consuming apps one predictable resource-page API shape:

- root lists and tab lists use the same paged list envelope
- detail pages use one base record envelope
- child collections inside detail tabs behave like ordinary resource lists, not
  bespoke tab payloads
- profiles are optional and explicit when a resource truly needs multiple
  approved projections

The goal is to stop every site inventing a different page-shaped DTO for the
same UI problems.

## Scope Boundary

This contract is intentionally about admin page shells, not every collection in
an app.

In scope:

- admin root list pages
- admin detail pages
- child collections rendered as detail tabs or inline list modules on those
  detail pages

Out of scope:

- auth/account helper collections such as passkeys or current-user sessions
- front-app/public collections unless an app explicitly adopts this same page
  shape there
- relation selectors, suggestions, assignment helpers, and other bounded
  workflow collections that are not page shells

If a collection is not a page-shaped admin resource surface, `115` does not
force it onto the paged envelope.

## Shared Boundary

### Canonical route families

Resource routes should be canonical and boring.

Allowed:

- `GET /v1/admin/{resource}`
- `GET /v1/admin/{resource}/{id}`
- `GET /v1/admin/{parent}/{parentId}/{child}`
- explicit action subpaths for non-CRUD operations such as `/reorder`,
  `/restore`, `:batch-delete`

Rules:

- pagination, filtering, sorting, and projection are query-level concerns
- child collections rendered inside detail tabs should still use their own
  canonical child list routes
- tab content must not require bespoke `/tab`, `-for-list`, `-for-filter`, or
  `/paginated` route families

### Paged list envelope

The canonical wire shape for paged resource collections is:

```json
{
  "data": [],
  "total": 0,
  "has_more": false
}
```

Meaning:

- `data`: already-projected list items
- `total`: total matching rows for the current filter scope
- `has_more`: whether another page exists after the current page

Rules:

- this shape applies to root list pages and child collection tabs alike
- `data` is flat; do not wrap it under `items`, `rows`, `results`, `list`, or
  tab-specific nesting
- list items should already be list/card/table projections, not full detail DTOs
- TypeScript clients may normalize `has_more` into `hasMore`, but the wire
  contract stays `has_more`

### Simple bounded list envelope

For bounded collections that are not paginated page surfaces, the canonical wire
shape remains:

```json
{
  "data": []
}
```

Use this only when the collection is genuinely small and caller-owned:

- small suggestion lists
- bounded relation helpers
- other non-page list surfaces where pagination is unnecessary

Do not use `ListResponse<T>` for:

- root `EntityListPage` surfaces
- detail-tab child collections
- admin trash/library pages that are real page-level browsing surfaces
- any list that needs pagination, filtering, batch actions, or reorder behavior

### Detail envelope

The canonical wire shape for one detail record is:

```json
{
  "data": {}
}
```

Rules:

- base detail endpoints return one typed record under `data`
- create and update operations that return the saved record use this same shape
- edit-capable admin detail/update routes should emit `ETag` and honor
  `If-Match` per the transport contract
- the detail record should contain scalar/read-only fields and any directly
  rendered structured content needed for non-list tabs

### Detail-tab child collection shape

Tabs that show child collections should use normal child list routes and the
same paged list envelope as root list pages.

Pattern:

- project detail page renders a tasks tab
- tasks tab calls `GET /v1/admin/projects/{projectId}/tasks?page=...&limit=...`
- response shape is the canonical paged list envelope

Rules:

- tab collection endpoints should behave like ordinary lists with query params,
  filters, batch actions, and reorder support where relevant
- do not return tab collections embedded inside the detail record just because
  they render in a tab
- do not invent special tab-only envelopes like
  `{ "tasks": { "items": [], "total": 0 } }`

### Detail-tab summary and badge data

If a detail page shows badge counts or summary metrics for tabs, that summary
belongs to the main detail fetch, not to count-only side routes.

Recommended shape:

- app-owned summary fields on the detail DTO, or
- one small typed summary object on the detail DTO

Rules:

- badge counts should be simple scalars or small summaries, not embedded list
  payloads
- do not create `*-count`, `*-summary`, or `*-for-badge` endpoints just to feed
  tab labels
- if a page does not show badge counts, no summary block is required

### Profile variants

Profiles are optional and explicit.

Allowed examples:

- `profile=list`
- `profile=filter`
- `profile=details`

Rules:

- use profiles only when one canonical route genuinely needs multiple approved
  projections
- keep profiles enum-like and documented; never accept arbitrary include strings
- `profile=list` and `profile=filter` still return the same canonical list
  envelope; only the row projection changes
- `profile=details` still returns the same canonical detail envelope; only the
  detail payload enrichments change
- profiles do not replace canonical child routes for tab collections

## Client and Template Boundary

### Template data-loader seam

`EntityListPage` and `EntityList` expect:

- `data: T[]`
- optional `total`
- optional `hasMore`

That maps directly onto the canonical paged list envelope.

`EntityDetailPage` expects:

- one caller-owned detail loader returning the detail record itself
- optional caller-owned child tab loaders for related collections

Rules:

- shared templates should not need per-site adapter logic for list envelope
  shape
- TypeScript clients should expose page-shaped admin list routes as
  `PagedListResponse<T>` at the public client boundary
- client command wrappers may unwrap the outer `data` for detail endpoints, but
  the underlying route still uses the canonical detail envelope
- client command wrappers should not hide a route-family mismatch by
  remapping wildly different server shapes into the same frontend types

## Invariants

- one paged list envelope for root lists and detail-tab child collections
- one simple detail envelope for resource detail reads and write returns
- profiles vary projection only, not envelope family
- tab collections remain real collection routes, not ad hoc nested DTOs
- badge counts, when present, come from the main detail fetch rather than
  separate count-only endpoints

## Extension Points

Allowed:

- resource-specific item DTOs for list, filter, and detail projections
- optional profile enums for resources that need more than one approved
  projection
- app-owned summary fields on detail DTOs for tab badges or detail header
  metrics
- bounded non-paged `ListResponse<T>` helpers for small selector/suggestion
  surfaces

Not allowed:

- route families that encode projection in the path
- special tab-only collection envelopes
- mixing root list and tab list envelope shapes for the same app family
- count-only badge endpoints when the count belongs to the main detail page

## Rollout Checklist

When normalizing a consumer app onto this contract:

1. classify each collection as one of:
   - root admin page list
   - admin detail-tab child collection
   - bounded utility collection
   - auth/account or front-app collection outside `115`
2. normalize root admin page lists onto:
   - `data`
   - `total`
   - `has_more`
3. normalize admin detail-tab child collections onto the same paged envelope
4. move badge/count data needed by detail pages onto the main detail response
5. leave bounded utility collections on simple `{ "data": [] }` only when they
   are not page shells
6. do not spend migration effort on out-of-scope auth/account or front-app
   helpers unless another contract explicitly adopts them

`underlay-reference` is the proof case for this boundary: project detail task
summary moved onto the main detail DTO, media/user tab collections moved onto
the paged envelope, and selector/helper endpoints stayed bounded
`ListResponse<T>` surfaces.

## Known Drift To Repair

- `docs/guides/073-api-profiles-and-query-contract.md` previously overstated
  profile usage and still needed alignment to the current page/limit and
  paged-list envelope posture
- some reference-consumer detail pages still compute local summaries ad hoc
  instead of receiving a dedicated typed summary block from the detail response
- older consuming apps still carry multiple page-shaped list envelopes that do
  not match the reference consumer posture

## Next Task

Use this contract as the normalization target when rolling the template system
across consumer apps. The next shared proof is to audit a second live app
family against the same root-list, detail-summary, and child-tab rules instead
of rediscovering the boundary ad hoc.
