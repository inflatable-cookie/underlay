# g09.003 - Slugify Test God-File Split

Status: complete
Owner: repo maintainers
Roadmap: `g09`
Depends on: `001`

## Scope

Split the oversized slugify pattern test by exported helper:

- slug generation
- slug format and reserved-word checks
- validation and integration checks

## Goals

- [x] `ts/tests/patterns/slugify.test.ts` is replaced by focused test files.
- [x] Slugify assertions remain unchanged.
- [x] Doctor god-file findings are reduced without changing runtime exports.

## Acceptance Criteria

- [x] targeted Vitest run for slugify tests
- [x] `effigy doctor`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`

## Validation Notes

Targeted Vitest run passed for the three split slugify test files: 49 tests.
`effigy doctor` now reports 7 `scan.god-files` findings, down from 8.

## Stop Conditions

- A split requires changing runtime code.
- A test assertion must be weakened to pass.
- Doctor output gains a new warning family.

## Next Task

Continue with `g09.004`.
