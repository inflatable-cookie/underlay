# g07.020 - Client Types Subpath Split

Status: complete
Owner: repo maintainers
Started: 2026-06-06
Completed: 2026-06-06

## Purpose

Split the broad `@inflatable-cookie/underlay/client/types` holding area into focused
client subpaths while keeping the aggregate path valid.

## Governing References

- [010 foundation primitives and envelopes](../../contracts/010-foundation-primitives-and-envelopes.md)
- [030 auth and session systems](../../contracts/030-auth-and-session-systems.md)
- [090 TS runtime and client orchestration](../../contracts/090-ts-runtime-and-client-orchestration.md)
- [190 upgrade compatibility matrix](../../guides/190-upgrade-compatibility.md)

## Evidence

`client/types` mixed three unrelated caller intents:

- success and error envelope DTOs
- restore-blocker DTOs and the `isRestoreBlockedResult()` guard
- auth-facing user, credential, session, and auth-error DTOs

Consumer usage also split that way. Known health command imports only need
`SingleResponse`; Acowtancy restore flows only need restore-blocker shapes and
guards.

## Changes

- [x] Add `@inflatable-cookie/underlay/client/envelopes` for `Uuid`,
  `SingleResponse`, `ListResponse`, `PagedListResponse`, `ErrorBody`, and
  `ErrorEnvelope`.
- [x] Add `@inflatable-cookie/underlay/client/restore` for restore-blocker DTOs,
  formatter types, and `isRestoreBlockedResult()`.
- [x] Add `@inflatable-cookie/underlay/client/auth-types` for auth-facing user,
  credential, session, and auth-error DTOs.
- [x] Keep `@inflatable-cookie/underlay/client/types` as the aggregate compatibility
  path.
- [x] Move Underlay internal imports to the focused files.
- [x] Extend package compatibility coverage for the new client subpaths.

## Consumer Upgrade Impact

Impact class: `additive`.

Existing `@inflatable-cookie/underlay/client/types` imports remain valid. New code can
prefer:

- `@inflatable-cookie/underlay/client/envelopes`
- `@inflatable-cookie/underlay/client/restore`
- `@inflatable-cookie/underlay/client/auth-types`

## Runtime AI Decision

`runtime/ai` remains whole. It is a tiny retained runtime entrypoint over the AI
routing ops controller, not a broad mixed helper surface. No focused split is
needed before a deeper AI patterns contract audit.

## Validation

- [x] `effigy check:exports`
- [x] `effigy check:types`
- [x] `bun x vitest run ts/tests/package-compatibility.test.ts`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`
- [x] `effigy validate`

## Next Task

The bounded consumer migration was completed in
[021 client types focused consumer migration](021-client-types-focused-consumer-migration.md).
