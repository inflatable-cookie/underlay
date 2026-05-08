# 003 - Foundation Primitives And Envelopes Contract

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.002` defined the first contract-writing tranche. The next valid move was
to turn the lowest shared layer into a real contract before touching transport,
auth, storage, jobs, content, or templates.

## Goals

- write the first substantive system contract in `docs/contracts/`
- pin the shared primitive boundary around ids, envelopes, and validation
- make later transport and implementation assessment work depend on explicit
  authority instead of inference

## Outputs

- [`docs/contracts/010-foundation-primitives-and-envelopes.md`](/Users/tom/Dev/projects/underlay/docs/contracts/010-foundation-primitives-and-envelopes.md)
- refreshed contract and roadmap front doors so `g04` no longer points at
  completed launch cards

## Outcome

The foundation contract now exists.

It settles:

- `Uuid` as the shared identifier primitive
- `SingleResponse<T>` and `ListResponse<T>` as the canonical success envelopes
- `ErrorEnvelope` and `AppError` as the canonical transport error spine
- the boundary between richer internal validation errors and the thinner shared
  wire contract

It also records a live implementation drift: `ValidatedJson` currently leaks
the wrong transport shape and needs later repair under the assessment lane.

## Next Task

Execute `g04.004`: write `020-http-transport-and-server-boundary.md`.
