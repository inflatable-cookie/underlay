# g07.008 - TS Testing And Guardrail Support Gap Inventory

Status: ready
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.001` through `g07.007` classified the retained TS runtime, client,
pattern, relation selector, list, template, and helper boundaries.

The next step is to check whether Underlay's TS tests and guardrail tooling are
strong enough to keep those boundaries intact as consuming apps adopt them.

## Goals

- [ ] inventory TS unit tests, component tests, export checks, type checks, and
  guardrail scanners
- [ ] map coverage to retained `runtime/*`, `client/*`, `patterns/*`,
  `templates/*`, `testing/*`, and `tools/*` surfaces
- [ ] identify missing tests or guardrails for preferred import paths and
  compatibility-only exports
- [ ] classify which gaps need code/test work versus documentation-only
  follow-up
- [ ] preserve consumer compatibility while improving enforcement posture

## Non-Goals

- broad rewrite of TS test infrastructure
- adding a new test framework without proving a concrete support gap
- changing public exports without a consumer sweep
- moving app-local UI behavior into Underlay

## Execution Plan

- [ ] inspect Effigy task coverage for TS checks, component tests, export
  checks, and docs/guardrail tasks
- [ ] inspect existing TS tests and guardrail scripts
- [ ] compare support coverage with contracts `090`, `100`, `110`, `111`,
  `115`, and `120`
- [ ] write a gap inventory with bounded follow-on cards for any required
  implementation work

## Acceptance Criteria

- [ ] retained TS surfaces have an explicit test or guardrail support posture
- [ ] compatibility-only paths are identified with enforcement or retirement
  options
- [ ] missing coverage is queued as bounded work, not left as prose drift
- [ ] no test/tooling change lands without matching validation

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`
- `effigy check:types`
- `effigy test --plan`
- targeted TS and guardrail scans

## Consumer Upgrade Impact

None for the inventory posture.

Update this section if the card makes public API, import, validation, or
behavior changes.

## Next Task

Execute this TS testing and guardrail support gap inventory.
