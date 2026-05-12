# g05.009 — Rust Runtime Contract Audit and Next Contract Set

## Why

The TypeScript/template side is now substantially contracted across the six
consumer apps, but the Rust/server side is not at the same level.

Underlay has lower-layer contracts for transport, auth, storage, jobs, and
admin resource page shapes. What is still under-specified is the app-runtime
layer that turns those shared crates into a normal Underlay API:

- how an app API workspace is composed
- how routes are split between shared, front/public, admin, and operator
  families
- which route families require authentication, role gates, CSRF, versioning,
  rate limiting, or trusted-proxy posture
- which endpoint shapes are canonical for front/shared read surfaces, not just
  admin template pages

That missing layer is now the main reason new apps cannot yet "paint by
numbers" end to end.

## Goal

Audit the live Rust/runtime patterns in Underlay plus the current consumer APIs,
then freeze the next contract set needed to consolidate all six sites onto one
shared runtime posture.

## Current Inventory

Primary shared Rust contract surface already exists in:

- [`docs/contracts/020-http-transport-and-server-boundary.md`](/Users/tom/Dev/projects/underlay/docs/contracts/020-http-transport-and-server-boundary.md)
- [`docs/contracts/030-auth-and-session-systems.md`](/Users/tom/Dev/projects/underlay/docs/contracts/030-auth-and-session-systems.md)
- [`docs/contracts/040-storage-blob-and-media-systems.md`](/Users/tom/Dev/projects/underlay/docs/contracts/040-storage-blob-and-media-systems.md)
- [`docs/contracts/060-jobs-events-and-operator-systems.md`](/Users/tom/Dev/projects/underlay/docs/contracts/060-jobs-events-and-operator-systems.md)
- [`docs/contracts/115-admin-resource-api-shapes.md`](/Users/tom/Dev/projects/underlay/docs/contracts/115-admin-resource-api-shapes.md)

Consumer runtime evidence audited here:

- `underlay-reference/acme-api`
- `acowtancy/farmyard`
- `compli-me/api`
- `contact-patch/cp-api`
- `songsprout/nursery`
- `loophole/composer/composer-api`

## Audit Readout

### 1. The API workspace shape is already repeating

Across the current consumer family, the normal Rust app shape is clearly
converging on:

- one workspace root
- an `api` crate for HTTP assembly
- `auth`, `db`, `infra`, `domain`, and usually `jobs` crates
- a thin `main.rs` that wires config, DB, auth, blob/email/jobs, state, and
  middleware before calling a router builder

That pattern is real, but it is not yet a contract.

### 2. Route-family topology is real, but only partly written down

The live APIs already separate route families in recognizable ways:

- `shared`
  - health
  - auth
  - account
  - some globally shared helpers such as public media download
- `front` or public read routes
  - student/customer/user-facing reads
  - sometimes mixed with public lookup/scan helpers
- `admin`
  - CRUD, moderation, trash, jobs, scheduled tasks, activity, media, and
    template-backed resource surfaces

But the exact rules for where a route belongs are still local app judgment.

### 3. Access-control posture is repeated, but still app-local

Repeated current pattern:

- app `AppState` implements `underlay_auth::HasAuthProvider`
- app-local `AuthenticatedUser` extractor wraps Underlay `Authenticated`
- app-local `AdminUser` extractor checks role locally
- some apps add `SuperadminUser`
- public endpoints often add rate limiting; admin endpoints rely on auth and
  app-local role checks

This means the auth lower seam is shared, but the route-family and role-gate
surface is not yet contracted as an app-runtime rule.

### 4. Middleware stacking is the largest repeated uncodified runtime surface

Repeated shared middleware:

- request-id
- tracing
- error logging

Sometimes repeated, but inconsistent:

- API-version header middleware
- CSRF middleware and CSRF-token endpoint
- trusted-proxy/IP extraction posture
- public rate limiting
- admin-specific rate limiting
- Swagger/OpenAPI exposure

