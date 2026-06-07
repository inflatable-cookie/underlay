# g07.024 - Migration-Core Policy Field Retirement

Status: complete
Owner: repo maintainers
Roadmap: `g07`
Depends on: `001`, `023`, `122`

## Scope

Retire public fields on:

- `underlay_migration_core::PipelinePolicy`
- `underlay_migration_core::AiThresholdPolicy`
- `underlay_migration_core::IntegrityPolicy`

Keep serialized shapes stable. This batch does not change public report structs,
bundle manifests, governance-policy JSON, or devtools option structs.

## Consumer Upgrade Impact

Impact: breaking for direct field construction or mutation of the scoped policy
types.

Current six-consumer scan found no named consumer direct field construction.
`acowtancy/farmyard` uses `PipelinePolicy::default()` only.

## Goals

- [x] Add constructors/builders/accessors for scoped policy types.
- [x] Move Underlay internals and tests off direct field reads/writes.
- [x] Preserve serde field names and default behavior.
- [x] Prove current consumers do not need code changes.

## Acceptance Criteria

- [x] `cargo test -p underlay-migration-core`
- [x] `cargo check -p underlay-devtools`
- [x] Consumer scan for scoped policy literals and field access remains clean.
- [x] `effigy rust:check`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`

## Stop Conditions

- A named consumer owns direct policy construction that cannot move to builders
  without changing migration behavior.
- Serde compatibility requires field visibility for a scoped type.
- The batch starts changing governance, OCI, manifest, or devtools option model
  semantics.

## Next Task

After this batch, continue with `g07.025`.
