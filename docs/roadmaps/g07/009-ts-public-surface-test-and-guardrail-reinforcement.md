# g07.009 - TS Public-Surface Test And Guardrail Reinforcement

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.008` found that Underlay's TS implementation tests are broad, but several
retained public paths are tested indirectly. Guardrails also run as a health
check but have little direct self-test coverage.

This card adds bounded support tests before the consumer import compatibility
sweep.

## Goals

- [x] add direct tests for `client/suggestions` through the preferred client
  helper path
- [x] strengthen package-compatibility smoke tests for retained public TS
  subpaths
- [x] add guardrail scanner/config/template self-tests for the existing rule
  behavior
- [x] keep changes additive and support-only

## Non-Goals

- changing public exports
- retiring compatibility exports
- changing consumer imports
- adding a new test framework
- rewriting guardrail policy

## Execution Plan

- [x] add focused tests for suggestion request helper behavior under
  `ts/tests/client`
- [x] extend package-compatibility coverage for retained runtime, client,
  template, testing, and tools exports
- [x] add guardrail tests for module-scope detection, suppressions, Svelte
  script scanning, config loading, and template imports
- [x] validate with repo-owned TS support tasks

## Acceptance Criteria

- [x] preferred public paths have direct smoke or behavior coverage
- [x] guardrail support behavior is tested without adding new policy
- [x] compatibility-only exports remain unchanged
- [x] no consumer update is required

## Validation

- `effigy check:exports`
- `effigy check:types`
- `effigy check:guardrails`
- `effigy check:component-test-hygiene`
- `effigy test --plan`
- targeted `bun x vitest run` tests for the changed TS support files

## Consumer Upgrade Impact

None.

No public API, import, validation, or runtime behavior changed.

## Next Task

Execute `g07.010`: consumer import compatibility sweep.
