# g09.026-g09.029 Parallel Readiness Audit

Date: 2026-08-26
Roadmap: `g09`
Status: handoffs published; awaiting worker PRs

## Trigger

The Underlay Reference worker for `g09.025` is in flight. The operator asked
to parallelize the downstream rollout where safe.

Four consumer repositories were audited read-only in parallel. No consumer
manifest, lock, source file, branch, or handoff changed.

## Remote Baselines

| Roadmap | Consumer | Inspected `origin/main` | Open PRs | Local caution |
| --- | --- | --- | ---: | --- |
| `g09.026` | Contact Patch | `bd596e55b3a8ec8e68352c045f6ecd12b8effb4f` | 0 | local main two behind; modified API lock; ignored nested Book checkout |
| `g09.027` | Compli Me | `db5741f63dfb1cc82d9b49436370edfe66366bb2` | 0 | local main two behind; separate dirty adoption worktree |
| `g09.028` | Songsprout | `480ab4fc55a8f6258857a3f96516b74d8c2d5201` | 0 | local main two behind; modified API lock |
| `g09.029` | Composer | `a088e7b260e4d4aceb3eff9e81178c8d76a2f001` | 0 | clean and current |

Every implementation worker must use a fresh worktree based on the recorded
remote base. Existing dirty state stays untouched.

## Structural Findings

| Consumer | Root JS members | Child locks | Internal `file:` edges | Checker result |
| --- | ---: | ---: | ---: | --- |
| Contact Patch | 4 | 4 | 2 | 7 committed-shape failures; 1 extra local nested-Git failure |
| Compli Me | 4 | 4 | 3 | 7 failures |
| Songsprout | 4 | 4 | 2 | 7 failures |
| Composer | 3 | 3 | 2 | 6 failures |

All four root manifests lack pinned `packageManager` and explicit workspaces.
All four lack a root Bun lock. Package roles and final `apps/*`, `packages/*`,
and root `docs/` maps are otherwise clear.

Compli Me's empty API JavaScript manifest must be deleted. Composer has no UI
package, so its root workspace correctly has three members. Songsprout's code
proves Stem is the client and Petal is the UI package; only its Effigy mapping
is reversed.

## Dependency Findings

Remote main in every consumer now requests released Underlay `v0.9.4` and
Poodle `0.2.2`. Compli Me's merged release adoption proves Bun `1.3.14` can
resolve the tag; no SHA exception is justified.

App locks in all four consumers retain stale Underlay `v0.9.2` metadata through
file-linked internal packages. Each generated root lock must prove the old
`ddba2640` resolution is gone and the `v0.9.4` tag result remains.

## Execution Refinements

- Use the `g09.025` result as the exact one-frozen-root-install pattern. Several
  consumers still hydrate children independently or lack an effective root
  bootstrap selector.
- Update root bundle dirs, child Effigy guards, aliases, config-root depth,
  rollout scripts, QA targets, AGENTS, READMEs, and active docs atomically.
- Preserve historical logs, closed roadmaps, archived specs, and Songsprout's
  merged-polyrepo origin evidence.
- Keep sibling Underlay/Poodle paths only for explicit QA and tooling. Remove
  Composer's runtime `/underlay` symlink setup and sibling Underlay validation.
- Name validation gaps explicitly. Songsprout test planning misses Petal and
  Trellis. Full health was not used where it could write generated config or
  where local state was stale and dirty.

## Baseline Debt Boundary

- Contact Patch: existing Admin subject-test and generated-config debt.
- Compli Me: existing reorder-conflict checker failure.
- Songsprout: existing doctor god-file and shared-manifest findings.
- Composer: docs health already fails semantic-role and parameter freshness
  checks; doctor also reports generated-asset and god-file debt.

Workers update moved paths in these checks. They do not repair unrelated
application behavior inside topology PRs.

## Contact Patch Book Contract

Contact Patch's ignored `book/` checkout is a nested Git repository and fails
the workspace checker. API import/parity tooling also assumes
`cp-api/../book`, which breaks when API moves under `apps/`.

No product decision is needed. `g09.026` now prescribes Book as an external
sibling checkout at `../book`, bootstrapped as a child and mounted as ordinary
source at `/workspace-root/book`, not as a catalog or Bun workspace member.
The moved API resolves it through `../../../book`. The worker stops only if
Effigy cannot express that sibling-source contract.

## Current State

Underlay Reference PR
[#3](https://github.com/inflatable-cookie/underlay-reference/pull/3) merged as
`40924bc93fc9bf29a0a5d686cd1870f728ca48ce`. Roadmaps `g09.026`–`g09.029` carry
exact readiness evidence and remain `ready` until their workers begin.

Each target `main` now contains one handoff commit directly above the audited
baseline. Existing local checkouts and dirty files were not changed.

| Roadmap | Consumer handoff | Published `main` |
| --- | --- | --- |
| `g09.026` | `docs/handoffs/20260826-112215-g09-026-contact-patch-normalization.md` | `ddffc8587b73c8c23ec95108dace3e8b1bf59050` |
| `g09.027` | `docs/handoffs/20260826-112215-g09-027-compli-me-normalization.md` | `a5ccf0b92df64d8687e8255051219ccffbabe126` |
| `g09.028` | `docs/handoffs/20260826-112215-g09-028-songsprout-normalization.md` | `71ae0235df32322cef781eb5aca3f14f288c2740` |
| `g09.029` | `docs/handoffs/20260826-112215-g09-029-composer-normalization.md` | `230297ee6638e100365eba37e6499613c0dda954` |

## Next Task

Launch the four independent workers with only their repository-relative handoff
paths. Relay meaningful reports and PR URLs to the orchestrator.
