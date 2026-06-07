# g07.029 - TypeScript Test God-File Split

Status: complete
Owner: repo maintainers
Roadmap: `g07`
Depends on: `001`

## Scope

First TypeScript doctor warning cleanup batch:

- split oversized auth-oriented client test files
- keep test behavior and assertions intact
- leave runtime code unchanged

## Goals

- [x] `ts/tests/client/useAuth.test.ts` is replaced by focused test files.
- [x] `ts/tests/client/http/auth.test.ts` is replaced by focused test files.
- [x] Shared test fixtures live outside `.test.ts` files.
- [x] Doctor god-file findings are reduced without changing runtime exports.

## Acceptance Criteria

- [x] targeted Vitest run for the split auth tests
- [x] `effigy doctor`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`

## Validation Notes

Targeted Vitest run passed for the five split auth test files: 23 tests.
`effigy doctor` now reports 8 `scan.god-files` findings, down from 10.

## Stop Conditions

- A split requires changing runtime code.
- A test assertion must be weakened to pass.
- Doctor output gains a new warning family.

## Next Task

Continue with `g07.030`.
