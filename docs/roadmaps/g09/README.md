# g09 - Config Convergence And Contract Fidelity

Status: active
Owner: repo maintainers
Started: 2026-08-03

## Current Generation

`g09` remains the active delivery generation. Its original config-convergence,
dependency, and elective-major work completed on 2026-08-04. The generation
then continued into consumer workspace normalization and contract fidelity.
`g09.061` and `g09.062` are ready as parallel doctor-error repairs.

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
- [x] repair and prove the confirmed bootstrap/runtime/access drift
- [x] assess canonical-path retirement, runtime maturity, and workflow action
  grammar across all six roots
- [x] retire the authorised auth aliases and slash-form batch-delete routes
  across all five affected targets
- [x] normalize Contract `023` onto the released Git-tag dependency rule
- [ ] clear the two Underlay doctor error families without hiding advisory debt

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

### Phase 7 — Bootstrap/runtime/access repair (`g09.046`–`g09.056`, complete)

1. repair Underlay authority and conformance (`g09.046`, complete)
2. prove the repaired boundary in Underlay Reference (`g09.047`, complete)
3. run Contact Patch, Compli Me, Songsprout, Composer, and Acowtancy as
   independent repo-owned lanes (`g09.048`–`g09.052`, complete)
4. repair Underlay Reference's cross-tab CSRF token issuance (`g09.053`, complete)
5. close exact-head six-root proof (`g09.054`, complete after two honest repair
   pauses)
6. repair Acowtancy FAQ JSON-LD serialization (`g09.055`, complete)
7. make the merged SSR regression portable (`g09.056`, complete)

### Phase 8 — Canonical route and runtime assessment (`g09.057`, complete)

Contracts `027`–`029` were assessed across the six exact roots. Runtime
maturity conforms. Three auth alias families and two mixed batch-delete
families remain bounded drift.

### Phase 9 — Route retirement (`g09.058`–`g09.059`, complete)

The operator declared the supported fleet caller set closed-world, chose no
compatibility windows, and standardised batch deletion on `:batch-delete`.
The two batch-grammar lanes completed in Underlay Reference PR9 and Compli Me
PR8. Songsprout PR7, Composer PR7, and Acowtancy PR67 completed the three auth
lanes. All five remote `main` tips match the recorded merge commits.

### Phase 10 — Released dependency contract normalization (`g09.060`, complete)

Contract `023` now matches the released-tag dependency rule already required by
Contract `024`, active guides, the checker, and all six consumer roots.
`g09.060` is one serial docs-only correction. It does not change consumers,
versions, releases, or tooling. Underlay PR12 completed the overlapping
papercuts wave 3 contract-link lane at reviewed head `d2cb5cd9`, merge commit
`9e26ba9a`. Underlay PR13 then merged the normalization at reviewed head
`1bfe15c2`, merge commit `a65797f0`.

### Phase 11 — Doctor error normalization (`g09.061`–`g09.062`, ready)

The operator chose an honest green-doctor finish line rather than a zero-warning
threshold sweep. `g09.061` makes attention markers action-bearing without
deleting compatibility metadata. `g09.062` splits the only high-severity god
file behind its stable public facade. The lanes are independent and may run in
parallel.

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
47. [x] [`g09.047`](047-underlay-reference-runtime-access-proof.md) — Underlay Reference runtime/access proof (`complete`)
48. [x] [`g09.048`](048-contact-patch-runtime-access-rollout.md) — Contact Patch runtime/access rollout (`complete`)
49. [x] [`g09.049`](049-compli-me-runtime-access-rollout.md) — Compli Me runtime/access rollout (`complete`)
50. [x] [`g09.050`](050-songsprout-runtime-access-rollout.md) — Songsprout runtime/access rollout (`complete`)
51. [x] [`g09.051`](051-composer-runtime-access-rollout.md) — Composer runtime/access rollout (`complete`)
52. [x] [`g09.052`](052-acowtancy-runtime-access-rollout.md) — Acowtancy runtime/access rollout (`complete`)
53. [x] [`g09.053`](053-underlay-reference-cross-tab-csrf-stability.md) — Underlay Reference cross-tab CSRF stability (`complete`)
54. [x] [`g09.054`](054-bootstrap-runtime-access-fleet-closeout.md) — bootstrap/runtime/access fleet closeout (`complete`)
55. [x] [`g09.055`](055-acowtancy-faq-json-ld-hardening.md) — Acowtancy FAQ JSON-LD hardening (`complete`)
56. [x] [`g09.056`](056-acowtancy-faq-json-ld-regression-portability.md) — Acowtancy FAQ JSON-LD regression portability (`complete`)
57. [x] [`g09.057`](057-canonical-path-runtime-surface-workflow-assessment.md) — canonical path, runtime surface, and workflow action assessment (`complete`; `drifting`)
58. [x] [`g09.058`](058-auth-mutation-compatibility-retirement.md) — auth mutation compatibility retirement (`complete`)
59. [x] [`g09.059`](059-batch-delete-action-grammar-convergence.md) — batch-delete action grammar convergence (`complete`)
60. [x] [`g09.060`](060-released-dependency-rollout-contract-normalization.md) — released dependency rollout contract normalization (`complete`)
61. [ ] [`g09.061`](061-attention-marker-policy-normalization.md) — attention-marker policy normalization (`ready`)
62. [ ] [`g09.062`](062-workspace-shape-internal-modularization.md) — workspace-shape internal modularization (`ready`)

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
- `g09.047` is complete in Underlay Reference PR5, merge commit `6af27837`.
- `g09.048`–`g09.051` are complete in Contact Patch PR5, Compli Me PR7,
  Songsprout PR5, and Composer PR5.
