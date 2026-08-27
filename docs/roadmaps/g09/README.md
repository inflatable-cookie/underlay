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
completed and remaining work as `g09.021`–`g09.045`. Git history and delivery
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
- [x] repair the confirmed migration/testing drift with bounded ownership and
  disposable-state proof
- [x] close the repair wave before starting the next contract assessment group
- [x] assess bootstrap, runtime assembly, and access posture across all six roots
- [ ] repair and prove the confirmed bootstrap/runtime/access drift

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

### Phase 5 — Migration/testing repair (`g09.037`–`g09.044`, complete)

1. repair Underlay's health gate and TypeScript mock contract (`g09.037`,
   complete)
2. prove the baseline migration and `TestServer` posture in Underlay Reference
   (`g09.038`, complete)
3. run the five repository-owned consumer repairs independently after the
   reference proof (`g09.039`–`g09.043` complete)
4. close fleet evidence and retain fixed-schema whole-app suites as app-owned
   (`g09.044`, complete)

`g09.038` merged in Underlay Reference PR4. Contact Patch PR4, Compli Me PR6,
Songsprout PR4, Composer PR4, and Acowtancy PR59 then merged and closed
`g09.039`–`g09.043`. Every lane retains an independent repo-owned local state
boundary. `g09.044` closed the fleet proof and kept `TestDb` on its
shared-crate/single-schema seam.

### Phase 6 — Bootstrap/runtime assessment (`g09.045`, complete)

The six-root read-only assessment found the workspace rollout intact and
confirmed env/secret, runtime-topology, test-seam, and access/security drift.

### Phase 7 — Bootstrap/runtime/access repair (`g09.046`–`g09.053`, active)

1. repair Underlay authority and conformance (`g09.046`, complete)
2. prove the repaired boundary in Underlay Reference (`g09.047`, planned)
3. run Contact Patch, Compli Me, Songsprout, Composer, and Acowtancy as
   independent repo-owned lanes (`g09.048`–`g09.052`, planned)
4. close exact-head six-root proof (`g09.053`, planned)

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
40. [x] [`g09.040`](040-compli-me-migration-and-workflow-gate.md) — Compli Me migration and workflow gate (`complete`)
41. [x] [`g09.041`](041-songsprout-migration-and-fail-closed-gates.md) — Songsprout migration and fail-closed gates (`complete`)
42. [x] [`g09.042`](042-composer-migration-and-fail-closed-gates.md) — Composer migration and fail-closed gates (`complete`)
43. [x] [`g09.043`](043-acowtancy-state-and-test-orchestration-repair.md) — Acowtancy state and test orchestration repair (`complete`)
44. [x] [`g09.044`](044-migration-testing-fleet-closeout.md) — migration/testing fleet closeout (`complete`)
45. [x] [`g09.045`](045-bootstrap-and-runtime-contract-assessment.md) — bootstrap and runtime contract assessment (`complete`; `drifting`)
46. [x] [`g09.046`](046-bootstrap-runtime-access-authority.md) — bootstrap/runtime/access authority (`complete`)
47. [ ] [`g09.047`](047-underlay-reference-runtime-access-proof.md) — Underlay Reference runtime/access proof (`planned`)
48. [ ] [`g09.048`](048-contact-patch-runtime-access-rollout.md) — Contact Patch runtime/access rollout (`planned`)
49. [ ] [`g09.049`](049-compli-me-runtime-access-rollout.md) — Compli Me runtime/access rollout (`planned`)
50. [ ] [`g09.050`](050-songsprout-runtime-access-rollout.md) — Songsprout runtime/access rollout (`planned`)
51. [ ] [`g09.051`](051-composer-runtime-access-rollout.md) — Composer runtime/access rollout (`planned`)
52. [ ] [`g09.052`](052-acowtancy-runtime-access-rollout.md) — Acowtancy runtime/access rollout (`planned`)
53. [ ] [`g09.053`](053-bootstrap-runtime-access-fleet-closeout.md) — bootstrap/runtime/access fleet closeout (`planned`)

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
- `g09.039`–`g09.043` are complete.
- `g09.044` is complete after all five merged consumer roadmaps.
- `g09.045` is complete.
- `g09.046` is complete and merged in PR9.
- `g09.047` is planned behind its release and app-owner decision gates as the
  reference proof.
- `g09.048`–`g09.052` may run in parallel only after `g09.047` and each named
  product/security decision gate.
- `g09.053` closes the fleet after all five independent rollout lanes merge.
- Whole-app fixed-schema suites stay app-owned. `TestDb` remains the
  shared-crate/single-schema seam.

## Accepted Uncertainty

- a shared multi-schema or database-per-test lifecycle remains uncompiled and
  needs new evidence plus a separate design roadmap
- collection convergence and drift-prevention follow-through remain later
  uncompiled horizons, not implied work in the active repair wave

## Consumer Upgrade Impact

`g09.037` completed compatible Underlay test-contract hardening. `g09.038`
completed the reference proof. `g09.039` cut Contact Patch from retired package
`db:*` selectors to root state plan/apply plus routed `migration:*` tasks with
no compatibility window. `g09.040`–`g09.042` completed the same cutover while
preserving migration history and correcting the declared fail-open local dev
overlays. `g09.043` repaired Acowtancy local state application and merge-gate
reachability.

`g09.044` published the fleet upgrade note and final matrix. Retired consumer
`db:*` selectors and fail-open local overlays have no compatibility window.

`g09.045` found no regression in the supported monorepo shape. Later consumer
impact is bounded by `g09.047`–`g09.052`; security and path compatibility
changes cannot begin before their roadmap decisions are explicit.

## Next Task

Resolve the release and app-owner decision gates for `g09.047`. Do not promote
it until Underlay Reference is current with explicit security decisions.
