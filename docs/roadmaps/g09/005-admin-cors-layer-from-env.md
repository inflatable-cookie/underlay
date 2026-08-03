# g09.005 - admin_cors_layer_from_env + Collapse CORS_ORIGINS Clones

Status: complete
Completed: 2026-08-03
Owner: repo maintainers

## Purpose

acme, cp, and compli each carry an identical ~7-line `CORS_ORIGINS` env
parse at the `admin_cors_layer` call site (comma-split, trim, drop empties)
— the last piece of CORS duplication. One helper kills it.

## Evidence

- `acme-api/crates/api/src/routes/mod.rs`
- `cp-api/crates/api/src/routes/mod.rs`
- `compli-me/api/crates/api/src/routes/mod.rs`
- Audit item 5

## Planned Changes

- [x] underlay-http: `admin_cors_layer_from_env(environment)` — reads
  `CORS_ORIGINS` (same parse semantics) and delegates to
  `admin_cors_layer`. Unit tests for the parse (empty, single, list,
  whitespace).
- [x] acme/cp/compli: call sites collapse to the helper; delete the local
  parse.
- [x] composer/farmyard/nursery keep their config-file origin sources
  (unchanged).

## Consumer Upgrade Impact

Impact class: `additive`. Consumers adopt the helper; behavior identical.

## Validation

- [x] `cargo test -p underlay-http`; `effigy validate`
- [x] `cargo check --workspace --all-features --all-targets` in the three
  consumers; `effigy qa:security` green (cors-canonical still passes)

## Stop Conditions

None expected.

## Completion Notes

Completed 2026-08-03. underlay-http gained `cors_origins_from_env` + `admin_cors_layer_from_env`; acme/cp/compli call sites collapsed and their local parses deleted (compli's `parse_cors_origins` gone). 150 underlay-http tests green; cargo check green in all three consumers.

## Next Task

`g09.006` nursery env precedence flip.
