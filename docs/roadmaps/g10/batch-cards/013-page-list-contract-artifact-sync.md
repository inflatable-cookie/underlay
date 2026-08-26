# g10.013 - Page List Contract Artifact Sync

Status: complete
Completed: 2026-08-26
Owner: repo maintainers
Contracts: `020-http-transport-and-server-boundary.md`, `115-admin-resource-api-shapes.md`, `032-openapi-quality-and-declaration.md`
Found by: `g10.011`

## Purpose

Make the canonical page-list wire shape and the normalized TypeScript boundary
explicit across Rust, OpenAPI, tests, and architecture docs.

## Scope

- keep Rust `PageList<T>` wire authority as `{ data, total, has_more }`
- keep TypeScript `PagedListResponse<T>` as the normalized public client shape
  `{ data, total, hasMore }`
- change the OpenAPI page-list schema to the raw wire field `has_more`
- replace the current same-spelling drift guard with assertions that preserve
  the intentional wire-to-client normalization
- repair `docs/architecture/015-error-and-envelopes.md`, which currently
  attributes the flat page-list shape to legacy `Paginated<T>`
- clarify contract `020`'s relationship to the later contract `115`

## Acceptance

- OpenAPI declares the canonical raw page-list wire shape from contract `115`
- tests fail if Rust/OpenAPI raw wire casing or the TS normalized boundary drifts
- active architecture docs name `PageList<T>`, not `Paginated<T>`, as the flat
  admin page-list producer
- legacy nested `Paginated<T>` remains documented as a distinct compatibility
  shape; no runtime response changes land in this card

## Validation

- `effigy test:unit`
- `effigy rust:test`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Stop Conditions

Stop if a consumer depends on OpenAPI `hasMore` as raw server wire authority.
That would require a consumer cutover card, not an artifact-only repair.

## Consumer Upgrade Impact

- Impact class: documentation and declaration correction
- Affected consumers: OpenAPI generators consuming `PagedListResponse`
- Required action: confirm generated raw transport types retain `has_more` or
  normalize it explicitly at the client boundary

## Completion Evidence

OpenAPI now declares `PagedListResponse` with raw `has_more`. The contract drift
test reads Rust, OpenAPI, and TypeScript sources and asserts the intentional
`PageList<T>.has_more` / OpenAPI `has_more` / TypeScript `hasMore` boundary.

Architecture and transport docs now attribute the flat wire shape to
`PageList<T>` and keep legacy nested `Paginated<T>` distinct. A six-consumer
search found no generated or OpenAPI client surface expecting raw `hasMore`;
Acowtancy's committed generated OpenAPI types already use `has_more`.

No Rust or TypeScript runtime serialization changed.

## Next Task

Execute `g10.014`, the bounded HTTP-client constructor fallback repair.
