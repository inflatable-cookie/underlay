# g06.008 - Six-Consumer Compatibility Proof And Release-Note Closeout

## Why

`g06` changed shared Rust contracts across identifier handling, blob keys,
cookies, session rotation, media adapter config, devtools bundles, and
soft-delete macros.

Most changes were additive or internal, but the generation touched enough public
surface that it needs one consolidated consumer proof and upgrade-note pass.

## Goal

Prove the current six-consumer family still builds against the `g06` Rust
contract changes and record release-facing upgrade notes for the changed public
surfaces.

## Scope

In scope:

- run targeted checks for affected consumer API workspaces
- classify each public change as additive, deprecation, internal, or breaking
- record consumer actions and no-action surfaces
- note the existing `SessionStore` breaking trait change for unknown direct
  implementers
- capture validation gaps that remain outside this generation

Out of scope:

- broad consumer feature migrations
- fixing unrelated consumer dirty files
- full `effigy validate` cleanup for pre-existing TS/component-test drift
- release execution or publishing

## Contract References

- `023`: release and compatibility rollout
- `030`: auth and session systems
- `040`: storage, blob, and media systems
- `120`: tooling, testing, and contract artifacts
- `122`: Rust public API inventory

## Consumer Upgrade Impact

Impact classification: mixed.

- `additive`: typed identifiers, typed blob keys, typed cookie fields,
  typed migration bundle refs, media table config constructors
- `internal`: module splits in media, devtools, and cookie validation
- `breaking`: `SessionStore::rotate_session_if_current` for unknown direct
  external implementations
- `compatibility fix`: soft-delete restore/purge macros now expand against
  `sqlx::PgPool`
- `deprecation`: none landed in this generation; raw string compatibility
  paths remain available where consumer proof is still useful

Release-note summary:

- `underlay-db` now exposes `SqlIdentifier`, `QualifiedTableName`, quoted
  identifier formatting, typed existence helpers, and typed schema-drop helper
  paths. Raw existence and schema helpers remain compatibility surfaces.
- `underlay-blob` now exposes `BlobObjectKey` and typed upload/download
  constructors. The local adapter rejects traversal-style object keys before
  filesystem access.
- `underlay-http` now exposes typed cookie name/path/domain builders and shared
  CSRF cookie helpers. Invalid SameSite/Secure/domain/path/name combinations
  fail during config construction.
- `underlay-auth-jwt` requires `SessionStore` implementations to rotate refresh
  sessions atomically through the expanded `rotate_session_if_current`
  signature. No named consumer owns a direct implementation today.
- `underlay-media` keeps `MediaRepository` / `MediaRepositoryExt` as the
  app-facing seam while Postgres operation modules stay private. Postgres table
  config now has typed constructors.
- `underlay-devtools` exposes `MigrationBundleRef` and
  `BundleRunOptions::from_bundle_ref`; local store and remote registry handling
  remain implementation modules.
- `underlay-soft-delete` restore/purge macros now expand against `sqlx::PgPool`
  instead of an invalid crate-local pool alias.

## Consumer Proof

| Consumer root | Check | Result | Notes |
| --- | --- | --- | --- |
| `underlay-reference/acme-api` | `cargo check -p acme-api` | passed | Uses shared CSRF helpers, typed cookie startup config, and shared media storage-key helpers. |
| `contact-patch/cp-api` | `cargo check -p cp-api` | passed | Uses shared CSRF helpers, typed cookie startup config, and shared media storage-key helpers. |
| `compli-me/api` | `cargo check -p compli-me-api` | passed | Uses typed auth cookie configuration and shared auth cookie helpers. |
| `acowtancy/farmyard` | `cargo check -p farmyard-api` | passed | Verifies the soft-delete macro compatibility fix. App-local media repository and raw existence compatibility paths remain intentionally retained. |
| `songsprout/nursery` | `cargo check -p nursery-api` | passed | Uses typed auth cookie config helper and shared media storage-key helpers. |
| `loophole/composer/composer-api` | `cargo check -p composer-api` | passed | Verifies soft-delete and media storage-key compatibility; no CSRF cookie migration was needed. |

## Validation

- `effigy test --plan` confirms the repo-level test surface is Vitest plus
  Cargo nextest.
- Six targeted consumer API `cargo check` commands passed.
- `git diff --check` remains required in Underlay and the six consumer API
  roots after final doc pointer updates.
- `effigy qa:northstar` and `effigy qa:docs` remain required after final doc
  pointer updates.

Known validation gaps:

- `effigy doctor` still fails on the existing structural backlog: unsupported
  `isolation` key in `effigy.toml`, attention-marker findings, comment-ratio
  findings, and god-file findings.
- Full `effigy validate` cleanup remains outside this card. Earlier validation
  exposed existing TypeScript/component-test drift around resolving
  `$app/navigation` from `ts/src/client/navigation.ts`.

## Acceptance Criteria

- current six-consumer proof is recorded
- release-note style impact summary exists
- targeted affected consumer checks pass or failures are classified
- next remaining structural backlog is explicit

## Current State

`g06.008` is complete.

## Next Task

Execute `g06.009`: Effigy doctor structural backlog triage for remaining Rust
god-files and stale markers.
