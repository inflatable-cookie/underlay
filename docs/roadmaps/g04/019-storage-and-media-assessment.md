# 019 - Storage And Media Assessment

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.018` repaired the auth boundary drift. The next assessment wave in the
contract order is storage and media.

This lane covers the lower durable-storage seam in `040` plus its relationship
to the older retained media contract in `050`.

## Goals

- assess the live storage/blob/media implementation against `040` and `050`
- separate true contract failures from older dual-contract wording or package
  residue
- identify the smallest honest repair set for the shared storage/media boundary
- leave explicit findings and a bounded next lane instead of broad media churn

## Non-Goals

- executing large storage/media refactors in the same batch
- skipping ahead to jobs/operator work before media findings are explicit
- consumer app rollout work

## Inputs

- [docs/contracts/040-storage-blob-and-media-systems.md](/Users/tom/Dev/projects/underlay/docs/contracts/040-storage-blob-and-media-systems.md)
- [docs/contracts/050-media-library-and-usage.md](/Users/tom/Dev/projects/underlay/docs/contracts/050-media-library-and-usage.md)
- `rust/crates/underlay-db/**`
- `rust/crates/underlay-blob/**`
- `rust/crates/underlay-media/**`
- `rust/crates/underlay-soft-delete/**`
- `rust/crates/underlay-aws/**`

## Exit Criteria

- the live storage/media implementation is reviewed against `040` and `050`
- the real findings are documented in severity order
- the next repair step is expressed as one bounded roadmap lane or a small
  repair set
- jobs/operator assessment can start without ambiguity about the shared
  storage/media boundary

## Findings

### 1. Shipped media soft delete does not implement the canonical batch-delete contract

Severity: high

`040` says `deleted_at` and `delete_batch_id` are the canonical shared
soft-delete conventions. The shipped media repository only toggles
`deleted_at`, returns plain booleans/counts, and exposes no batch correlation
or restore/purge outcome model.

Evidence:

- [rust/crates/underlay-soft-delete/src/lib.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-soft-delete/src/lib.rs:1)
- [rust/crates/underlay-media/src/postgres.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-media/src/postgres.rs:212)
- [rust/crates/underlay-media/src/postgres.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-media/src/postgres.rs:824)

Impact:

- media is a special-case soft-delete domain outside the shared retained
  contract
- batch restore/purge semantics cannot compose cleanly with the rest of the
  shared soft-delete system

### 2. The richer `050` usage-edge and migration-binding contract is only partially live

Severity: medium

The repo now has the newer generalized types and sync traits in
`underlay-media::sync`, but the shipped Postgres repository still only
implements the older `MediaUsage` / `track_usage` / `sync_usages(entity_type,
field_name, media_ids)` model.

Evidence:

- [rust/crates/underlay-media/src/repository.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-media/src/repository.rs:1)
- [rust/crates/underlay-media/src/sync.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-media/src/sync.rs:1)
- [rust/crates/underlay-media/src/postgres.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-media/src/postgres.rs:657)

Impact:

- the lower/higher media boundary is not just layered; it is split between an
  old shipped repository contract and a newer additive trait family
- `050` currently overstates how much of the richer usage-edge/media-binding
  model is actually implemented in the default shared backend

### 3. `050` still presents itself as proposed even though the rest of the repo treats it as active authority

Severity: medium

This is a real authority mismatch, not just a wording nit.

Evidence:

- [docs/contracts/050-media-library-and-usage.md](/Users/tom/Dev/projects/underlay/docs/contracts/050-media-library-and-usage.md:1)
- [docs/contracts/040-storage-blob-and-media-systems.md](/Users/tom/Dev/projects/underlay/docs/contracts/040-storage-blob-and-media-systems.md:319)

Impact:

- the lower/higher contract chain is not self-consistent in file state
- maintainers cannot tell whether `050` is governing contract, aspiration, or
  historical design sketch

### 4. `underlay_blob::MediaConfig` still sits at an awkward ownership layer

Severity: low

The current config spans upload size policy and thumbnail concerns even though
thumbnail generation and rendition policy are media-layer concerns.

Evidence:

- [rust/crates/underlay-blob/src/config.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-blob/src/config.rs:1)
- [docs/contracts/040-storage-blob-and-media-systems.md](/Users/tom/Dev/projects/underlay/docs/contracts/040-storage-blob-and-media-systems.md:327)

Impact:

- this is an ownership-clarity issue, not the first repair target

## Assessment Result

The next bounded repair lane should focus on the media boundary itself:

- decide whether media should adopt the shared batch soft-delete model or be
  explicitly carved out
- make `050` either fully active and truthful or narrower and clearly staged
- reconcile the old repository usage model with the newer generalized sync
  surface so there is one honest shared story

## Next Task

Execute `g04.020`: repair the media boundary and soft-delete drift before
promoting the next higher assessment lane.
