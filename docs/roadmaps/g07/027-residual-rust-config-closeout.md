# g07.027 - Residual Rust Config Closeout

Status: complete
Owner: repo maintainers
Roadmap: `g07`
Depends on: `001`, `023`, `122`

## Scope

Close out the residual Rust public config, option, and policy field sweep after:

- `g07.024`: migration-core runtime policy fields
- `g07.025`: devtools bundle/seed option fields
- `g07.026`: migration-core governance, OCI, and manifest policy model audit

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

Prior residual Rust breaking surfaces were limited to direct field construction
of:

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
- [x] Close this lane without opening a new compatibility-retirement batch by
  implication.

## Acceptance Criteria

- [x] Residual public config/options scan reviewed.
- [x] Consumer literal scans for scoped residual Rust types are clean.
- [x] `effigy rust:check`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`

## Stop Conditions

- A remaining public field is a config/option/policy surface with construction
  invariants and live consumer use.
- A consumer directly constructs one of the newly retired residual Rust types.
- Closing this lane would leave an active queue item with no owner.

## Next Task

No active residual Rust field-retirement task remains. Re-enter planning before opening another Rust
compatibility-retirement lane.
