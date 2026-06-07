# g07.034 - CSP Test God-File Split

Status: complete
Owner: repo maintainers
Roadmap: `g07`
Depends on: `001`

## Scope

Split the oversized CSP server test by exported helper group:

- nonce generation
- CSP/security config
- CSP header construction
- response/header application
- SvelteKit resolve options

## Goals

- [x] `ts/tests/server/csp.test.ts` is replaced by focused test files.
- [x] CSP assertions remain unchanged.
- [x] Doctor god-file findings are reduced without changing runtime exports.

## Acceptance Criteria

- [x] targeted Vitest run for CSP tests
- [x] `effigy doctor`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`

## Validation Notes

Targeted Vitest run passed for the five split CSP test files: 37 tests.
`effigy doctor` now reports 3 `scan.god-files` findings, down from 4.

## Stop Conditions

- A split requires changing runtime code.
- A test assertion must be weakened to pass.
- Doctor output gains a new warning family.

## Next Task

Continue with `g07.035`.
