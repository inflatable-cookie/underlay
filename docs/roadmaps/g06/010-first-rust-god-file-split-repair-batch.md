# g06.010 - First Rust God-File Split Repair Batch

## Why

`g06.009` classified the remaining `effigy doctor` scanner backlog.

The highest-value current-lane issue is not every large file. It is the Rust
tooling and migration-core mass that sits directly on the platform-contract
boundary.

## Goal

Split the first batch of oversized Rust implementation files without changing
public behavior or widening public APIs.

## Scope

In scope:

- split `rust/crates/underlay-devtools/src/bin/underlay-devtools.rs` into
  focused command modules
- split the largest `underlay-migration-core` pipeline/test mass where a stable
  internal responsibility boundary is already visible
- keep command output, exit behavior, and public crate exports stable
- rerun focused Rust validation for touched crates

Out of scope:

- auth, blob, jobs, media, HTTP, and TS god-file cleanup
- changing migration-core plugin contracts
- release execution or publishing
- suppressing scanner findings by raising thresholds

## Contract References

- `001`: working rules
- `120`: tooling, testing, and contract artifacts
- `122`: Rust public API inventory

## Acceptance Criteria

- devtools CLI parsing/dispatch is split into smaller modules
- selected migration-core pipeline/test structure is split without public API
  drift
- targeted Rust checks pass for touched crates
- remaining scanner backlog is still visible after this batch

## Changes

- Split `rust/crates/underlay-devtools/src/bin/underlay-devtools.rs` into
  focused CLI modules for usage, sync migrations, migration bundle/run/report,
  and seed bundle commands.
- Split migration pipeline public report and stage-output types into
  `rust/crates/underlay-migration-core/src/pipeline/types.rs` and re-exported
  them through the existing `pipeline` module.
- Kept CLI output strings, exit paths, crate exports, and migration-core plugin
  contracts stable.

## Validation

- `cargo check -p underlay-devtools --all-features --bins`
- `cargo test -p underlay-devtools --all-features`
- `cargo check -p underlay-migration-core --all-features`
- `cargo test -p underlay-migration-core --all-features`
- `effigy scan god-files`

Scanner impact:

- `rust/crates/underlay-devtools/src/bin/underlay-devtools.rs` no longer
  appears in god-file findings.
- `rust/crates/underlay-migration-core/src/pipeline.rs` moved from critical to
  high: 664 code lines / 722 total lines.
- Remaining god-file findings are still visible: 59 total, with two critical
  Rust files left.

## Current State

`g06.010` is complete.

## Next Task

Execute `g06.011`: second Rust structural split repair batch for remaining
critical files.
