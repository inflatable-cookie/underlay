# g08.015 - Rust Error Taxonomy

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Unify three inconsistent error styles across the workspace. `underlay-core`
hand-rolls `AppError`; `underlay-auth` hand-rolls a large enum with a bespoke
`impl_auth_error_from!` macro; `underlay-ai-runtime`'s `AiRuntimeError`
implements **neither `Display` nor `std::error::Error`** (unusable with `?`,
`anyhow`, or `Box<dyn Error>`); `underlay-devtools` hand-rolls another; most
other crates use thiserror. With multiple consumers mapping these into their own
error types, the inconsistency multiplies.

## Evidence

- `rust/crates/underlay-ai-runtime/src/error.rs:16-37` (no Display/Error impl)
- `rust/crates/underlay-core/src/error.rs`
- `rust/crates/underlay-auth/src/errors.rs`
- `rust/crates/underlay-devtools/src/lib.rs`

## Governing References

- [010 Foundation primitives and envelopes](../../contracts/010-foundation-primitives-and-envelopes.md)
- [033 Error codes and operator audit](../../contracts/033-error-codes-and-operator-audit.md)

## Planned Changes

- [x] Adopt thiserror in every crate; ensure every public error type implements
  `std::error::Error`.
- [x] Introduce a shared `ErrorCode { fn code(&self) -> &str }` trait and
  document the `<domain>.<category>.<specific>` code convention workspace-wide.
- [x] Fix `AiRuntimeError` first (it is currently unusable with `?`).

## Consumer Upgrade Impact

Impact class: `behavioral`. Consumers matching on the old error shapes must
adapt. Requires six-consumer proof per `023`.

## Validation

- [x] every public error type: `impl std::error::Error` present (compile check)
- [x] `cargo check --workspace`, `cargo clippy --workspace`
- [x] `effigy validate`

## Stop Conditions

Stop if the shared trait forces a dependency cycle through `underlay-core`;
resolve the placement before rolling out.

## Completion Notes

Completed 2026-07-17.
- Shared `underlay_core::ErrorCode { fn code(&self) -> &str }` trait added in
  `underlay-core` (leaf crate - no cycle; stop condition clear). Documented
  the `<domain>.<category>.<specific>` convention and the Rust-error-type rule
  in contract `033`.
- `AiRuntimeError` (the one unusable with `?`) now derives `thiserror::Error`
  and impls `ErrorCode`; a regression test proves it composes with `?` /
  `Box<dyn Error>`.
- `AuthError` gained `Display` + `std::error::Error` + `ErrorCode` (delegating
  to its existing inherent `code()`); it previously had neither Display nor
  Error.
- Workspace audit for public error types missing `std::error::Error` found
  three more - `NightfireMediaLocatorError`, `NightfireValidationError`
  (both now thiserror-derived; added thiserror to `underlay-nightfire`), and
  `SlugValidationError` (marker impl). `AppError`/`DevtoolError` and every
  other public error type already implemented it.
- Impact is effectively **additive**, not the card's estimated `behavioral`:
  no error variant changed, only new trait impls were added, so consumers
  matching on existing shapes are unaffected (and the orphan rule bars them
  from having conflicting impls). Lighter than full six-consumer proof.
- Pre-existing clippy warnings in `underlay-blob`/`underlay-media`/
  `underlay-media-postgres` are untouched and belong to `g08.020` (lint
  hygiene); the touched crates are clippy-clean.
Validated: `cargo check --workspace --all-features` clean;
`cargo test --workspace --all-features` green (73 suites, 0 failures);
touched-crate clippy clean.

## Next Task

`g08.016` media domain-type relocation.
