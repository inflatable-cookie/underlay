---
title: Sibling Repo Package Boundary Recovery
owner: Codex
status: complete
updated: 2026-03-30
---

# Sibling Repo Package Boundary Recovery

## Goal

Close the loop on the Underlay package-boundary reshaping by proving that the
active sibling repos no longer depend on stale `components`, `embed`, or
deep/legacy `patterns` entrypoints and that the earlier Dairy import-recovery
work is actually green.

## Outcome

The package-boundary recovery is complete across the active sibling repo
surface.

The important results are:

- Dairy validates cleanly again after the broad manual import recovery
- there are no live active-source imports of:
  - `@decodelabs/underlay/components`
  - `@decodelabs/underlay/embed`
  - legacy/deep `@decodelabs/underlay/patterns/...` paths
- the only remaining references are historical docs, roadmap/accounting
  records, and scratch notes outside the supported live source surface

## Judgment

This was a real recovery closeout, not bookkeeping.

The earlier Underlay surface contraction and runtime split were correct, but a
bad broad import rewrite in Dairy temporarily obscured whether the sibling repo
surface was still sound. That meant the package-boundary line was not honestly
finished until:

- Dairy was repaired manually without losing its open work
- the active admin and Dairy repos validated cleanly again
- the residue scan confirmed that only historical references remained

## Changes

- repaired the broad Dairy import fallout manually in grouped batches instead
  of using compatibility shims or destructive rollback
- restored the correct package boundaries across the active sibling repos:
  - retained UI shells on `@decodelabs/underlay/patterns`
  - helper/controller/runtime imports on `@decodelabs/underlay/runtime`
  - retained editor/runtime imports on `@decodelabs/underlay/nightfire`
  - direct Poodle imports where the old Underlay wrapper boundary was gone
- revalidated the live sibling repo surface after recovery

## Validation

- `bun x svelte-check --tsconfig ./tsconfig.json` in `acowtancy/dairy`
- `effigy check:exports`
- `effigy qa:docs`
- `effigy qa:northstar`
- live residue scan across the active sibling repo surface for:
  - `@decodelabs/underlay/components`
  - `@decodelabs/underlay/embed`
  - `@decodelabs/underlay/patterns/...`

## Next Task

This recovery line is complete. If work continues immediately, the next honest
follow-on is a fresh retained-boundary challenge such as `client` reshaping or
future standalone extraction planning for `nightfire`, not more package-boundary
cleanup on the already-green sibling repo surface.
