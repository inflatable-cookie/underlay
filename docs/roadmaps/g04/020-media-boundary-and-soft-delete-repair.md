# 020 - Media Boundary And Soft Delete Repair

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.019` assessed the live storage/media implementation against `040` and
`050`.

The main storage primitives are usable, but the shared media boundary is still
split in two important ways:

- the shipped media repository does not follow the shared soft-delete batch
  contract
- the richer `050` usage-edge and migration-binding contract is only partially
  represented by the live repository implementation

This lane exists to repair the boundary before the jobs/operator assessment
moves further up the stack.

## Goals

- align shared media soft delete with the retained `delete_batch_id` contract
- make the lower/higher media authority chain explicit and honest
- identify whether the generalized usage-edge contract should be implemented in
  the shipped Postgres repository now or narrowed back in the contract/docs
- leave the shared media surface in one coherent state instead of two
  overlapping stories

## Non-Goals

- broad media feature redesign
- application-level media workflow or UI work
- jumping ahead to operator systems before the storage/media boundary is clear

## Inputs

- [docs/roadmaps/g04/019-storage-and-media-assessment.md](/Users/tom/Dev/projects/underlay/docs/roadmaps/g04/019-storage-and-media-assessment.md)
- [docs/contracts/040-storage-blob-and-media-systems.md](/Users/tom/Dev/projects/underlay/docs/contracts/040-storage-blob-and-media-systems.md)
- [docs/contracts/050-media-library-and-usage.md](/Users/tom/Dev/projects/underlay/docs/contracts/050-media-library-and-usage.md)
- `rust/crates/underlay-media/**`
- `rust/crates/underlay-soft-delete/**`
- `rust/crates/underlay-blob/**`

## Exit Criteria

- the shared media soft-delete story is aligned with the canonical contract
- the lower/higher media contract chain is truthful in both docs and code
- the next higher assessment lane can treat storage/media as a stable lower
  dependency

## Result

Completed as an honesty repair.

Implemented:

- `040` now distinguishes baseline `deleted_at` soft-delete semantics from the
  fuller `delete_batch_id` batch restore/purge model and explicitly records
  that shipped media currently only guarantees trash semantics
- `050` is now marked active instead of proposed and explicitly states that the
  richer usage-edge/migration-binding model is only partially implemented by
  the shipped default backend

Why this lane stopped there:

- `underlay-media` ships repository code but no canonical shared media schema
  migration of its own, so forcing `delete_batch_id` and the full richer
  usage-edge backend model in this batch would invent storage authority the
  repo does not actually own yet

Remaining media drift:

- the shipped media repository still exposes the old simple usage model while
  the newer generalized sync traits are additive rather than the default shared
  backend path
- media soft delete still does not implement retained batch restore/purge
  semantics in code

## Next Task

Execute `g04.021`: assess the live jobs and operator-systems implementation
against `060`.
