# 079 - Auth And Browser Runtime Seam

Status: Complete
Owner: Platform
Created: 2026-03-30
Depends on: 078

## Overview

The next retained-runtime question after the feedback boundary was the
auth/browser seam:

- which exports in `runtime/auth` are truly retained runtime helpers
- which exports are just duplicate client surface leaking through runtime
- whether the active docs still teach the honest narrowed runtime boundary

## Findings

The strongest real duplication was in `patterns/auth.ts`:

- `createAuthStore`
- `AuthState`
- `AuthStore`

Those already belong to `@inflatable-cookie/underlay/client` via `client/useAuth`.
They are not runtime helpers and had no live consumer usage through
`@inflatable-cookie/underlay/runtime`.

By contrast, these still earn retained `runtime/auth` ownership:

- `configureAuth()` / `getAuthConfig()`
- `useAuthenticatedData()`
- passkey hooks
- account/profile helper types and display-name helpers
- small auth-state guards like `isAuthenticated()` / `requireAuth()`

## Delivery

- removed the duplicate client auth-store re-export from `patterns/auth.ts`
- kept `runtime/auth` focused on retained auth runtime helpers rather than
  duplicating `client/useAuth`
- updated active guides to import auth runtime helpers from the narrower
  `@inflatable-cookie/underlay/runtime/auth` subpath
- updated the roadmap/front-door layer so the auth/browser seam decision is
  explicit

## Boundary Result

Auth store construction and transport stay on `@inflatable-cookie/underlay/client`.

Retained auth runtime orchestration stays on
`@inflatable-cookie/underlay/runtime/auth`.

This is the right split because it keeps app/runtime auth fetch/config hooks in
Underlay runtime without turning `runtime/auth` into a second client package.

## Consumer Upgrade Impact

- `createAuthStore`, `AuthState`, and `AuthStore` should be imported from
  `@inflatable-cookie/underlay/client` when needed
- `configureAuth`, `useAuthenticatedData`, passkey hooks, and account helper
  types/functions should be imported from
  `@inflatable-cookie/underlay/runtime/auth`
- root `@inflatable-cookie/underlay/runtime` imports continue to work where already
  used

## Next Task

The strongest next retained-runtime follow-on is the browser seam: review
`runtime/browser` and decide whether `timezone`, `storage`, `clipboard`, and
keyboard-shortcut helpers should stay together in Underlay runtime or split
further into `client` or a smaller standalone browser-runtime package.
