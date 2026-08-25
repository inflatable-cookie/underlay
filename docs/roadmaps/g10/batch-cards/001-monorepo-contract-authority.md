# g10.001 - Monorepo Contract Authority

Status: complete
Completed: 2026-08-25
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

## Evidence

Changed surfaces:

- `docs/contracts/024-new-app-bootstrap-and-bring-up.md` — rewritten
- `docs/guides/README.md` — guide front door
- `docs/guides/000-overview.md` — guide architecture overview
- `docs/guides/020-project-structure.md` — project-structure guide
- `docs/guides/030-underlay-integration.md` — integration guide
- `docs/architecture/060-new-project-quickstart.md` — new-project quickstart
- `docs/patterns/new-project-bootstrap-prompt.md` — bootstrap prompt
- `docs/contracts/README.md`, `docs/contracts/contract-index.md` — index
  currentness for `024`

Acowtancy evidence used (read-only):

- root `package.json` — `@acowtancy/market`, `private`, `packageManager`
  `bun@1.3.14`, explicit `workspaces`
- `apps/cream`, `apps/dairy`, `apps/farmyard`, `packages/cattle-grid`,
  `packages/froyo`, root `docs/`
- one root `bun.lock`, no child locks
- internal `workspace:*` edges; Underlay via
  `git+ssh://…/underlay.git#v0.9.4`; Poodle via released `0.2.2`
- `apps/farmyard/Cargo.toml` — Underlay crates pinned to `tag = "v0.9.4"`,
  app-local Cargo workspace
- `infra/tasks.toml` — `workspace:js:prepare` = `bun install --frozen-lockfile`
- `effigy.toml` — sibling `../underlay/scripts/*` used only by QA tasks

Validation run:

- `effigy qa:docs` — pass (links, vision index, forbidden, next-action)
- `effigy qa:northstar` — pass (three heading checks)
- `effigy health` — pass (exports, component-test hygiene, Poodle prop names,
  release-version sync at `0.9.4`, guardrails)
- targeted `rg` over the seven scoped files — remaining `libs/`, `symlink`,
  `submodule`, `--repo .`, and `multi-repo` hits are prohibitions or the
  quickstart's historical supersession note only
- `git diff --check` — clean

Pre-existing debt not touched: `effigy doctor` still reports the unsupported
`isolation` key in `effigy.toml` and the attention-marker/god-file scan
findings. Both predate this card.

Review round 1 (orchestrator, PR #6) requested three corrections — retired
`db:*` aliases in contract `024`, a contradictory child-lockfile deletion
instruction in the bootstrap prompt, and stale routing on three changed front
doors. All three applied; see the execution log.

Review round 2 approved the corrected head
`f79d699c03f830e803df193abe12e5bfa938f024` for operator-authorised merge. PR
[#6](https://github.com/inflatable-cookie/underlay/pull/6) merged to `main` as
`ec21e51cc918284fd9b306144c8d44a2ce1cae96` on 2026-08-25. The canonical
approval record is the
[orchestrator review comment](https://github.com/inflatable-cookie/underlay/pull/6#issuecomment-5414069218).

Out-of-scope adjacency: contract `021` still teaches `effigy db:migrate` /
`effigy db:reset`. Same retired-alias problem, different contract. Left for the
orchestrator to card. It is now the first bounded repair in `g10.002`, before
the active guide sweep.

## Stop Conditions

Stop if the current Acowtancy manifest no longer matches the strict spec, a
scoped guide owns a supported non-Bun package-manager variant, or a change would
rewrite frozen historical evidence.

## Continuation

Closed after orchestrator review, operator-authorised merge, and planning
currentness repair. Continue through a fresh `g10.002` worker handoff.

## Next Task

Execute `g10.002` — migration contract and active guide normalization. No other
card is ready.
