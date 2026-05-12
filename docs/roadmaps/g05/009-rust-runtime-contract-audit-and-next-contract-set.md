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
- `farmyard` front learning/content catalogs now read as deliberate bounded
  bootstrap libraries under `118`, not accidental paged-read drift:
  - they return bounded `data: []` collections
  - they do not expose paging controls today
  - moving them to `data + total + has_more` would be a product/API redesign,
    not a safe normalization patch
- `compli-me` is close, but lighter
  - shared `compli` read routes now explicitly declare their bounded list and
    detail envelopes in OpenAPI, instead of relying on implicit wire-shape
    behavior
  - shared handle lookup now uses the canonical helper detail envelope too,
    instead of a bespoke top-level body
- `contact-patch` shared auth helper/status routes are cleaner now:
  - password requirements use an explicit typed helper envelope
  - TOTP status and 2FA status are explicitly declared instead of being only
    incidentally correct on the wire
- `songsprout/nursery` is now structurally aligned at the router-assembly
  level, but still carries older front/product read-shape posture
  - the artist family is cleaner now:
    - read resources stay under `/v1/artists/*`
    - task workflow mutations live under `/v1/artist-task-actions/*`
    - page-shaped admin collection reads now return `data + total + has_more`
  - shared auth helper/status routes are tighter too:
    - `/v1/auth/me` now declares a typed canonical `SingleResponse`
- `composer-api` is now broadly compatible at the route-family level:
  - mixed catalog families keep flat/shared reads where that shape is real
  - admin-owned mutations now sit on canonical `/v1/admin/*` paths
  - shared auth now uses canonical `/v1/auth/*` paths, with the older
    `/v1/auth/local/*` endpoints retained only as compatibility aliases
  - shared helper reads are tighter now too:
    - single variant lookup already used the canonical `data` envelope
    - batch lookup now does too, instead of returning a bespoke top-level body
  - the first retirement proof is now done:
    - legacy flat `rules` routes removed
    - legacy flat `moderation` routes removed
  - the second retirement proof is now done:
    - legacy flat vendor/product write routes removed
    - legacy flat parameter/mapping/hardware write routes removed
  - the final retirement proof is now done:
    - version status mutation moved to `/v1/admin/versions/{id}`
    - semantic-role flat write aliases removed
  - remaining composer drift is now mostly historical payload/read-shape
    variation, not route-family ambiguity

That means the right next step is contract-first consolidation, not ad hoc
consumer rewrites.

## Six-Site Drift Readout

### Remaining real drift

The broad route-family and runtime-assembly drift is no longer the main
problem.

The meaningful remaining drift is narrower:

- `songsprout/nursery`
  - some older API-layer product ownership remains in the API crate
  - some read-model hardening is still lighter than the strongest apps, even
    after the artist split and paged envelope cleanup
- `compli-me/api`
  - runtime/OpenAPI posture is now present, but still lighter than the fuller
    reference implementations
  - this is maturity drift, not route-family drift
- `loophole/composer/composer-api`
  - remaining drift is mostly historical shared/runtime shape:
    - `/v1/auth/local/*` compatibility aliases still exist
    - runtime posture is still leaner than the OpenAPI-rich reference apps
  - mixed catalog flat reads are not counted as drift anymore; they are now a
    declared family shape under `026` and `118`

### Explicit exceptions

Some differences are now deliberate and should stop reading as unexplained
drift:

- `acowtancy/farmyard`
  - unversioned richer runtime surface under `025`:
    - `/health`
    - `/health/*`
    - `/metrics`
    - `/openapi.json`
  - bounded front bootstrap libraries under `118`:
    - learning/content catalogs keep bounded `{ "data": [] }` reads because
      they do not expose real paging controls
- `compli-me/api`
  - shared `/v1/compli/*` routes stay in the shared family because the product
    really uses them outside the admin lane
- `loophole/composer/composer-api`
  - flat/shared catalog reads remain valid where reads are genuinely
    cross-context and only writes are admin-owned

### What is effectively closed

The following normalization line is now in a clean stop state:

- `025` runtime assembly and router-family topology
- `026` route-family placement and access model
- first-pass `118` front/shared/helper read-shape normalization
- `composer-api` admin write-path canonicalization and compatibility retirement
- obvious helper-envelope drift across:
  - `compli-me`
  - `composer-api`
  - `contact-patch`
  - `songsprout/nursery`

### What should not become another churn lane

The next work should not be:

- broad forced paging retrofits for bounded front libraries
- flattening advanced runtime profiles just because they are richer
- moving mixed read families under `/v1/admin/*` when only the write surface is
  admin-owned
- more route churn in apps that are already contract-compatible

### Next contract wave

No equally urgent runtime-family contract gap is left after `025`, `026`, and
`118`.

If a next shared contract wave is needed, it should be narrower and producted
around one of these:

- compatibility retirement policy for canonical-path cutovers

That is a new wave, not a continuation of the original runtime-assembly audit.

## Contracts Landed In This Lane

### 025 — Rust app runtime assembly and router topology

Landed.

Defines the standard server-app shape:

- workspace crate roles
- thin `main.rs` responsibilities
- `AppState` composition rules
- router builder seam
- middleware stack order
- health / metrics / OpenAPI posture
- config/bootstrap hooks for DB, blob, email, jobs, and auth

This is now the base contract for "what a normal Underlay API app looks like."

### 026 — Route families and access model

Landed.

Defines the endpoint-family taxonomy:

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

This is now the base contract for admin vs front vs shared semantics.

### 118 — Front and shared read API shapes

Landed.

Extends the API-shape line beyond admin templates:

- front list/detail envelopes
- public lookup/search/result shapes
- bounded shared read helpers
- when a front read surface should mirror the admin canonical route family
- when a helper endpoint is allowed to stay non-page-shaped

