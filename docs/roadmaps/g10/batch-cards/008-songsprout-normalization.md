# g10.008 - Songsprout Normalization

Status: ready
Owner: Songsprout maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

## Scope

- Move `nursery`, `bloom`, and `greenhouse` into `apps/`.
- Move `stem` and `petal` into `packages/`.
- Move `trellis` to root `docs/`.
- Consolidate four child locks and replace internal `file:` edges.
- Correct the reversed Effigy role mapping: `stem` is the client and `petal`
  is the UI package.
- Replace per-package bundle hydration with the `g10.005` single frozen root
  install pattern.
- Update Effigy catalogs, bundle dirs, aliases, tests, docs paths, and
  instruction surfaces.
- Preserve the separate-repository origin story as historical evidence.

## Acceptance And Validation

- One frozen root install; workspace-shape check green.
- Effigy role mapping matches package responsibilities.
- `effigy health`, planned targeted tests/checks, and `git diff --check` green.

## Stop Conditions

Stop if the current package exports contradict the documented stem/petal roles.

## Readiness Evidence

Songsprout `origin/main` at
`480ab4fc55a8f6258857a3f96516b74d8c2d5201` has no open PR. The local checkout
is two commits behind with a user-owned `nursery/Cargo.lock` modification;
workers must branch from clean `origin/main`.

The role map is proven, so the stop condition is clear:

- `nursery` -> `apps/nursery` — Rust API
- `bloom` -> `apps/bloom` — artist app
- `greenhouse` -> `apps/greenhouse` — admin app
- `stem` -> `packages/stem` — typed API client and command surface
- `petal` -> `packages/petal` — deliberately minimal UI extension
- `trellis` -> root `docs`

Only root `[bundle.dirs]` reverses Stem and Petal. Their exports and consumers
confirm the card's intended correction. The root JavaScript workspace has four
members: Bloom, Greenhouse, Stem, and Petal.

The root manifest lacks `packageManager` and `workspaces`; four child locks
remain; Bloom and Greenhouse each use one internal `file:` Stem edge; and no
root lock exists. The workspace checker reports the expected seven violations.
No tracked submodule, Git link, symlink, or runtime source override exists.

Released dependencies are correct on remote main: JavaScript and Rust request
Underlay `v0.9.4`, while Poodle requests `0.2.2`. Bloom and Greenhouse locks
still embed stale Underlay `v0.9.2` metadata through file-linked Stem. Root lock
generation must remove that stale resolution.

Path-sensitive work spans all six bundle dirs, root conformance targets,
bundle bootstrap and `ui-setup.rhai`, app Svelte/Vite aliases, postinstall
hooks, public-config generation, Nursery config candidates, three Trellis
rollout scripts, Trellis Effigy docs checks, README/AGENTS links, and active
docs. Historical logs, closed roadmaps, and archived specs keep their wording.
The root README's merged-polyrepo origin story remains, but its proof path must
follow `apps/nursery`; its claim that Trellis is still a cloned repo must go.

Read-only task discovery, bundle inspection, and test planning pass. Test
planning does not detect Petal's smoke test or Trellis validation, so the
handoff must name both explicitly. Doctor already reports shared Underlay
manifest debt, god files, and heavy sibling health. Full health was not run
from the stale dirty checkout.

## Next Task

Return a reviewable PR to the orchestrator; do not merge.
