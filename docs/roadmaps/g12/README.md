# g12 - Standalone Nightfire Package

Status: active
Owner: repo maintainers and Acowtancy cross-repo Coordinator
Started: 2026-09-04

## Current Generation

`g12` moves the generic Rust and TypeScript/Svelte Nightfire system into one
standalone repository and removes its accidental coupling to Underlay. The
operator selected crate name `nightfire` and immutable Git-tag distribution.

## Roadmap Sequence

1. [ ] [`g12.001`](001-standalone-nightfire-extraction-and-consumer-rollout.md)
   — complete both language extractions, release, retain Underlay compatibility
   facades, move Froyo and Farmyard, then remove Underlay from Bovine Desktop's
   frozen graph (`active`)

## Queue

- Closed incomplete: Card 272 created the repository and TS tranche but wrongly
  excluded Rust.
- Ready: Card 278, dual-language repository reshape and Rust extraction.
- Gated: Card 273, first Nightfire release, after Card 278 and explicit
  operator release confirmation.
- Serial after release: Card 274 Underlay compatibility, Card 275 Froyo direct
  adoption, and Card 279 Farmyard Rust adoption may run in parallel.
- Serial after Froyo: Card 276 Bovine Desktop frozen-graph cutover.

## Dependencies And Parallelism

`g12` is independent of `g11` media consumer rollout. The Nightfire release is
serial behind Card 278. Underlay compatibility, Froyo adoption, and Farmyard
Rust adoption touch separate implementation lanes and may run in parallel after
that release. Desktop is serial behind Froyo.

## Next Task

Dispatch Acowtancy Market Card 278 only.
