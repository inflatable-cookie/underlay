# g10 - Contract Fidelity And Fleet Convergence

Status: active (scoping)
Owner: repo maintainers
Started: 2026-08-17

## Current Generation

`g10` opens after `g09` closeout. The config-convergence era, dependency
upgrades, and elective majors are done. The next substantial work is making the
written contract surface true in implementation and closing the remaining fleet
convergence gaps that still let consumer apps drift.

Source: 2026-08-17 Northstar refresh and atlas pass
(`docs/logs/2026-08/17-160000-northstar-refresh-atlas-g10-scoping.md`).

## Strategic Direction

Underlay should behave like a reference foundation whose guarantees are written,
tested, and provable across the six-consumer family — not a crate collection
where correctness depends on each app assembling primitives correctly.

This generation is about **fidelity and convergence**, not new product features.

## Horizon Model

### Phase 1 — Authority repair and first cards (now)

- close `g09` on all front doors (done in the refresh pass)
- choose the first bounded execution lane from the candidate set below
- compile `g10.001` with explicit acceptance, validation, and stop conditions

### Phase 2 — Contract implementation assessment

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

## Candidate Lanes (not yet carded)

| Lane | Outcome | Primary authority | Notes |
| --- | --- | --- | --- |
| A | First contract-assessment wave (foundation + transport) | `contract-index.md`, `system-inventory.md` | Likely first `g10.001` candidate |
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

- Lane A (assessment) can start immediately and should inform B–E
- Lane B and C should not run as parallel unbounded refactors; pick one consumer
  proof anchor first (`underlay-reference` unless another app is clearer)
- Lane D must respect `product-guardrails.md` — no app-local behavior smuggled
  into shared crates
- Lane E is cheap to start but should target gaps found in A, not speculative
  guards

## Accepted Uncertainty

- exact card count and ordering within `g10` — maintainer choice after `g10.001`
- whether collection convergence or drift follow-through yields the better first
  proof slice
- how many compatibility exports can retire in one batch without violating `023`

## Queue

Cards are not compiled yet. The maintainer should pick the first lane and open
`g10.001` as a bounded card with:

- explicit scope and non-goals
- named consumer proof targets when fleet-visible
- validation commands (`effigy validate`, targeted consumer checks)
- stop conditions and promotion route for any contract updates

## Next Task

Select the first lane and compile `g10.001`. Recommended default: **Lane A —
foundation + transport contract assessment** starting with `010` and `020`.
