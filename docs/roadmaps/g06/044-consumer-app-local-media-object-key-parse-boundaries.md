# g06.044 - Consumer App-Local Media Object-Key Parse Boundaries

## Why

`g06.043` typed the shared Underlay media domain and default Postgres adapter.
The six consumers still mostly use app-local media row and DTO models, where
stored object keys remain raw strings.

Reference-grade storage safety needs the same parse boundary in those app-local
models, or a clear decision to migrate onto the shared Underlay media shapes.

## Goal

Normalize consumer app-local stored media object-key boundaries.

## Scope

In scope:

- audit app-local media row/model object-key fields across the six consumers
- decide per consumer whether to type local row fields or move closer to
  Underlay media domain shapes
- parse database-loaded object keys before DTO/public URL/delete/download use
- keep JSON DTOs and SQL binds as string edges
- validate all six consumer Rust workspaces

Out of scope:

- changing database column types
- changing persisted object-key values
- changing blob adapter trait signatures
- TypeScript/Svelte work
- release execution or publishing

## Acceptance Criteria

- consumer-local stored object-key models are no longer silently raw where the
  app owns the media row boundary
- invalid stored object keys fail before DTO/public URL/delete/download paths
- generated-key and stored-key paths both use `BlobObjectKey` where practical
- all six consumer checks pass

## Consumer Upgrade Impact

Expected impact: breaking source change inside consumer repos.

The apps are not in production, so roll the six-app family in one batch.

## Current State

`g06.044` is complete.

Artifact:

- [044 artifact](./044-consumer-app-local-media-object-key-parse-boundaries-artifact.md)

## Next Task

Execute `g06.045`: media object-key boundary closeout audit.
