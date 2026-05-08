# 001 - Underlay Contract Coverage And Assessment Program

Status: active
Owner: repo maintainers
Updated: 2026-05-08

## Context

Large parts of Underlay predate the Northstar planning system.

The repo has many significant shared systems, but only a thin contract surface.
The goal of this generation is not to start broad implementation churn. It is
to establish the contract spine first, then assess whether the implementations
and contracts actually satisfy each system's purpose.

## Goals

- inventory the full significant Underlay system surface
- compile the missing contract set
- write contracts in a bounded order from lower layers upward
- define the later implementation-assessment wave for each system family

## Non-Goals

- pausing or replacing the active `g03` template-system thread
- broad implementation fixes before the governing contracts exist
- reopening closed historical generations

## Inputs

- [system-inventory.md](../../architecture/system-inventory.md)
- [contract-index.md](../../contracts/contract-index.md)
- the live Rust and TS package surfaces under `rust/` and `ts/src/`

## Execution Plan

### Batch 1.1 - Authority Repair

- [x] identify planning drift and stale front doors
- [x] create a real system inventory
- [x] create a canonical contract index
- [x] align the remaining stale authority surfaces to the live generation state

### Batch 1.2 - Contract Compilation

- [x] write foundation and transport contracts
- [x] write auth, storage, and jobs/operator contracts
- [x] write Nightfire, AI, runtime, pattern, template, and tooling contracts

### Batch 1.3 - Assessment Sequencing

- [x] compile the implementation-vs-contract assessment chain
- [x] identify the first bounded repair lane after contract coverage exists

## Exit Criteria

- the significant system inventory is explicit
- the contract set is fully named and sequenced
- the repo front doors acknowledge parallel mode cleanly
- the next implementation-assessment batch is ready without reopening planning
  from scratch

## Next Task

The program is complete. `g04` is closed.
