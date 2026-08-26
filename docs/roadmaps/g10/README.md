# g10 - Contract Fidelity And Fleet Convergence

Status: active
Owner: repo maintainers
Started: 2026-08-17

## Current Generation

`g10` opens after `g09` closeout. The config-convergence era, dependency
upgrades, and elective majors are done. The next substantial work is making the
written contract surface true in implementation and closing the remaining fleet
convergence gaps that still let consumer apps drift.

Source: 2026-08-17 Northstar refresh and atlas pass
(`docs/logs/2026-08/17-160000-northstar-refresh-atlas-g10-scoping.md`).

The first execution lane was selected on 2026-08-25 after Acowtancy proved the
single-repo `apps/*` / `packages/*` workspace shape. See
`docs/logs/2026-08/25-174056-monorepo-rollout-compiled.md`.

## Strategic Direction

Underlay should behave like a reference foundation whose guarantees are written,
tested, and provable across the six-consumer family — not a crate collection
where correctness depends on each app assembling primitives correctly.

This generation is about **fidelity and convergence**, not new product features.

## Horizon Model

### Phase 1 — Authority repair and fleet normalization (complete)

- close `g09` on all front doors (done in the refresh pass)
- retire polyrepo support in the bootstrap authority
- normalize the six-consumer workspace family through `g10.001`–`g10.010`

### Phase 2 — Contract implementation assessment (now)

Run the assessment loop from `docs/architecture/system-inventory.md` against
the contract index order:

1. foundation and transport (`010`, `020`)
2. migration and testing posture (`021`, `022`)
3. bootstrap and runtime assembly (`024`, `025`, `026`)
4. auth, routes, and compatibility retirement (`030`, `027`, `029`)
5. storage, jobs, Nightfire, AI, TS runtime, patterns, templates
6. tooling and audit artifacts (`120`, `121`, `122`)

Each assessment batch should produce bounded repair cards, not open-ended drift.

### Phase 3 — Collection and hybrid-shell convergence

Close the API/page-shape debt called out in contracts `116`/`117` and sweeps
`029`/`030`:

- canonical collection routes and query profiles
- hybrid collection shells between plain `EntityList` and app-owned composites
- child-collection capability classification before further template extraction

### Phase 4 — Consumer drift follow-through

Finish the remaining proposed mechanisms in
`docs/architecture/070-consumer-drift-prevention.md` where a stable reusable
boundary exists. Prefer conformance checks and reference-app shape over new
crate APIs when the boundary is not yet stable.

### Phase 5 — Reference-grade surface diet (deferred within g10)

`docs/architecture/020-reference-grade-underlay-architecture.md` phases 2–3
(public surface diet and adapter isolation) stay on the longer runway. Open
cards here only when they do not collide with an active assessment or convergence
lane.

## Candidate Lanes

| Lane | Outcome | Primary authority | Notes |
| --- | --- | --- | --- |
| A | First contract-assessment wave (foundation + transport) | `contract-index.md`, `system-inventory.md` | Assessment and `g10.012`–`g10.013` complete; `g10.014` ready |
| B | Collection route convergence pilot | `116`, `117`, sweep `029` | Needs one consumer as proof anchor |
| C | Hybrid shell extraction for one real tab | `117`, sweep `030`, Dairy `ModulesList` evidence | Bounded to one shell, not a framework rewrite |
| D | Consumer drift B-items with stable boundaries | `070`, `021` sweep family | Only items with a clear Underlay-owned seam |
| E | Conformance guard expansion | `120`, `070` §5 | Extend checks that fail when fleet deviates |

## Non-Goals

- typescript 7 adoption (backlog until 7.1 or concrete need)
- graphql, background-job dashboard, or other backlog product bets
- moving app-local domain behavior into Underlay without a reusable boundary
- a second architecture reset while assessment cards are still open
- generation rollover before `g10` has a meaningful card queue (~20–40 cards is
  the healthy era size; do not open `g11` prematurely)

## Dependencies And Sequencing

- The completed monorepo rollout is preserved under `docs/specs/archive/`.
- `g10.001`–`g10.005` are serial: authority, narrative, conformance,
  Acowtancy evidence, then Underlay Reference.
- `g10.006`–`g10.010` are complete.
- Lane A assessment is complete. Run bounded repairs `g10.012`–`g10.014` in
  sequence.
- Lane B and C should not run as parallel unbounded refactors; pick one consumer
  proof anchor first (`underlay-reference` unless another app is clearer)
- Lane D must respect `product-guardrails.md` — no app-local behavior smuggled
  into shared crates
- Lane E is cheap to start but should target gaps found in A, not speculative
  guards

## Accepted Uncertainty

- whether collection convergence or drift follow-through yields the better first
  proof slice
- how many compatibility exports can retire in one batch without violating `023`

## Queue

1. [x] [`g10.001`](batch-cards/001-monorepo-contract-authority.md) — monorepo contract authority (`complete`)
2. [x] [`g10.002`](batch-cards/002-active-guide-normalization.md) — migration contract and active guide normalization (`complete`)
3. [x] [`g10.003`](batch-cards/003-workspace-shape-conformance.md) — workspace-shape conformance (`complete`)
4. [x] [`g10.004`](batch-cards/004-acowtancy-evidence-repair.md) — Acowtancy evidence repair (`complete`)
5. [x] [`g10.005`](batch-cards/005-underlay-reference-normalization.md) — Underlay Reference normalization (`complete`)
6. [x] [`g10.006`](batch-cards/006-contact-patch-normalization.md) — Contact Patch normalization (`complete`)
7. [x] [`g10.007`](batch-cards/007-compli-me-normalization.md) — Compli Me normalization (`complete`)
8. [x] [`g10.008`](batch-cards/008-songsprout-normalization.md) — Songsprout normalization (`complete`)
9. [x] [`g10.009`](batch-cards/009-composer-normalization.md) — Composer normalization (`complete`)
10. [x] [`g10.010`](batch-cards/010-fleet-proof-and-closeout.md) — fleet proof and closeout (`complete`)
11. [x] [`g10.011`](batch-cards/011-foundation-and-transport-contract-assessment.md) — foundation and transport contract assessment (`complete`)
12. [x] [`g10.012`](batch-cards/012-context-rejection-envelope-normalization.md) — context rejection envelope normalization (`complete`)
13. [x] [`g10.013`](batch-cards/013-page-list-contract-artifact-sync.md) — page-list contract artifact sync (`complete`)
14. [ ] [`g10.014`](batch-cards/014-http-client-bounded-constructor-fallback.md) — bounded HTTP-client constructor fallback (`ready`)

## Next Task

Execute `g10.014`. Preserve Underlay's timeout bounds across infallible
HTTP-client constructor fallbacks.
