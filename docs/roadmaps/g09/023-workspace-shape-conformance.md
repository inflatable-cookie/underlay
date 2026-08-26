# g09.023 - Workspace Shape Conformance

Status: complete
Completed: 2026-08-25
Owner: repo maintainers
Spec: `docs/specs/archive/monorepo-consumer-workspace-rollout.md`

## Purpose

Make workspace drift fail mechanically instead of relying on another fleet
sweep.

## Scope

- Add a focused consumer workspace-shape check separate from the existing
  security conformance script.
- Check one Git root, no nested repositories, root manifest fields, explicit
  workspaces, one root `bun.lock`, no child locks, no internal `file:` edges,
  and expected `workspace:*` references.
- Update contracts `120` and `121` plus the app-review checklist artifact.
- Document how consumer roots expose the check through their Effigy health or
  validation surface.
- Add fixture coverage for compliant and failing workspace shapes.

## Acceptance

- The check has deterministic diagnostics and a non-zero drift exit.
- It accepts Acowtancy's proven shape.
- It rejects each fleet baseline defect named in the strict spec.
- It does not mix package-shape rules into security policy.

## Validation

- focused script/fixture tests
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy validate`
- `git diff --check`

## Stop Conditions

Stop if implementation requires app-specific package names or a new Effigy
schema surface.

## Review And Merge Evidence

The first orchestrator review requested portable fixture-only health, complete
and root-contained workspace membership, and a real published consumer bin.
Those corrections landed in `25499ef2226648dd130a981b0474fd3dfd0190fa`.

The second review found one realpath escape through an in-root symlink. The
final correction landed in `d1baca3d0d715bd235e8dbd6fd01b5887a3a639d`,
and final re-review approved that head. PR
[#8](https://github.com/inflatable-cookie/underlay/pull/8) merged to `main` as
`dc5d26668b554d00520a404dd6d17137f3663f74` on 2026-08-25. The canonical
approval record is the
[orchestrator review comment](https://github.com/inflatable-cookie/underlay/pull/8#issuecomment-5416874040).

## Next Task

Closed after orchestrator review, operator-authorized merge, and planning
currentness repair. Execute `g09.024` through a fresh Acowtancy worker handoff.
No other roadmap is ready.
