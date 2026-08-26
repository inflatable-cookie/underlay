# Contract: Rust App Runtime Assembly and Router Topology

Status: active
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`, `030-auth-and-session-systems.md`, `060-jobs-events-and-operator-systems.md`

## Purpose

Define the standard Rust app-runtime shape for normal Underlay API services.

This contract covers:

- API workspace crate roles
- thin binary entrypoint responsibilities
- `AppState` composition
- router builder seams
- middleware stack order
- health, metrics, OpenAPI, and shutdown posture
- test-support and bootstrap expectations for a normal Underlay API app

It does not define domain route trees, product permission policy, or client/UI
behavior. Those build on top of this layer.

## Sources of Truth

Primary shared sources:

- [`docs/contracts/020-http-transport-and-server-boundary.md`](/Users/tom/Dev/projects/underlay/docs/contracts/020-http-transport-and-server-boundary.md)
- [`docs/contracts/030-auth-and-session-systems.md`](/Users/tom/Dev/projects/underlay/docs/contracts/030-auth-and-session-systems.md)
- [`docs/contracts/060-jobs-events-and-operator-systems.md`](/Users/tom/Dev/projects/underlay/docs/contracts/060-jobs-events-and-operator-systems.md)
- [`docs/architecture/020-rust-api-foundation.md`](/Users/tom/Dev/projects/underlay/docs/architecture/020-rust-api-foundation.md)

Reference consumer evidence:

- [`underlay-reference/apps/acme-api/crates/api/src/main.rs`](/Users/tom/Dev/projects/underlay-reference/apps/acme-api/crates/api/src/main.rs)
- [`underlay-reference/apps/acme-api/crates/api/src/routes/mod.rs`](/Users/tom/Dev/projects/underlay-reference/apps/acme-api/crates/api/src/routes/mod.rs)
- [`underlay-reference/apps/acme-api/crates/api/src/state.rs`](/Users/tom/Dev/projects/underlay-reference/apps/acme-api/crates/api/src/state.rs)
- [`acowtancy/farmyard/crates/api/src/main.rs`](/Users/tom/Dev/projects/acowtancy/farmyard/crates/api/src/main.rs)
- [`acowtancy/farmyard/crates/api/src/routes/router.rs`](/Users/tom/Dev/projects/acowtancy/farmyard/crates/api/src/routes/router.rs)
- [`acowtancy/farmyard/crates/api/src/routes/shared/router.rs`](/Users/tom/Dev/projects/acowtancy/farmyard/crates/api/src/routes/shared/router.rs)
- [`acowtancy/farmyard/crates/api/src/state.rs`](/Users/tom/Dev/projects/acowtancy/farmyard/crates/api/src/state.rs)

Additional consumer evidence:

- [`compli-me/api/crates/api/src/main.rs`](/Users/tom/Dev/projects/compli-me/api/crates/api/src/main.rs)
- [`compli-me/api/crates/api/src/routes/mod.rs`](/Users/tom/Dev/projects/compli-me/api/crates/api/src/routes/mod.rs)
- [`contact-patch/cp-api/crates/api/src/main.rs`](/Users/tom/Dev/projects/contact-patch/cp-api/crates/api/src/main.rs)
- [`contact-patch/cp-api/crates/api/src/routes/mod.rs`](/Users/tom/Dev/projects/contact-patch/cp-api/crates/api/src/routes/mod.rs)
- [`songsprout/nursery/crates/api/src/main.rs`](/Users/tom/Dev/projects/songsprout/nursery/crates/api/src/main.rs)
- [`songsprout/nursery/crates/api/src/routes/mod.rs`](/Users/tom/Dev/projects/songsprout/nursery/crates/api/src/routes/mod.rs)
- [`loophole/composer/composer-api/crates/api/src/main.rs`](/Users/tom/Dev/projects/loophole/composer/composer-api/crates/api/src/main.rs)
- [`loophole/composer/composer-api/crates/api/src/routes/mod.rs`](/Users/tom/Dev/projects/loophole/composer/composer-api/crates/api/src/routes/mod.rs)

If these diverge, the contract plus the clearest modern reference posture
(`underlay-reference`, `farmyard`) win. Older or flatter assemblies must catch
up.

## Contract Goal

Underlay should make a new Rust API app boring to assemble.

A normal app should be able to follow one declared runtime shape for:

- which crates exist
- what the binary does
- what lives in `AppState`
- how routes are composed
- which shared middleware layers are applied
- how metrics, health, OpenAPI, and shutdown are exposed

The goal is not a macro framework. The goal is one stable assembly posture that
all six apps can converge on and future apps can copy.

## Scope Boundary

In scope:

- normal HTTP API apps built on Axum and Underlay crates
- workspace layout around `api`, `auth`, `db`, `infra`, `domain`, and `jobs`
- runtime bootstrap and middleware composition
- route topology at the server-assembly level

Out of scope:

- product/domain module design
- detailed endpoint shapes beyond what `020`, `115`, and later route-family
  contracts define
- front-end SSR app runtime
- one-off CLIs, migration binaries, or jobs-only workers except where they
  share bootstrap helpers with the API

## Shared Boundary

### Workspace crate roles

A normal Underlay API workspace should separate concerns into explicit crate
roles.

The default app family is:

- `crates/api`
  - HTTP entrypoint, router assembly, DTOs, extractors, API-local errors
- `crates/auth`
  - app auth provider, local auth service, provider adapters over
    `underlay-auth*`
- `crates/db`
  - SQLx pool setup, migrations, seed/bootstrap, query modules
- `crates/infra`
  - app config, tracing bootstrap, email/template engines, trusted proxy or
    other runtime integrations
- `crates/domain`
  - domain repositories/services not owned by HTTP handlers
- `crates/jobs`
  - background job handlers and job-facing orchestration

Optional app-local crates are allowed when a product has a real extra seam:

- `test-utils`
- `platform`
- `notifications`
- `latex`
- `pdf-renderer`
- `nightfire`

Rules:

- do not collapse everything into the `api` crate by default
- do not pull product-specific domain behavior into Underlay just to match this
  shape
- keep the API crate focused on HTTP/runtime assembly plus DTO and handler
  ownership

### Binary entrypoint posture

`main.rs` should stay thin and predictable.

Normal responsibilities:

1. load app config
2. initialize tracing and log effective runtime config
3. build DB pool and run migrations
4. run dev/bootstrap seed logic when appropriate
5. initialize auth provider/service
6. initialize shared infrastructure adapters as needed:
   - blob
   - email/template engine
   - jobs repository
   - metrics registry
7. assemble `AppState`
8. set any required global shared handles used by middleware
9. build router
10. apply shared middleware layers
11. bind listener and serve with graceful shutdown

Rules:

- `main.rs` may validate environment/runtime safety, but domain logic must stay
  out of it
- route definitions should not live in `main.rs`
- long-lived app services should be assembled before router construction, then
  injected through `AppState`
- panic or shutdown helpers are allowed locally when the shared crates do not
  already own them

### AppState composition

Each app owns its `AppState`, but the shape should follow one predictable seam.

Expected categories:

- auth
  - local auth service
  - `Arc<dyn underlay_auth::AuthProvider>`
  - auth-cookie config when browser auth is used
- persistence and repositories
  - DB-backed repositories or service wrappers used by handlers
- infrastructure adapters
  - blob adapter
  - email manager/template engine
  - job repository
  - metrics registry
- app config
  - small typed config surface needed by handlers/middleware
- optional runtime policy helpers
  - trusted proxy config
  - provider-specific API config

Rules:

- `AppState` must implement `underlay_auth::HasAuthProvider`
- `AppState` is the HTTP/runtime dependency bag, not a dumping ground for raw
  domain globals
- prefer typed repositories/services over passing the DB pool into every
  handler when a domain seam already exists
- a global `DB_POOL`/similar `OnceCell` is acceptable only for shared
  middleware that cannot read the pool cleanly from generic state

### Auth extractor posture

App-local extractors are still expected above the shared Underlay auth seam.

Normal extractor family:

- `AuthenticatedUser`
- `AdminUser`
- optional `SuperadminUser` or equivalent elevated role extractor when the app
  truly has a second operator class

Rules:

- these extractors should wrap Underlay `Authenticated`, not replace it
- they should convert the shared `Principal` into the app-local principal type
- admin/superadmin role checks should happen in extractors, not be reimplemented
  ad hoc in handlers
- extractor rejections should emit canonical `auth.forbidden` / auth-envelope
  shapes rather than app-local bespoke error bodies

The generic auth mechanics are owned by `030`; this contract only fixes the
assembly posture above that seam.

### Router builder seam

The API crate should expose a dedicated router builder surface.

Allowed patterns:

- one `routes::build_router(...)`
- or one top-level builder that merges sub-routers such as:
  - `shared`
  - `admin`
  - `front` or public/user-facing

Rules:

- route registration should live in router modules, not in `main.rs`
- one app should have one obvious HTTP root builder
- route families may be merged from submodules, but the composition should stay
  shallow and explicit
- route modules should follow route-family boundaries before domain boundaries
  when that keeps the HTTP surface clearer

Preferred modern posture:

- top-level router builder
- explicit `shared`, `admin`, and optional `front`/public family builders

Older flat route files are tolerated as migration evidence, not as the desired
reference posture.

### Route-family topology

This contract does not fully define route-family semantics; `026` will do that.
But it does define the expected assembly topology.

Normal route families:

- shared
  - health
  - auth
  - account
  - globally shared utility reads such as public media download
- admin
  - operator/admin CRUD and workflow endpoints
- optional front/public read family
  - user-facing or student/customer-facing reads and writes that are not admin

Rules:

- a normal app should expose those families explicitly in the router tree, even
  if one family is small
- apps without a true front/public API may omit the front family
- route families should be evident from source layout, not only from path
  strings

### Middleware stack order

The shared middleware posture should be consistent across normal apps.

Core shared layers:

- request-id
- tracing
- error logging

Common optional layers:

- API-version logging or enforcement
- CSRF protection
- public or admin rate limiting
- panic catching

Expected order, outer to inner:

1. request-id
2. tracing
3. error logging
4. route-family or app policy middleware such as:
   - API-version
   - CSRF
   - rate limiting
   - panic catch
5. router with typed state

Rules:

- request-id and tracing are baseline, not optional luxury features
- error logging should sit high enough to see failures from route execution and
  inner middleware
- CSRF and version middleware are allowed to be app-local, but their placement
  should stay consistent
- public rate limiting may be route-family scoped rather than whole-app scoped
- do not hide major runtime policy inside arbitrary handler helpers

This contract fixes order and responsibility. Specific policy values remain
app-local or future route-family-contract concerns.

### Health, metrics, and OpenAPI posture

A normal Underlay API app should have a declared posture for operational
endpoints.

Expected surfaces:

- health endpoint
- metrics endpoint when metrics are enabled
- OpenAPI JSON, and optionally Swagger UI, when the app exposes OpenAPI

Rules:

- every normal API app should expose a health endpoint
- metrics may be `/metrics` rather than versioned
- OpenAPI may be `/openapi.json` or `/api/openapi.json`, but the app should
  choose one clear posture and document it
- Swagger UI is optional, but if present it should be mounted explicitly in the
  router builder rather than improvised elsewhere
- health/metrics/OpenAPI routes belong to the runtime assembly layer, not to
  domain route modules

Advanced runtime profile is allowed when the service has a real platform or
operations need for it.

Examples:

- readiness/liveness split
- build/runtime info endpoint
- dependency-specific health checks such as email adapter readiness

Rules:

- use the richer runtime profile only when the extra surfaces are genuinely
  consumed by platform operations, deployment checks, or support diagnostics
- keep the richer runtime family internally coherent:
  - health, readiness, liveness, metrics, and OpenAPI should still read as one
    runtime surface
- do not treat a richer runtime profile as permission to scatter runtime
  endpoints across domain modules or admin route trees

### Config and runtime validation posture

The binary entrypoint should own runtime safety validation.

Typical validated seams:

- database URL/config presence
- auth key availability
- cookie security posture
- allowed origins / CORS posture
- trusted proxy configuration
- blob/email provider config

Rules:

- fail fast on missing mandatory runtime configuration
- environment-specific defaults are allowed, but they must be explicit
- local/dev convenience fallbacks are allowed when clearly bounded and logged
- runtime validation belongs near bootstrap, not scattered across handlers

### Observability and shutdown

Normal apps should share one operational baseline:

- tracing initialized before major bootstrap steps
- request-id propagation enabled
- graceful shutdown on SIGINT/SIGTERM or Ctrl+C

Rules:

- shutdown handling belongs in the binary, not route modules
- startup should log bind address and key runtime posture
- local background cleanup tasks are allowed, but they should be started from
  bootstrap code and kept visible there

### Test-support seam

A normal API crate should make runtime-level route testing feasible without
running the whole binary.

Expected support:

- test app-state builder or fixtures
- route tests that can instantiate the router directly
- auth test helpers or extractor-compatible principals

Rules:

- handlers and routers should be testable without invoking `main()`
- route tests should prefer router-level integration over unit-testing handler
  internals in isolation
- test-support modules may be local to the API crate

## Invariants

- a normal Underlay API app has one thin binary entrypoint and one obvious root
  router builder
- `AppState` implements `HasAuthProvider`
- request-id, tracing, and error logging are baseline runtime layers
- route definitions do not live in `main.rs`
- app-local auth and role extractors sit above the shared Underlay auth seam
- operational endpoints are explicit runtime surfaces, not accidental leftovers

## Extension Points

Allowed:

- app-local extra crates when the domain genuinely needs them
- app-local config validation rules
- optional OpenAPI/Swagger exposure
- optional trusted-proxy config and client-IP extraction
- optional public/admin rate limiting
- optional elevated-role extractors beyond `AdminUser`

Not allowed:

- pushing product-specific runtime policy into Underlay without a real shared
  boundary
- letting every app invent a different bootstrap and router pattern when the
  problem is the same
- hiding route-family assembly inside opaque registration cascades when a
  clearer merged-router structure is available

## Current Drift To Repair Later

The audit shows one shared family plus some remaining runtime drift:

- `underlay-reference` and `farmyard` are the clearest modern references
- `compli-me` and `contact-patch` are close, but lighter
- `songsprout/nursery` still uses older route-registration posture
- `composer-api` remains the flattest outlier and the least obviously split
  into `shared` / `admin` / front-family builders

That drift should be repaired after `026` and `118` exist, not by treating the
current diversity as equally canonical.

## Next Task

Write `026`: the route-family and access-model contract that sits on top of
this runtime assembly layer.
