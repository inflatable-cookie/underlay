# g10.017 - Underlay Test Gate And Mock Contract

Status: ready
Owner: repo maintainers
Contracts: `022-testing-posture-and-shared-harnesses.md`,
`120-tooling-testing-and-contract-artifacts.md`
Found by: `g10.016`
Depends on: `g10.016`

## Purpose

Restore a cheap Underlay health gate and make the TypeScript HTTP mock prove its
structural compatibility with the public client contract.

## Scope

- remove the Vitest-backed workspace-shape check from `health`
- retain workspace-shape proof in `validate` or a stronger existing gate
- make `MockHttpClient` structurally satisfy the exported `HttpClient`
  interface at compile time
- add focused compatibility coverage without changing runtime HTTP behavior
- keep this card inside the Underlay repository; consumer cleanup follows in
  later cards

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
  cleanup is owned by `g10.023`

## Next Task

Complete this card, then promote `g10.018` for the Underlay Reference proof.
