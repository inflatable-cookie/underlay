# g10.002 - Migration Contract And Active Guide Normalization

Status: ready
Owner: repo maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

## Purpose

Repair the retired migration-selector authority found during `g10.001`, then
remove the retired package and repository model from the remaining active
bootstrap, development, testing, CI, upgrade, and checklist guidance.

## Scope

- Normalize contract `021` around root `effigy state plan` /
  `effigy state apply local --yes` state operations and package-owned
  `migration:*` schema tasks routed from the workspace root.
- Remove its retired root and package `db:migrate` / `db:reset` contract while
  preserving the durable-migration, dev-overlay, replay, and validation rules.
- Normalize guides `040`, `050`, `080`, `130`, `140`, `150`, `160`, `170`,
  `172`, `175`, `190`, and `200` where the old layout or install model appears.
- Normalize `patterns/llm-project-bootstrap.md`.
- Use `apps/*`, `packages/*`, one root frozen install, root Effigy routing,
  released dependencies, and root lockfile language.
- Remove raw per-package install loops, committed `file:` dependency examples,
  multi-repo alternatives, and obsolete path-translation sections.
- Preserve unrelated domain guidance and historical evidence.

## Acceptance

- Active contract and guide surfaces no longer teach root or package `db:*`
  aliases as the migration operator contract.
- Active bootstrap/development guidance no longer teaches the retired shape.
- Checklists cover root `private`, pinned `packageManager`, explicit workspaces,
  one root lock, no child locks, and internal `workspace:*` edges.
- Upgrade guidance treats the root as the only Bun install/lock boundary.

## Evidence

- `g10.001` review found contract `021` still teaching the aliases removed from
  current Acowtancy and contract `024`.
- Acowtancy's live operator proof uses root `effigy state plan` /
  `effigy state apply local --yes` for state and package-owned `migration:*`
  selectors for schema work.
- Guide `175` is already marked not-current by the guide overview until this
  card removes its flat paths, committed `file:` dependencies, and per-package
  install loop.

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy health`
- targeted active-doc `rg` proof
- `git diff --check`

## Stop Conditions

Stop if a match is historical evidence, migration semantics would change beyond
the retired selector/routing correction, or a required change reaches a
contract not owned by this card.

## Next Task

After merge, promote `g10.003` to ready.
