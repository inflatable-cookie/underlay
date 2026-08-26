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

### Phase 2 — Contract implementation assessment and repair (now)

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

## Generation Runway

- completed proof: consumer workspace normalization plus foundation and
  transport assessment/repair (`g10.001`–`g10.014`)
- completed assessments: database migration workflow and testing posture
  (`g10.015`–`g10.016`, both `drifting`)
- current repair wave: Underlay/shared proof, Underlay Reference baseline,
  independent consumer rollouts, then fleet closeout (`g10.017`–`g10.024`)
- immediate card: Underlay test-gate and mock-contract repair (`g10.017`,
  `ready`)
- next planning checkpoint: after that repair wave, confirm the `024`–`026`
  bootstrap/runtime assessment group remains the right transition
- longer runway: collection/hybrid-shell convergence, consumer drift
  follow-through, then the reference-grade surface diet

## Goals

- [x] Normalize the six-consumer workspace family onto one supported shape.
- [x] Assess and repair the foundation and transport contract boundary.
- [x] Assess migration and testing posture across every consumer root and
  affected child package.
- [ ] Repair only confirmed contract drift with bounded ownership and proof.
- [ ] Continue the contract-index assessment order without losing the
  collection and drift-prevention horizons.

## Execution Plan

- [x] Batch 1 — authority repair and six-consumer workspace normalization
  (`g10.001`–`g10.010`).
- [x] Batch 2 — foundation/transport assessment and bounded repairs
  (`g10.011`–`g10.014`).
- [x] Batch 3 — database migration and testing posture assessments
  (`g10.015`–`g10.016`).
- [ ] Batch 4A — shared and reference migration/testing proof
  (`g10.017`–`g10.018`).
- [ ] Batch 4B — independent consumer rollout lanes
  (`g10.019`–`g10.023`).
- [ ] Batch 4C — fleet proof and DB-harness decision checkpoint (`g10.024`).
- [ ] Batch 5 — planning checkpoint, then compile the next coherent assessment
  group from contracts `024`–`026`.

## Acceptance Criteria

- [x] Completed cards are removed from ready state on every queue authority.
- [x] Consumer workspaces use the supported `apps/*` / `packages/*` evidence
  shape.
- [x] Every `021` and `022` clause receives a fleet evidence verdict.
- [ ] Every confirmed finding is matched, repaired through a bounded card, or
  returned to an explicit operator decision.
- [x] Assessment cards remain read-only across consumer repositories.
- [ ] The migration/testing repair wave closes before the next contract group
  becomes ready.

## Candidate Lanes

| Lane | Outcome | Primary authority | Notes |
| --- | --- | --- | --- |
| A | Contract-assessment sequence | `contract-index.md`, `system-inventory.md` | Foundation/transport complete; migration/testing is the current batch |
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
- Lane A assessment and bounded repairs `g10.012`–`g10.014` are complete.
- `g10.015` and `g10.016` are serial because migration/reset proof ownership
  informs the API testing verdict.
- `g10.017` repairs Underlay before `g10.018` proves the reference-consumer
  baseline.
- `g10.019`–`g10.023` may run in parallel only after `g10.018` completes.
- `g10.024` waits for all five consumer repairs and returns the whole-app
  `TestDb` boundary to an explicit operator decision.
- Do not compile a shared DB-harness design card until the operator selects
  multi-schema/database-per-test ownership instead of explicit app ownership.
- Lane B and C should not run as parallel unbounded refactors; pick one consumer
  proof anchor first (`underlay-reference` unless another app is clearer)
- Lane D must respect `product-guardrails.md` — no app-local behavior smuggled
  into shared crates
- Lane E is cheap to start but should target gaps found in A, not speculative
  guards

## Accepted Uncertainty

- whether whole-app fixed-schema DB suites should remain app-owned or receive a
  new multi-schema/database-per-test `TestDb` lifecycle; `g10.024` is the
  operator decision checkpoint
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
14. [x] [`g10.014`](batch-cards/014-http-client-bounded-constructor-fallback.md) — bounded HTTP-client constructor fallback (`complete`)
15. [x] [`g10.015`](batch-cards/015-database-migration-contract-assessment.md) — database migration contract assessment (`complete`; verdict `drifting`)
16. [x] [`g10.016`](batch-cards/016-testing-posture-contract-assessment.md) — testing posture contract assessment (`complete`; verdict `drifting`)
17. [ ] [`g10.017`](batch-cards/017-underlay-test-gate-and-mock-contract.md) — Underlay test gate and mock contract (`ready`)
18. [ ] [`g10.018`](batch-cards/018-underlay-reference-migration-and-test-proof.md) — Underlay Reference migration and test proof (`planned`; gated by `g10.017`)
19. [ ] [`g10.019`](batch-cards/019-contact-patch-migration-rollout.md) — Contact Patch migration rollout (`planned`; gated by `g10.018`)
20. [ ] [`g10.020`](batch-cards/020-compli-me-migration-and-workflow-gate.md) — Compli Me migration and workflow gate (`planned`; gated by `g10.018`)
21. [ ] [`g10.021`](batch-cards/021-songsprout-migration-and-fail-closed-gates.md) — Songsprout migration and fail-closed gates (`planned`; gated by `g10.018`)
22. [ ] [`g10.022`](batch-cards/022-composer-migration-and-fail-closed-gates.md) — Composer migration and fail-closed gates (`planned`; gated by `g10.018`)
23. [ ] [`g10.023`](batch-cards/023-acowtancy-state-and-test-orchestration-repair.md) — Acowtancy state and test orchestration repair (`planned`; gated by `g10.018`)
24. [ ] [`g10.024`](batch-cards/024-migration-testing-fleet-closeout.md) — migration/testing fleet closeout (`planned`; gated by `g10.019`–`g10.023`)

## Consumer Upgrade Impact

`g10.017` is compatible Underlay test-contract hardening. `g10.018`–`g10.022`
cut the five baseline consumers from retired package `db:*` selectors to root
state plan/apply plus routed `migration:*` tasks with no compatibility window.
Songsprout and Composer also correct fail-open local dev overlays. `g10.023`
repairs Acowtancy local state application and merge-gate reachability.

## Next Task

Execute `g10.017`. Keep the work inside Underlay; consumer mutations start only
after the shared repair and `g10.018` reference proof are promoted in order.
