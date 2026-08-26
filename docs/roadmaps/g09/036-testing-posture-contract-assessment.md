# g09.036 - Testing Posture Contract Assessment

Status: complete
Owner: repo maintainers
Contract: `022-testing-posture-and-shared-harnesses.md`
Depends on: `g09.035`

## Purpose

Assess Underlay's shared test harnesses and the six consumer workspaces against
contract `022` after migration proof ownership is settled.

## Promotion Gate

- [x] `g09.035` is complete
- [x] no unresolved migration-policy decision changes the expected API proof bar
- [x] the live consumer roots and `apps/*` child paths have been rechecked

## Scope

- inspect `underlay-testing`, TypeScript testing exports, and the supporting
  tooling contract
- inspect each consumer root and all API, admin, and front packages under
  `apps/*`
- map `health`, `validate`, and `qa` meaning at root and package boundaries
- classify every runtime package as minimum, strong, drifted, or not
  applicable
- assess use of `TestDb`, `TestServer`, and `createMockHttpClient()` where those
  shared seams fit
- compile bounded repair roadmaps from confirmed drift without changing tests,
  task manifests, or production code during the assessment

## Acceptance

- one timestamped evidence matrix covers every `022` rule and all six roots
- every API, admin, and front package has an explicit proof-posture verdict
- root orchestration and package-owned test tasks are assessed separately
- shared-harness fit is distinguished from justified app-local fixtures
- every finding has one disposition: contract match, documentation repair,
  bounded implementation roadmap, consumer rollout roadmap, or operator decision
- no package is called weak merely because it meets the declared minimum

## Evidence Requirements

For each package record:

- effective root and package task routes
- `health`, `validate`, and `qa` coverage
- current test roots and harness use
- minimum-versus-strong classification with contract clause
- confirmed gaps, repair owner, and validation boundary

Use `effigy test --plan` where execution shape matters. Do not launch broad
consumer test suites merely to inventory their configured proof surface.

## Validation

- `effigy health`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Stop Conditions

Stop and return to planning if minimum-versus-strong classification requires a
new product risk policy, or if a shared harness would force app-specific test
behavior into Underlay. Do not turn the assessment into a fleet test rewrite.

## Consumer Upgrade Impact

- Impact class: assessment only
- Affected consumers: six-consumer family
- Required action: none until a finding is promoted into a repair roadmap

## Completion Evidence

The assessment matrix is recorded in
[`g09.036 - Testing Posture Contract Assessment`](../../../logs/2026-08/26-164407-g09-036-testing-posture-assessment.md).

Verdict: `drifting`. Fourteen runtime packages meet the declared minimum, two
are strong, and two API packages drift because `health` does not contain a
build/check baseline. Confirmed candidates are:

- restore trustworthy root/package test gates without making health expensive
- prove a fleet-capable Rust harness boundary before pushing `TestDb` into
  fixed multi-schema apps, and prove `TestServer` in one reference route slice
- remove the TS mock compatibility cast with a structural type proof

No consumer tests ran and no consumer repository changed. No repair roadmap is
ready yet; the migration and testing findings now have enough evidence for one
bounded planning pass.

## Next Task

Execute `g09.037`, the first roadmap in the compiled migration/testing repair wave.
The remaining roadmaps stay dependency-gated through `g09.044`.
