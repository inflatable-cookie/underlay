# g08.002 - Devtools Option Field Retirement

Status: complete
Owner: repo maintainers
Roadmap: `g08`
Depends on: `001`, `023`, `122`

## Scope

Retire public fields on:

- `underlay_devtools::BundleBuildOptions`
- `underlay_devtools::BundlePublishOptions`
- `underlay_devtools::BundlePullOptions`
- `underlay_devtools::BundleRunOptions`
- `underlay_devtools::SeedBundleBuildOptions`
- `underlay_devtools::SeedBundlePullOptions`

Report structs remain public data records. Raw OCI refs remain accepted at the
publish/pull CLI/tooling edge because tag references are valid there; replay
still uses `MigrationBundleRef` for digest-pinned refs.

## Consumer Upgrade Impact

Impact: breaking for direct field construction or mutation of scoped devtools
option types.

Current six-consumer scan found no named consumer direct construction of these
types.

## Goals

- [x] Add constructors/builders/accessors for scoped option types.
- [x] Move devtools internals, CLI entrypoints, and tests off direct field
  construction/access.
- [x] Preserve report structs and raw CLI string behavior.
- [x] Prove current consumers do not need code changes.

## Acceptance Criteria

- [x] `cargo test -p underlay-devtools`
- [x] `cargo check -p underlay-devtools`
- [x] Consumer scan for scoped option literals remains clean.
- [x] `effigy rust:check`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`

## Stop Conditions

- A named consumer owns direct option construction that requires app migration
  in the same batch.
- Raw OCI tag refs become invalid at publish/pull edges.
- Report structs or bundle package JSON shapes need to change.

## Next Task

After this batch, continue with `g08.003`.
