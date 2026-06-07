# g07.031 - Forms Test God-File Split

Status: complete
Owner: repo maintainers
Roadmap: `g07`
Depends on: `001`

## Scope

Split the oversized forms pattern test by behavior area:

- form state transitions
- enhanced submit handling
- draft autosave
- helper functions

## Goals

- [x] `ts/tests/patterns/forms.test.ts` is replaced by focused test files.
- [x] Shared form fixtures live outside `.test.ts` files.
- [x] Forms assertions remain unchanged.
- [x] Doctor god-file findings are reduced without changing runtime exports.

## Acceptance Criteria

- [x] targeted Vitest run for forms tests
- [x] `effigy doctor`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`

## Validation Notes

Targeted Vitest run passed for the four split forms test files: 18 tests.
`effigy doctor` now reports 6 `scan.god-files` findings, down from 7.

## Stop Conditions

- A split requires changing runtime code.
- A test assertion must be weakened to pass.
- Doctor output gains a new warning family.

## Next Task

Continue with `g07.032`.
