# g10.016 - Testing Posture Contract Assessment

Status: ready
Owner: repo maintainers
Contract: `022-testing-posture-and-shared-harnesses.md`
Depends on: `g10.015`

## Purpose

Assess Underlay's shared test harnesses and the six consumer workspaces against
contract `022` after migration proof ownership is settled.

## Promotion Gate

- [x] `g10.015` is complete
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
- compile bounded repair cards from confirmed drift without changing tests,
  task manifests, or production code during the assessment

## Acceptance

- one timestamped evidence matrix covers every `022` rule and all six roots
- every API, admin, and front package has an explicit proof-posture verdict
- root orchestration and package-owned test tasks are assessed separately
- shared-harness fit is distinguished from justified app-local fixtures
- every finding has one disposition: contract match, documentation repair,
  bounded implementation card, consumer rollout card, or operator decision
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
- Required action: none until a finding is promoted into a repair card

## Next Task

After completion, compile a findings-driven repair wave. Do not reserve or mark
repair cards ready before the assessment evidence exists.
