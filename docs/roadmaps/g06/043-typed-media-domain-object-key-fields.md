# g06.043 - Typed Media Domain Object-Key Fields

## Why

`g06.042` decided that stored media object keys should parse at the shared
repository/domain boundary. The current media domain still exposes stored object
keys as raw strings.

This keeps invalid stored keys alive until DTO, URL, delete, or download paths.

## Goal

Move Underlay media domain/input object-key fields to `BlobObjectKey` and roll
the breaking source change through the six consumers.

## Scope

In scope:

- update `underlay-media` domain/input object-key fields to `BlobObjectKey`
- add or update constructors/helpers where optional stored keys need conversion
- parse object-key strings in `underlay-media-postgres` row mapping
- update SQL binds to use `as_str()`
- update Underlay media/rendition helpers to use typed fields where practical
- update six consumer DTO, public URL, delete/purge, download, and job paths
- classify and document the breaking consumer impact

Out of scope:

- changing database column types
- changing persisted object-key values
- changing `BlobAdapter` trait signatures
- TypeScript/Svelte work
- release execution or publishing

## Acceptance Criteria

- Underlay media domain object-key fields are typed
- invalid stored object keys fail at repository row mapping
- generated and stored object-key paths both use `BlobObjectKey` in Rust domain
  code
- six consumer Rust checks pass
- contracts and roadmap artifact record the breaking rollout

## Consumer Upgrade Impact

Expected impact: breaking source change.

All six known consumers are non-production and should be updated in the same
batch.

## Current State

`g06.043` is complete.

Artifact:

- [043 artifact](./043-typed-media-domain-object-key-fields-artifact.md)

## Next Task

Execute `g06.044`: consumer app-local media object-key parse boundaries.
