# g01.095 - UI Guide Long-Tail Prune Wave

Status: Complete

## Summary

Prune the remaining large Underlay integration guides that still carry too much
Poodle-facing implementation detail after the main UI-guide translation wave.

## Scope

- `077-media-library.md`
- `100-frontend-web.md`
- `110-admin.md`
- related `code/100-frontend-web/` and `code/110-admin/` integration stubs if
  they still imply canonical UI ownership

## Goals

- keep retained Underlay guidance focused on runtime, client, backend, and
  full-stack integration
- move or remove generic visible UI implementation examples that now belong in
  Poodle or ACME reference apps
- leave only the minimum retained examples needed to show how Underlay package
  surfaces fit into real apps

## Decisions

- Poodle remains the implementation home for generic UI.
- ACME remains the real visible reference family for concrete app examples.
- Underlay guide examples should survive only when they demonstrate retained
  integration or runtime posture, not generic component composition.

## Progress

- pruned the largest embedded generic UI example families out of:
  - `077-media-library.md`
  - `100-frontend-web.md`
  - `110-admin.md`
- replaced those broad sections with Poodle-guide and ACME-reference links
  where the visible implementation no longer belongs in Underlay
- kept the retained integration/runtime guidance in place instead of deleting
  the pages outright
- removed the stale guide cross-link family that still pointed readers at
  `090-ui-kit.md` as if it were the old implementation manual
- pruned the last obvious toy visible-UI guide stubs from:
  - `code/100-frontend-web/+layout.svelte`
  - `code/100-frontend-web/+page.svelte`
  - `code/110-admin/(app)/+page.svelte`
  - `code/110-admin/(auth)/+layout.svelte`
  - `code/110-admin/(auth)/login/+page.svelte`
- tightened the remaining `code/100-frontend-web/README.md` and
  `code/110-admin/README.md` files so they describe only the retained
  integration-oriented snippet set

## End State

The remaining Underlay UI-adjacent docs are now in the right shape:

- Poodle owns the generic visible UI implementation guides
- ACME files are the real visible app references
- Underlay keeps only retained integration, runtime, client, and full-stack
  documentation
- the surviving `code/100-frontend-web/` and `code/110-admin/` files are now
  clearly integration evidence rather than toy UI demos

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`

## Next Task

Open a fresh roadmap only if there is a new retained-boundary challenge. The
UI-guide translation and long-tail prune line is complete.
