# Contract: Route Families and Access Model

Status: active
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`, `025-rust-app-runtime-assembly-and-router-topology.md`, `030-auth-and-session-systems.md`

## Purpose

Define the route-family taxonomy and access-model rules for normal Underlay API
apps.

This contract covers:

- route families and when a route belongs to each family
- auth and role posture by family
- CSRF posture by family
- versioning posture by family
- rate-limit posture by family
- trusted-proxy/client-IP posture by family

It does not define detailed list/detail envelopes. Those stay with `115` and
later read-shape contracts.

## Sources of Truth

Primary shared sources:

- [`docs/contracts/020-http-transport-and-server-boundary.md`](/Users/tom/Dev/projects/underlay/docs/contracts/020-http-transport-and-server-boundary.md)
- [`docs/contracts/025-rust-app-runtime-assembly-and-router-topology.md`](/Users/tom/Dev/projects/underlay/docs/contracts/025-rust-app-runtime-assembly-and-router-topology.md)
- [`docs/contracts/030-auth-and-session-systems.md`](/Users/tom/Dev/projects/underlay/docs/contracts/030-auth-and-session-systems.md)

Reference consumer evidence:

- [`underlay-reference/acme-api/crates/api/src/routes/mod.rs`](/Users/tom/Dev/projects/underlay-reference/acme-api/crates/api/src/routes/mod.rs)
- [`underlay-reference/acme-api/crates/api/src/state.rs`](/Users/tom/Dev/projects/underlay-reference/acme-api/crates/api/src/state.rs)
- [`acowtancy/farmyard/crates/api/src/routes/shared/router.rs`](/Users/tom/Dev/projects/acowtancy/farmyard/crates/api/src/routes/shared/router.rs)
- [`acowtancy/farmyard/crates/api/src/routes/front/router.rs`](/Users/tom/Dev/projects/acowtancy/farmyard/crates/api/src/routes/front/router.rs)
- [`acowtancy/farmyard/crates/api/src/routes/admin/router/misc_routes.rs`](/Users/tom/Dev/projects/acowtancy/farmyard/crates/api/src/routes/admin/router/misc_routes.rs)
- [`acowtancy/farmyard/crates/api/src/state.rs`](/Users/tom/Dev/projects/acowtancy/farmyard/crates/api/src/state.rs)

Additional consumer evidence:

- [`compli-me/api/crates/api/src/routes/mod.rs`](/Users/tom/Dev/projects/compli-me/api/crates/api/src/routes/mod.rs)
- [`contact-patch/cp-api/crates/api/src/routes/mod.rs`](/Users/tom/Dev/projects/contact-patch/cp-api/crates/api/src/routes/mod.rs)
- [`songsprout/nursery/crates/api/src/routes/mod.rs`](/Users/tom/Dev/projects/songsprout/nursery/crates/api/src/routes/mod.rs)
- [`songsprout/nursery/crates/api/src/routes/auth.rs`](/Users/tom/Dev/projects/songsprout/nursery/crates/api/src/routes/auth.rs)
- [`songsprout/nursery/crates/api/src/routes/admin.rs`](/Users/tom/Dev/projects/songsprout/nursery/crates/api/src/routes/admin.rs)
- [`loophole/composer/composer-api/crates/api/src/routes/mod.rs`](/Users/tom/Dev/projects/loophole/composer/composer-api/crates/api/src/routes/mod.rs)
- [`loophole/composer/composer-api/crates/api/src/extractors.rs`](/Users/tom/Dev/projects/loophole/composer/composer-api/crates/api/src/extractors.rs)

If these diverge, the contract plus the clearest modern reference posture
(`underlay-reference`, `farmyard`) win.

## Contract Goal

Underlay should make route placement and access posture predictable.

A new app should not have to rediscover:

- whether something belongs in `shared`, `front/public`, or `admin`
- whether a route should be public, authenticated, admin-only, or elevated
- whether CSRF, versioning, or rate limiting should apply
- whether a route is an operator/system surface or a normal product resource

The point is one stable route-family grammar across the six APIs.

## Scope Boundary

In scope:

- runtime/operational endpoints
- shared auth/account/helper endpoints
- front/public/product-user endpoints
- admin/operator endpoints

Out of scope:

- exact DTO shapes for page resources
- product-specific permission rules inside a domain
- app-specific role vocabularies beyond shared posture
- UI route trees

## Route Families

### 1. Runtime endpoints

These are operational surfaces for the service itself.

Typical examples:

- `/health`
- `/v1/health`
- `/health/live`
- `/health/ready`
- `/metrics`
- `/openapi.json`
- `/api/openapi.json`
- optional Swagger UI routes

Rules:

- runtime endpoints are not product resource endpoints
- they should not require user authentication
- they may sit outside `/v1/*`
- they are owned by the API runtime layer, not by domain route modules

### 2. Shared endpoints

These are shared app endpoints used by more than one client surface or by both
admin and non-admin products.

Typical examples:

- `/v1/auth/*`
- `/v1/account/*`
- shared media delivery
- shared Nightfire strategy reads
- shared lookup/helper routes that are not admin-only

Rules:

- shared does not mean public
- a shared route exists because multiple clients use it, not because it has no
  auth
- account/profile routes belong here even though they are authenticated

### 3. Front/public product endpoints

These are product-user routes, not operator routes.

Typical examples:

- front content reads
- learning reads
- assessment session flows
- public lookup/search routes
- customer or student workflow routes

Rules:

- these routes are user-facing domain surfaces
- they are not nested under `/v1/admin/*`
- some may be public, some authenticated
- they should stay domain-oriented rather than carrying admin/operator verbs

### 4. Admin endpoints

These are operator/admin routes for the admin UI and related operator tooling.

Typical examples:

- `/v1/admin/users/*`
- `/v1/admin/media/*`
- `/v1/admin/activity`
- `/v1/admin/jobs/*`
- `/v1/admin/scheduled-tasks/*`
- `/v1/admin/validation/*`
- admin CRUD, trash, reorder, moderation, and system-control surfaces

Rules:

- admin routes are always under `/v1/admin/*`
- operator/system surfaces are still part of the admin family, even if they are
  logically "platform" or "system" features
- do not create parallel top-level `/v1/system/*` or `/v1/operator/*` API
  roots for ordinary admin app work

## Access Model

### Runtime endpoints

Default posture:

- auth: none
- role gate: none
- CSRF: none
- version header: not required
- rate limiting: optional, usually not needed

Notes:

- readiness/liveness/metrics should stay callable by platform infrastructure
- if an app exposes OpenAPI or Swagger in non-dev environments, that is a
  deployment policy question, not a route-family change

### Shared auth endpoints

These split into unauthenticated bootstrap, authenticated maintenance, and
session/cookie helpers.

#### Unauthenticated bootstrap

Examples:

- register
- login
- password reset request/verify/complete
- passkey login start/finish
- email OTP request/verify when used for login bootstrap

Posture:

- auth: none
- role gate: none
- CSRF: required only when the route relies on cookie-backed browser session
  mutation rather than bearer/bootstrap semantics
- rate limiting: strongly recommended
- trusted proxy/IP posture: important when rate limiting or lockout uses client
  IP

#### Authenticated maintenance

Examples:

- change password
- TOTP setup/enable/disable/status
- passkey list/register/delete/rename
- current-session list/revoke
- `/v1/auth/me`

Posture:

- auth: `AuthenticatedUser`
- role gate: none
- CSRF: required for browser-session mutations
- version header: expected on normal `/v1/*` client traffic

### Shared account endpoints

Examples:

- `/v1/account/profile`

Posture:

- auth: `AuthenticatedUser`
- role gate: none
- CSRF: required for browser-session mutations
- version header: expected

### Front/public product endpoints

This family splits into public reads, authenticated reads, and authenticated
workflow mutations.

#### Public reads/helpers

Examples:

- public lookup
- public media download
- public catalog/list reads

Posture:

- auth: none
- role gate: none
- CSRF: none
- rate limiting: recommended when anonymous traffic can be abused
- trusted proxy/IP posture: relevant if rate limiting uses client IP

#### Authenticated product-user reads

Examples:

- current-user progress
- current-user learning/assessment state

Posture:

- auth: `AuthenticatedUser`
- role gate: product-user role or equivalent app-local check when needed
- CSRF: none for pure reads
- version header: expected

#### Authenticated product-user mutations

Examples:

- create assessment session
- submit question response
- other normal user workflow mutations

Posture:

- auth: `AuthenticatedUser`
- role gate: product-user role or equivalent app-local check when needed
- CSRF: required when browser cookie/session state is used
- rate limiting: recommended for abuse-sensitive flows

### Admin endpoints

Admin endpoints are authenticated operator routes by default.

Posture:

- auth: `AdminUser`
- role gate: admin baseline
- CSRF: required for browser-session mutations
- version header: expected
- trusted proxy/IP posture: relevant for audit, logging, and rate-limited admin
  actions

Subclasses:

- normal admin CRUD
- admin workflow/control actions
- admin operator/system reads

#### Elevated admin endpoints

Some routes may require stronger gates than `AdminUser`, for example
`SuperadminUser`.

Allowed uses:

- unusually destructive system controls
- role/identity operations that exceed normal admin power
- platform/security administration

Rules:

- elevated gates should be explicit extractor-level posture
- do not hand-roll repeated inline role checks in handlers when a dedicated
  extractor is warranted

## Route Placement Rules

Use this decision order:

1. Is this service liveness, metrics, or OpenAPI?
   - runtime
2. Is this auth/account or another shared non-admin support surface?
   - shared
3. Is this a product-user or public domain route?
   - front/public
4. Is this an operator/admin route?
   - admin

Additional rules:

- if a route exists to drive the admin UI, it belongs under `/v1/admin/*`
- if a route exists for both admin and non-admin callers and is not an
  operator-only surface, prefer `shared`
- if a route is anonymous but product-domain-facing, it is still front/public,
  not runtime

## Policy Rules By Concern

### CSRF

CSRF is a browser-session concern, not a blanket API concern.

Rules:

- require CSRF for state-changing routes when the app uses cookie-backed browser
  auth/session semantics
- do not require CSRF for pure bearer-token/server-to-server flows unless the
  app has explicitly chosen that stricter posture
- CSRF token fetch routes belong in shared auth, not in admin

### Versioning

Business endpoints are versioned API surfaces. Runtime endpoints are not.

Rules:

- `/v1/*` routes may participate in API-version header policy
- `/health`, `/metrics`, and OpenAPI routes do not require business-client
  version headers
- if an app logs or validates an API-version header, it should apply that
  policy consistently across `/v1/*` business families rather than only to one
  domain pocket

### Rate limiting

Rate limiting should follow abuse posture, not arbitrary file layout.

Rules:

- anonymous auth/bootstrap endpoints are the first-class rate-limit candidates
- anonymous lookup/search/scan endpoints are also strong candidates
- admin endpoints may add special throttles for expensive actions, but do not
  need blanket anonymous-style rate limits
- route-family or subgroup scoping is preferred over random per-handler drift

### Trusted proxy and client IP

Trusted-proxy posture matters where the app uses client IP as policy input.

Rules:

- if the app uses IP for rate limiting, lockout, audit, or auth events, client
  IP extraction should be centralized
- proxy trust belongs to runtime config/middleware posture, not handler-local
  parsing
- routes that do not use client IP as policy input do not need special proxy
  handling logic

## Invariants

- every business endpoint belongs to one clear family
- admin/operator routes live under `/v1/admin/*`
- runtime endpoints are not mixed into business route families
- `AuthenticatedUser` is the baseline authenticated extractor
- `AdminUser` is the baseline admin extractor
- elevated admin gates, when needed, are explicit and rare
- CSRF is applied because of browser-session mutation posture, not because a
  route "looks important"

## Extension Points

Allowed:

- app-local elevated role extractors
- app-local subgroup namespaces under `/v1/admin/*`
- app-local public helper families such as lookup/search when they are clearly
  non-admin product routes
- stricter app-local CSRF or rate-limit posture when justified

Not allowed:

- treating every non-admin route as "shared" and losing the product-user/admin
  distinction
- creating parallel operator root families outside `/v1/admin/*` for ordinary
  admin app work
- reimplementing auth/role gate semantics ad hoc in handlers instead of through
  route-family and extractor posture

## Current Drift To Repair Later

The current apps still diverge in some surface naming:

- `/v1/admin/jobs` vs `/v1/admin/platform/jobs`
- `/v1/admin/error-logs` vs `/v1/admin/error-log`
- passkey mutation verbs as path suffixes vs REST-style item routes

This contract does not force those specific path repairs by itself. It fixes
the route-family and access-model grammar first so later normalization can be
done without rearguing the basics.

## Next Task

Write `118`: the front and shared read API shape contract that sits on top of
this route-family model.
