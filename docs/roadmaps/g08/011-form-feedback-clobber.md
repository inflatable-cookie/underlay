# g08.011 - Form-Feedback Clobber

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Fix a confirmed correctness bug in the flagship form template. The prop-sync
`$effect` in `EntityFormPage` reads local `success`/`error`/`fieldErrors` in its
conditions, so the moment the SPA submit handler writes results the effect
re-runs and resets them to the initial props. Success/error/field feedback is
clobbered immediately after every submit. `SpaFormShell` assigns unconditionally
(props-only) and is correct.

## Evidence

- bug `ts/src/templates/EntityFormPage.svelte:86-96`, submit `159-163`
- correct pattern `ts/src/patterns/SpaFormShell.svelte:80-84`

## Governing References

- [100 Shared patterns and workflow shells](../../contracts/100-shared-patterns-and-workflow-shells.md)
- [110 Admin template system](../../contracts/110-admin-template-system.md)

## Planned Changes

- [x] Track only props in the sync effect and assign unconditionally, mirroring
  `SpaFormShell`.
- [x] Add an early-return while `submitting` to guard double-submit
  (`EntityFormPage.svelte:147-175`, `SpaFormShell.svelte:96-139`).

## Consumer Upgrade Impact

Impact class: `none`. Bug fix; corrects existing behavior.

## Validation

- [x] component test: submit result feedback persists after the sync effect runs
- [x] `bun x vitest run` (templates suite)
- [x] `effigy validate`

## Stop Conditions

None.

## Completion Notes

Completed 2026-07-17. Prop-sync `$effect` now tracks only props and assigns
unconditionally (mirrors `SpaFormShell`); double-submit guards added to both
`EntityFormPage` and `SpaFormShell`. `EntityFormPage` post-submit redirect
also routed through `resolveRedirectTo` (parity with `g08.003`). Component
tests: feedback persists after effects run; second submit ignored in flight.
`effigy validate` green.

## Next Task

`g08.012` Google login dead handler.
