# g12 - Standalone Nightfire Package

Status: active
Owner: repo maintainers and Acowtancy cross-repo Coordinator
Started: 2026-09-04

## Current Generation

`g12` removes the accidental package coupling between the generic
TypeScript/Svelte Nightfire runtime and the rest of Underlay. The operator
selected a standalone repository and immutable Git-tag distribution.

## Roadmap Sequence

1. [ ] [`g12.001`](001-standalone-nightfire-extraction-and-consumer-rollout.md)
   — extract, release, retain an Underlay compatibility facade, move Froyo,
   then remove Underlay from Bovine Desktop's frozen graph (`active`)

## Queue

- Ready: Acowtancy Market Card 272, standalone repository bootstrap and exact
  extraction proof.
- Gated: Card 273, first Nightfire release, after Card 272 and explicit
  operator release confirmation.
- Serial after release: Card 274 Underlay compatibility adoption and Card 275
  Froyo direct adoption may run in parallel.
- Serial after Froyo: Card 276 Bovine Desktop frozen-graph cutover.

## Dependencies And Parallelism

`g12` is independent of `g11` media consumer rollout. The Nightfire release is
serial behind extraction. Underlay compatibility work and Froyo adoption touch
different repositories and may run in parallel after that release. Desktop is
serial behind Froyo because it must consume the accepted package graph.

## Next Task

Dispatch Acowtancy Market Card 272 only.
