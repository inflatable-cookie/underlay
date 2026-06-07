# g07.037 - OAuth Rust File Cleanup

Status: complete
Owner: repo maintainers
Roadmap: `g07`
Depends on: `001`, `122`

## Scope

Close the final Effigy doctor warning by splitting
`rust/crates/underlay-auth-oauth/src/lib.rs` into focused modules:

- app service orchestration
- config
- error mapping
- public models
- provider trait

## Goals

- [x] `lib.rs` becomes a small crate front door.
- [x] Public OAuth exports remain stable.
- [x] OAuth app-service behavior remains unchanged.
- [x] `effigy doctor` reports no findings.

## Acceptance Criteria

- [x] `cargo test -p underlay-auth-oauth`
- [x] `effigy doctor`
- [x] `effigy rust:check`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`

## Validation Notes

`cargo test -p underlay-auth-oauth` passed with 11 tests. `effigy doctor`
reported no findings.

## Next Task

No active doctor-warning closeout work remains.
