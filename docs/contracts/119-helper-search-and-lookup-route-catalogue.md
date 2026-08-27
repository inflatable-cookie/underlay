# Contract: Helper Search and Lookup Route Catalogue

Status: active
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`, `026-route-families-and-access-model.md`, `118-front-and-shared-read-api-shapes.md`

## Purpose

Define the shared catalogue patterns for non-resource helper routes such as
lookup, search, suggest, detect, requirements, and status surfaces.

This contract covers:

- when a route is a helper instead of a resource family
- the preferred path grammar for helper families
- bounded response-shape rules for helper outputs
- how helper routes should be grouped and named

It does not define auth workflow semantics. That stays with `030`.
It does not define page-shaped resource reads. That stays with `118`.

## Sources of Truth

Primary shared sources:

- [`020-http-transport-and-server-boundary.md`](./020-http-transport-and-server-boundary.md)
- [`026-route-families-and-access-model.md`](./026-route-families-and-access-model.md)
- [`118-front-and-shared-read-api-shapes.md`](./118-front-and-shared-read-api-shapes.md)
- [`docs/contracts/api-surface/endpoint-family-matrix.csv`](./api-surface/endpoint-family-matrix.csv)

Reference consumer evidence:

- `compli-me/api/crates/api/src/routes/shared/lookup.rs`
- `loophole/composer/composer-api/crates/api/src/routes/lookup.rs`
- `contact-patch/cp-api/crates/api/src/routes/shared/auth/totp.rs`
- `contact-patch/cp-api/crates/api/src/routes/shared/auth/password.rs`
- `songsprout/nursery/crates/api/src/handlers/auth.rs`

If these diverge, the contract plus the clearest modern helper posture win.

## Contract Goal

Underlay should stop treating helper routes as a bag of exceptions.

A new app should not have to guess:

- whether a route should live under `/lookup`, `/search`, or a resource family
- whether a small helper should return `data: {}` or `data: []`
- whether batch lookup summaries belong at top level or under `data`
- whether capability and status routes should be bespoke one-offs

The goal is one narrow helper catalogue instead of helper drift hiding under
otherwise good resource families.

## Scope Boundary

In scope:

- lookup helpers
- search helpers
- suggest helpers
- detect helpers
- requirements/status/capability helpers
- bounded helper lists and helper detail objects

Out of scope:

- full resource collections and details
- auth/account workflow mutation routes
- admin CRUD routes
- action-route grammar for non-read workflows

## Shared Boundary

### Helper versus resource rule

A route is a helper when it does not represent a first-class browsable resource
family.

Typical helper cases:

- lookup by handle, key, slug, or variant signature
- bounded batch lookup
- search suggestion results
- capability or requirements reads
- small status reads
- detection or recommendation helpers

Rules:

- if the surface is a real list/detail family, use a resource route instead
- if the surface is workflow support rather than a browsable entity family, a
  helper route is the right tool

### Preferred helper families

Use these family names when the problem fits them:

- `/v1/lookup/*`
  - deterministic keyed lookup
- `/v1/search/*`
  - query-driven result discovery
- `/v1/suggest/*`
  - recommendation or suggestion output
- `/v1/detect/*`
  - classification or detection result
- `/v1/.../status`
  - state or capability read attached to a known family
- `/v1/.../requirements`
  - requirement/capability read attached to a known family

Rules:

- prefer a small shared helper family over inventing route-local names like
  `/check-something`, `/for-modal`, or `/helper`
- attach `status` and `requirements` to an existing family when they describe
  that family directly
- use top-level helper families when the helper is cross-resource or
  cross-client in nature

### Path grammar rule

Preferred patterns:

- `GET /v1/lookup/handle/{handle}`
- `GET /v1/lookup/variant`
- `POST /v1/lookup/batch`
- `GET /v1/search/plugins`
- `GET /v1/auth/totp/status`
- `GET /v1/auth/password/requirements`

Rules:

- deterministic lookups may use `GET`
- batch or structured lookup input may use `POST`
- helper paths should describe the helper purpose, not a UI screen
- do not add UI-oriented suffixes such as:
  - `-for-list`
  - `-for-form`
  - `-for-dialog`

### Envelope rule

Helpers still use canonical `data` envelopes.

Use:

- `{ "data": {} }` for helper detail or status objects
- `{ "data": [] }` for bounded helper lists
- `{ "data": { ...summary fields..., ...results... } }` for helper responses
  that need both list data and summary scalars

Rules:

- do not invent bespoke top-level bodies for helper summaries
- batch lookup counts belong inside the helper object under `data`
- tiny helpers do not need paged envelopes unless they become real browse
  surfaces

### Status and requirements rule

Status and requirements routes are helper reads, not workflow mutations.

Examples:

- password requirements
- TOTP status
- 2FA status
- domain verification status

Rules:

- these should return typed helper objects under `data`
- they should be grouped with the family they describe
- they should not return raw `json!` bodies or anonymous `Object` declarations

### Suggest and detect rule

Suggestion and detection outputs are still helper reads unless they become a
real domain family.

Rules:

- keep them bounded
- return canonical helper envelopes
- use helper naming that matches the behavior:
  - `suggest`
  - `detect`
- do not smuggle workflow mutations into suggest/detect routes

## What Good Looks Like

Good outcomes:

- helper routes are easy to spot and classify
- lookups, search, and status routes share one naming posture
- helper responses always use canonical `data` envelopes
- audits can separate helper drift from resource drift

Bad outcomes:

- helper routes borrow resource grammar for non-resource behavior
- tiny status routes return bespoke top-level JSON
- batch helpers invent one-off response bodies
- UI-oriented route names leak into shared APIs

## Questions This Contract Should Settle

- When should a read route live under `/lookup` versus under a resource family?
- When should a helper use `GET` versus `POST`?
- How should batch lookup summaries be shaped?
- Which tiny status/capability reads should be treated as first-class helpers?

## Next Task

Use this contract when the next app adds or normalizes lookup, search, suggest,
detect, status, or requirements routes.
