---
title: Data Media Relations Runtime Seam
owner: Codex
status: complete
updated: 2026-03-30
---

# Data Media Relations Runtime Seam

## Goal

Confirm the retained runtime boundary for the `data`, `media`, and `relations`
sub-barrels, and tighten any namespace that is still acting like a compatibility
dump instead of a deliberate helper API.

## Outcome

The retained runtime boundary is explicit:

- `@decodelabs/underlay/runtime/data`
  - selection history
  - pagination controllers/types
  - list, reorder, batch-selection, and batch-action controllers
- `@decodelabs/underlay/runtime/media`
  - browser upload helpers and media-upload flow/controller state
  - shared blob/media types
- `@decodelabs/underlay/runtime/relations`
  - relation search/suggestion helper contracts
  - drilldown search helpers
  - retained relation-selector context/types only

## Judgment

There is no hidden broad migration tail here.

`runtime/data` is still a real retained Underlay runtime family. The live caller
surface in admin apps and guides is broad, and the value is controller/state
orchestration rather than a Poodle design-system contract.

`runtime/media` is also a real retained runtime family. The upload helpers and
flow state own browser-upload orchestration, hashing, validation, and
shared media/blob types. That is not Poodle UI work, and it is not generic HTTP
client work either.

`runtime/relations` needed one real cleanup: it was still re-exporting the old
`RelationSelector/index.ts` barrel wholesale. The public UI wrapper family is
already gone, so the runtime barrel now re-exports only the retained helper
layer explicitly:

- local search helpers
- drilldown search helpers
- relation-selector context/types

## Changes

- narrowed `runtime/relations` to explicit helper/context/type exports
- updated active guides to teach the narrower `runtime/data`,
  `runtime/media`, and `runtime/relations` subpaths where those contracts are
  already stable
- updated roadmap front doors and durable inventory

## Next Task

Take the next retained-runtime audit on the root `@decodelabs/underlay/runtime`
barrel itself, and decide whether the root convenience barrel should stay broad
or be trimmed now that the stable subpaths are explicit and documented.
