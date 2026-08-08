# g07.022 - Client Query Pagination Boundary Audit

Status: complete
Owner: repo maintainers
Started: 2026-06-06
Completed: 2026-06-06

## Purpose

Audit `@inflatable-cookie/underlay/client/query` and
`@inflatable-cookie/underlay/client/pagination` after the focused client DTO split.

## Governing References

- [020 HTTP transport and server boundary](../../contracts/020-http-transport-and-server-boundary.md)
- [090 TS runtime and client orchestration](../../contracts/090-ts-runtime-and-client-orchestration.md)
- [115 admin resource API shapes](../../contracts/115-admin-resource-api-shapes.md)

## Decision

Do not split `client/query`.

`client/query` is already the focused TS owner for the shared query-string
vocabulary:

- sort fields and directions
- filter fields and operators
- page and limit query keys
- URL query-string build, append, parse, and flat-record conversion helpers

Do not add separate sort/filter/page client subpaths. Consumers and templates
use these concepts together when preserving list state and building admin API
commands.

Keep `client/pagination` as a compatibility export for cursor pagination types.
Prefer `client/page-lists` plus `client/envelopes` `PagedListResponse<T>` for
page-shaped admin/resource browse surfaces.

## Evidence

- `020` already defines one shared wire vocabulary for sort, filter, page, and
  limit.
- `client/query` implements that vocabulary directly through
  `buildQueryString()`, `appendQueryParams()`, `queryParamsToFlatRecord()`, and
  `parseQueryParams()`.
- Underlay templates import `QueryParams`, `SortField`, and query helpers from
  `client/query`.
- Six-consumer usage is broad and coherent. Admin list pages, API command
  helpers, media commands, moderation pages, and local list wrappers all use
  `client/query` for the same route-state and API-command vocabulary.
- No live six-consumer source imports `@inflatable-cookie/underlay/client/pagination`.

## Consumer Upgrade Impact

Impact class: `none`.

No consumer code changes are required.

## Validation

- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`
- [x] `effigy validate`

## Next Task

Audit the remaining broad TS client/runtime surface after `client/query` and
`client/pagination` are recorded as settled for this generation.
