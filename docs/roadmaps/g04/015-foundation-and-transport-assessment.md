# 015 - Foundation And Transport Assessment

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.014` closes the contract-coverage wave and compiles the implementation
assessment order.

The first execution lane has to start at the bottom of the stack. `010` and
`020` carry the shared identifier, envelope, validation-normalization, query,
pagination, HTTP-client, cookie, and server-boundary contracts that every
higher system inherits.

## Goals

- assess the live implementation against `010` and `020`
- confirm which current drift is real contract failure versus only awkward
  packaging
- identify the smallest honest repair set needed to bring the lower transport
  layer into contract
- leave explicit findings and follow-on repair cards instead of vague drift

## Non-Goals

- assessing auth and higher layers in the same batch
- broad API redesign without concrete contract failure evidence
- consumer rollout work

## Inputs

- [docs/contracts/010-foundation-primitives-and-envelopes.md](/Users/tom/Dev/projects/underlay/docs/contracts/010-foundation-primitives-and-envelopes.md)
- [docs/contracts/020-http-transport-and-server-boundary.md](/Users/tom/Dev/projects/underlay/docs/contracts/020-http-transport-and-server-boundary.md)
- `rust/crates/underlay-core/**`
- `rust/crates/underlay-validation/**`
- `rust/crates/underlay-http/**`
- `rust/crates/underlay-http-client/**`
- `ts/src/client/http.ts`
- `ts/src/client/errors.ts`
- `ts/src/client/query.ts`
- `ts/src/client/types.ts`
- `ts/src/server/csp*.ts`
- [`contracts/openapi/underlay.openapi.yaml`](/Users/tom/Dev/projects/underlay/contracts/openapi/underlay.openapi.yaml)

## Exit Criteria

- the live lower transport implementation is reviewed against `010` and `020`
- the real findings are documented in severity order
- the next repair step is expressed as one bounded roadmap lane or a small
  repair set
- higher-layer assessment can start without ambiguity about the shared lower
  boundary

## Findings

### 1. Rust canonical error envelopes still serialize the wrong field name

Severity: high

The contract-canonical wire key is `error.fieldErrors`, and both the TS client
types and the checked-in OpenAPI artifact already assume that shape. The live
Rust lower layer still serializes `field_errors` instead.

Evidence:

- [`rust/crates/underlay-core/src/dto.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-core/src/dto.rs)
- [`rust/crates/underlay-core/src/error.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-core/src/error.rs)
- [`rust/crates/underlay-http/src/openapi.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/openapi.rs)
- [`rust/crates/underlay-core/src/tests/core_tests.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-core/src/tests/core_tests.rs)

Impact:

- TS callers reading `error.fieldErrors` do not see Rust field-level feedback
  from the canonical shared handler path
- the Rust serialization surface, TS types, and machine-readable OpenAPI
  artifact no longer describe one shared transport contract

### 2. `ValidatedJsonRejection` bypasses canonical normalization and leaks the
internal validation shape

Severity: high

`ValidatedJsonRejection::ValidationError` manually constructs JSON instead of
going through `AppError`/`ApiError` normalization. It emits the wrong field
name and also serializes `HashMap<String, FieldError>` values directly, so the
wire payload contains nested `{ message, code }` objects instead of the
contract-canonical `Record<string, string>` field-error map.

Evidence:

- [`rust/crates/underlay-validation/src/axum_integration.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-validation/src/axum_integration.rs)
- [`rust/crates/underlay-validation/src/error.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-validation/src/error.rs)
- [`rust/crates/underlay-validation/src/tests/axum_integration_tests.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-validation/src/tests/axum_integration_tests.rs)
- [`rust/crates/underlay-validation/src/tests/error_tests.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-validation/src/tests/error_tests.rs)

Impact:

- validation failures from `ValidatedJson` do not match either the contract or
  the rest of the shared HTTP error path
- richer internal validation structures leak onto the wire from the lowest
  extraction seam

### 3. TS transport authority is behaviorally aligned but structurally diffuse

Severity: medium

The transport behavior is mostly aligned now: query serialization, replacement
merge behavior, retry bounds, and page/limit vocabulary all match the contract
well enough. The remaining issue is ownership clarity.

Evidence:

- [`ts/src/client/types.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/types.ts)
- [`ts/src/client/query.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/query.ts)
- [`ts/src/client/pagination.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/pagination.ts)

Notes:

- `ts/src/client/types.ts` mixes the lower primitive envelope contract with
  higher auth and restore-domain types
- `ts/src/client/pagination.ts` is still only a re-export from pattern types
- this is a packaging problem, not a first-priority wire-compatibility failure

## Assessment Result

The first bounded repair lane should focus on lower transport envelope
normalization, not on broader TS transport package cleanup.

That means:

- fix the shared Rust error wire shape first
- normalize `ValidatedJsonRejection` through the canonical error path
- add direct tests for the repaired wire contract
- defer the TS surface-ownership cleanup to a later narrower assessment/repair
  lane once the lower compatibility break is closed

## Next Task

Execute `g04.016`: repair the lower transport error normalization path before
promoting the next higher assessment lane.
