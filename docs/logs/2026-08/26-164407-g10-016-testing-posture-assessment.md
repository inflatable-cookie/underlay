# g10.016 - Testing Posture Contract Assessment

Date: 2026-08-26
Card: `g10.016`
Contract: `022-testing-posture-and-shared-harnesses.md`
Verdict: `drifting`

## Scope And Method

Read-only assessment of:

- Underlay's root task semantics, `underlay-testing`, TypeScript testing export,
  and contract `120`
- the root Effigy surface and API, admin, and front package in each consumer
  workspace
- configured `health`, `validate`, `qa`, and test-plan routes
- current Rust, TypeScript, and Svelte test roots
- fit and use of `TestDb`, `TestServer`, and `createMockHttpClient()`

No consumer test, task manifest, package manifest, source file, database, or
state stack was changed. No broad consumer test suite ran. Task evidence came
from `effigy --json tasks`, `effigy --json test --plan`, targeted
`effigy --json doctor health`, and deterministic file inspection.

## Shared Implementation Evidence

| Boundary | Evidence | Verdict |
| --- | --- | --- |
| Root semantics | Underlay `validate` and `qa` have the declared ordering, but `health` reaches `check:workspace-shape`, a Vitest command; Effigy reports the health task as heavy | drift |
| `TestDb` | Feature-gated, directly tested, supports an external Postgres URL, creates one generated schema, sets its search path, and exposes migrations, fixtures, seeds, and explicit cleanup | sound for single-schema shared-crate tests; not a fleet fit |
| `TestServer` | Feature-gated, directly tested Axum wrapper with method, header, body, response, and assertion helpers | sound but unproved outside its own crate |
| TS mock | `createMockHttpClient()` mirrors the shared `HttpClient` methods, records calls, supports queued/keyed responders, has direct tests, and has a package-compatibility import test | sound with one consumer compatibility strain |
| Shared consumption | Underlay shared Rust crates use `TestDb`; no consumer depends on `underlay-testing`. Acowtancy's Cattle Grid is the only consumer using `createMockHttpClient()` | partial adoption |

All six API migration trees create and query stable named schemas such as
`auth`, `platform`, and an app schema. `TestDb` isolates one generated search
path schema, so it cannot isolate those migrations without a new generic
multi-schema or database-per-test boundary. Existing app-local DB fixtures are
therefore justified for the current shape; forcing the shared type into them
would be false convergence.

`TestServer` is a closer fit. Underlay Reference, Contact Patch, and Acowtancy
all construct Axum requests and call `Router::oneshot` directly. Their state,
auth, and fixture composition should stay local, but the base request/response
mechanics can be proved against the shared wrapper.

Acowtancy uses the TS mock for command tests through
`packages/cattle-grid/tests/test-http-client.ts`, but narrows it through an
`as unknown` adapter. That is real adoption and real type friction, not proof
that every app-local module mock should be replaced. The other inspected mocks
mostly replace app client factories, SvelteKit modules, or domain services above
the shared `HttpClient` seam.

## Root Orchestration Matrix

| Consumer | `health` | `validate` | `qa` | `effigy test --plan` | Root verdict |
| --- | --- | --- | --- | --- | --- |
| Underlay Reference | six child health routes; API contributes formatting only | Underlay plus docs and all child validates; API tests run | health, validate, docs QA | admin, API, front, client | drifted: API minimum is absent from health |
| Contact Patch | six child health routes | all child validates; API DB suite and front test run | health, validate, docs QA | admin, front, client; API intentionally excluded because its test task owns container execution | match; mixed minimum/strong packages |
| Compli Me | six child health routes | all child validates | health, validate, docs QA | admin and API discovered, but neither suite is in validate/qa | match at declared minimum |
| Acowtancy | config guards plus docs, Farmyard health, and direct client/UI checks | Underlay, docs, and all child validates | bundle health, validate, and docs QA | Cream, Dairy, Farmyard managed suite, client, UI | drifted: API health has no build/check and root QA bypasses `farmyard/qa` |
| Songsprout | six child health routes | all child validates | health, validate, docs QA | Bloom, Greenhouse, Nursery, client | match at declared minimum |
| Composer | docs plus all five runtime/package health routes | explicit docs plus all runtime/package validates | health, validate, docs QA | admin, API, front, client | match at declared minimum |

The five bundle-backed roots expose additional security, template, and
workspace-shape selectors. Contract `022` permits those as broader QA layers;
their composition policy is outside this testing assessment. Root QA does add
docs quality, so it is not merely an alias for validate.

## Runtime Package Matrix

`minimum` means the package meets the ordinary baseline. It is not a weak or
failed verdict. Test-root counts below are file inventory, not executed-test
counts.