This is the clearest missing "paint by numbers" layer.

### 5. Endpoint grammar is partly canonical and partly fragmented

Clearly repeated:

- `GET /v1/health`
- `POST /v1/auth/*`
- `GET|PATCH /v1/account/profile`
- `GET /v1/admin/activity`
- jobs, scheduled tasks, error logs
- media admin CRUD, trash, usage, versions, upload lifecycle

Still fragmented:

- admin operator namespaces:
  - `/v1/admin/jobs`
  - `/v1/admin/platform/jobs`
- destructive action grammar:
  - `:batch-delete`
  - `/batch-delete`
  - `/soft-delete`
  - `/restore`
  - `/purge`
- OpenAPI surface:
  - `/api/docs` + `/api/openapi.json`
  - `/openapi.json`
- health/metrics posture:
  - `/v1/health`
  - `/health`
  - `/metrics`

### 6. The admin page-shape contract is ahead of the front/shared read contract

`115`, `116`, and `117` now give strong guidance for:

- admin list pages
- admin detail pages
- admin tab lists
- canonical admin collection routes

But the front/shared side is not equally defined.

There is no matching contract yet for:

- student/customer-facing list and detail read surfaces
- public lookup/search helpers
- bounded shared read helpers that are not admin pages

### 7. The six-site target is realistic

The six consumers are not six unrelated server designs.

The scan shows one repeated runtime family with a few maturity bands:

- `underlay-reference`, `contact-patch`, and `acme/farmyard` are the clearest
  modern Underlay-style APIs
- `compli-me` is close, but lighter
- `songsprout/nursery` still carries older registration-style router assembly
- `composer-api` is the clearest structural outlier:
  - flatter route tree
  - local middleware/extractor posture
  - less obvious `shared/front/admin` separation

That means the right next step is contract-first consolidation, not ad hoc
consumer rewrites.

## Contract Gaps To Fill

### 025 — Rust app runtime assembly and router topology

This should define the standard server-app shape:

- workspace crate roles
- thin `main.rs` responsibilities
- `AppState` composition rules
- router builder seam
- middleware stack order
- health / metrics / OpenAPI posture
- config/bootstrap hooks for DB, blob, email, jobs, and auth

This is the missing contract for "what a normal Underlay API app looks like."

### 026 — Route families and access model

This should define the endpoint-family taxonomy:

- `shared`
- `front` or public/user-facing
- `admin`
- operator/system

And for each family:

- auth requirement
- role requirement
- CSRF/cookie expectations
- token vs cookie expectations
- rate-limit posture
- versioning requirement
- when trusted-proxy client IP matters

This is the missing contract for admin vs front vs shared semantics.

### 118 — Front and shared read API shapes

This should extend the API-shape line beyond admin templates:

- front list/detail envelopes
- public lookup/search/result shapes
- bounded shared read helpers
- when a front read surface should mirror the admin canonical route family
- when a helper endpoint is allowed to stay non-page-shaped

This is the missing contract for non-admin read surfaces.

## Recommended Contract Artifacts

The contract docs should be backed by one durable inventory artifact:

- `contracts/api-surface/endpoint-family-matrix.csv`

Suggested columns:

- app
- route_pattern
- family
- auth_level
- role_gate
- shape_class
- notes

That keeps future consolidation honest and gives new apps a concrete checklist.

## Execution Posture

1. Write `025` first.
2. Write `026` second.
3. Write `118` third.
4. Compile the endpoint-family matrix artifact.
5. Only then start the cross-app runtime normalization sweep.

## Consumer Upgrade Impact

Expected.

This lane is contract-writing and inventory work first, but the follow-on
normalization will likely touch:

- route placement
- middleware posture
- auth/role extractors
- endpoint naming
- front-vs-admin API shapes

## Next Task

`025` and `026` are now written.

Next:

- write `118`: the front/shared read API shape contract
- then compile the machine-readable endpoint-family inventory artifact
