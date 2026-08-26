# g09 - Config Convergence And Contract Fidelity

Status: active
Owner: repo maintainers
Started: 2026-08-03

## Current Generation

`g09` remains the active delivery generation. Its original config-convergence,
dependency, and elective-major work completed on 2026-08-04. The generation
then continued into consumer workspace normalization and contract fidelity.

An invalid rollover created `g10` with no roadmap files and used batch cards as
the queue. The 2026-08-26 recovery removed that generation and rehomed its
completed and remaining work as `g09.021`–`g09.044`. Git history and delivery
logs preserve the work; this front door is the corrected execution authority.
The operator explicitly chose continuation beyond the normal generation-size
default; no later generation opens without another explicit rollover decision.

## Strategic Direction

Make Underlay a reference foundation whose config, workspace, migration,
transport, and testing guarantees are written, implemented, and proved across
the six-consumer family.

This generation is about convergence and contract fidelity, not new product
features.

## Goals

- [x] close config-convergence gaps and retire vestigial config machinery
- [x] complete the dependency and elective-major upgrades
- [x] normalize the six-consumer family onto one supported `apps/*` /
  `packages/*` workspace shape
- [x] assess and repair the foundation and transport contracts
- [x] assess database migration and testing posture across the fleet
- [ ] repair the confirmed migration/testing drift with bounded ownership and
  disposable-state proof
- [ ] close the repair wave before starting the next contract assessment group

## Roadmap Sequence

### Phase 1 — Config convergence (`g09.001`–`g09.012`, complete)

The original self-audit follow-through: runtime config gaps, dead compatibility
machinery, consumer seams, and conformance guards.

### Phase 2 — Dependency extensions (`g09.013`–`g09.020`, complete)

Security-floor, baseline, Rust-major, and elective JavaScript dependency work.

### Phase 3 — Consumer workspace convergence (`g09.021`–`g09.030`, complete)

Contract authority, migration guidance, conformance tooling, reference proof,
four consumer rollouts, and six-consumer closeout.

### Phase 4 — Contract assessment and bounded repair (`g09.031`–`g09.036`, complete)

Foundation/transport assessment and repairs, followed by migration and testing
assessments. Both later assessments found bounded drift.

### Phase 5 — Migration/testing repair (`g09.037`–`g09.044`, active)

1. repair Underlay's health gate and TypeScript mock contract (`g09.037`,
   complete)
2. prove the baseline migration and `TestServer` posture in Underlay Reference
   (`g09.038`, complete)
3. run the five repository-owned consumer repairs independently after the
   reference proof (`g09.039` complete; `g09.040`–`g09.042`
   changes-requested; `g09.043` remains independently owned)
4. close fleet evidence and settle the whole-app `TestDb` ownership decision
   (`g09.044`)

`g09.038` merged in Underlay Reference PR4. Contact Patch PR4 then merged and
closed `g09.039`. First review requested changes on `g09.040`–`g09.042`;
`g09.043` remains owned by the separate Acowtancy thread. Every lane retains an
independent repo-owned local state boundary.

### Phase 6 — Bootstrap/runtime assessment (`g09.045`, planned)

After the repair wave, assess contracts `024`–`026` across the six-consumer
family. Repair roadmaps are not pre-numbered; they must come from the evidence.

## Queue

