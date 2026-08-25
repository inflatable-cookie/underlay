# g10.002 - Active Guide Normalization

Status: planned
Blocked by: `g10.001`
Owner: repo maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

## Purpose

Remove the retired package and repository model from the remaining active
bootstrap, development, testing, CI, upgrade, and checklist guidance.

## Scope

- Normalize guides `040`, `050`, `080`, `130`, `140`, `150`, `160`, `170`,
  `172`, `175`, `190`, and `200` where the old layout or install model appears.
- Normalize `patterns/llm-project-bootstrap.md`.
- Use `apps/*`, `packages/*`, one root frozen install, root Effigy routing,
  released dependencies, and root lockfile language.
- Remove raw per-package install loops, committed `file:` dependency examples,
  multi-repo alternatives, and obsolete path-translation sections.
- Preserve unrelated domain guidance and historical evidence.

## Acceptance

- Active bootstrap/development guidance no longer teaches the retired shape.
- Checklists cover root `private`, pinned `packageManager`, explicit workspaces,
  one root lock, no child locks, and internal `workspace:*` edges.
- Upgrade guidance treats the root as the only Bun install/lock boundary.

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- targeted active-doc `rg` proof
- `git diff --check`

## Stop Conditions

Stop if a match is historical evidence or if removing it changes a contract not
owned by this card.

## Next Task

After merge, promote `g10.003` to ready.
