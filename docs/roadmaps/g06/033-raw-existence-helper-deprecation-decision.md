# g06.033 - Raw Existence Helper Deprecation Decision

## Why

`g06.032` added `TypedExistsCheck` and migrated the known consumer usage.

The raw existence helpers still validate and quote identifiers, but they remain
the weaker public shape.

## Goal

Decide whether raw existence helpers should be deprecated, retained, or removed.

## Scope

In scope:

- inspect raw `ExistsCheck`
- inspect raw value existence helper functions
- rescan the six consumers for remaining usage
- classify posture as retained compatibility, deprecation, or removal
- update contracts and guides

Out of scope:

- new typed builder functionality
- media Postgres config refactor
- test-only `TestDb` cleanup
- TypeScript/Svelte work
- release execution or publishing

## Contract References

- `021`: database migration and schema workflow
- `023`: release and compatibility rollout
- `122`: Rust public API inventory

## Acceptance Criteria

- raw existence helper posture is explicit
- current consumer usage is proved
- any deprecation/removal is reflected in docs
- targeted Rust and consumer checks pass or failures are classified

## Consumer Upgrade Impact

Impact: deprecation or breaking only if the batch changes the raw helper public
surface. Known consumer usage was migrated in `g06.032`.

## Current State

`g06.033` is ready after `g06.032`.

## Next Task

Execute `g06.033`: raw existence helper deprecation decision.