1. [x] [`g09.001`](001-prod-empty-origins-warning.md) — production empty-origins warning (`complete`)
2. [x] [`g09.002`](002-legacy-env-var-deprecation-signal.md) — legacy env-var deprecation signal (`complete`)
3. [x] [`g09.003`](003-operator-local-toml-strip-note.md) — operator local-TOML strip note (`complete`)
4. [x] [`g09.004`](004-retire-with-environment-from-env.md) — retire `with_environment_from_env` (`complete`)
5. [x] [`g09.005`](005-admin-cors-layer-from-env.md) — admin CORS layer from env (`complete`)
6. [x] [`g09.006`](006-nursery-env-precedence-flip.md) — Nursery env precedence (`complete`)
7. [x] [`g09.007`](007-farmyard-dev-gate-decision.md) — Farmyard dev-gate decision (`complete`)
8. [x] [`g09.008`](008-config-model-guide.md) — config-model guide (`complete`)
9. [x] [`g09.009`](009-songsprout-config-seam.md) — Songsprout config seam (`complete`)
10. [x] [`g09.010`](010-farmyard-seed-bundle-credentials.md) — Farmyard seed-bundle credentials (`complete`)
11. [x] [`g09.011`](011-shell-tab-schema-env.md) — shell-tab schema env (`complete`)
12. [x] [`g09.012`](012-build-time-environment-guard.md) — build-time environment guard (`complete`)
13. [x] [`g09.013`](013-js-vitest-security-floor.md) — Vitest security floor (`complete`)
14. [x] [`g09.014`](014-underlay-rust-majors.md) — Underlay Rust majors (`complete`)
15. [x] [`g09.015`](015-consumer-rust-follow-on.md) — consumer Rust follow-on (`complete`)
16. [x] [`g09.016`](016-js-baseline-catchup.md) — JavaScript baseline catch-up (`complete`)
17. [x] [`g09.017`](017-vite-8-and-plugin-svelte-7.md) — Vite 8 and plugin-svelte 7 (`complete`)
18. [x] [`g09.018`](018-lucide-svelte-1.md) — lucide-svelte 1 (`complete`)
19. [x] [`g09.019`](019-jsdom-30.md) — jsdom 30 (`complete`)
20. [x] [`g09.020`](020-zod-4.md) — zod 4 (`complete`)
21. [x] [`g09.021`](021-monorepo-contract-authority.md) — monorepo contract authority (`complete`)
22. [x] [`g09.022`](022-active-guide-normalization.md) — active guide normalization (`complete`)
23. [x] [`g09.023`](023-workspace-shape-conformance.md) — workspace-shape conformance (`complete`)
24. [x] [`g09.024`](024-acowtancy-evidence-repair.md) — Acowtancy evidence repair (`complete`)
25. [x] [`g09.025`](025-underlay-reference-normalization.md) — Underlay Reference normalization (`complete`)
26. [x] [`g09.026`](026-contact-patch-normalization.md) — Contact Patch normalization (`complete`)
27. [x] [`g09.027`](027-compli-me-normalization.md) — Compli Me normalization (`complete`)
28. [x] [`g09.028`](028-songsprout-normalization.md) — Songsprout normalization (`complete`)
29. [x] [`g09.029`](029-composer-normalization.md) — Composer normalization (`complete`)
30. [x] [`g09.030`](030-fleet-proof-and-closeout.md) — workspace fleet proof and closeout (`complete`)
31. [x] [`g09.031`](031-foundation-and-transport-contract-assessment.md) — foundation and transport assessment (`complete`)
32. [x] [`g09.032`](032-context-rejection-envelope-normalization.md) — context rejection envelope normalization (`complete`)
33. [x] [`g09.033`](033-page-list-contract-artifact-sync.md) — page-list contract artifact sync (`complete`)
34. [x] [`g09.034`](034-http-client-bounded-constructor-fallback.md) — bounded HTTP-client fallback (`complete`)
35. [x] [`g09.035`](035-database-migration-contract-assessment.md) — database migration assessment (`complete`; `drifting`)
36. [x] [`g09.036`](036-testing-posture-contract-assessment.md) — testing-posture assessment (`complete`; `drifting`)
37. [x] [`g09.037`](037-underlay-test-gate-and-mock-contract.md) — Underlay test gate and mock contract (`complete`)
38. [x] [`g09.038`](038-underlay-reference-migration-and-test-proof.md) — Underlay Reference migration and test proof (`complete`)
39. [x] [`g09.039`](039-contact-patch-migration-rollout.md) — Contact Patch migration rollout (`complete`)
40. [ ] [`g09.040`](040-compli-me-migration-and-workflow-gate.md) — Compli Me migration and workflow gate (`changes-requested`)
41. [ ] [`g09.041`](041-songsprout-migration-and-fail-closed-gates.md) — Songsprout migration and fail-closed gates (`changes-requested`)
42. [ ] [`g09.042`](042-composer-migration-and-fail-closed-gates.md) — Composer migration and fail-closed gates (`changes-requested`)
43. [ ] [`g09.043`](043-acowtancy-state-and-test-orchestration-repair.md) — Acowtancy state and test orchestration repair (`ready`; parallel lane)
44. [ ] [`g09.044`](044-migration-testing-fleet-closeout.md) — migration/testing fleet closeout (`planned`; after `g09.039`–`g09.043`)
45. [ ] [`g09.045`](045-bootstrap-and-runtime-contract-assessment.md) — bootstrap and runtime contract assessment (`planned`; after `g09.044`)

Deferred with a promotion trigger (TypeScript 7.1 or concrete need):
[`backlog/ts-7-adoption.md`](../backlog/ts-7-adoption.md).

## Dependencies And Parallelism

- `g09.021`–`g09.025` were serial; `g09.026`–`g09.029` ran in parallel;
  `g09.030` closed their fleet proof.
- `g09.031` assessed foundation/transport before `g09.032`–`g09.034` repaired
  confirmed findings.
- `g09.035` and `g09.036` were serial because migration ownership informed the
  testing verdict.
- `g09.037` and `g09.038` are complete.
- `g09.039` is complete. `g09.040`–`g09.042` are changes-requested and may be
  revised in parallel; `g09.043` remains independently owned. Their repository,
  state, and destructive-mutation boundaries do not overlap.
- `g09.044` waits for all five consumer roadmaps.
- `g09.045` waits for repair-wave closeout and remains assessment-only.
- No shared whole-app DB harness roadmap may be added until the operator chooses
  multi-schema/database-per-test ownership or explicit app-owned fixtures.

## Accepted Uncertainty

- whole-app fixed-schema DB ownership remains an operator decision at
  `g09.044`
- the next assessment group (`024`–`026`) must be reconfirmed after this repair
  wave before `g09.045` is promoted
- collection convergence and drift-prevention follow-through remain later
  uncompiled horizons, not implied work in the active repair wave

## Consumer Upgrade Impact

`g09.037` completed compatible Underlay test-contract hardening. `g09.038`
completed the reference proof. `g09.039` cut Contact Patch from retired package
`db:*` selectors to root state plan/apply plus routed `migration:*` tasks with
no compatibility window. Revised `g09.040`–`g09.042` must finish the same
cutover while preserving migration history and correcting the declared
fail-open local dev overlays. `g09.043` repairs Acowtancy local state
application and merge-gate reachability.

## Next Task

Revise and re-review `g09.040`–`g09.042`; accept `g09.043` evidence from the
separate Acowtancy thread. Keep `g09.044` blocked until all four remaining
consumer PRs are reviewed and merged.
