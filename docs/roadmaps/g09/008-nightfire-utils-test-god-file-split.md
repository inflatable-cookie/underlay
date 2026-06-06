# g09.008 - Nightfire Utils Test God-File Split

Status: complete
Owner: repo maintainers
Roadmap: `g09`
Depends on: `001`

## Scope

Split the oversized Nightfire utils test by exported helper:

- Nightfire value normalization
- Nightfire block normalization
- empty-value detection
- FormData writing

## Goals

- [x] `ts/tests/nightfire/utils.test.ts` is replaced by focused test files.
- [x] Nightfire utils assertions remain unchanged.
- [x] Doctor god-file findings are reduced without changing runtime exports.

## Acceptance Criteria

- [x] targeted Vitest run for Nightfire utils tests
- [x] `effigy doctor`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`

## Validation Notes

Targeted Vitest run passed for the four split Nightfire utils test files: 47
tests. `effigy doctor` now reports 2 `scan.god-files` findings, down from 3.

## Stop Conditions

- A split requires changing runtime code.
- A test assertion must be weakened to pass.
- Doctor output gains a new warning family.

## Next Task

Continue with `g09.009`.
