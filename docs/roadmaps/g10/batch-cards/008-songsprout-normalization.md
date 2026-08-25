# g10.008 - Songsprout Normalization

Status: planned
Blocked by: `g10.005`
Owner: Songsprout maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

## Scope

- Move `nursery`, `bloom`, and `greenhouse` into `apps/`.
- Move `stem` and `petal` into `packages/`.
- Move `trellis` to root `docs/`.
- Consolidate four child locks and replace internal `file:` edges.
- Correct the reversed Effigy role mapping: `stem` is the client and `petal`
  is the UI package.
- Update Effigy catalogs, bundle dirs, aliases, tests, docs paths, and
  instruction surfaces.
- Preserve the separate-repository origin story as historical evidence.

## Acceptance And Validation

- One frozen root install; workspace-shape check green.
- Effigy role mapping matches package responsibilities.
- `effigy health`, planned targeted tests/checks, and `git diff --check` green.

## Stop Conditions

Stop if the current package exports contradict the documented stem/petal roles.

## Next Task

Return a reviewable PR to the orchestrator; do not merge.
