# g07.028 - Doctor Warning Rust Cleanup

Status: complete
Owner: repo maintainers
Roadmap: `g07`
Depends on: `001`, `122`

## Scope

First Effigy doctor warning cleanup batch:

- repair roadmap front-door drift after `g07.027`
- trim the comment-ratio warning in `underlay-media` storage docs
- split the oversized `underlay-devtools` migration-bundle model file without
  changing public exports

## Goals

- [x] `docs/roadmaps/README.md` and `generation-index.md` agree on active
  generation.
- [x] `rust/crates/underlay-media/src/storage/mod.rs` no longer triggers the
  comment-ratio warning.
- [x] `rust/crates/underlay-devtools/src/migration_bundle/model.rs` becomes a
  small model front door.
- [x] Devtools public exports stay stable.

## Acceptance Criteria

- [x] `cargo test -p underlay-media`
- [x] `cargo test -p underlay-devtools`
- [x] `effigy doctor`
- [x] `effigy rust:check`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`

## Validation Notes

`effigy doctor` now reports one warning family, `scan.god-files`, with 10
remaining findings. The cleared findings are the `underlay-media` comment-ratio
warning and `underlay-devtools` migration-bundle model god-file warning.

## Stop Conditions

- A split would require changing crate-root public exports.
- Doctor output shows a new error-level finding.
- A cleanup starts changing behavior instead of file shape.

## Next Task

Continue with `g07.029`.
