# Flattened-task switchover: preservation manifest and old-to-new map

Date: 2026-09-09
Authority: `docs/handoffs/20260909-140500-flattened-task-switchover.md`
Planning base: `41501e29`
Model: the generation README owns the roadmap and approved frontier;
`docs/roadmaps/gNN/NNN-<slug>.md` is the sole executable planning unit,
referenced as `gNN.NNN`. No milestone wrapper or nested `batch-cards/`
hierarchy is supported.

## Historic classification

| Generation | State | Disposition |
| --- | --- | --- |
| `g01` | Closed at `g01.098`, rolled into `g02` | Safely closed; compact to `archive/g01.md` |
| `g02` | All `g02.001`–`g02.007` complete | Safely closed; compact to `archive/g02.md` |
| `g03` | All `g03.001`–`g03.051` complete | Safely closed; compact to `archive/g03.md` |
| `g04` | All `g04.001`–`g04.035` complete | Safely closed; compact to `archive/g04.md` |
| `g05` | Convergence lines complete | Safely closed; compact to `archive/g05.md` |
| `g06` | Closed after reference-grade reset, Rust hardening, six-consumer proof | Safely closed; compact to `archive/g06.md` |
| `g07` | Complete after `g07.037` | Safely closed; compact to `archive/g07.md` |
| `g08` | Complete, all 32 roadmaps, `v0.8.0` tagged | Safely closed; compact to `archive/g08.md` |
| `g09` | Closed 2026-08-27, all 62 roadmaps, doctor green | Safely closed; compact to `archive/g09.md` |
| `g10` | Closed 2026-09-01, PR 22 merged | Safely closed; compact to `archive/g10.md` |
| `g11` | Active | Stays expanded; flattened below |
| `g12` | Active | Stays expanded; already flat |

No unresolved generations. Parallel-active `g11`/`g12` are independent
streams and both stay expanded.

## Unique authority

No live rule exists only inside a closed generation. Durable outcomes
already live on canonical surfaces: `docs/contracts/` (notably the
`g04`-authored contract set and its `g06`/`g08`/`g09` repairs),
`docs/architecture/`, `docs/guides/`, and `docs/usage/`. Nothing is
promoted; the roll-ups point at those destinations.

## Open commitments

No open commitment lives only in closed history. The one deferred item,
TypeScript 7 adoption, lives in the retirement triage note at
`docs/triage/20260909-170000-roadmap-backlog-retirement.md` (referenced
from the `g09` roll-up). No explicit removals.

## Selected evidence retained per roll-up

- `g08`: `v0.8.0` tag, 742 TS unit + 33 component tests, 17 Postgres
  adapter integration tests.
- `g09`: Underlay PR12 (`9e26ba9a`), PR13 (`a65797f0`), PR14
  (`5129356b`), PR15 (`c55a6fe6`); consumer PRs listed in the roll-up;
  exact-`main` doctor `ok:18 warn:2 err:0`.
- `g10`: PR 22 merged as `453c44d3`.
- Earlier generations: closeout lanes (`g01.098`, `g02.007`,
  `g03.051`, `g06` six-consumer proof, `g07.037`) named in their
  roll-ups. Git history remains the full-fidelity archive.

## Exact deletion list

- `docs/roadmaps/g01/` (99 task files + README)
- `docs/roadmaps/g02/` (7 task files + README + `batch-cards/`)
- `docs/roadmaps/g03/` (51 task files + README)
- `docs/roadmaps/g04/` (35 task files + README)
- `docs/roadmaps/g05/` (24 task files + README)
- `docs/roadmaps/g06/` (366 task files + README)
- `docs/roadmaps/g07/` (49 task files + README)
- `docs/roadmaps/g08/` (31 task files + README)
- `docs/roadmaps/g09/` (62 task files + README)
- `docs/roadmaps/g10/` (1 task file + README + `batch-cards/`)
- `docs/roadmaps/g11/batch-cards/` (4 completed cards + README)

## Current links rewritten

Contract evidence links in `023`, `027`, `051`, `070`, `080`, `090`,
`110`, `111` repoint from removed task files to the matching
`archive/gNN.md` roll-up. `contract-index.md` and `AGENTS.md` front-door
pointers repoint to archive/active front doors. `070`/`080`/`090`
stale repair-lane pointers repoint to `archive/g04.md`.
Historical links inside logs, handoffs, and `specs/archive/` are frozen
evidence and intentionally untouched.

## Active-generation old-to-new map

- `g11/001-...md` wrapper + `g11/batch-cards/001..004` (all complete:
  PR #23 at `27bde7b4`, `v0.9.6` at `4f6d7552`, PR #25 at `c8378e6b`,
  `v0.9.7` at `8a7ce84b`) collapse into `g11.001` (complete;
  outcome, evidence, limits, and dependencies all have durable
  destinations in tags, logs, and Contract `040`).
- Remaining consumer adoption (old `001` steps 6–7, unblocked
  target-owned lanes, Underlay Reference first) becomes `g11.002`
  (ready). Keeps no card ID; the milestone ID stays with the completed
  shared outcome per dependency order.
- Fleet closeout (old `001` step 8, blocked on consumer merges) becomes
  `g11.003` (blocked, gated on `g11.002`).
- `g12.001` is unchanged: it already is the sole executable unit with
  no `batch-cards/` directory.

## New frontier

- Ready: `g11.002` (five-consumer adoption from `v0.9.7`,
  Underlay Reference first), `g12.001` Card 278 lane (unchanged).
- Blocked: `g11.003` (fleet closeout behind consumer merges);
  `g12` release/adoption lanes behind Card 278 (unchanged).