This is now the base contract for non-admin read surfaces.

### 027 — API canonical path cutovers and compatibility retirement

Landed.

Defines the next narrower runtime/API migration seam:

- canonical path introduction
- compatibility alias posture
- client/server cutover order
- read-vs-write retirement rules
- stop conditions for path cleanup batches

This is the contract that turns the `composer-api` cleanup from one app proof
into a reusable Underlay cutover rule.

### 028 — Runtime surface and OpenAPI maturity levels

Landed.

Defines the remaining shared runtime-classification seam:

- minimal versus standard versus operator-rich runtime surfaces
- health, metrics, OpenAPI JSON, and Swagger posture
- how six-site audits should classify lean versus rich runtime exposure

This is the contract that turns the remaining runtime/OpenAPI maturity
differences into a declared ladder instead of vague drift language.

### 029 — Non-resource workflow action route grammar

Landed.

Defines the remaining workflow-action seam:

- lifecycle verbs such as `soft-delete`, `restore`, and `purge`
- collection actions such as `reorder` and `batch-delete`
- transition verbs such as `complete`, `skip`, `claim`, `release`, and
  `revoke`
- resource-scoped action routes versus dedicated action families

This is the contract that turns repeated workflow action naming into one
declared route grammar instead of app-local verb drift.

### 119 — Helper search and lookup route catalogue

Landed.

Defines the remaining non-resource helper seam:

- lookup versus search versus suggest versus detect families
- status and requirements helper placement
- bounded helper response-shape rules
- helper naming and `GET` versus `POST` posture

This is the contract that turns the helper/status cleanup proofs into a
declared helper catalogue instead of leaving them scattered under `118`.

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

This lane has now done the runtime-contract sequence in the right order:

1. write `025`
2. write `026`
3. write `118`
4. compile the endpoint-family matrix artifact
5. run the cross-app runtime normalization sweep
6. promote `027` once canonical-path retirement proved to be a real repeated
   contract seam

## Compatibility Retirement Order

`composer-api` no longer needs route-family discovery work. The planned
compatibility retirement sequence is now effectively complete.

Recommended order:

1. `rules` and `moderation`
   - done
   - flat compatibility paths retired after client adoption
2. `vendors`, `products`, `parameters`, `mappings`, `hardware`
   - done
   - flat reads retained, flat write aliases retired
3. `variants`
   - done
   - version/status mutation moved under a canonical admin-owned version path
4. any remaining shared helper overlap
   - only after `118` read-shape work is stable

Retirement rule:

- remove legacy flat write paths only after the corresponding client/workflow
  commands are already on canonical admin paths
- do not remove flat read/helper routes in the same batch unless the family is
  being deliberately redesigned under `118`

## Six-Site Drift Readout

After the composer cleanup, the remaining six-site runtime drift is narrower
and easier to classify.

### Closest to target posture

- `underlay-reference/acme-api`
  - still the cleanest modern reference
  - explicit route-family split
  - canonical shared/admin grammar
  - runtime and OpenAPI posture align well with `025` and `026`
- `contact-patch/cp-api`
  - very close to reference posture
  - explicit admin root
  - shared auth/account family is on-contract
  - main remaining difference is lighter runtime policy surface, not route
    grammar

### Compatible, but still carrying known runtime drift

- `acowtancy/farmyard`
  - route-family split is good
  - strongest current evidence for `118` front/public read families
  - keeps a richer unversioned runtime family:
    - `/health`
    - `/health/live`
    - `/health/ready`
    - `/health/info`
    - `/health/email`
    - `/metrics`
    - `/openapi.json`
  - this now reads as a deliberate advanced runtime profile under `025`, not
    accidental drift
- `compli-me/api`
  - route-family grammar is clean
  - shared/admin split is clear
  - still lighter than the reference runtime:
    - weaker explicit CSRF/runtime-policy posture than `underlay-reference` and
      `contact-patch`

### Remaining meaningful drift

- `songsprout/nursery`
  - `025` posture is improved:
    - `AppState` and app bootstrap no longer live inline in `main.rs`
    - entrypoint posture is now closer to the shared thin-binary shape
  - remaining `025` drift:
    - product repos are still in-memory at the API layer
  - `118` posture is improved:
    - artist read resources stay under `/v1/artists/*`
    - task actions moved to a narrower workflow family:
      - `/v1/artist-task-actions/{task_id}/complete`
      - `/v1/artist-task-actions/{task_id}/skip`
  - remaining `118` work there is now lighter:
    - payload/read-shape hardening rather than route-family mixing
- `loophole/composer/composer-api`
  - route-family ambiguity is basically gone
  - remaining drift is now mostly historical shape, not placement:
    - local auth still lives under `/v1/auth/local/*`
    - no OpenAPI runtime posture in current scan
    - several catalog reads remain intentionally flat/shared rather than being
      redesigned under stronger `118` resource-read rules

### Deliberate exceptions, not current contract failures

- `compli-me/api` shared `/v1/compli/*`
  - shared domain workflow surface used outside admin
  - should stay a documented shared-domain exception unless a broader reusable
    contract family emerges
- `farmyard` Nightfire strategy reads
  - shared support reads
  - already fit `118` as bounded shared support routes, not admin drift

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

The broad runtime-contract discovery lane is no longer the active problem.

What remains should be treated as narrower follow-on work inside `g05`, not as
another open-ended runtime audit:

- use `027` when the next API family needs a canonical-path cutover or alias
  retirement plan
- open a separate narrower contract only if one of the stop-state candidates in
  this roadmap proves to be a repeated seam:
  - OpenAPI/runtime maturity levels
  - helper/search/lookup catalogue patterns beyond `118`
  - action-route grammar for non-resource workflows
