# Contract: Runtime Surface and OpenAPI Maturity Levels

Status: active
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`, `025-rust-app-runtime-assembly-and-router-topology.md`, `026-route-families-and-access-model.md`, `027-api-canonical-path-cutovers-and-compatibility-retirement.md`

## Purpose

Define the shared maturity levels for runtime exposure in Underlay API apps.

This contract covers:

- minimum runtime surface every normal app should expose
- when richer runtime surfaces are appropriate
- OpenAPI and Swagger maturity levels
- how runtime surfaces should be classified in audits

It does not define route-family placement in general. That stays with `026`.
It does not define DTO envelopes. That stays with `010`, `115`, and `118`.

## Sources of Truth

Primary shared sources:

- [`020-http-transport-and-server-boundary.md`](./020-http-transport-and-server-boundary.md)
- [`025-rust-app-runtime-assembly-and-router-topology.md`](./025-rust-app-runtime-assembly-and-router-topology.md)
- [`026-route-families-and-access-model.md`](./026-route-families-and-access-model.md)
- [`027-api-canonical-path-cutovers-and-compatibility-retirement.md`](./027-api-canonical-path-cutovers-and-compatibility-retirement.md)
- [`docs/contracts/api-surface/endpoint-family-matrix.csv`](./api-surface/endpoint-family-matrix.csv)

Reference consumer evidence:

- `underlay-reference/apps/acme-api/crates/api/src/main.rs`
- `contact-patch/cp-api/crates/api/src/main.rs`
- `compli-me/api/crates/api/src/main.rs`
- `acowtancy/farmyard/crates/api/src/main.rs`
- `songsprout/nursery/crates/api/src/main.rs`
- `loophole/composer/composer-api/crates/api/src/main.rs`

If these diverge, the contract plus the clearest current maturity proofs win.

## Contract Goal

Underlay should make runtime exposure boring too.

A new app should not have to rediscover:

- whether `/health` is enough
- whether `/metrics` belongs in the normal runtime family
- whether OpenAPI JSON is optional, recommended, or expected
- whether Swagger UI is part of the baseline or a richer maturity level

The goal is one declared runtime maturity ladder instead of treating every
difference as either full drift or full equivalence.

## Scope Boundary

In scope:

- health, readiness, liveness, metrics, OpenAPI JSON, and Swagger runtime
  surfaces
- audit classification for lean versus richer runtime posture
- when a richer runtime surface is a deliberate maturity level

Out of scope:

- domain route trees
- auth workflow routes
- admin/page resource shapes
- deployment policy details beyond runtime surface classification

## Shared Boundary

### Runtime family rule

Health, metrics, OpenAPI, and Swagger belong to the runtime family.

Rules:

- runtime exposure should stay coherent as one family
- these routes should be mounted by runtime assembly, not by domain modules
- richer runtime posture is allowed, but it should read as one deliberate
  maturity level rather than a pile of unrelated local endpoints

### Maturity level A: minimal runtime

The minimal acceptable runtime posture for a normal Underlay API app is:

- one health route

Allowed examples:

- `/v1/health`
- `/health`

Rules:

- this is acceptable for very small or early apps
- this is not the preferred reference posture for new admin-backed products

### Maturity level B: standard runtime

The standard runtime posture is:

- one health route
- one OpenAPI JSON route
- optional Swagger UI route

Allowed examples:

- `/v1/health` plus `/api/openapi.json`
- `/v1/health` plus `/api/docs`

Rules:

- this is the preferred baseline for normal modern Underlay apps
- OpenAPI JSON should be treated as expected at this level
- Swagger UI is optional, but when present it should be explicit and mounted as
  runtime posture rather than hidden in ad hoc dev wiring

### Maturity level C: operator-rich runtime

The richer operator-facing runtime posture may add:

- readiness and liveness
- metrics
- info or support-health endpoints
- richer OpenAPI/runtime combinations

Allowed examples:

- `/health`
- `/health/live`
- `/health/ready`
- `/metrics`
- `/openapi.json`

Rules:

- this is an advanced runtime profile, not a contract failure
- if used, it should stay coherent as one runtime family
- apps should not drift into this level accidentally; there should be a real
  support, platform, or operator reason

This is the correct reading for `farmyard`.

### OpenAPI posture rule

OpenAPI has three valid states in the current Underlay family:

1. absent
   - tolerated only for older or still-normalizing apps
2. JSON only
   - acceptable
3. JSON plus Swagger UI
   - strongest current posture

Rules:

- new or modernizing apps should target at least JSON exposure
- Swagger UI is a maturity upgrade, not a mandatory baseline
- audits should distinguish:
  - missing
  - present but lean
  - present and strong

### Audit classification rule

When scoring runtime posture in the six-site inventory:

- do not mark richer runtime surfaces as drift merely because they are richer
- do mark missing OpenAPI posture as lighter maturity when stronger reference
  apps already expose it
- do not confuse runtime maturity with route-family correctness

Examples:

- `contact-patch`
  - strong runtime/OpenAPI posture
- `compli-me`
  - compatible runtime posture with lighter maturity
- `farmyard`
  - advanced runtime profile
- `composer-api`
  - compatible runtime family with leaner maturity because OpenAPI posture is
    still absent

## What Good Looks Like

Good outcomes:

- a new app can choose a declared runtime maturity level
- audits can say "lean but compatible" instead of inventing vague drift
- richer runtime profiles stay explicit
- OpenAPI posture becomes a deliberate maturity choice instead of an accident

Bad outcomes:

- every app invents a different runtime surface without classification
- richer runtime profiles are treated as failures by default
- missing OpenAPI posture goes undocumented
- readiness/liveness/metrics are scattered through product routers

## Questions This Contract Should Settle

- When is a runtime surface merely lean, versus actually off-contract?
- When is a richer runtime surface a deliberate advanced profile?
- What should a new app expose by default if it wants the normal modern
  Underlay posture?

## Assessment State

Assessed across all six consumer APIs by `g09.057` on 2026-08-27.

Verdict: `conforming` with declared maturity profiles.

- Underlay Reference, Contact Patch, and Compli Me are level B with health,
  OpenAPI JSON, and Swagger UI.
- Songsprout is an operator-extended level C profile with health, metrics, and
  OpenAPI JSON.
- Acowtancy is level C operator-rich with health, live, ready, info, email
  health, metrics, and OpenAPI JSON.
- Composer is level A lean plus metrics. Its absent OpenAPI surface remains
  explicitly classified as lighter maturity, not route-family drift.

Every runtime family is assembled outside product routers. No implementation
repair is warranted. See the
[`g09.057` assessment](../logs/2026-08/27-175930-g09-057-canonical-path-runtime-workflow-assessment.md).

## Next Task

Retain these maturity classifications in future app audits. No runtime repair
is queued by `g09.057`.
