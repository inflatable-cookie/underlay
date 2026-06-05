# g06.058 - DB Pagination Internal Split

## Why

`g06.057` found that `underlay-db/src/pagination.rs` is stable app-facing
module surface with a safe internal split shape.

The pagination helpers are widely imported by consumers, so the split must
preserve the existing `underlay_db::pagination::*` front door.

## Goal

Split `underlay-db/src/pagination.rs` into focused private modules while
preserving public item names, cursor behavior, SQL helper behavior, and
serialized shapes.

## Scope

In scope:

- split pagination constants, params, and direction
- split response wrapper type
- split cursor error and generic cursor encoding/decoding helpers
- split `PaginationBuilder` SQL/keyset helpers
- split typed cursor helpers
- preserve `underlay_db::pagination::*` compatibility
- update tests only where module parent imports need to become explicit

Out of scope:

- changing cursor encoding or decoding semantics
- changing response or params serialized shapes
- changing SQL fragment generation
- changing HTTP offset pagination helpers
- consumer rollout unless public imports move

## Acceptance Criteria

- `pagination.rs` becomes a small module front door
- public exports remain source-compatible
- `underlay-db` tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal module split. If public imports, cursor encoding, SQL
helper behavior, or serialized shapes must move, stop and re-enter planning.

## Current State

`g06.058` is next after `g06.057`.

## Next Task

Execute `g06.058`: DB pagination internal split.
