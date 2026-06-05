# g06.032 - Typed ExistsCheck Execution And Rollout

## Why

`g06.031` found that `underlay-db::ExistsCheck` is the remaining runtime helper
that still exposes raw schema/table/column strings as the primary public shape.

The implementation validates and quotes identifiers today, but reference-grade
Underlay should make typed identifier construction the normal path.

## Goal

Add a typed composite existence-check builder and migrate current consumer
usage.

## Scope

In scope:

- add a typed builder or typed constructor for existence checks
- use `QualifiedTableName` and `SqlIdentifier` at the construction boundary
- support current composite use cases:
  - string equality
  - integer equality
  - UUID scope equality
  - nullable integer equality
  - exclusion by record id
  - include/exclude soft-deleted rows
- keep raw helpers initially as compatibility wrappers
- migrate Farmyard call sites
- decide whether raw helpers should be deprecated after rollout

Out of scope:

- media Postgres config refactor
- test-only `TestDb` cleanup
- broad DB helper redesign
- TypeScript/Svelte work
- release execution or publishing

## Contract References

- `021`: database migration and schema workflow
- `023`: release and compatibility rollout
- `122`: Rust public API inventory

## Acceptance Criteria

- typed existence builder compiles and is documented
- existing raw helper behavior is preserved during additive rollout
- Farmyard call sites migrate or bounded exceptions are recorded
- targeted Underlay and Farmyard checks pass or failures are classified

## Consumer Upgrade Impact

Impact: additive first. Deprecation is allowed only after Farmyard migration
proof.

## Current State

`g06.032` is ready after `g06.031`.

## Next Task

Execute `g06.032`: typed `ExistsCheck` execution and consumer rollout.
