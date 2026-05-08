# 026 - TS AI And Suggestion Authority Repair

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.025` assessed the AI runtime and suggestions systems against `080`.

The lower Rust runtime and server-side suggestion crates are broadly aligned.
The real shared drift is on the TS side:

- suggestion request helpers are duplicated across `client` and `patterns`
- the contract still carries stale wording about the TS AI surface even though
  `runtime/ai.ts` now exists as a thin compatibility barrel

## Goals

- dedupe the TS suggestion helper boundary onto one retained owner
- repair the `080` wording so it matches the actual TS AI surface
- leave the AI runtime and suggestion contracts easier to assess later without
  split authority

## Non-Goals

- redesigning app-local AI UX or prompt flows
- inventing a larger TS AI runtime family than the repo actually has
- broad Rust AI runtime changes when the live lower layer is already aligned

## Inputs

- [docs/roadmaps/g04/025-ai-runtime-and-suggestions-assessment.md](/Users/tom/Dev/projects/underlay/docs/roadmaps/g04/025-ai-runtime-and-suggestions-assessment.md)
- [docs/contracts/080-ai-runtime-and-suggestions.md](/Users/tom/Dev/projects/underlay/docs/contracts/080-ai-runtime-and-suggestions.md)
- `ts/src/client/suggestions.ts`
- `ts/src/patterns/selection-history.ts`
- `ts/src/runtime/ai.ts`

## Exit Criteria

- the TS suggestion helper authority is no longer duplicated
- the AI contract/docs describe the actual TS AI surface honestly
- the next TS runtime/pattern assessment can treat the AI/suggestion boundary
  as stable lower context

## Changes

- made `ts/src/client/suggestions.ts` the single logic owner for the shared TS
  suggestion request helpers
- turned `ts/src/patterns/selection-history.ts` into a compatibility re-export
  surface for those helpers instead of a second implementation
- repaired `080` so it names the real `runtime/ai.ts` surface and stops
  claiming that no such file exists
- updated the system inventory so the TS AI surface list matches the real repo

## Result

The AI/suggestion boundary is now honest and narrower:

- one shared TS helper implementation owns the `suggestions` /
  `recentHints` request vocabulary
- `selection-history` still exposes the helper names for compatibility, but it
  no longer carries duplicate logic
- the contract now describes `runtime/ai.ts` as the thin compatibility barrel
  it actually is

## Next Task

Execute `g04.027`: assess the TS runtime and client orchestration layer
against `090`.
