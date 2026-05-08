# 013 - Tooling Testing And Contract Artifacts Contract

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.012` settles the admin template system boundary. The last contract lane in
this coverage wave is the repo support layer: testing helpers, devtools,
guardrails, scanners, and machine-readable contract artifacts.

## Goals

- define the tooling, testing, and contract-artifact contract
- separate core retained support surfaces from repo-local delivery glue
- complete the first full contract-coverage wave so implementation assessment
  can be planned honestly

## Non-Goals

- implementation repair beyond light authority alignment needed to write the
  contract
- reopening earlier system-family contracts in the same batch
- consumer rollout execution work

## Inputs

- `rust/crates/underlay-testing/**`
- `rust/crates/underlay-devtools/**`
- `ts/src/tools/**`
- `ts/src/testing/**`
- `contracts/**`

## Exit Criteria

- `120-tooling-testing-and-contract-artifacts.md` exists
- the contract names the canonical support interfaces, invariants, extension
  points, and known drift
- `g04` front doors point at the final tooling lane

## Next Task

Execute `g04.014`: compile the implementation-assessment sequence and confirm
the first bounded repair lane.