| Consumer package | Effective proof route | Test roots and harnesses | Classification | Gap and disposition |
| --- | --- | --- | --- | --- |
| `underlay-reference/apps/acme-api` | health: fmt; validate: build, Rust tests, clippy, fmt; qa: validate | 21 test files under crate `src/tests`/`tests`; app-local fixed-schema pool; direct Axum `oneshot` | drifted, with strong suites | API health lacks build/check. Fleet implementation card; validate with task trace plus targeted package proof |
| `underlay-reference/apps/acme-admin` | health: check; validate/qa: check + build | `tests/` with component/util tests; Vitest discovered | minimum | configured tests are outside merge gates. Strong-posture rollout candidate, not a minimum failure |
| `underlay-reference/apps/acme-front` | health: check; validate/qa: check + build | three files under `tests/`; Vitest discovered | minimum | configured tests are outside merge gates. Strong-posture rollout candidate |
| `contact-patch/apps/cp-api` | health: cargo check; validate/qa: check + container DB suite + build | 18 files across crate test roots; app-local fixed-schema pool; direct Axum `oneshot` | strong | shared `TestDb` does not fit fixed schemas; app fixture retained. `TestServer` proof candidate |
| `contact-patch/apps/cp-admin` | health: check; validate/qa: check + build | seven files under `tests/`; Vitest discovered | minimum | configured tests are outside merge gates. Strong-posture rollout candidate |
| `contact-patch/apps/cp-front` | health: check; validate/qa: check + Bun test + build | one routing test | strong | contract match |
| `compli-me/apps/api` | health: cargo check; validate/qa: check + build + conditional audit | one DB integration file; local fixed-schema setup | minimum | DB suite is outside merge gates. Strong-posture rollout candidate; shared `TestDb` not yet a fit |
| `compli-me/apps/admin` | health: check; validate/qa: check + build | one reorder workflow test; configured Bun suite | minimum | shared-workflow proof is outside merge gates. Consumer rollout card |
| `compli-me/apps/front` | health: check; validate/qa: check + build | no current test root | minimum | contract match; no strong suite required by current evidence |
| `acowtancy/apps/farmyard` | health: task-wrapper check; validate: policy, build, clippy, test compile, file-size check; package qa adds managed DB suite | 125 files across broad API/DB/jobs/migration roots; managed fixed test DB and app-local state/router helpers | drifted, with strong managed suite | health lacks API build/check; root QA bypasses package QA. Bounded Acowtancy task repair; retain app-local DB composition |
| `acowtancy/apps/dairy` | root health calls check; root validate calls lint, guardrails, policy, check, build; no package health/qa selector | broad `tests/` root (103 files including fixtures/harnesses); Vitest discovered | minimum through root orchestration | strong suite is outside root validate/qa and package task semantics are incomplete. Consumer rollout card |
| `acowtancy/apps/cream` | root health calls check; root validate calls lint, guardrails, check, build; no package health/qa selector | two tests; Vitest discovered | minimum through root orchestration | tests are outside root validate/qa and package task semantics are incomplete. Consumer rollout card |
| `songsprout/apps/nursery` | health: cargo check; validate/qa: check + build | one DB integration file | minimum | configured test is outside merge gates. Strong-posture rollout candidate |
| `songsprout/apps/greenhouse` | health: check; validate/qa: check + build + lint | 12 files under `tests/` including setup; Vitest discovered | minimum | server/auth workflow tests are outside merge gates. Consumer rollout card |
| `songsprout/apps/bloom` | health: check; validate/qa: check + build + lint | 13 files under `tests/` including setup; Vitest discovered | minimum | server/auth workflow tests are outside merge gates. Consumer rollout card |
| `composer/apps/composer-api` | health: cargo check; validate/qa: check + build | five API module test files; Cargo suite discovered | minimum | configured tests are outside merge gates. Strong-posture rollout candidate |
| `composer/apps/composer-admin` | health: check; validate/qa: check + build | one freshness test plus Vitest config | minimum | shared freshness proof is outside merge gates. Consumer rollout card |
| `composer/apps/composer-front` | health: check; validate/qa: check + build | no current test file | minimum | contract match; no strong suite required by current evidence |

Summary: 14 packages are at minimum, two are strong, and two are drifted because
their API health route violates the minimum. The two drifted APIs also own
substantial test suites; the classification is about the broken baseline, not
the absence of proof.

## Clause Matrix

