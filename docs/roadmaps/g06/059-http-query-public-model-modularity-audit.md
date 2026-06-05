# g06.059 - HTTP Query Public Model Modularity Audit

## Why

After `g06.058`, `underlay-http/src/query.rs` is the largest remaining
production/shared Rust god-file outside tests. It is app-facing HTTP helper
surface, so it should be audited before any split.

## Goal

Classify the HTTP query public model surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-http/src/query.rs` by helper family
- classify stable app-facing query parsing and extraction contracts
- identify private/internal helpers that can move without source breakage
- decide whether the next batch should split HTTP query parsing, target cookie
  helpers, or defer HTTP behind a broader boundary checkpoint
- preserve crate-root exports unless a breaking change is explicitly approved

Out of scope:

- changing HTTP query parsing semantics
- changing pagination semantics
- changing cookie or error-logging behavior
- consumer rollout unless the audit finds a public import risk

## Acceptance Criteria

- produce an audit artifact with a recommended next code batch
- classify expected consumer impact
- update the Rust public API inventory if the HTTP query boundary needs tighter
  notes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an audit and planning checkpoint. Any breaking HTTP query contract
change must be promoted into an explicit follow-up card before execution.

## Current State

`g06.059` is next after `g06.058`.

## Next Task

Execute `g06.059`: HTTP query public model modularity audit.
