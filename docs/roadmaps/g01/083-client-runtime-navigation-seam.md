---
title: Client Runtime Navigation Seam
owner: Codex
status: complete
updated: 2026-03-30
---

# Client Runtime Navigation Seam

## Goal

Review the `client` / `runtime` seam and decide whether any currently retained
runtime helpers should instead live under `client`, especially where browser
orchestration and transport helpers sit close together.

## Outcome

The current navigation split is correct and should stay as-is.

- `@decodelabs/underlay/client`
  - SvelteKit-specific navigation and transport integration
  - `gotoWithContext`
  - `navigateBack`
  - `navigateOnCancel`
  - `initPageState`
  - `capturePageState`
  - query parsing and HTTP/client helpers
- `@decodelabs/underlay/runtime`
  - framework-agnostic navigation context/state
  - `pushNavigationContext`
  - `consumeNavigationContext`
  - `getBackButtonInfo`
  - page-state storage helpers
  - navigation config/types

## Judgment

There is no worthwhile extraction batch here.

The live caller surface is already using the right split:

- route-level SvelteKit navigation uses `@decodelabs/underlay/client`
- retained context/state orchestration and back-button computation use
  `@decodelabs/underlay/runtime`

Moving the runtime navigation helpers into `client` would make the framework
boundary worse, not better, because the retained navigation context/state layer
is still framework-agnostic and is also used as a shared contract in docs and
app-owned shells.

Likewise, the `client` barrel still earns its current scope:

- HTTP/client construction
- query helpers
- auth-cookie and token store helpers
- route protection
- SvelteKit navigation helpers

So the correct result is an explicit retained seam, not another namespace move.

## Changes

- documented the navigation seam as an explicit retained boundary
- clarified the architecture/guides so `client` is framed as transport and
  framework integration, while `runtime` stays the home of framework-agnostic
  orchestration/state
- updated roadmap front doors and durable inventory

## Next Task

Take the next broad TS audit on the `nightfire` package surface, and decide
whether any remaining editor/runtime helpers can be reorganized now for a
future standalone extraction without changing the current public package
boundary.
