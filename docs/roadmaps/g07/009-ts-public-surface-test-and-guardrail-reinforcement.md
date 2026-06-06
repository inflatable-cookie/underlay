# g07.009 - TS Public-Surface Test And Guardrail Reinforcement

Status: ready
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.008` found that Underlay's TS implementation tests are broad, but several
retained public paths are tested indirectly. Guardrails also run as a health
check but have little direct self-test coverage.

This card adds bounded support tests before the consumer import compatibility
sweep.

## Goals

- [ ] add direct tests for `client/suggestions` through the preferred client
  helper path
- [ ] strengthen package-compatibility smoke tests for retained public TS
  subpaths
- [ ] add guardrail scanner/config/template self-tests for the existing rule
  behavior
- [ ] keep changes additive and support-only

## Non-Goals

- changing public exports
- retiring compatibility exports
- changing consumer imports
- adding a new test framework
- rewriting guardrail policy

## Execution Plan

- [ ] add focused tests for suggestion request helper behavior under
  `ts/tests/client`
- [ ] extend package-compatibility coverage for retained runtime, client,
  template, testing, and tools exports
- [ ] add guardrail tests for module-scope detection, suppressions, Svelte
  script scanning, config loading, and template imports
- [ ] validate with repo-owned TS support tasks

## Acceptance Criteria

- [ ] preferred public paths have direct smoke or behavior coverage
- [ ] guardrail support behavior is tested without adding new policy
- [ ] compatibility-only exports remain unchanged
- [ ] no consumer update is required

## Validation

- `effigy check:exports`
- `effigy check:types`
- `effigy check:guardrails`
- `effigy check:component-test-hygiene`
- `effigy test --plan`
- targeted `bun x vitest run` tests for the changed TS support files

## Consumer Upgrade Impact

None expected.

Update this section if the card changes public API, imports, validation, or
runtime behavior.

## Next Task

Execute this TS public-surface test and guardrail reinforcement batch.
