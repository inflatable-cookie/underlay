# g07.033 - SvelteKit Test God-File Split

Status: complete
Owner: repo maintainers
Roadmap: `g07`
Depends on: `001`

## Scope

Split the oversized SvelteKit client test by behavior area:

- auth cookie helpers and token store
- auth handle request protection
- refresh adapter behavior
- auth locals behavior

## Goals

- [x] `ts/tests/client/sveltekit.test.ts` is replaced by focused test files.
- [x] Shared cookie fixtures live outside `.test.ts` files.
- [x] SvelteKit assertions remain unchanged.
- [x] Doctor god-file findings are reduced without changing runtime exports.

## Acceptance Criteria

- [x] targeted Vitest run for SvelteKit tests
- [x] `effigy doctor`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`

## Validation Notes

Targeted Vitest run passed for the four split SvelteKit test files: 9 tests.
`effigy doctor` now reports 4 `scan.god-files` findings, down from 5.

## Stop Conditions

- A split requires changing runtime code.
- A test assertion must be weakened to pass.
- Doctor output gains a new warning family.

## Next Task

Continue with `g07.034`.
