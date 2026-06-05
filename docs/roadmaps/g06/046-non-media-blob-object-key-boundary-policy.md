# g06.046 - Non-Media Blob Object-Key Boundary Policy

## Why

`g06.045` closed the media object-key boundary. The audit also surfaced
non-media blob flows, especially PDF output keys and migration/replay storage
paths in consumers.

Those flows should not be silently pulled into the media-library contract, but
Underlay still needs a clear rule for when generic blob callers should adopt
`BlobObjectKey`.

## Goal

Define and, where small enough, apply the boundary policy for non-media blob
object keys.

## Scope

In scope:

- classify generic blob object-key flows outside the media library
- decide which paths should use `BlobObjectKey` now
- decide which paths stay raw because they are adapter compatibility, tests,
  migration tooling, or externally supplied request strings
- patch small Underlay repair candidates if the boundary is obvious
- queue consumer-specific follow-up if a non-media app path needs a larger
  migration

Out of scope:

- changing blob adapter trait signatures
- changing database column types
- changing persisted object-key values
- forcing migration/replay tooling onto the same model as live runtime paths
- TypeScript/Svelte work
- release execution or publishing

## Acceptance Criteria

- `BlobObjectKey` policy is explicit for non-media blob paths
- raw generic blob edges are classified
- obvious Underlay repair candidates are patched or consciously deferred
- consumer follow-up is queued only for live runtime paths, not historical
  migration evidence

## Consumer Upgrade Impact

Expected impact: none to small breaking Rust source changes if a live
consumer-owned non-media blob path is migrated.

## Current State

`g06.046` is next after `g06.045`.

## Next Task

Execute `g06.046`: non-media blob object-key boundary policy.
