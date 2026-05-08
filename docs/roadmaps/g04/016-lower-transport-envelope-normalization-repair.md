# 016 - Lower Transport Envelope Normalization Repair

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.015` assessed the live implementation against the `010` and `020`
contracts.

That assessment found a real lower-layer contract failure, not just authority
drift:

- Rust still serializes shared error field feedback as `field_errors`
- `ValidatedJsonRejection` bypasses canonical `AppError`/`ApiError`
  normalization and leaks the richer `ValidationError` value shape directly

This repair lane exists to fix the canonical wire shape before higher-layer
assessment starts depending on the lower boundary.

## Goals

- normalize the shared Rust error wire shape to the contract-canonical
  `error.fieldErrors`
- stop `ValidatedJsonRejection` from leaking internal validation structures
  onto the wire
- align Rust serialization, TS client expectations, OpenAPI artifacts, and
  tests on one error-envelope contract
- leave the remaining TS transport ownership drift for a later narrower lane

## Non-Goals

- broad transport package reorganization in the same batch
- auth, storage, or higher-layer repair
- introducing a second compatibility envelope shape as a permanent contract

## Inputs

- [docs/roadmaps/g04/015-foundation-and-transport-assessment.md](/Users/tom/Dev/projects/underlay/docs/roadmaps/g04/015-foundation-and-transport-assessment.md)
- [docs/contracts/010-foundation-primitives-and-envelopes.md](/Users/tom/Dev/projects/underlay/docs/contracts/010-foundation-primitives-and-envelopes.md)
- [docs/contracts/020-http-transport-and-server-boundary.md](/Users/tom/Dev/projects/underlay/docs/contracts/020-http-transport-and-server-boundary.md)
- `rust/crates/underlay-core/**`
- `rust/crates/underlay-http/**`
- `rust/crates/underlay-validation/**`
- `contracts/openapi/underlay.openapi.yaml`
- `ts/src/client/errors.ts`
- `ts/src/client/types.ts`

## Exit Criteria

- Rust shared error envelopes serialize `fieldErrors` on the wire
- `ValidatedJsonRejection` normalizes validation failures to the canonical
  string-map transport shape
- tests cover the canonical wire shape directly
- OpenAPI and TS lower transport types still match the repaired wire contract

## Result

Completed.

Implemented:

- Rust shared error envelopes now serialize `fieldErrors` on the wire through
  the canonical shared `ErrorBody`
- `ValidatedJsonRejection` now normalizes validation failures into the
  canonical shared envelope with a flattened `Record<string, string>` field
  map instead of leaking `FieldError` objects
- direct lower-layer tests now assert the repaired wire shape in
  `underlay-core`, `underlay-http`, and `underlay-validation`

Validated with:

- `cargo test -p underlay-core -p underlay-http -p underlay-validation --all-features`

Remaining lower transport drift:

- TS transport ownership is still somewhat diffuse across `client/types.ts` and
  `client/pagination.ts`, but that is now a packaging/authority cleanup issue,
  not a wire-compatibility failure blocking higher-layer assessment

## Next Task

Execute `g04.017`: assess the live auth and session implementation against
`030`.
