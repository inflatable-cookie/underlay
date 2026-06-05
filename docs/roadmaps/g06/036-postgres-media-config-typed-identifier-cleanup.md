# g06.036 - Postgres Media Config Typed Identifier Cleanup

## Why

`g06.035` found one remaining meaningful runtime dynamic identifier cleanup:
`underlay-media-postgres::PostgresMediaConfig`.

The config currently validates and quotes table names before query execution,
but stores public raw string fields and has a raw `with_schema` constructor. This
keeps the adapter safe at query time, but leaves the public shape weaker than the
reference-grade typed boundary.

## Goal

Move `PostgresMediaConfig` toward typed schema/table storage while preserving
practical construction ergonomics for apps.

## Scope

In scope:

- inspect current consumer usage of `PostgresMediaConfig`
- decide whether public raw fields become private or remain compatibility
  accessors
- store schema/table names internally as `SqlIdentifier` or
  `QualifiedTableName`
- ensure all media SQL operation modules consume already-typed table names
- update contracts and guides
- run targeted Underlay and consumer checks

Out of scope:

- media repository trait redesign
- media schema migration changes
- blob `MediaConfig` refactor
- TypeScript/Svelte work
- release execution or publishing

## Acceptance Criteria

- media Postgres table config validates identifiers at construction
- query operations do not re-parse public raw strings at every call
- public compatibility impact is classified
- current six-consumer usage is checked
- targeted Rust checks pass or failures are classified

## Consumer Upgrade Impact

Expected impact: likely none or narrow breaking cleanup.

Prior scans found no current custom `PostgresMediaConfig` table usage, but this
batch must prove that again before changing public field shape.

## Current State

`g06.036` is next after `g06.035`.

## Next Task

Execute `g06.036`: Postgres media config typed identifier cleanup.
