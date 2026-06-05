# g06.013 - Security-Adjacent Rust Adapter Split Batch

## Why

`g06.012` classified the remaining high-severity Rust structural backlog.

The next best scoped repair is the production code closest to security and
storage boundaries.

## Goal

Split the high-severity auth WebAuthn and blob S3 adapter files into smaller
internal modules without changing public behavior.

## Scope

In scope:

- split `rust/crates/underlay-auth-webauthn/src/lib.rs` around config,
  challenge/session state, credential conversion, and service behavior
- split `rust/crates/underlay-blob/src/adapters/s3.rs` around config/client,
  upload/download/delete behavior, and key/request handling
- keep public exports and adapter behavior stable
- rerun focused Rust validation for touched crates

Out of scope:

- changing WebAuthn policy semantics
- changing S3 request signing or storage-key behavior
- broader auth provider refactors
- TS scanner findings
- release execution or publishing

## Contract References

- `001`: working rules
- `030`: auth and session systems
- `040`: storage, blob, and media systems
- `122`: Rust public API inventory

## Acceptance Criteria

- both target files are downgraded or materially reduced by stable module
  boundaries
- focused auth-webauthn and blob checks pass
- public API drift is avoided
- remaining scanner backlog stays visible

## Changes

- Split `rust/crates/underlay-auth-webauthn/src/lib.rs` into:
  - `error.rs`
  - `service.rs`
  - `types.rs`
- Kept attestation feature exports and public WebAuthn types available through
  the crate root.
- Split `rust/crates/underlay-blob/src/adapters/s3.rs` into:
  - `s3/config.rs`
  - `s3/client.rs`
  - the retained adapter implementation in `s3.rs`
- Kept `S3Adapter` and `S3Config` public exports stable through
  `underlay_blob::adapters` and the crate root.

## Validation

- `cargo test -p underlay-auth-webauthn --all-features`
- `cargo check -p underlay-auth-webauthn --all-features`
- `cargo test -p underlay-blob --all-features`
- `cargo check -p underlay-blob --all-features`
- `effigy scan god-files`

Scanner impact:

- critical findings remain at 0.
- high findings dropped from 19 to 17.
- `underlay-auth-webauthn/src/lib.rs` no longer appears in high findings.
- `underlay-blob/src/adapters/s3.rs` no longer appears in high findings.

## Current State

`g06.013` is complete.

## Next Task

Execute `g06.014`: Rust platform transition validation and release-readiness
closeout.
