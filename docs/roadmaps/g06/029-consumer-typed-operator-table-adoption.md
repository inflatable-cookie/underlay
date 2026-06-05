# g06.029 - Consumer Typed Operator Table Adoption

## Why

`g06.028` added typed audit and security-alert table APIs without breaking the
existing six consumers.

Reference-grade Underlay should prove the preferred API path in real apps
before deprecating or removing the compatibility wrappers.

## Goal

Move the six known consumers from raw audit/security-alert table strings to the
typed table config APIs.

## Scope

In scope:

- migrate direct audit callers to `AuditTable` and typed audit helpers
- migrate direct security-alert callers to `LoginAttemptsTable`,
  `SecurityAlertEventsTable`, or `SecurityAlertTables`
- keep table locations in typed app state/config where each consumer already
  has a config boundary
- run targeted checks for affected Rust packages
- decide whether raw-string wrappers should remain compatibility, become
  deprecated, or be removed in a later breaking batch

Out of scope:

- changing table schemas
- changing alert thresholds
- changing audit event semantics
- release execution or publishing

## Contract References

- `021`: database migration and schema workflow
- `023`: release and compatibility rollout
- `033`: error codes and operator audit
- `060`: jobs, events, and operator systems
- `122`: Rust public API inventory

## Acceptance Criteria

- all six consumers are inspected for affected call sites
- each live direct call site either migrates to typed APIs or records a bounded
  exception
- targeted Rust checks pass or failures are classified
- Underlay docs classify the final raw-wrapper posture

## Consumer Upgrade Impact

Impact: additive unless the batch chooses explicit deprecation. No raw wrapper
removal should happen until the six-consumer migration proof is complete.

## Current State

`g06.029` is complete.

See
[`029-consumer-typed-operator-table-adoption-artifact.md`](029-consumer-typed-operator-table-adoption-artifact.md).

## Next Task

Execute `g06.030`: raw operator wrapper removal readiness and remaining
dynamic-identifier audit.
