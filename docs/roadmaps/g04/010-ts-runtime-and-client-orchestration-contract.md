# 010 - TS Runtime And Client Orchestration Contract

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.009` settles the AI runtime and suggestion layer. The next dependency is
the retained TS orchestration surface: runtime helpers plus the still-shared
client-side controller glue that sits above transport and below workflow
patterns.

## Goals

- define the retained TS runtime and client orchestration contract
- separate true shared runtime/controller seams from helpers that really belong
  to higher workflow patterns
- prepare the later pattern and template contracts on top of a clear TS runtime
  boundary

## Non-Goals

- implementation repair beyond light authority alignment needed to write the
  contract
- app-specific workflow pages or state machines
- frontend template work owned by `g03`

## Inputs

- `ts/src/runtime/**`
- selected `ts/src/client/**`
- dependent contracts for transport, auth, media, and AI

## Outputs

- [`docs/contracts/090-ts-runtime-and-client-orchestration.md`](/Users/tom/Dev/projects/underlay/docs/contracts/090-ts-runtime-and-client-orchestration.md)
- refreshed contract and roadmap front doors so `g04` now points at the
  patterns lane

## Outcome

The TS runtime and client orchestration contract now exists.

It settles:

- the `runtime/*` subpath model as a retained public compatibility/controller
  surface
- the browser auth command/store seam
- the SvelteKit auth/cookie integration surface
- the SvelteKit navigation wrapper over pattern-owned navigation context
- the route-protection and browser-facing error/type helper layer

It also records the main drift to assess later, especially the fact that most
runtime files are thin re-export barrels, the muddy split between runtime and
patterns ownership, and the over-broad mixed helper families in
`runtime/data.ts`.

## Next Task

Execute `g04.011`: write `100-shared-patterns-and-workflow-shells.md`.