- `g09.052` is complete in Acowtancy PR62, merge commit `85c868e1`.
- `g09.053` is complete in Underlay Reference PR6, merge commit `f89e3616`.
- `g09.054` is complete with an exact-head six-root matrix.
- `g09.055` is complete in Acowtancy PR63, merge commit `ad74d23e`.
- `g09.056` is complete in Acowtancy PR65, merge commit `22219f59`.
- `g09.057` is complete with a `drifting` verdict.
- `g09.059` is complete in Underlay Reference PR9, merge commit `0109b906`, and
  Compli Me PR8, merge commit `a290d2a7`.
- `g09.058` is complete in Songsprout PR7, merge commit `1778d108`; Composer
  PR7, merge commit `4fce7baa`; and Acowtancy PR67, merge commit `030b5295`.
- papercuts wave 3 is complete in Underlay PR12, merge commit `9e26ba9a`
- `g09.060` is complete in Underlay PR13, merge commit `a65797f0`.
- `g09.061` and `g09.062` have disjoint implementation and lane-evidence files
  and may run in parallel. Workers do not edit shared front doors.
- full doctor closeout waits for both reviewed merges.
- Whole-app fixed-schema suites stay app-owned. `TestDb` remains the
  shared-crate/single-schema seam.

## Accepted Uncertainty

- a shared multi-schema or database-per-test lifecycle remains uncompiled and
  needs new evidence plus a separate design roadmap
- collection convergence and drift-prevention follow-through remain later
  uncompiled horizons, not implied work in the active repair wave
- fourteen god-file warnings and one comment-ratio warning remain advisory;
  promote only evidence-backed maintainability work, not threshold churn
- deprecated API retirement needs consumer caller migration and a later release
  plan; it is not part of doctor-error normalization

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

`g09.045` found no regression in the supported monorepo shape. `g09.047`
completed the reference env/runtime/access proof without changing any of its 95
public route paths. `g09.048`–`g09.052` completed the five consumer-owned
env/secret, runtime-topology, access, and CSRF rollouts under their recorded
compatibility decisions. `g09.053` completed the compatible Reference
cross-tab CSRF stability repair without changing the route or response shape.
`g09.054` closes the fleet as conforming. PR63 hardened Acowtancy FAQ JSON-LD;
PR65 made its security regression portable. No further consumer action is
introduced by closeout. `g09.057` then found no runtime repair but recorded
three auth alias families and two mixed batch-delete families. The operator has
authorised atomic closed-world retirement with no compatibility windows and
canonical `:batch-delete`.

`g09.059` completed the no-window cutover in Underlay Reference and Compli Me.
Songsprout, Composer, and Acowtancy completed their `g09.058` alias retirements.
No supported caller action remains.

`g09.060` corrects stale release/dependency teaching only. Existing consumer
tags and locks remain valid; no consumer action is introduced.

`g09.061` changes Underlay's local scan policy only. `g09.062` is an internal
checker refactor with stable exports, diagnostics, and CLI behavior. Neither
lane introduces consumer action.

## Next Task

Publish one worker handoff per doctor lane and dispatch `g09.061` and
`g09.062` in parallel. Merge neither without exact-head review and explicit
operator authorisation.
