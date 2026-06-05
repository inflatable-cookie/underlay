# g06.001 — Rust Platform Contract Transition

## Why

The recent Rust code-quality audit found the right direction: Underlay should
be less a collection of helpful shared crates and more a small platform
contract with strict extension points.

Several fixes already moved in that direction:

- central SQL identifier validation
- local blob object-key validation
- validated auth and CSRF cookie builders
- atomic refresh-token rotation semantics
- smaller media Postgres and migration-bundle modules

Those changes exposed the larger transition: Underlay needs typed safety
boundaries, explicit public API ownership, and consumer-proofed migration waves.

## Goal

Open the `g06` Rust platform-contract generation and complete the first
inventory gate before further migration work.

The intended end state is:

- `underlay-core`: IDs, errors, envelopes, shared primitives
- `underlay-db`: SQL safety, identifiers, schema helpers, pagination,
  migrations
- `underlay-http`: response contracts, cookies, CORS, request context, error
  handling
- `underlay-auth`: app-facing auth contracts, provider crates behind it
- `underlay-media`: media domain and repository contracts, adapters split
  internally
- `underlay-devtools`: tooling only, no app runtime assumptions

## Scope

In scope:

- create a Rust public API inventory by crate
- classify each public Rust surface as `stable`, `internal`, `adapter`,
  `candidate-remove`, or `candidate-type`
- identify consumer-owned implementations and call sites
- mark additive, deprecation, and breaking candidates
- define the first migration gates for typed safety primitives, auth/session,
  HTTP cookie builders, DB identifiers, media, and devtools

Out of scope for this milestone:

- broad code movement
- removing public APIs
- changing consumer runtime behavior beyond inventory proof
- adding compatibility shims without a rollout reason

## Contract References

- `001`: roadmap and generation authority
- `020`: HTTP transport and server boundary
- `023`: release and compatibility rollout
- `030`: auth and session systems
- `040`: storage, blob, and media systems
- `024`: new app bootstrap and bring-up
- `021`: database migration and schema workflow
- `g05.009`: Rust runtime audit evidence
- `g05.014`: compatibility rollout contract

## Inventory Targets

Inventory the Rust public surface for:

- `underlay-core`
- `underlay-db`
- `underlay-http`
- `underlay-auth`
- `underlay-auth-jwt`
- `underlay-blob`
- `underlay-media`
- `underlay-devtools`
- `underlay-audit`
- `underlay-security-alerts`
- `underlay-jobs`
- `underlay-migration-core`

The first pass may include other crates if they expose app-facing APIs, but the
milestone should not become an unbounded audit of every internal helper.

## Migration Gates

### Gate 1: Safety Types

Candidate typed boundaries:

- `SqlIdentifier`
- `QualifiedTableName`
- `BlobObjectKey`
- `CookieName`
- `CookiePath`
- `CookieDomain`
- `MigrationBundleRef`

Acceptance:

- invalid values are rejected before SQL, filesystem, HTTP header, or registry
  IO
- new APIs prefer typed inputs
- raw-string APIs are classified as stable compatibility, migration targets, or
  internal-only

### Gate 2: Auth And Session

Acceptance:

- `SessionStore` ownership and implementation expectations are documented
- refresh rotation compare-and-swap semantics are the only blessed shared model
- consumer-owned session stores, if any, are identified before further changes

### Gate 3: HTTP Safe Builders

Acceptance:

- auth and CSRF cookies are constructed through Underlay helpers
- app-local cookie string assembly is treated as migration debt
- invalid SameSite/Secure/domain/path/token combinations fail centrally

### Gate 4: DB Identifier Boundary

Acceptance:

- dynamic schema, table, and column names use validated identifier helpers or
  typed wrappers
- values remain bound parameters
- test helpers using dynamic schemas are explicitly classified

### Gate 5: Media And Devtools Modularity

Acceptance:

- public repository/tooling contracts are separated from adapter internals
- large module splits follow ownership boundaries, not line-count-only churn
- migration bundle refs cannot escape intended local/remote stores

## Consumer Upgrade Impact

Expected impact is mixed:

- additive: typed helpers and safer builders
- deprecation: raw-string construction paths that remain temporarily available
- breaking: any public trait reset, especially around session storage

Every consumer-affecting follow-up milestone must include:

- impact classification
- affected consumer inventory
- rollout order
- targeted validation commands
- release-note text or upgrade guidance

## Acceptance Criteria

- Rust public API inventory exists in a durable docs or contract artifact
- each inventory item has an ownership classification
- migration gates are explicit and sequenced
- known breaking candidates are visible before code execution continues
- no stale `g05` roadmap front door still advertises the Rust transition as a
  `g05` lane

## Validation

Use repo-owned validation surfaces:

```bash
effigy qa:northstar
effigy qa:docs
effigy validate
```

Use targeted Rust validation only after a code-bearing follow-up milestone.

## Current State

`g06.001` is complete.

The Rust public API inventory now lives in
[`docs/contracts/122-rust-public-api-inventory.md`](../../contracts/122-rust-public-api-inventory.md).

## Next Task

Execute `g06.002`: add the typed safety primitive layer behind the first
platform-contract migration gates.
