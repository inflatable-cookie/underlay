# g10.001 - Monorepo Contract Authority

Status: ready
Owner: repo maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

## Purpose

Replace Underlay's polyrepo-compatible bootstrap authority with the proven
Acowtancy monorepo contract.

## Scope

- Rewrite contract `024` around one Git root, `apps/*`, `packages/*`, root
  `docs/`, one root Bun manifest/lock, `workspace:*`, released dependencies,
  and Effigy-owned frozen installation.
- Put the exact root `package.json` shape in contract `024`.
- Align the guide front door, architecture overview, project-structure guide,
  integration guide, new-project quickstart, and bootstrap prompt.
- Remove `--repo .`, `libs/*`, child-repository, committed sibling-`file:`, and
  symlink/submodule guidance from those active surfaces.
- Name Acowtancy as current live proof and Underlay Reference as the fixture
  that must converge in `g10.005`.
- Keep active library-facing docs free of absolute local paths.

## Out Of Scope

- Second-tier guide cleanup owned by `g10.002`.
- Conformance implementation owned by `g10.003`.
- Consumer repository edits.
- Historical logs, closed roadmaps, and frozen migration evidence.

## Acceptance

- Polyrepos are explicitly unsupported.
- The normative manifest snippet matches the strict spec.
- No scoped front door presents multi-repo as an option or uses `libs/*`.
- Dependency guidance separates released app dependencies from QA/tooling
  sibling mounts.
- Root and package Effigy guidance follows the repo-local agent contract.

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy health`
- targeted `rg` proof over the scoped active files
- `git diff --check`

## Stop Conditions

Stop if the current Acowtancy manifest no longer matches the strict spec, a
scoped guide owns a supported non-Bun package-manager variant, or a change would
rewrite frozen historical evidence.

## Continuation

Return the PR for orchestrator review. Do not start `g10.002` in the same worker.

## Next Task

Open the orchestrator-dispatched worker handoff for this card.
