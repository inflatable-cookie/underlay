# g06.011 - Second Rust Structural Split Repair Batch

## Why

`g06.010` removed the devtools CLI god-file finding and downgraded
`underlay-migration-core/src/pipeline.rs` from critical to high.

The remaining critical Rust findings are now explicit and small enough for the
next scoped batch.

## Goal

Remove or downgrade the remaining critical Rust god-file findings without
changing public behavior.

## Scope

In scope:

- split `rust/crates/underlay-migration-core/src/tests/lib_tests.rs` into
  focused test modules
- split `rust/crates/underlay-media/src/nightfire.rs` around stable internal
  responsibilities
- keep public exports, test semantics, and Nightfire media behavior stable
- rerun focused Rust validation for touched crates

Out of scope:

- high-level Rust god-file cleanup outside the two critical files
- TS god-file cleanup
- broad media repository contract changes
- release execution or publishing

## Contract References

- `001`: working rules
- `040`: storage, blob, and media systems
- `070`: Nightfire and migration systems
- `120`: tooling, testing, and contract artifacts
- `122`: Rust public API inventory

## Acceptance Criteria

- both remaining critical Rust god-file findings are removed or downgraded
- targeted migration-core and media checks pass
- public API drift is avoided
- remaining high/warning scanner backlog stays visible

## Changes

- Split `rust/crates/underlay-migration-core/src/tests/lib_tests.rs` into
  focused modules:
  - `pipeline_basic_tests.rs`
  - `pipeline_decision_tests.rs`
  - `pipeline_integrity_tests.rs`
  - `support.rs`
- Split `rust/crates/underlay-media/src/nightfire.rs` into:
  - `nightfire/registry.rs` for public block handler and registration-map types
  - `nightfire/walk.rs` for private traversal, locator, and nested-block helpers
- Kept public re-exports stable through `underlay-media/src/nightfire.rs`.

## Validation

- `cargo test -p underlay-migration-core --all-features`
- `cargo test -p underlay-media --all-features nightfire`
- `cargo check -p underlay-media --all-features`
- `effigy scan god-files`

Scanner impact:

- `effigy scan god-files` now reports zero critical findings.
- Remaining backlog is 62 findings: 19 high, 43 warning.
- The highest remaining Rust findings are now high-level, not critical:
  - `underlay-migration-core/src/pipeline.rs`
  - `underlay-auth-jwt/src/tests/service_tests.rs`
  - `underlay-media/src/tests/nightfire_tests.rs`
  - `underlay-auth-webauthn/src/lib.rs`
  - `underlay-jobs/src/types.rs`
  - `underlay-blob/src/adapters/s3.rs`

## Current State

`g06.011` is complete.

## Next Task

Execute `g06.012`: high-severity Rust structural backlog triage and closeout
decision.
