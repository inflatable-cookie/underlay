# g06.183 Artifact - Media Authority Status Drift Cleanup

Status: complete
Owner: repo maintainers
Completed: 2026-06-06

## Purpose

Close stale contract drift that still described
`050-media-library-and-usage.md` as proposed.

## Result

`050-media-library-and-usage.md` is an active contract.

The media authority stack is now explicit in active contract state:

- `040` owns blob, storage, repository, adapter, and lower media mechanics
- `050` owns usage graph, structured-content sync, migration binding, and
  media-linked content semantics
- `070` may still assess TS Nightfire protocol strength, but no longer treats
  the media-linked content authority chain as blocked by a stale proposed label

The real open media questions remain in `040`:

- whether `TypedExistsCheck` over-assumes soft-delete defaults
- whether `MediaRepository` should continue to expose older simple usage
  helpers alongside generalized usage-edge sync
- whether `underlay_blob::MediaConfig` should remain in the blob crate

## Consumer Upgrade Impact

Impact class: `none`.

This is documentation and contract-state cleanup only.

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`

## Next Task

No active roadmap task remains. Open a bounded roadmap card before starting the
next compatibility-retirement, TS boundary, or Rust hardening lane.
