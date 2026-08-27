# 2026-08-27 10:10:32 - g09.047 Promotion Gate Checkpoint

## Outcome

Two g09.047 promotion gates are now proven. Underlay `v0.9.5` publishes the
env-authority tool, and Underlay Reference is clean at exact `origin/main`.
The roadmap remains planned because three app-owner security decisions are not
yet explicit.

## Evidence

- Underlay release commit and annotated tag: `8ffafb92`, `v0.9.5`
- GitHub Release: `v0.9.5`, published 2026-08-27
- hosted Rust CI passed on the exact pre-release candidate and release commit
- tagged-consumer smoke resolved `underlay-core v0.9.5` and the TypeScript
  workspace-shape/env-authority exports through the documented SSH tag form
- Underlay Reference: clean `main == origin/main == 854e5ad2`

## Remaining Decisions

- mandatory secret classes by environment
- environments allowed to disable CSRF, with fail-closed behavior elsewhere
- fatal-versus-warn policy for malformed deployed config and insecure deployed
  cookies

## Next Task

Get those three decisions from the Underlay Reference owner. Then mark g09.047
ready and publish its worker handoff; do not dispatch before that promotion.
