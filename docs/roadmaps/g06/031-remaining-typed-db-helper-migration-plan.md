# g06.031 - Remaining Typed DB Helper Migration Plan

## Why

`g06.030` removed raw audit/security-alert operator wrappers. The remaining
dynamic identifier surface is now mostly generic DB helpers and adapter config.

The next change should avoid a broad churn batch. First, classify the remaining
helpers and decide the narrowest typed migration path.

## Goal

Plan the remaining typed DB helper migration after the operator wrapper removal.

## Scope

In scope:

- inspect `underlay-db::ExistsCheck`
- inspect identifier helpers in `underlay-db`
- inspect `underlay-testing::TestDb` schema construction
- inspect `underlay-media-postgres::PostgresMediaConfig`
- decide which surfaces need additive typed APIs, deprecation, or no change
- open execution batches for any code changes

Out of scope:

- audit/security-alert wrapper removal, completed in `g06.030`
- broad media adapter refactors
- TypeScript/Svelte work
- release execution or publishing

## Contract References

- `021`: database migration and schema workflow
- `040`: storage, blob, and media systems
- `060`: jobs, events, and operator systems
- `122`: Rust public API inventory

## Acceptance Criteria

- remaining dynamic identifier helpers are classified
- `ExistsCheck` has a concrete typed migration plan
- validation impact is identified before code changes
- docs state whether any raw helper is retained intentionally

## Consumer Upgrade Impact

Impact: likely additive first. Removal or deprecation must be proven against the
six consumers.

## Current State

`g06.031` is ready after `g06.030`.

## Next Task

Execute `g06.031`: remaining typed DB helper migration plan.
