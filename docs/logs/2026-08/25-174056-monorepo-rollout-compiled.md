# Monorepo Consumer Rollout Compiled

Date: 2026-08-25
Roadmap: `g09.021`–`g09.030`

## Trigger

Acowtancy adopted one product monorepo with `apps/*`, `packages/*`, root
`docs/`, one explicit Bun workspace, one lock, and internal `workspace:*`
dependencies. The maintainer selected that shape as the default expectation and
retired polyrepo support.

## Assessment

- Acowtancy is the proven target, with one stale README explanation around
  sibling `file:` dependencies.
- Contract `024` and active bootstrap guides still permit or recommend
  multi-repo workspaces, flat package roots, `libs/*`, child installs, and
  committed source dependencies.
- The other five consumers are already single Git repositories. They need
  directory, manifest, lockfile, dependency-edge, Effigy, alias, and docs-path
  normalization.
- Underlay Reference must migrate before the remaining consumers because it is
  the bootstrap fixture.
- Contact Patch, Compli Me, Songsprout, and Composer become independent lanes
  after the reference fixture lands.

## Decisions

- One Git root is mandatory; polyrepos are unsupported.
- Root `package.json` owns an explicit Bun workspace and pinned Bun version.
- One root `bun.lock`; no child locks.
- Internal JavaScript dependencies use `workspace:*`.
- Released Underlay/Poodle dependencies compile the apps; sibling mounts serve
  QA/tooling only.
- No compatibility symlinks or old-path fallbacks during migration.
- Historical evidence remains frozen.

## Planning Outcome

- Opened strict spec `docs/specs/monorepo-consumer-workspace-rollout.md`.
- Compiled ten bounded roadmaps under `docs/roadmaps/g09/`.
- Marked only `g09.021` ready.
- Kept the authority, reference-fixture, and consumer dependency chain serial.
- Allowed the four downstream consumer roots to run in parallel only after
  `g09.025` merges.

## Consumer Upgrade Notes

Every consumer will move runtime apps under `apps/*`, reusable JavaScript
packages under `packages/*`, and documentation to root `docs/`; consolidate to
one root Bun manifest and lock; use `workspace:*`; and update Effigy/path
surfaces atomically.

## Next Task

Dispatch `g09.021` from the pushed orchestrator handoff and review its PR before
promoting `g09.022`.
