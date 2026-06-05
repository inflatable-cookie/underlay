# g06.015 - Rust Platform Transition Release-Note Handoff

## Why

`g06.014` confirmed the Rust platform-contract transition is ready for
release-note handoff.

The remaining work is not another broad refactor. It is to turn the completed
generation evidence into concise release and consumer-upgrade messaging.

## Goal

Prepare the release-note handoff for the `g06` Rust platform-contract
transition.

## Scope

In scope:

- summarize the Rust public API and contract changes
- call out additive safe-boundary APIs and internal module splits
- call out the one known breaking Rust trait surface
- state that the named six-consumer family needs no app update
- preserve validation evidence from `g06.014`
- classify remaining scanner findings as backlog

Out of scope:

- release execution or publishing
- another Rust refactor batch
- TS validation drift cleanup
- consumer feature migrations

## Contract References

- `001`: working rules
- `023`: release and compatibility rollout
- `030`: auth and session systems
- `040`: storage, blob, and media systems
- `120`: tooling, testing, and contract artifacts
- `122`: Rust public API inventory

## Acceptance Criteria

- release-note handoff says what changed and why
- breaking-change section names `SessionStore::rotate_session_if_current`
- consumer-impact section says no named app update is required
- validation section cites `g06.014`
- remaining scanner backlog is not presented as release-blocking

## Release-Note Handoff

The release-facing compatibility note is now in
[`docs/guides/190-upgrade-compatibility.md`](../../guides/190-upgrade-compatibility.md)
under `Rust Platform Contract Transition (2026-06-05)`.

Summary:

- Impact is breaking only for unknown direct
  `underlay_auth_jwt::SessionStore` implementers.
- The named six-consumer family needs no app update.
- Additive typed boundaries are available for DB identifiers/schema helpers,
  blob object keys, HTTP cookie/CSRF construction, media repository/table
  helpers, and migration-bundle references.
- Internal module splits preserve public root exports for WebAuthn, S3 blob,
  media PostgreSQL, Nightfire media usage, devtools CLI, and migration
  pipeline code.
- Validation evidence is retained in `g06.014`.
- Remaining scanner and supply-chain findings are classified as hardening
  backlog, not consumer migration work.

## Hardening Follow-Up

The rerun audit after commit/push found the next bounded hardening batch:

- add repo-owned `cargo-deny` policy and avoid default-policy license noise
- add advisory scanning through the Effigy surface or documented local tooling
- update the yanked `wasm-bindgen 0.2.111` lockfile path
- normalize `underlay-http::error_logging` dynamic filter SQL with a typed
  query builder
- keep remaining high-sized Rust files on backlog without reopening broad
  structural cleanup by default

## Current State

`g06.015` is complete.

## Next Task

Execute `g06.016`: Rust platform hardening backlog batch.
