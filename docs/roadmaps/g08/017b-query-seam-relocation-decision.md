# g08.017b - Query-Seam Relocation Decision

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Decide where dynamic SQL WHERE building lives. `WhereBuilder`/`FieldMapping`
currently sit in `underlay-http/src/query/`, but they generate SQL and are
imported by consumers' **db-layer** crates (e.g. `farmyard/crates/db`,
`cp-api/crates/db`) directly from `underlay-http` - a db-layer depending on the
http crate for SQL construction. Split out from `g08.017`, which resolved the
`PaginationParams` name collision and hardened `add_raw` but left this move as a
boundary decision per that card's stop condition.

## The Decision

`WhereBuilder::add_filter` consumes `underlay-http`'s `FilterField` /
`FilterOperator` - the wire-parsed filter model. Contract `020`/the g08.017 card
want wire-format parsing (`filter[...]`, `sort=`) to stay in `underlay-http`.
Moving SQL generation down to the db layer therefore forces one of:

- **A. Move `WhereBuilder`/`FieldMapping` (and the `FilterField` type, keeping
  only the parsing in http) into `underlay-db`.** Lowest new surface; but
  `underlay-db` gains the filter/operator vocabulary, and "parsing in http /
  type in db" splits one concept across two crates.
- **B. Introduce a small `underlay-query` crate** holding the shared query
  model (`FilterField`, `FilterOperator`, `WhereBuilder`, `FieldMapping`,
  typed values), depended on by both `underlay-http` (parsing) and
  `underlay-db`/consumers (SQL). Cleanest layering; adds a crate.
- **C. Leave location, add a trait seam** so db-layer callers depend on an
  abstraction rather than `underlay-http` directly. Least churn; least
  structural payoff.

Also in scope once located: make `WhereBuilder` values a typed enum
(`SqlValue`) instead of `Vec<String>`, so non-text binds are expressed
directly rather than stringified.

## Governing References

- [010 Foundation primitives and envelopes](../../contracts/010-foundation-primitives-and-envelopes.md)
- [020 HTTP transport and server boundary](../../contracts/020-http-transport-and-server-boundary.md)
- [116 Canonical collection routes and query profiles](../../contracts/116-canonical-collection-routes-and-query-profiles.md)

## Consumer Upgrade Impact

Impact class: `behavioral`. Consumers' db crates re-point `WhereBuilder`/
`FieldMapping` imports to the chosen home. Requires six-consumer proof per
`023`. Because this touches every consumer's db layer, the direction must be
decided before execution rather than discovered mid-migration.

## Recommendation

Option **B (`underlay-query` crate)** is the cleanest: it removes the
db-imports-http inversion, gives the typed-value model a natural home, and
keeps http as the parsing edge over a shared model. Confirm before building the
crate - this is the boundary the `g08.017` stop condition flagged.

## Resolution & Completion Notes

Completed 2026-07-17. Chose **Option B (`underlay-query` crate)**.

- New `underlay-query` crate owns the shared query model and SQL generation:
  `FilterField`/`FilterOperator` (filter.rs), `SortField`/`SortDirection`/
  `parse_sort_string` (sort.rs), `FieldMapping` (field_mapping.rs),
  `WhereBuilder` + the new typed `SqlValue` enum (where_builder.rs), plus the
  `field_mapping!` macro.
- `underlay-http` keeps `QueryParams` as the HTTP-side wire parser (parses
  `sort=`/`filter[...]` into the shared model) and depends on
  `underlay-query`. It re-exports the whole model at `underlay_http::query::*`
  and the macro at the crate root, so every existing
  `underlay_http::query::{...}` import keeps compiling - the relocation is
  backward-compatible.
- `SqlValue` (Text/Int/Float/Bool/Null, with `From` impls) is added as an
  additive typed-bind model; the string-based `WhereBuilder` API is unchanged
  so no consumer `build()` call breaks.
- Contract `020` records `underlay-query` as the query-model/SQL home and
  `underlay-http` as the parsing edge.

## Consumer Rollout

- **Reference proof:** `underlay-reference` (`acme-db`) migrated its six
  db-layer files off `underlay_http::query::{FieldMapping, WhereBuilder}` to
  `underlay_query::{...}` (keeping `QueryParams` from `underlay-http`), with
  `underlay-query` added to its workspace deps. `cargo check` clean - this is
  the canonical new-path proof.
- **Compatibility window:** the other five consumers keep compiling unchanged
  via the `underlay_http::query::*` re-export (acowtancy `farmyard-db`, the
  heaviest `WhereBuilder` user, verified clean). They should migrate their
  db-layer imports to `underlay-query` on their own cadence; the http
  re-export is the compat surface, retirement targeted `g09` alongside the
  pagination aliases.

Validated: `cargo build/test --workspace --all-features` green (75 suites, 0
failures - up from 73 with the new crate); `underlay-reference` and acowtancy
`cargo check` clean.

## Next Task

`g08.018` auth-postgres adapter decision.
