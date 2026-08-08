# 091 - Client Surface Organization

`g01.090` confirmed that the remaining public `patterns` surface is a true
retained stop point. The next honest retained-package challenge is
`@inflatable-cookie/underlay/client`.

`client` is still coherent, but its public shape is flatter than the live
boundary. The active apps mostly use SvelteKit/browser-facing navigation and
cookie helpers, while the generated client packages mostly use HTTP, error, and
type helpers.

This wave exists to make that boundary explicit without forcing churn across the
existing broad root barrel.

## Scope

- `ts/src/client/`
- `package.json` `./client*` exports
- guide and architecture references to `@inflatable-cookie/underlay/client`

## Goals

- Confirm the strict live caller boundary for the retained `client` package.
- Expose the real client subpaths explicitly so new imports do not have to rely
  on one flat root barrel.
- Keep the root `@inflatable-cookie/underlay/client` barrel stable as a convenience
  surface for existing callers.

## Non-Goals

- Do not force a client-package extraction.
- Do not migrate existing callers just to prefer narrower imports.
- Do not move framework-agnostic navigation state back out of
  `@inflatable-cookie/underlay/runtime`.

## Caller Matrix

The live caller surface splits cleanly into two groups:

### App-facing SvelteKit/browser helpers

Broad live usage in `acme-admin`, `cp-admin`, and especially `dairy`:

- `gotoWithContext`
- `navigateOnCancel`
- `initPageState`
- `createAuthCookieHelpers`
- `UnderlayHttpError` in server hooks

### Generated client and command-package helpers

Broad live usage in `acme-client`, `cp-client`, and `cattle-grid`:

- `createHttpClient`
- `UnderlayHttpError`
- transport response/types from `client/types`
- query helpers/types
- restore-blocked helpers/types

## Judgment

`@inflatable-cookie/underlay/client` still earns retained Underlay ownership for now.

There is no honest extraction batch here today. The useful cleanup is boundary
clarity:

- keep the root `@inflatable-cookie/underlay/client` barrel stable for existing broad
  callers
- expose explicit `client/*` subpaths for the real feature families
- teach narrower imports in docs for new focused contracts

## Consumer Upgrade Impact

No consumer migration is required in this wave.

The new `client/*` subpaths are additive:

- existing root imports continue to work
- existing narrower imports continue to work
- docs can now point at the more specific public homes

## Status

- [x] Sweep the live caller family for `@inflatable-cookie/underlay/client`.
- [x] Confirm whether the package needs extraction or only boundary tightening.
- [x] Expose the real `client/*` subpaths and align the front-door docs.

## Complete

`g01.091` is complete. `@inflatable-cookie/underlay/client` remains a retained package
surface, but its public subpaths are now explicit:

- `client/auth`
- `client/errors`
- `client/format`
- `client/http`
- `client/navigation`
- `client/query`
- `client/route-protection`
- `client/soft-delete`
- `client/sveltekit`
- `client/types`
- `client/useAuth`

The root `@inflatable-cookie/underlay/client` barrel remains a stable convenience
surface for existing consumers.

## Next Task

If work continues immediately, the next honest retained-package challenge is
future `nightfire` extraction planning rather than more namespace micro-splits
inside `client`.
