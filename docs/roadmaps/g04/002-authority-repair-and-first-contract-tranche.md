# 002 - Authority Repair And First Contract Tranche

Status: active
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.001` established the system inventory, the contract index, and the
parallel-generation posture. The remaining job is to turn that planning spine
into a real execution sequence.

The correct first tranche is the lower-layer foundation and transport surface.
Those contracts govern the envelope, error, validation, and client/server
transport seams that later auth, storage, jobs, content, and template contracts
depend on.

## Goals

- finish authority repair where repo-facing inventory docs were stale
- make the first contract tranche explicit and bounded
- sequence foundation before transport so the lower-level primitive contract is
  settled first

## Non-Goals

- writing auth, storage, jobs, Nightfire, or template contracts in this card
- broad implementation fixes before the governing lower-layer contracts exist
- changing `g03` ownership or priorities

## Outputs

- refreshed architecture inventory docs that match the live filesystem
- a bounded first contract-writing tranche:
  - `010-foundation-primitives-and-envelopes.md`
  - `020-http-transport-and-server-boundary.md`
- an explicit next roadmap step for the first actual contract file

## Exit Criteria

- package/inventory authority no longer contradicts the repo surface
- the first contract tranche is named and justified in roadmap state
- the next card can write the foundation contract without reopening tranche
  planning

## Next Task

Execute `g04.003`: write `010-foundation-primitives-and-envelopes.md`.
