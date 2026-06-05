# g06.060 - HTTP Query Internal Split

## Why

`g06.059` found that `underlay-http/src/query.rs` is stable app-facing helper
surface with a safe internal split shape.

Consumers use both `underlay_http::query::*` and crate-root exports, so the
split must preserve both front doors.

## Goal

Split `underlay-http/src/query.rs` into focused private modules while
preserving query parsing semantics, SQL helper behavior, macro behavior, and
public exports.

## Scope

In scope:

- split sort direction, sort field, and sort parser helpers
- split filter operator and filter field helpers
- split `QueryParams` extraction and filter parsing
- split `WhereBuilder`
- split `FieldMapping`
- preserve `field_mapping!` behavior and exported path
- preserve `underlay_http::query::*` compatibility
- preserve crate-root query exports
- update tests only where module parent imports need to become explicit

Out of scope:

- changing sort or filter query string semantics
- changing SQL operator strings or fragment generation
- changing pagination, cookies, or error logging behavior
- consumer rollout unless public imports move

## Acceptance Criteria

- `query.rs` becomes a small module front door
- public exports remain source-compatible
- `underlay-http` tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal module split. If public imports, macro paths, query parsing
semantics, or SQL helper behavior must move, stop and re-enter planning.

## Current State

`g06.060` is next after `g06.059`.

## Next Task

Execute `g06.060`: HTTP query internal split.
