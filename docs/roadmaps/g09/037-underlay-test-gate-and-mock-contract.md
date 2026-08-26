# g09.037 - Underlay Test Gate And Mock Contract

Status: complete
Owner: repo maintainers
Contracts: `022-testing-posture-and-shared-harnesses.md`,
`120-tooling-testing-and-contract-artifacts.md`
Found by: `g09.036`
Depends on: `g09.036`
Completed: 2026-08-26

## Purpose

Restore a cheap Underlay health gate and make the TypeScript HTTP mock prove its
structural compatibility with the public client contract.

## Scope

- remove the Vitest-backed workspace-shape check from `health`
- retain workspace-shape proof in `validate` or a stronger existing gate
- make `MockHttpClient` structurally satisfy the exported `HttpClient`
  interface at compile time
- add focused compatibility coverage without changing runtime HTTP behavior
- keep this roadmap inside the Underlay repository; consumer cleanup follows in
  later roadmaps

## Acceptance

- `effigy health` does not start a test runner and is no longer classified as
  heavy because of `check:workspace-shape`
- `effigy validate` still runs the workspace-shape proof
- a signature change to `HttpClient` fails the mock compatibility check unless
  the mock changes with it
- existing mock call recording, responders, and reset behavior remain intact
- no public client or mock method is widened to accommodate an app-local shape

## Validation

- `effigy --json doctor health`
- `effigy health`
- `effigy validate`
- `effigy qa:docs`
- `effigy qa:northstar`
- focused HTTP-mock and package-compatibility tests through `effigy test:unit`
- `git diff --check`

## Stop Conditions

Stop if structural compatibility requires adding an Acowtancy-specific method,
weakening `HttpClient`, or changing the public request/response contract. Record
the mismatch and return to planning instead.

## Consumer Upgrade Impact

- Impact class: compatible test-contract hardening
- Affected consumers: TypeScript callers of `createMockHttpClient()`
- Required action: none unless a consumer carries a compatibility cast; that
  cleanup is owned by `g09.043`

## Completion Evidence

- `health` no longer reaches the Vitest-backed workspace-shape check
- `validate` retains workspace-shape proof through the full `test:unit` gate
- `MockHttpClient` now extends the exported `HttpClient` interface directly,
  so client signature drift fails TypeScript checking
- focused mock, package-compatibility, and workspace-shape proof passed: 3 files,
  17 tests
- full validation passed: 126 unit files / 782 tests and 12 component files /
  49 tests
- no runtime HTTP behavior or consumer-visible method changed

See the
[`g09.037` execution log](../../logs/2026-08/26-173931-g09-037-underlay-test-gate-mock-contract.md).

## Next Task

Re-enter planning at the `g09.038` promotion gate. Underlay Reference `main` is
current, but the disposable PostgreSQL target must be positively identified
before that roadmap becomes ready.
