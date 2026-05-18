# Contract: Canonical Collection Routes and Query Profiles

Status: active
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`, `100-shared-patterns-and-workflow-shells.md`, `115-admin-resource-api-shapes.md`

## Purpose

Define the higher-level collection-route contract Underlay owns across page
lists, selectors, filter dropdowns, and other resource-backed collection reads.

This contract exists because `115` settled the page-shaped admin list seam, but
it intentionally left bounded helper collections outside that lane. The next
shared problem is not route naming alone. It is converging all resource-backed
collection consumers onto:

- one canonical route family per resource
- one shared query vocabulary
- one explicit projection model
- one explicit named baseline-query model for lists that need product views
- one explicit client-command posture

The goal is to stop every app from carrying parallel commands like
`listModulesForListAdmin`, `listModulesForFilterAdmin`, `listModulesForPicker`,
or route variants like `-for-list` and `-for-filter` just to satisfy different
consumers of the same underlying resource collection.

## Sources of Truth

Primary:

- [`docs/contracts/020-http-transport-and-server-boundary.md`](/Users/tom/Dev/projects/underlay/docs/contracts/020-http-transport-and-server-boundary.md)
- [`docs/contracts/100-shared-patterns-and-workflow-shells.md`](/Users/tom/Dev/projects/underlay/docs/contracts/100-shared-patterns-and-workflow-shells.md)
- [`docs/contracts/115-admin-resource-api-shapes.md`](/Users/tom/Dev/projects/underlay/docs/contracts/115-admin-resource-api-shapes.md)
- [`docs/guides/073-api-profiles-and-query-contract.md`](/Users/tom/Dev/projects/underlay/docs/guides/073-api-profiles-and-query-contract.md)
- [`docs/guides/080-typescript-client.md`](/Users/tom/Dev/projects/underlay/docs/guides/080-typescript-client.md)
- [`docs/guides/100-frontend-web.md`](/Users/tom/Dev/projects/underlay/docs/guides/100-frontend-web.md)
- [`ts/src/client/types.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/types.ts)

Reference consumer evidence:

- Acowtancy command families that already converged page and helper projections
  onto canonical routes during the `115` sweep
- `underlay-reference` page/list consumers that already use the shared route +
  profile posture without path-level projection variants

If these diverge, the contract wins.

## Contract Goal

Underlay should give consuming apps one predictable collection model:

- one canonical collection route per resource
- `profile` controls projection, not route identity
- `variant` controls named baseline queries, not projection
- selectors and filter dropdowns should use the same resource routes as page
  lists where they are reading the same resource family
- client packages should expose a clear canonical command posture per resource,
  even if thin typed wrappers remain for list vs filter projections

The goal is simpler API and client structure, not forcing every response to be
identical regardless of use case.

## Scope Boundary

In scope:

- resource-backed list routes used by:
  - admin page lists
  - admin detail-tab child collections
  - relation selectors
  - filter dropdowns
  - assignment/search helpers that read a real resource collection
- client command surfaces for those routes
- query/profile naming for those routes

Out of scope:

- domain-specific suggestion engines where the route is not a canonical
  resource list
- richer activity/event feeds with their own cursor or timeline contract
- app-local in-memory search helpers over already-loaded option sets
- write or action endpoints

This contract is about resource collection reads. It is not a general rule for
all helper endpoints in an app.

## Shared Boundary

### Canonical route rule

Use one collection route per resource family.

Allowed:

- `GET /v1/admin/learning/modules`
- `GET /v1/admin/learning/pathways`
- `GET /v1/admin/projects/{projectId}/tasks`

Rules:

- projection differences belong in query params
- pagination/filtering/sorting belong in query params
- do not create path variants like:
  - `-for-list`
  - `-for-filter`
  - `/selector`
  - `/dropdown`
  - `/paginated`
- a selector and a page list should hit the same route when they are reading
  the same resource family

### Query profile rule

Use `profile` for approved projection variants on the canonical route.

Allowed collection profiles:

- `profile=list`
- `profile=filter`
- `profile=list-config`

Meaning:

- `profile=list`: page/list/table/card projection
- `profile=filter`: lightweight selector/dropdown projection
- `profile=list-config`: capabilities payload for query variants and filter
  definitions

Rules:

- `profile=filter` is the retained query token even when the consumer is a
  selector shell; it means lightweight selection/filter projection, not a
  separate route family
- profiles must stay enum-like and documented
- do not invent app-local synonyms such as `picker`, `search`, `simple`, or
  `minimal` without a shared contract update

### Query variant rule

Use `variant` for named baseline list queries.

Allowed examples:

- `variant=pending`
- `variant=marked`
- `variant=void`
- `variant=all`

Meaning:

- the API applies the variant as the starting query scope
- `filter[...]`, `search`, `sort`, `page`, and `limit` apply after that scope
- `all` is an explicit variant when the product needs it, not an implicit
  "nothing selected" state

