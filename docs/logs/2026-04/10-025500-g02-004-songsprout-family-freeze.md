# 2026-04-10 02:55 - g02.004 Songsprout family freeze

## Summary

Completed `g02.004` Batch 4.1 by inventorying the active Songsprout UI
families and freezing the first rollout classification for `greenhouse` and
`bloom`.

## Why this mattered

The Songsprout gate needed to start from real route evidence rather than a
generic “Songsprout” label. The split between operator-facing `greenhouse`,
artist-facing `bloom`, and non-UI `stem` materially changes what should be
treated as shared rollout versus app-local workflow composition.

## Changes

- recorded the active route families for `greenhouse` and `bloom`
- classified the first direct-rollout targets:
  - overview shells
  - `bloom` signed-in security
  - `bloom` workflow browse routes
  - `greenhouse` ops browse shell
- classified the strongest local exceptions:
  - `greenhouse` catalogue browse and artist detail
  - workflow-local task/program/status rendering
  - local workflow copy and card/list posture
- explicitly deferred public auth entry routes, billing routes, `stem`, and
  Rust-side route work

## Validation

- local roadmap review in
  `~/Dev/projects/underlay/docs/roadmaps/g02`

## Next Task

Execute `g02.004` Batch 4.2: normalize the strongest direct-rollout Songsprout
families onto the frozen proof-app posture, starting with the overview shells,
`bloom` security, `bloom` workflow browse routes, and the `greenhouse` ops
browse shell, while keeping workflow-local copy and rendering app-owned.
