# g12 - Standalone Nightfire Package

Status: active
Owner: repo maintainers and Acowtancy cross-repo Coordinator
Started: 2026-09-04

## Current Generation

`g12` moves the generic Rust and TypeScript/Svelte Nightfire system
into one standalone repository and removes its accidental coupling to
Underlay. The operator selected crate name `nightfire` and immutable
Git-tag distribution.

This README owns the `g12` roadmap and approved frontier. The
`g12.NNN` file below is the sole executable planning unit; there is no
milestone wrapper or nested `batch-cards/` hierarchy.

## Task Sequence

1. [ ] [`g12.001`](001-standalone-nightfire-extraction-and-consumer-rollout.md)
   — complete both language extractions, release, retain Underlay
   compatibility facades, move Froyo and Farmyard, then remove
   Underlay from Bovine Desktop's frozen graph (`active`)

`g12.001` was already flat and is unchanged by the switchover except
for this frontier wording.

## Approved Frontier

Ready: Acowtancy Market Card 278 lane under `g12.001` only. The
release and consumer lanes remain serial behind its accepted
dual-language proof.

## Dependencies And Parallelism

`g12` is independent of `g11` media consumer rollout. The Nightfire
release is serial behind Card 278. Underlay compatibility, Froyo
adoption, and Farmyard Rust adoption touch separate implementation
lanes and may run in parallel after that release. Desktop is serial
behind Froyo.

## Next Task

Dispatch Acowtancy Market Card 278 only.
