# g07.008 - TS Testing And Guardrail Support Gap Inventory

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.001` through `g07.007` classified the retained TS runtime, client,
pattern, relation selector, list, template, and helper boundaries.

The next step is to check whether Underlay's TS tests and guardrail tooling are
strong enough to keep those boundaries intact as consuming apps adopt them.

## Goals

- [x] inventory TS unit tests, component tests, export checks, type checks, and
  guardrail scanners
- [x] map coverage to retained `runtime/*`, `client/*`, `patterns/*`,
  `templates/*`, `testing/*`, and `tools/*` surfaces
- [x] identify missing tests or guardrails for preferred import paths and
  compatibility-only exports
- [x] classify which gaps need code/test work versus documentation-only
  follow-up
- [x] preserve consumer compatibility while improving enforcement posture

## Non-Goals

- broad rewrite of TS test infrastructure
- adding a new test framework without proving a concrete support gap
- changing public exports without a consumer sweep
- moving app-local UI behavior into Underlay

## Execution Plan

- [x] inspect Effigy task coverage for TS checks, component tests, export
  checks, and docs/guardrail tasks
- [x] inspect existing TS tests and guardrail scripts
- [x] compare support coverage with contracts `090`, `100`, `110`, `111`,
  `115`, and `120`
- [x] write a gap inventory with bounded follow-on cards for any required
  implementation work

## Acceptance Criteria

- [x] retained TS surfaces have an explicit test or guardrail support posture
- [x] compatibility-only paths are identified with enforcement or retirement
  options
- [x] missing coverage is queued as bounded work, not left as prose drift
- [x] no test/tooling change lands without matching validation

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`
- `effigy check:types`
- `effigy test --plan`
- targeted TS and guardrail scans

## Consumer Upgrade Impact

None.

No public API, import, validation, or runtime behavior changed.

## Next Task

Execute `g07.009`: TS public-surface test and guardrail reinforcement.
