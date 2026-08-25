# g10.003 - Workspace Shape Conformance

Status: ready
Owner: repo maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

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

## Next Task

Execute this card through an orchestrator-dispatched worker handoff. After its
review and operator-authorized merge, prepare `g10.004` in Acowtancy.
