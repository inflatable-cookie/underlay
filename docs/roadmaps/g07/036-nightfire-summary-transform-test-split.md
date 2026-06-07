# g07.036 - Nightfire Summary Transform Test Split

Status: complete
Owner: repo maintainers
Roadmap: `g07`
Depends on: `001`

## Scope

Split the oversized Nightfire summary-transform test by transition class:

- page-layout transitions
- slider transitions
- malformed input handling

## Goals

- [x] `ts/tests/nightfire/summary-transform.test.ts` is replaced by focused
  test files.
- [x] Summary-transform assertions remain unchanged.
- [x] Doctor god-file findings are reduced without changing runtime exports.

## Acceptance Criteria

- [x] targeted Vitest run for Nightfire summary-transform tests
- [x] `effigy doctor`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`

## Validation Notes

Targeted Vitest run passed for the three split summary-transform test files:
13 tests. `effigy doctor` now reports 1 `scan.god-files` finding, down from 2.

## Stop Conditions

- A split requires changing runtime code.
- A test assertion must be weakened to pass.
- Doctor output gains a new warning family.

## Next Task

Continue with `g07.037`.
