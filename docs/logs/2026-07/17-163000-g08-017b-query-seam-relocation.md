# 2026-07-17 - g08.017b query-seam relocation (new underlay-query crate)

## Context

`g08.017` resolved the `PaginationParams` collision and hardened `add_raw`, but
left the query-seam relocation as a surfaced boundary decision: `WhereBuilder`/
`FieldMapping` lived in `underlay-http` yet were imported by consumers'
db-layer crates for SQL construction (a db-imports-http inversion). Options
were db-crate move, a new crate, or a trait seam.

## Decision

Chose **Option B: a new `underlay-query` crate.** It removes the inversion,
gives the typed-value model a home, and keeps HTTP as the parsing edge over a
shared model.

## Changes

- New `underlay-query` crate owns the shared query model + SQL generation:
  `FilterField`/`FilterOperator`, `SortField`/`SortDirection`/
  `parse_sort_string`, `FieldMapping`, `WhereBuilder` (+ `add_raw_indexed`),
  the new `SqlValue` typed-bind enum, and the `field_mapping!` macro. Pure
  serde/std, no http/db deps.
- `underlay-http` keeps `QueryParams` as the wire parser (`sort=`/`filter[...]`
  -> shared model), depends on `underlay-query`, and re-exports the whole
  model at `underlay_http::query::*` plus the macro at crate root. Every
  existing `underlay_http::query::{...}` import keeps compiling - the move is
  backward-compatible.
- `SqlValue` is additive; the string-based `WhereBuilder` API is unchanged so
  no `build()` caller breaks.
- Contract `020` records `underlay-query` as the query-model/SQL home and
  `underlay-http` as the parsing edge.

## Consumer Rollout

- **Reference proof:** `underlay-reference`/`acme-db` migrated its six db files
  to `underlay_query::{FieldMapping, WhereBuilder}` (keeping `QueryParams` from
  `underlay-http`), with `underlay-query` added to workspace deps. `cargo
  check` clean - the canonical new-path proof.
- **Compatibility window:** the other five consumers compile unchanged via the
  `underlay_http::query::*` re-export; acowtancy `farmyard-db` (heaviest
  `WhereBuilder` user) verified clean. They migrate off the re-export on their
  own cadence; retirement targeted `g09` with the pagination aliases.

## Validation

- `cargo build/test --workspace --all-features`: green, **75 suites** (up from
  73 with the new crate), 0 failures.
- `underlay-reference` (`acme-db`/`acme-api`) and acowtancy (`farmyard-db`)
  `cargo check` clean.

## Consumer Upgrade Notes

Impact class **additive/behavioral (opt-in)**. No consumer must change - the
http re-export preserves all `underlay_http::query::{...}` imports. New/updated
db-layer code should import the query model from `underlay-query` to avoid
depending on `underlay-http` for SQL building.

## Next

`g08.018` auth-postgres adapter decision.
