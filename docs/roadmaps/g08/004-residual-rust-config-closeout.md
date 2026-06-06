# g08.004 - Residual Rust Config Closeout

Status: complete
Owner: repo maintainers
Roadmap: `g08`
Depends on: `001`, `023`, `122`

## Scope

Close out the residual Rust public config, option, and policy field sweep after:

- `g08.001`: migration-core runtime policy fields
- `g08.002`: devtools bundle/seed option fields
- `g08.003`: migration-core governance, OCI, and manifest policy model audit

## Findings

The remaining public fields are retained categories:

- serialized DTOs and API/report payloads
- migration-core stage/run/checkpoint records
- devtools report and package records
- governance and manifest policy JSON document records
- blob/media request/response objects
- soft-delete restore diagnostics

These are data records, not mutable config surfaces with hidden invariants. The
right contract is stable serialized shape plus validator/evaluator behavior,
not private fields and builders.

## Consumer Upgrade Impact

Impact: no new consumer code change in this closeout card.

Prior `g08` breaking surfaces were limited to direct field construction of:

- `PipelinePolicy`
- `AiThresholdPolicy`
- `IntegrityPolicy`
- devtools bundle/seed option structs
- `OciBundleConfig`

Current six-consumer scans found no direct construction of those scoped types.

## Goals

- [x] Confirm no remaining app-facing config/option/policy field retirement
  target is unclassified.
- [x] Classify retained public-field records as DTO/report/document shapes.
- [x] Confirm current consumers do not construct the scoped retired types.
- [x] Close `g08` without opening a new compatibility-retirement batch by
  implication.

## Acceptance Criteria

- [x] Residual public config/options scan reviewed.
- [x] Consumer literal scans for `g08` scoped types are clean.
- [x] `effigy rust:check`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`

## Stop Conditions

- A remaining public field is a config/option/policy surface with construction
  invariants and live consumer use.
- A consumer directly constructs one of the newly retired `g08` types.
- Closing `g08` would leave an active queue item with no owner.

## Next Task

No active `g08` task remains. Re-enter planning before opening another Rust
compatibility-retirement lane.
