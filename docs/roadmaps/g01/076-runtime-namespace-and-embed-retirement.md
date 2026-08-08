# 076 - Runtime Namespace and Embed Retirement

Status: Complete
Owner: Platform
Created: 2026-03-30
Depends on: 075

## Overview

The TS audit showed two immediate cleanup truths:

- `embed` had no live consumer usage and no longer earned a public Underlay
  package surface
- `patterns` was still mixing retained workflow shells with a broad runtime
  helper/controller namespace

This batch closes both problems in one cut:

- retire public `@inflatable-cookie/underlay/embed`
- introduce public `@inflatable-cookie/underlay/runtime`
- narrow `@inflatable-cookie/underlay/patterns` to retained workflow/page shells
- migrate the live sibling-repo callers onto the honest namespace split

## Delivery

- added public `runtime` barrel at `ts/src/runtime/index.ts`
- removed public `embed` export from `package.json`
- deleted `ts/src/embed/` plus its dead test suite under `ts/tests/embed/`
- narrowed `ts/src/patterns/index.ts` to retained workflow/page-shell exports
- updated `ts/src/index.ts` so the root barrel re-exports `runtime`
- moved live helper/controller callers in `acme-admin`, `cp-admin`,
  `acme-front`, `cp-front`, `acme-client`, and `cp-client` onto
  `@inflatable-cookie/underlay/runtime`
- updated active guide and architecture references so the namespace boundary is
  explicit

## Boundary Result

Public Underlay TS namespace is now:

- `@inflatable-cookie/underlay/patterns`
  - retained workflow/page shells only
- `@inflatable-cookie/underlay/runtime`
  - shared app/runtime helpers, controllers, and browser utilities
- `@inflatable-cookie/underlay/nightfire`
  - retained editor/runtime package surface
- `@inflatable-cookie/underlay/client`
  - retained client package surface
- `@inflatable-cookie/underlay/server`
  - retained server helpers
- `@inflatable-cookie/underlay/utils`
  - retained small utility surface

## Consumer Upgrade Impact

- helper/controller imports that previously came from
  `@inflatable-cookie/underlay/patterns` should now come from
  `@inflatable-cookie/underlay/runtime`
- retained workflow/page shells stay on `@inflatable-cookie/underlay/patterns`
- `@inflatable-cookie/underlay/embed` is retired

## Next Task

The strongest next cleanup is a focused retained-runtime review inside
`@inflatable-cookie/underlay/runtime`, starting with the toast helper boundary and the
remaining root-barrel question.
