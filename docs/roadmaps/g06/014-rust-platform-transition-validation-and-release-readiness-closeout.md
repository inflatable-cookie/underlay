# g06.014 - Rust Platform Transition Validation And Release-Readiness Closeout

## Why

`g06` has completed the main Rust platform-contract transition work:

- typed safety boundaries
- auth/session reset
- HTTP cookie/CSRF safe builders
- DB identifier normalization
- media/devtools adapter splits
- six-consumer compatibility proof
- critical god-file cleanup
- security-adjacent adapter splits

The next step is a validation and release-readiness pass, not another broad
repair by default.

## Goal

Confirm the current `g06` state is ready for release-note handoff or identify a
small blocking follow-up.

## Scope

In scope:

- rerun targeted Rust validation for touched platform crates
- rerun docs QA and diff hygiene checks
- summarize remaining known scanner and validation backlog
- decide whether `g06` should close, continue with one more bounded repair, or
  pause for release-note review

Out of scope:

- fixing every high/warning scanner finding
- TS validation drift cleanup
- consumer feature migrations
- release execution or publishing

## Contract References

- `001`: working rules
- `023`: release and compatibility rollout
- `030`: auth and session systems
- `040`: storage, blob, and media systems
- `120`: tooling, testing, and contract artifacts
- `122`: Rust public API inventory

## Acceptance Criteria

- touched Rust platform crates have current focused validation
- docs QA and diff hygiene pass or failures are classified
- remaining scanner backlog is explicit
- next release-readiness decision is explicit

## Breaking-Change Assessment

The latest `g06.013` changes are not breaking for consumers.

They are internal module splits with crate-root/public re-exports preserved:

- `underlay_auth_webauthn::WebAuthnService`
- `underlay_auth_webauthn::WebAuthnConfig`
- WebAuthn request/response/storage types
- `underlay_blob::S3Adapter`
- `underlay_blob::S3Config`

No consuming app code needs to change for `g06.013`.

Across the broader `g06` generation, the known breaking surface is still the
`underlay-auth-jwt` `SessionStore::rotate_session_if_current` signature and
atomic refresh-rotation contract. The named six-consumer family has no direct
consumer-owned `SessionStore` implementation, so no app update is required
there either.

Earlier consumer-touching `g06` changes were either additive/internal or were
already migrated in the affected apps:

- typed cookie setters and shared CSRF cookie helpers
- DB identifier/schema helpers
- typed blob object keys
- media/devtools module boundaries
- soft-delete macro compatibility fix

## Consumer Validation

Current six-consumer checks all pass:

| Consumer root | Check | Result |
| --- | --- | --- |
| `underlay-reference/acme-api` | `cargo check -p acme-api` | passed |
| `contact-patch/cp-api` | `cargo check -p cp-api` | passed |
| `compli-me/api` | `cargo check -p compli-me-api` | passed |
| `acowtancy/farmyard` | `cargo check -p farmyard-api` | passed |
| `songsprout/nursery` | `cargo check -p nursery-api` | passed |
| `loophole/composer/composer-api` | `cargo check -p composer-api` | passed |

## Underlay Validation

Current focused Underlay validation passes:

- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- `cargo fmt -p underlay-db -p underlay-blob -p underlay-http -p underlay-auth-jwt -p underlay-auth-webauthn -p underlay-media -p underlay-devtools -p underlay-migration-core -p underlay-audit -p underlay-security-alerts -p underlay-soft-delete --check`
- `cargo test -p underlay-db --all-features`
- `cargo test -p underlay-http --all-features`
- `cargo test -p underlay-auth-jwt --all-features`
- `cargo test -p underlay-media --all-features`
- `cargo test -p underlay-devtools --all-features`
- `cargo test -p underlay-blob --all-features`
- `cargo test -p underlay-auth-webauthn --all-features`
- `cargo test -p underlay-migration-core --all-features`
- `cargo test -p underlay-audit --all-features`
- `cargo test -p underlay-security-alerts --all-features`
- `cargo test -p underlay-soft-delete --all-features`

`cargo fmt --all --check` still reports the unrelated
`underlay-aws/src/tests/lib_tests.rs` assertion wrapping difference. The
touched-crate formatting check above passes and this card does not normalize
that unrelated file.

`effigy doctor` still fails on non-critical structural backlog:

- `scan.attention-markers`: 11 findings
- `scan.comment-ratio`: 12 findings
- `scan.god-files`: 61 findings

The god-file scanner now reports `critical=0`, `high=17`, and `warning=44`.
The remaining high bucket is backlog, not a release blocker for this Rust
platform-contract transition.

## Release-Readiness Decision

`g06` is ready for release-note handoff.

No current named consumer app update is required. The release note still needs
to call out the `underlay-auth-jwt` `SessionStore::rotate_session_if_current`
trait signature and atomic refresh-rotation contract for any unknown external
direct implementers.

## Current State

`g06.014` is complete.

## Next Task

Execute `g06.015`: Rust platform transition release-note handoff.
