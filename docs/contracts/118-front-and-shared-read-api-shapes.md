# Contract: Front and Shared Read API Shapes

Status: active
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`, `025-rust-app-runtime-assembly-and-router-topology.md`, `026-route-families-and-access-model.md`, `115-admin-resource-api-shapes.md`, `116-canonical-collection-routes-and-query-profiles.md`

## Purpose

Extend the API-shape contract beyond admin template pages so normal front,
public, and shared read surfaces are also declared.

This contract covers:

- front and shared read collection envelopes
- front and shared detail/read-record envelopes
- bounded helper/read endpoint rules
- when a front/shared read should mirror the canonical admin route family
- when a read surface is truly page-shaped versus helper-shaped

It does not define auth/account workflow endpoints or admin CRUD page shapes.

## Sources of Truth

Primary shared sources:

- [`020-http-transport-and-server-boundary.md`](./020-http-transport-and-server-boundary.md)
- [`025-rust-app-runtime-assembly-and-router-topology.md`](./025-rust-app-runtime-assembly-and-router-topology.md)
- [`026-route-families-and-access-model.md`](./026-route-families-and-access-model.md)
- [`115-admin-resource-api-shapes.md`](./115-admin-resource-api-shapes.md)
- [`116-canonical-collection-routes-and-query-profiles.md`](./116-canonical-collection-routes-and-query-profiles.md)
- [`rust/crates/underlay-core/src/dto.rs`](../../rust/crates/underlay-core/src/dto.rs)

Reference consumer evidence:

- `acowtancy/apps/farmyard/crates/api/src/routes/front/router.rs`
- `acowtancy/apps/farmyard/crates/api/src/routes/shared/router.rs`
- `compli-me/apps/api/crates/api/src/routes/shared/compli.rs`
- `underlay-reference/apps/acme-api/crates/api/src/routes/front/tasks.rs`
- `underlay-reference/apps/acme-api/crates/api/src/routes/shared/account.rs`
- `songsprout/apps/nursery/crates/api/src/routes/artist.rs`

If these diverge, the contract plus the clearest modern resource-read posture
win. Helper endpoints remain allowed, but they should stop drifting silently.

## Contract Goal

Underlay should give non-admin read surfaces one declared shape family too.

A new app should not have to guess:

- whether a front collection should be paged or bounded
- whether a helper read should use `SingleResponse` or `ListResponse`
- whether a front route should mirror the admin canonical route family
- whether summary counts belong in the main detail payload or on helper
  side-routes

The goal is not to force front APIs into the admin template model. The goal is
to stop normal read surfaces from becoming arbitrary one-offs.

## Scope Boundary

In scope:

- front/public read collections
- front/public detail reads
- shared non-auth read helpers
- shared read surfaces used by more than one client

Out of scope:

- admin page-shaped CRUD/list/detail endpoints
- auth/account workflow endpoints
- write/mutation workflow endpoints
- AI suggestion or action endpoints with their own contract family

## Read Surface Classes

### 1. Page-shaped read resources

These are real resource reads that users browse as first-class lists or details.

Examples:

- front content libraries
- front learning resource lists
- front module/pathway details
- user-facing project/task reads

These should be treated like canonical resources, not helper endpoints.

### 2. Bounded helper reads

These are small read helpers that support a workflow but are not full resource
pages.

Examples:

- lookup by handle
- duplicate checks
- tiny capability/status reads
- bounded related-item helpers

These should stay small and purpose-built.

### 3. Shared support reads

These are shared reads used by more than one client but not full admin page
resources.

Examples:

- public media download metadata or delivery endpoints
- shared Nightfire strategy reads
- cross-client lookup surfaces

These may be page-shaped or bounded depending on their real use.

## Shared Boundary

### Canonical route family rule

When a front/shared read is a real resource family, prefer canonical resource
routes over helper-specific path variants.

Allowed examples:

- `GET /v1/content/summaries`
- `GET /v1/content/summaries/{id}` when the product has a real detail page
- `GET /v1/learning/modules`
- `GET /v1/learning/modules/{id}`
- `GET /v1/projects/{project_id}/tasks`

Rules:

- canonical resource reads should not invent `-for-list`, `-for-card`,
  `/paginated`, or `/detail` path variants
- if the admin and front app are reading the same underlying resource family,
  the route family should stay recognizably parallel unless a real access-model
  or projection difference justifies separation
- helper endpoints remain allowed when the surface is not a normal resource
  browse/detail problem

### Collection envelope rule

There are two valid collection shapes for front/shared reads.

#### Bounded collection envelope

For genuinely small or helper-owned collections, use:

```json
{
  "data": []
}
```

This is the normal shape for:

- small front/shared reads
- lookup results
- bounded support collections

Rules:

- this is the default for front/shared collections unless the surface is a real
  page-level browse problem
- do not add `total` and `has_more` to tiny helper lists just for symmetry
- a bounded collection is still allowed for a full-library bootstrap read when
  the product intentionally loads the whole live catalog, there are no paging
  controls, and the data volume is kept within a known operational bound

#### Paged collection envelope

For real page-level browse surfaces, use the same collection family as `115`:

```json
{
  "data": [],
  "total": 0,
  "has_more": false
}
```

This is appropriate for:

- front libraries
- large student/customer resource lists
- front detail child lists that behave like real browsable collections

Rules:

- page-shaped read surfaces should use the paged envelope, not bespoke list
  wrappers
- do not force bounded helper reads onto the paged envelope
- front/page reads may share the same wire collection shape as admin pages
  without implying the same UI template usage
- do not relabel an existing full-library bootstrap read as paged unless the
  route also grows real paging ownership such as `limit`/`offset` or cursor
  inputs and the product actually treats it as a browsable paged surface

### Full-library bootstrap read rule

Some front collections are intentionally used as bounded bootstrap libraries
rather than paged browse surfaces.

Examples:

- live syllabus catalogs loaded once into client state
- small-to-medium content libraries used for local filtering
- per-user session lists where the expected cardinality stays bounded

Rules:

- these may keep the bounded `{ "data": [] }` envelope
- they should not expose fake paging metadata when the server does not own
  real paging controls
- they should be documented as bounded bootstrap reads in audits and inventory
  so they do not look like silent drift
- if the product later grows real paging inputs or server-driven browse
  behavior, the route should move to the paged envelope at the same time

### Detail envelope rule

Front/shared detail reads use the canonical single-record envelope:

```json
{
  "data": {}
}
```

Rules:

- detail reads should return one typed record under `data`
- do not wrap front/shared details in app-local `item`, `record`, or
  `resource` layers
- detail DTOs may include summary scalars and directly rendered structured
  content needed by the screen

### Child read collection rule

If a front/shared detail screen has a real child collection, it should use a
canonical child route rather than embedding the entire collection into the
detail DTO by default.

Preferred pattern:

- `GET /v1/learning/modules/{module_id}`
- `GET /v1/learning/modules/{module_id}/activities`

Rules:

- if the child collection is a real browsable surface, prefer a child route
- if the child data is tiny and inseparable from the detail surface, embedding
  it is allowed
- do not create count-only side routes for tab badges or summary chips when the
  count can live on the main detail DTO

### Summary and badge data rule

When a front/shared detail page shows counts or summary metadata, that summary
belongs on the detail DTO unless the summary itself is expensive enough to
justify a separate read model.

Rules:

- prefer scalar counts or a small summary object on the main detail response
- avoid helper endpoints like `*-count`, `*-summary`, or `*-for-badge` unless
  there is a real performance or ownership reason

### Helper endpoint rule

Bounded helper reads are allowed, but they must stay honest about being helper
surfaces.

Use helper reads for:

- lookup/search convenience
- verification/status checks
- bounded support information

Rules:

- helper routes should not pretend to be canonical resource collection routes
- helper responses should prefer `SingleResponse<T>` or `ListResponse<T>`
  depending on whether they return one thing or many
- helper endpoints should not become a shadow resource API just because they
  started small
- lookup and status helpers should still use the canonical `data` envelope
  rather than bespoke top-level bodies, even when the payload is tiny or pure
  computation
- when a bounded helper needs both a result list and helper-specific summary
  scalars, prefer `SingleResponse<T>` where `T` owns those fields instead of
  inventing a bespoke top-level body

Examples:

- `GET /v1/lookup/handle/{handle}` -> `{ "data": { ... } }`
- `GET /v1/auth/totp/status` -> `{ "data": { ... } }`
- `POST /v1/lookup/batch` -> `{ "data": { "results": [...], ... } }`
- `GET /v1/search/plugins` -> `{ "data": [] }`

### Projection rule

Front/shared reads may still need distinct projections, but projection should
normally be a query concern rather than a route-family split.

Allowed:

- `profile=list`
- `profile=details`
- `profile=summary`

Rules:

- use profiles only when the same canonical route genuinely serves more than
  one approved read projection
- keep profiles enum-like and documented
- do not use arbitrary include strings as a substitute for a real contract
- helper endpoints are still allowed when the surface is not truly the same
  resource read problem

### Client posture rule

For front/shared reads, the client surface should mirror the route intent:

- canonical resource reads get canonical read commands
- helper reads get helper-specific commands

Rules:

- do not split one resource family into many route/command families just
  because one consumer is a page and another is a smaller card or summary
- thin typed wrappers over the same route family are acceptable when they
  improve call-site clarity
- the smell is route drift, not wrapper names

## Invariants

- front/shared details use `{ "data": {} }`
- bounded front/shared collections use `{ "data": [] }` by default
- page-shaped front/shared collections use the paged envelope from `115`
- helper reads stay helper reads and do not silently become shadow resource
  APIs
- summary counts belong on the main detail DTO unless there is a real reason
  otherwise

## Extension Points

Allowed:

- front/product-specific child routes
- explicit profiles for real projection differences
- helper reads for bounded support problems
- using the paged envelope for real non-admin browse surfaces

Not allowed:

- bespoke envelope families for every front list or detail
- helper-only path variants for normal resource reads
- count-only route sprawl where the main detail DTO should own the summary
- treating every non-admin list like an admin page when the surface is actually
  small and bounded

## Current Drift To Repair Later

The current apps still show three kinds of drift:

- some front reads are canonical resource routes with simple envelopes
- some are true page-level browse surfaces that should probably converge on the
  paged envelope over time
- some helper reads are doing real resource work and should be reassessed

This contract does not force that cleanup in one shot. It gives the vocabulary
needed to classify and normalize those reads honestly.

## Next Task

Compile the machine-readable endpoint-family inventory artifact and then use
`025`, `026`, and `118` to drive the six-site API normalization sweep.
