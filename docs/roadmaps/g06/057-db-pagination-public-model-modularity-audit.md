# g06.057 - DB Pagination Public Model Modularity Audit

## Why

After `g06.056`, `underlay-db/src/pagination.rs` is the largest remaining
production/shared Rust god-file outside tests. It is stable app-facing helper
surface, so it should be audited before any split.

## Goal

Classify the pagination public model surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-db/src/pagination.rs` by helper family
- classify stable app-facing pagination contracts
- identify private/internal helpers that can move without source breakage
- decide whether the next batch should split pagination, target HTTP query
  parsing instead, or defer both behind a broader HTTP/DB boundary checkpoint
- preserve crate-root exports unless a breaking change is explicitly approved

Out of scope:

- changing pagination response/query semantics
- changing SQL query behavior
- moving HTTP query behavior
- consumer rollout unless the audit finds a public import risk

## Acceptance Criteria

- produce an audit artifact with a recommended next code batch
- classify expected consumer impact
- update the Rust public API inventory if the DB pagination boundary needs
  tighter notes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an audit and planning checkpoint. Any breaking pagination contract
change must be promoted into an explicit follow-up card before execution.

## Current State

`g06.057` is complete.

Artifact:

- [057 artifact](./057-db-pagination-public-model-modularity-audit-artifact.md)

## Next Task

Execute `g06.058`: DB pagination internal split.
