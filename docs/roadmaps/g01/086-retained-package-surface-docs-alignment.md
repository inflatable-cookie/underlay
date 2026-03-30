---
title: Retained Package Surface Docs Alignment
owner: Codex
status: complete
updated: 2026-03-30
---

# Retained Package Surface Docs Alignment

## Goal

Align the front-door and architecture docs so Underlay’s retained TypeScript and
Svelte package surfaces are described consistently after the contraction and
runtime audit line.

## Outcome

The top-level docs no longer describe the remaining Underlay package surfaces
as vague leftovers from the old Svelte contraction work.

They now present the real package boundary explicitly:

- Poodle owns primitives and generic composites
- Underlay `patterns` owns retained workflow/page-shell UI
- Underlay `runtime` owns shared app/runtime helpers and controllers
- Underlay `utils` owns the small standalone helper surface
- Underlay `client` owns transport and SvelteKit-facing client helpers
- Underlay `nightfire` owns the structured-content editor/runtime package

## Changes

- updated architecture language in `docs/architecture/040-svelte-ui-kit.md`
- updated guide front doors in:
  - `docs/guides/000-overview.md`
  - `docs/guides/090-ui-kit.md`
  - `docs/guides/README.md`
- updated the repo front door in `README.md`

## Next Task

This TS/package-surface audit line is complete. If work continues immediately,
the next honest follow-on is a fresh boundary challenge on one retained package
surface rather than more taxonomy cleanup.
