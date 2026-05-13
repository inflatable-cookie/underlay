# Contract: Testing Posture and Shared Harnesses

Status: active
Owner: repo maintainers
Depends on: `021-database-migration-and-schema-workflow.md`, `024-new-app-bootstrap-and-bring-up.md`, `120-tooling-testing-and-contract-artifacts.md`

## Purpose

Define the normal testing posture for Underlay app workspaces and how shared
harnesses should be used.

This contract covers:

- the minimum test posture for normal API and admin packages
- stronger target posture for mature apps
- when to use shared Rust and TS harnesses
- expected task surfaces for health, validate, and QA flows

It does not redefine the lower shared testing tools themselves. Those stay with
`120`.

## Sources of Truth

Shared testing support:

- [`docs/contracts/120-tooling-testing-and-contract-artifacts.md`](/Users/tom/Dev/projects/underlay/docs/contracts/120-tooling-testing-and-contract-artifacts.md)
- [`rust/crates/underlay-testing/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-testing/src/lib.rs)
- [`ts/src/testing/index.ts`](/Users/tom/Dev/projects/underlay/ts/src/testing/index.ts)

Reference consumer evidence:

- [`underlay-reference/acme-api/effigy.toml`](/Users/tom/Dev/projects/underlay-reference/acme-api/effigy.toml)
- [`underlay-reference/acme-admin/effigy.toml`](/Users/tom/Dev/projects/underlay-reference/acme-admin/effigy.toml)
- [`acowtancy/farmyard/effigy.toml`](/Users/tom/Dev/projects/acowtancy/farmyard/effigy.toml)
- [`acowtancy/dairy/effigy.toml`](/Users/tom/Dev/projects/acowtancy/dairy/effigy.toml)
- [`compli-me/api/effigy.toml`](/Users/tom/Dev/projects/compli-me/api/effigy.toml)
- [`compli-me/admin/effigy.toml`](/Users/tom/Dev/projects/compli-me/admin/effigy.toml)
- [`contact-patch/cp-api/effigy.toml`](/Users/tom/Dev/projects/contact-patch/cp-api/effigy.toml)
- [`contact-patch/cp-admin/effigy.toml`](/Users/tom/Dev/projects/contact-patch/cp-admin/effigy.toml)
- [`songsprout/nursery/effigy.toml`](/Users/tom/Dev/projects/songsprout/nursery/effigy.toml)
- [`songsprout/greenhouse/effigy.toml`](/Users/tom/Dev/projects/songsprout/greenhouse/effigy.toml)
- [`loophole/composer/composer-api/effigy.toml`](/Users/tom/Dev/projects/loophole/composer/composer-api/effigy.toml)
- [`loophole/composer/composer-admin/effigy.toml`](/Users/tom/Dev/projects/loophole/composer/composer-admin/effigy.toml)

Observed test roots:

- Rust:
  - `crates/*/tests`
  - `src/tests/*`
- TS/Svelte:
  - `tests/`
  - `vitest.config.ts`

If these diverge, the contract plus the clearest modern proof posture
(`underlay-reference`, `dairy`, `farmyard`) win.

## Contract Goal

Underlay should make app-level proof expectations predictable.

A normal app team should not have to rediscover:

- what `health` versus `validate` should cover
- which packages need only build/type baselines versus deeper tests
- when to use shared HTTP, DB, and client mocks
- what minimum proof is expected before merging normal admin or API work

The goal is one declared baseline with a stronger target posture where the app
surface justifies it.

## Scope Boundary

In scope:

- API package proof posture
- admin/front package proof posture
- shared harness usage
- Effigy task semantics for test-related loops

Out of scope:

- CI implementation details
- one-off app-specific test environments
- performance/load/security special test lanes

## Shared Boundary

### Root task semantics

Normal meaning:

- `health`
  - fast local baseline
- `validate`
  - stronger merge-ready baseline
- `qa`
  - validate plus extra quality surfaces when the repo owns them

Rules:

- `health` should stay cheap and trustworthy
- `validate` should represent the normal local merge gate
- `qa` may layer docs, lint, audits, or other broader checks

### API minimum posture

Every normal API package should prove at least:

- build or check baseline
- schema migration/reset posture through the owned DB tasks

Allowed minimal examples today:

- `cargo build`
- `cargo check --workspace --all-features`

Rules:

- every API package must have a cheap baseline in `health`
- `validate` must be stronger than or equal to `health`
- DB task ownership belongs in the API package even if tests are still light

### API strong target posture

The stronger target for mature APIs is:

- build/check baseline
- DB-backed integration coverage for shared persistence seams
- route-level HTTP coverage where the API surface is central to product risk

Shared harness rule:

- use `underlay-testing::TestDb` for DB-backed integration tests where the
  app can fit the shared harness
- use `underlay-testing::TestServer` for in-memory HTTP/router tests where the
  route surface is being exercised

Rules:

- do not re-invent one-off DB container and router harnesses when the shared
  harness fits
- app-local fixtures are allowed, but the base DB and server mechanics should
  prefer the shared seam

### Rich-state API rule

Apps with heavier migration/state posture may expose additional test DB or
managed-suite flows.

Example:

- `farmyard` managed DB suite with setup and teardown

Rules:

- richer suites are allowed and often necessary
- they extend the baseline; they do not replace the boring `health` and
  `validate` story

### Admin and front minimum posture

Every normal SvelteKit admin or front package should prove at least:

- sync step
- `svelte-check`
- build in `validate`

Observed shared posture:

- `health` -> `check`
- `validate` -> `check` + `build`

Rules:

- this is the minimum merge-ready baseline for normal app shells
- package-local `dev`, `check`, `build`, and `validate` should remain obvious

### Admin and front strong target posture

The stronger target for mature app shells is:

- sync + typecheck + build
- lint when the app already owns a stable lint posture
- component or workflow-shell tests where shared UI/runtime behavior is the
  point of risk

Rules:

- do not require every app shell to have large test suites before the baseline
  is healthy
- do require component/workflow tests when shared Underlay behavior is being
  extended or normalized

### TS mock rule

Use the shared HTTP client mock when testing shared runtime, command, pattern,
or template behavior that depends on the `HttpClient` seam.

Rules:

- prefer `createMockHttpClient()` over ad hoc command mocks when the shared
  client contract is what matters
- app-local mocks are still allowed when the shared mock does not fit the test
  shape, but they should not replace it by default

### Minimum versus strong posture rule

Minimum posture means the app is on-contract for ordinary development.

Strong posture means the app proves more:

- DB-backed test suites
- HTTP route tests
- lint and guardrails
- component/workflow-shell tests
- managed test DB orchestration

Rules:

- audits should classify an app as minimum versus strong, not just “tested” or
  “untested”
- minimum is acceptable for smaller or earlier app surfaces
- strong is expected where the app owns more critical shared behavior

## What Good Looks Like

Good outcomes:

- `health`, `validate`, and `qa` mean roughly the same thing across repos
- API packages own DB and route proof posture
- admin/front packages own sync, typecheck, and build posture
- shared harnesses are used when they fit
- stronger apps extend the baseline without redefining it

Bad outcomes:

- `health` is expensive and mistrusted
- packages duplicate custom DB/server harnesses unnecessarily
- some apps rely only on build while others silently assume route or component
  proof with no contract saying so
- app-local mocks drift from the shared client seam

## Questions This Contract Should Settle

- What is the minimum proof expected for a normal Underlay API package?
- What is the minimum proof expected for a normal admin or front package?
- When should shared DB, HTTP, and client harnesses be used?
- How should audits describe minimum versus strong test posture?

## Next Task

Use this contract when assessing consumer test posture or when tightening the
review bar for shared API, runtime, template, or workflow changes.
