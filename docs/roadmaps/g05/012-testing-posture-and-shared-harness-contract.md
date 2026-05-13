# g05.012 — Testing Posture And Shared Harness Contract

## Why

Underlay already owns shared test harnesses and utilities, but the normal app
testing posture is still not contracted tightly enough:

- what every app should test
- which shared harnesses to use
- minimum versus strong coverage
- route, DB, client, template, and component proof boundaries

This still leaves room for uneven test posture between consumer apps.

## Goal

Write the shared testing posture contract for normal Underlay apps across Rust,
TypeScript, templates, and workflow shells.

## Audit Readout

The app family already repeats one broad proof ladder.

Baseline repeated posture:

- API packages:
  - `health` on build or check
  - `validate` on build/check plus any app-local stronger checks
- admin/front packages:
  - `health` on `check`
  - `validate` on `check` plus `build`

The stronger app proofs are visible too:

- `farmyard`
  - managed DB-backed Rust suite
  - richer guardrails and load/reporting checks
- `dairy`
  - repo validation includes lint, guardrails, typecheck, and build
- `greenhouse`
  - validate includes lint in addition to typecheck and build

Shared-harness evidence is also clear:

- `underlay-testing::TestDb`
- `underlay-testing::TestServer`
- `ts/src/testing/createMockHttpClient`

Those shared seams exist and are ready, but consumer use is still uneven. That
is exactly what this contract needs to normalize.

## Scope

Primary targets:

- route-level HTTP tests
- DB-backed integration tests
- API client and command-surface tests
- template/component tests
- when to use shared harnesses versus app-local fixtures
- minimum versus strong coverage language
- expected test task surface in consumer repos

Likely outputs:

- one new contract
- possible tightening of testing artifacts or examples

## Contracts Landed In This Lane

### 022 — Testing posture and shared harnesses

Landed.

Defines:

- minimum versus strong proof posture
- API package baseline
- admin/front package baseline
- root `health` / `validate` / `qa` semantics
- shared Rust and TS harness usage expectations

## Consumer Upgrade Impact

Expected:

- clearer minimum test expectations
- more consistent harness usage
- easier cross-app review of test posture

## Next Task

Use `022` as the source of truth for later delivery and review contracts instead
of leaving proof expectations distributed across consumer repos.
