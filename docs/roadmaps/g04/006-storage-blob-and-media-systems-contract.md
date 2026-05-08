# 006 - Storage Blob And Media Systems Contract

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.005` settles the auth and session layer. The next dependency is the shared
storage/media boundary: DB bootstrap, blob adapters, soft delete, media
orchestration, and storage ownership.

## Goals

- define the shared storage and media contract across Rust and TS-facing
  consumers
- separate durable storage/media semantics from app-local upload UX
- prepare the existing media contract for later implementation assessment under
  a clearer lower-level storage boundary

## Non-Goals

- implementation repair beyond light authority alignment needed to write the
  contract
- app-specific media workflows or content schemas
- jobs/events/operator infrastructure work

## Inputs

- [`docs/contracts/050-media-library-and-usage.md`](/Users/tom/Dev/projects/underlay/docs/contracts/050-media-library-and-usage.md)
- `rust/crates/underlay-db/**`
- `rust/crates/underlay-soft-delete/**`
- `rust/crates/underlay-blob/**`
- `rust/crates/underlay-aws/**`
- `rust/crates/underlay-media/**`

## Outputs

- [`docs/contracts/040-storage-blob-and-media-systems.md`](/Users/tom/Dev/projects/underlay/docs/contracts/040-storage-blob-and-media-systems.md)
- refreshed contract and roadmap front doors so `g04` now points at the
  operator-systems lane

## Outcome

The storage/media contract now exists.

It settles:

- shared DB bootstrap, migration, schema-guard, and DB-diagnostic helpers
- soft-delete semantics and batch-correlation rules
- blob adapter, upload-plan, signed-download, and backend config seams
- lower-level media repository, storage-key, and sync-support ownership
- the authority split between this lower contract and the higher `050` media
  graph contract

It also records current drift to assess later, especially the stale status of
`050`, the mixed old/new usage-sync repository shape, and the placement of
`MediaConfig` inside the blob crate.

## Next Task

Execute `g04.007`: write `060-jobs-events-and-operator-systems.md`.
