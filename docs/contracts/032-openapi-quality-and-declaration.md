# Contract: OpenAPI Quality and Declaration

Status: active
Owner: repo maintainers
Depends on: `010-foundation-primitives-and-envelopes.md`, `028-runtime-surface-and-openapi-maturity-levels.md`, `115-admin-resource-api-shapes.md`, `118-front-and-shared-read-api-shapes.md`, `119-helper-search-and-lookup-route-catalogue.md`

## Purpose

Define the shared OpenAPI quality bar for normal Underlay APIs.

This contract covers:

- minimum versus strong route declaration coverage
- how response envelopes should be represented in OpenAPI
- helper, status, and workflow-route declaration rules
- what counts as unacceptable anonymous schema posture
- how OpenAPI quality should be scored in cross-app audits

It does not define whether an app exposes OpenAPI at all. That stays with
`028`. It defines what quality means once an app claims OpenAPI posture.

## Sources of Truth

Shared schema and response support:

- [`contracts/openapi/underlay.openapi.yaml`](../../contracts/openapi/underlay.openapi.yaml)
- [`rust/crates/underlay-http/src/openapi.rs`](../../rust/crates/underlay-http/src/openapi.rs)
- [`rust/crates/underlay-core/src/dto.rs`](../../rust/crates/underlay-core/src/dto.rs)

Supporting contracts:

- [`028-runtime-surface-and-openapi-maturity-levels.md`](./028-runtime-surface-and-openapi-maturity-levels.md)
- [`115-admin-resource-api-shapes.md`](./115-admin-resource-api-shapes.md)
- [`118-front-and-shared-read-api-shapes.md`](./118-front-and-shared-read-api-shapes.md)
- [`119-helper-search-and-lookup-route-catalogue.md`](./119-helper-search-and-lookup-route-catalogue.md)

Reference consumer evidence:

- `underlay-reference/apps/acme-api`
- `compli-me/apps/api`
- `contact-patch/apps/cp-api`
- `songsprout/apps/nursery`
- `loophole/composer/apps/composer-api`

If these diverge, the contract plus the clearest typed route posture win.

## Contract Goal

Underlay should make OpenAPI quality predictable.

A normal API team should not have to guess:

- which routes must be declared
- when `ApiSingleResponse<T>` or `ApiListResponse<T>` should be used
- when a raw `Object` schema is still tolerated
- whether status and helper routes deserve typed schemas too

The goal is one declared documentation quality bar instead of uneven app-local
judgment.

## Scope Boundary

In scope:

- OpenAPI declaration quality for runtime, shared, front, and admin routes
- success and error envelope schema posture
- helper/status route schema posture
- minimum versus strong declaration coverage

Out of scope:

- route-family placement itself
- domain DTO design beyond schema declaration quality
- Swagger UI/runtime exposure decisions

## Shared Boundary

### Coverage level rule

OpenAPI quality has three valid states:

1. absent
   - tolerated only where `028` still classifies the app as leaner maturity
2. minimum declared
   - runtime routes plus major shared/admin/front families are typed
3. strong declared
   - helper, status, workflow, and edge routes are typed too

Rules:

- new or modernizing APIs should target at least minimum declared posture
- reference-grade APIs should target strong declared posture
- once an app exposes OpenAPI JSON, the declared routes should be truthful, not
  decorative

### Envelope declaration rule

OpenAPI should use the shared response-envelope vocabulary.

Preferred posture:

- `ApiSingleResponse<T>` for `{ "data": T }`
- `ApiListResponse<T>` for bounded `{ "data": T[] }`
- typed paged response DTOs for page-shaped `data + total + has_more` families
- shared error envelope typing for failures

Rules:

- do not document normal success responses as raw `Object` when a typed shared
  envelope exists
- do not document helper summary bodies as bespoke top-level objects when the
  wire shape is actually `SingleResponse<T>`
- OpenAPI examples and route declarations should match the real wire envelope,
  not an imagined flatter body

### Resource-route rule

Real resource list/detail routes should be declared with typed DTOs and the
correct shared envelope family.

Rules:

- admin resource routes should align with `115`
- front/shared resource reads should align with `118`
- page-shaped collections should not be documented as bounded lists
- bounded lists should not be decorated with fake paging metadata in schema

### Helper and status rule

Helper, status, requirements, and lookup routes still need typed OpenAPI
declarations.

Rules:

- status/requirements routes should not return anonymous `Object` declarations
- helper lists should use typed bounded-list envelopes
- helper detail and summary objects should use typed single-response envelopes
- raw `json!` response bodies are not an excuse for weak schema posture

### Workflow-route rule

Non-resource workflow actions should be declared explicitly when they are part
of the public API surface.

Examples:

- `restore`
- `purge`
- `reorder`
- `complete`
- `skip`

Rules:

- action routes may return `ApiSingleResponse<()>` or another typed payload
- do not leave workflow routes undocumented just because they are narrow
- workflow routes do not need bespoke envelope families

### Anonymous-object rule

Anonymous `Object` response declarations are unacceptable for stable normal
routes once the payload shape is known.

Allowed temporary cases:

- older apps still in normalization where the route is already marked as lean
  maturity
- internal or transitional routes that are about to be replaced in the same
  lane

Disallowed posture:

- `/v1/auth/me` documented as generic `Object`
- password requirements or TOTP status documented as generic `Object`
- helper batch results documented with no typed summary object

### Typed DTO rule

OpenAPI declarations should prefer named DTOs for any payload shape that is
shared, reused, or non-trivial.

Rules:

- tiny one-field wrappers may still be named DTOs when they are public helper
  or status surfaces
- use named DTOs for helper status and requirements shapes
- use named DTOs for helper summary objects carrying both counts and results

### Audit rule

When scoring OpenAPI quality in the six-site inventory:

- separate runtime exposure from declaration quality
- classify apps as:
  - missing
  - minimum declared
  - strong declared
- distinguish true drift from lean-but-compatible maturity

## What Good Looks Like

Good outcomes:

- OpenAPI routes use the same envelope family the wire uses
- helper and status routes are typed, not anonymous
- page-shaped and bounded collections are documented honestly
- audits can describe quality level without hand-waving

Bad outcomes:

- OpenAPI exists but major helper or workflow routes are undocumented
- normal routes still use generic `Object`
- route declarations flatten envelopes that the wire still wraps
- schema posture implies paging or helper shape that the real API does not own

## Next Task

Use this contract when adding OpenAPI to a lean API, hardening route
declarations in a modern API, or scoring declaration quality across the
consumer fleet.