| Contract clause | Fleet evidence | Verdict | Disposition |
| --- | --- | --- | --- |
| Root `health` / `validate` / `qa` meaning | Most roots preserve cheap health, stronger validate, and docs-extended QA. Underlay health reaches Vitest; Acowtancy root QA bypasses its API's declared full gate | drift | bounded Underlay and Acowtancy task repairs |
| API minimum | Contact Patch, Compli Me, Songsprout, and Composer expose cargo check in health; Underlay Reference and Farmyard do not. All six own DB/reset task surfaces assessed by `g10.015` | drift | fleet task-normalization card; migration selector repair remains in the combined wave |
| API strong target | Contact Patch runs DB tests in validate. Underlay Reference runs tests but fails minimum health. Farmyard owns a managed suite but only package QA reaches it. Three APIs remain valid minimum | partial | repair existing strong-gate disconnects; do not require new large suites |
| Rich-state API rule | Farmyard's managed setup/test/always-teardown suite extends validate at package QA | match at package, drift at root | bounded Acowtancy orchestration repair |
| Admin/front minimum | All twelve shells reach sync + Svelte check in effective health/check routes and build in validate; Dairy/Cream do so through root-owned direct routes | match | package selector consistency may be repaired with the test-gate wave; not a proof failure |
| Admin/front strong target | Contact Patch front runs its test in validate. Existing workflow tests in other mature shells are discoverable but generally outside validate/qa | strained | risk-led consumer rollout cards; preserve minimum classification |
| `TestDb` preference | Used by Underlay shared crates. All consumers use fixed named schema sets that the generated single-schema harness cannot isolate | justified app-local fixtures | decision prototype before changing shared DB isolation; no forced rollout |
| `TestServer` preference | Wrapper is sound and direct Axum `oneshot` mechanics recur in three consumers, but no consumer uses it | drift | Underlay Reference proof rollout, then decide fleet adoption |
| TS mock preference | Acowtancy uses the shared mock; other inspected mocks mostly sit above the client seam. Cattle Grid needs an `as unknown` adapter | mostly match, strained compatibility | bounded type-compatibility proof/repair; no blanket mock rewrite |
| Minimum versus strong | Every runtime package has an explicit classification; minimum packages are not reported as failures merely for lacking deep suites | match | retain this audit grammar |

## Findings And Candidates

### A. Restore trustworthy root and package test gates

- Pressure: two API health routes miss the minimum, root Acowtancy QA bypasses
  its declared managed suite, and mature configured suites commonly sit outside
  validate/qa.
- Consequence: the same selector can mean formatting only, compile/build, or a
  full managed DB proof depending on where it is called. Existing regressions
  can remain invisible to the advertised merge gate.
- Improvement: normalize the two API health routes, make root orchestration
  reach package-owned full gates where those gates already exist, and add only
  current risk-bearing suites to validate/qa. Keep small/no-test packages at
  minimum.
- Rejected alternative: put every discovered test in every health route. That
  destroys the cheap baseline and ignores managed-environment cost.
- Risk/cost: medium; mostly task composition, but DB-backed suites need explicit
  environment and runtime bounds.
- Validation: task traces, `effigy test --plan`, package health/validate/qa,
  targeted existing suites, then root health/validate/qa.
- Promotion: combined migration/testing roadmap wave.
- Confidence: high.

### B. Prove a fleet-capable Rust harness boundary

- Pressure: `TestDb` works for single generated schemas while every consumer
  migration owns multiple fixed schemas; `TestServer` fits repeated Axum
  mechanics but has no external proof.
- Consequence: contract `022` recommends a DB seam consumers cannot safely use
  and an HTTP seam whose practical value is unproved.
- Improvement: run a bounded decision prototype for generic multi-schema or
  database-per-test isolation. Separately prove `TestServer` in one Underlay
  Reference route slice while keeping app state and auth fixtures local.
- Rejected alternative: replace consumer fixtures mechanically. It would lose
  fixed-schema and managed-suite semantics or drag app behavior into Underlay.
- Risk/cost: medium-high for DB lifecycle/isolation; low-medium for the HTTP
  proof.
- Validation: concurrent isolation, migration replay, cleanup/failure proof,
  and focused route-test parity.
- Promotion: decision prototype for DB isolation; bounded Underlay
  Reference/Underlay test card for `TestServer`.
- Confidence: high on the mismatch, medium on the right DB design.
- Operator question: should the shared DB seam grow a generic multi-schema
  lifecycle, or should `022` explicitly keep whole-app DB suites app-owned?

### C. Remove TS mock compatibility casts at the shared seam

- Pressure: the only real consumer wraps `createMockHttpClient()` with an
  `as unknown` cast before command tests can use it.
- Consequence: the shared mock can drift from the exact client shape while its
  own tests stay green.
- Improvement: add a compile-time structural compatibility assertion against
  `HttpClient`, repair only genuine signature gaps, and simplify the Cattle Grid
  helper if the assertion proves assignability.
- Rejected alternative: replace every `vi.mock`/Bun mock. Most are app-module
  seams and outside the shared mock's contract.
- Risk/cost: low; type/export compatibility plus one consumer helper.
- Validation: Underlay type/package compatibility checks and targeted Cattle
  Grid test/typecheck.
- Promotion: bounded Underlay plus Acowtancy rollout card.
- Confidence: medium-high; the adapter is concrete, but the minimum repair still
  needs typecheck proof.

## Architecture Verdict

`drifting`.

The minimum-versus-strong policy is coherent and most packages meet it. Drift
is concentrated in task composition and shared-harness fitness: two APIs have
invalid health baselines, one rich suite is disconnected from root QA, mature
configured suites are often not merge gates, and the shared Rust harness story
does not yet match whole-app schema topology.

## Operator Decisions

No decision is required to close the assessment. Candidate B needs operator
authority before a DB-harness design becomes a spec or roadmap commitment.

Implementation authorized: no. This assessment changes only Underlay planning
and evidence state.

## Next Route

Re-enter Northstar planning and compile one findings-driven migration/testing
repair wave from `g10.015` and `g10.016`. Candidate A is the recommended first
testing slice. Do not mark a repair card ready until ownership, environment
requirements, and validation bounds are explicit.
