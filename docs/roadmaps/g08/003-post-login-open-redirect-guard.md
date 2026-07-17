# g08.003 - Post-Login Open-Redirect Guard

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Close the third link of the takeover chain. Route protection writes the current
path into the login-return query param with no validation, and `url.pathname`
can be protocol-relative (`//evil.com`). The library ships the producer but no
validating consumer, and the shell shows the blind-navigate pattern apps will
copy, so a post-login redirect can phish to a credential-harvesting clone.

## Evidence

- producer `ts/src/client/route-protection.ts:44`
- blind-navigate pattern `ts/src/patterns/SpaFormShell.svelte:128-130`
- related unnormalized-path match `route-protection.ts:16-19`

## Governing References

- [030 Auth and session systems](../../contracts/030-auth-and-session-systems.md)
- [090 TS runtime and client orchestration](../../contracts/090-ts-runtime-and-client-orchestration.md)

## Planned Changes

- [x] Add a `resolveRedirectTo()` helper that accepts only single-leading-slash
  same-origin paths (reject `//`, `\`, and any scheme).
- [x] Guard the producer against protocol-relative pathnames at write time.
- [x] Route the shell post-login navigate through the helper and document it as
  the required consumer pattern.
- [x] Normalize (percent-decode + collapse `../`) before `isPublicPath` prefix
  matching to close the encoding-bypass variant.

## Consumer Upgrade Impact

Impact class: `behavioral`. Consumers copying the blind-navigate pattern must
adopt the helper. Requires six-consumer proof per `023`.

## Validation

- [x] unit tests: `//evil.com`, `\evil`, `https://evil`, `%2e%2e` rejected;
  legitimate same-origin paths accepted
- [x] `bun x vitest run` (route-protection suite)
- [x] `effigy validate`

## Stop Conditions

None expected; bounded helper addition.

## Completion Notes

Completed 2026-07-17. Added `resolveRedirectTo()` and `normalizePath()` to
`client/route-protection`; producer (`createLoginRedirect`) refuses
protocol-relative pathnames; `SpaFormShell` routes post-login navigation
through the helper; `isPublicPath` normalizes (percent-decode + collapse
`../`) before prefix matching. Guide `068` documents the required consumer
pattern. Validated with the route-protection vitest suite (10 tests green).

## Next Task

`g08.004` upload content-type, SVG, and size enforcement (Batch 2).
