# g09.025 — Underlay Reference normalization

Date: 2026-08-26
Roadmap: `docs/roadmaps/g09/025-underlay-reference-normalization.md`
Consumer PR:
[underlay-reference#3](https://github.com/inflatable-cookie/underlay-reference/pull/3)

## Summary

Normalized the bootstrap fixture to the supported single-repository monorepo
shape. Runtime apps now live under `apps/*`, reusable packages under
`packages/*`, and docs at root `docs/`. The JavaScript surface uses one root
manifest, one root lock, and internal `workspace:*` edges.

## Shared Bundle Prerequisite

The review found that the shared Effigy bundle coupled physical package paths
to task-selector aliases and hydrated JavaScript packages independently.
Bundle PR
[`underlay-effigy-bundle#1`](https://github.com/inflatable-cookie/underlay-effigy-bundle/pull/1)
repaired that contract and merged as
`e680157eebbdb4a14e98b53bd3f9ec38b9a936b7`.

Underlay Reference now supplies `[bundle.catalogs]` and
`[bundle.workspace] js_root = true`. The bundle owns root lifecycle tasks,
bootstrap performs one frozen root install, and container isolation covers the
root `node_modules`. No repo-local lifecycle override remains.

## Review And Merge Evidence

The canonical final review is the
[PR comment](https://github.com/inflatable-cookie/underlay-reference/pull/3#issuecomment-5423730990).
The reviewed head `e337bd9e626076e9f8238aede3c49de11c7786d7` merged to Underlay
Reference `main` as `40924bc93fc9bf29a0a5d686cd1870f728ca48ce` on 2026-08-26.
The merge commit has the reviewed head as its second parent, and remote `main`
points at the merge.

## Validation

- merged bundle sync and rendered ownership inspection: pass
- frozen root install: pass, no lock drift
- `effigy health`, docs QA, and workspace-shape conformance: pass
- Admin, Front, UI, and Client checks plus API build: pass
- three retained rollout checks: pass
- template and security conformance: pass
- `git diff --check`: pass

Full `effigy validate` ran. Underlay's full gate and the Admin, API, and Client
test targets passed. The aggregate retained the known Front Vitest routing
baseline: its configured include searches `src/**/*.{test,spec}.ts` while its
tests live under `tests/`. The worker added no exception or compatibility shim.

## Next Task

Publish independent worker handoffs for ready roadmaps `g09.026`–`g09.029`.