Rules:

- variants must be enum-like and documented per endpoint
- unknown variants should fail with a clear request error
- do not model named product views as hidden UI filters
- do not use `profile` for named query state

### Shared query vocabulary

Canonical collection routes should converge on one query vocabulary:

- `profile`
- `variant`
- `page`
- `limit`
- `search`
- `sort`
- `filter[...]`

Rules:

- page-shaped list consumers should use `page` + `limit`
- selector/filter consumers may still use the same route with small `limit`
  values and `search`
- route-level query keys should not change per consumer type
- `variant` is optional; routes that do not have named product views do not
  need to accept it
- if a route still needs a lower-level cursor contract for legacy reasons, that
  is compatibility debt, not the preferred shared posture
- if that compatibility debt is still live, keep it explicit:
  - one canonical route family
  - a documented legacy profile or query mode
  - thin wrappers that make the split obvious instead of pretending the
    payloads are already unified

### Response posture

For resource-backed collection routes, the preferred response family is the
page-shaped list envelope from `115`:

```json
{
  "data": [],
  "total": 0,
  "has_more": false
}
```

Rules:

- page lists and detail-tab child lists must use this envelope
- selector and filter consumers may use the same route and simply ignore
  `total` / `hasMore`
- this is preferred over adding a second helper-only endpoint or helper-only
  command surface

Bounded `ListResponse<T>` remains allowed only when the route is not a
canonical resource collection, for example:

- AI suggestion endpoints
- tiny dedicated support helpers
- feeds that are intentionally not modeled as resource lists

### Client command posture

The hard rule is route/query convergence, not one mandatory exported function
shape.

Preferred options:

- one canonical list command per resource family, for example:
  - `listModulesAdmin(fetch, token, params)`
  - `listPathwaysAdmin(fetch, token, params)`
  - `listTasksAdmin(fetch, token, params)`
- or thin typed wrappers over that same route family, for example:
  - `listModulesForListAdmin(...)`
  - `listModulesForFilterAdmin(...)`

Where params may include:

- `profile`
- `variant`
- `page`
- `limit`
- `search`
- `sort`
- resource-specific filters

Rules:

- separate exported wrappers are acceptable when they improve type clarity,
  caller intent, or DTO narrowness
- those wrappers should still target the same canonical route family and shared
  query vocabulary
- wrappers that preserve a temporary legacy cursor-vs-page split are acceptable
  during migration, but should be classified as compatibility posture, not the
  long-term clean state
- route duplication is the smell; thin wrapper duplication is often acceptable
- selector shells may unwrap `response.data` locally, but that does not require
  every repo to collapse to one exported command name immediately

### Pattern-layer implication

Relation selector and filter dropdown workflows in `100` should assume:

- app code provides async search/suggest functions
- when those functions are backed by a real resource collection, they should
  normally call the same canonical route family with `profile=filter`
- local search helpers stay valid only when the dataset is already loaded

This keeps the selector workflow generic without making every app invent a
second API vocabulary for the same resource family.

## Invariants

- one canonical collection route per resource family
- one shared query vocabulary for page and selector consumers
- profiles vary projection, not route identity
- variants vary named baseline query, not projection or envelope shape
- page and selector consumers may share the same resource route even if one
  consumer only uses `response.data`
- thin typed wrappers over the same route family are acceptable; duplicate
  route families are not

## Extension Points

Allowed:

- resource-specific filter params
- resource-specific variant enums
- resource-specific list and filter DTOs
- thin typed list/filter wrappers over one canonical route family
- temporary compatibility wrappers while a consumer fleet converges

Not allowed:

- path-level projection variants for routine list/filter/selector use
- helper-only route families for selectors when a canonical resource list
  already exists
- per-app query vocabularies for the same class of collection consumer
- hidden filter defaults that represent named product views

## Rollout Target

When normalizing a consumer app:

1. identify every resource collection with duplicate page/filter/selector
   routes or commands
2. collapse them onto one canonical route
3. keep `profile=list|filter` only when both projections are genuinely needed
4. move selectors and filter dropdowns onto that canonical route family
5. keep thin typed wrappers where they materially improve intent or typing
6. delete specialist helper routes, and only delete helper commands when they
   no longer add value

## Known Drift

- `115` settles page shells but still leaves too much room for helper-only
  client command duplication
- `073`, `080`, and `100` teach the route/profile idea but do not yet make the
  route-versus-wrapper distinction hard enough
- current consumer apps still carry legacy command families like
  `*ForListAdmin` / `*ForFilterAdmin`, some of which are acceptable thin
  wrappers and some of which still hide duplicate route families or divergent
  query vocabularies

## Next Task

Use this contract to compile the next shared cleanup lane: identify which
consumer command pairs are acceptable thin wrappers over one route family, and
which ones still hide duplicate routes or divergent query posture.
