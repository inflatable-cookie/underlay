# g09.005 - I18n Test God-File Split

Status: complete
Owner: repo maintainers
Roadmap: `g09`
Depends on: `001`

## Scope

Split the oversized i18n pattern test by formatter area:

- date and time formatting
- number, percent, file-size, and currency formatting
- pluralization and formatter facade
- global locale/timezone configuration

## Goals

- [x] `ts/tests/patterns/i18n.test.ts` is replaced by focused test files.
- [x] I18n assertions remain unchanged.
- [x] Doctor god-file findings are reduced without changing runtime exports.

## Acceptance Criteria

- [x] targeted Vitest run for i18n tests
- [x] `effigy doctor`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`

## Validation Notes

Targeted Vitest run passed for the four split i18n test files: 58 tests.
`effigy doctor` now reports 5 `scan.god-files` findings, down from 6.

## Stop Conditions

- A split requires changing runtime code.
- A test assertion must be weakened to pass.
- Doctor output gains a new warning family.

## Next Task

Continue with `g09.006`.
