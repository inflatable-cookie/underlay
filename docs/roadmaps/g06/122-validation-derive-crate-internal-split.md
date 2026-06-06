# g06.122 - Validation Derive Crate Internal Split

## Why

`g06.121` found that `underlay-validation-derive/src/lib.rs` mixes the public
proc macro entry with input shape validation, field traversal, validator
attribute parsing, token generation, and compile-time error handling.

The next split should make the derive macro easier to reason about without
changing macro syntax or generated behavior.

## Goal

Split the validation derive crate into focused private modules while preserving
the public `#[derive(Validate)]` macro contract.

## Scope

In scope:

- keep `lib.rs` as the proc macro front door
- move struct-shape validation and generated impl construction into `derive.rs`
- move field-level validation collection into `field.rs`
- move `#[validate(...)]` rule parsing and validator token generation into
  `rules.rs`
- keep helper functions crate-private
- preserve existing derive tests and integration tests

Out of scope:

- changing validation public APIs
- changing derive macro syntax
- changing generated validation behavior
- changing consumer apps

## Acceptance Criteria

- the proc macro entry remains the only exported macro surface
- helper logic is grouped by stable compile-time responsibility
- derive crate tests pass with `--all-features`
- downstream validation derive integration tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal proc-macro implementation split. If macro syntax or
generated behavior must change, stop and re-enter planning.

## Current State

`g06.122` is complete.

Artifact:

- [122 artifact](./122-validation-derive-crate-internal-split-artifact.md)

## Next Task

Execute `g06.123`: media Nightfire walk modularity audit.
