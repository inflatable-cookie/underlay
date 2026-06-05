# g06.045 - Media Object-Key Boundary Closeout Audit

## Why

`g06.039` through `g06.044` moved generated, shared-domain, adapter, and
consumer-local media object-key paths onto `BlobObjectKey`.

Before leaving the object-key lane, Underlay needs a final audit that separates
acceptable string edges from remaining unsafe raw object-key flows.

## Goal

Close out the media object-key boundary lane with a focused audit and any small
repair batch it reveals.

## Scope

In scope:

- audit Underlay and the six consumers for remaining `object_key` string flows
- classify each remaining raw string as SQL edge, JSON/API edge, non-media
  storage path, or repair candidate
- patch small repair candidates where the boundary is obvious
- record any larger follow-up as a new card instead of expanding this one
- validate affected Rust workspaces

Out of scope:

- changing database column types
- changing blob adapter trait signatures
- replacing all non-media storage keys with `BlobObjectKey`
- TypeScript/Svelte work
- release execution or publishing

## Acceptance Criteria

- remaining raw object-key flows are classified
- no known media DTO/public URL/delete/download path accepts unparsed stored
  object keys
- any obvious missed media repair is patched and validated
- residual work is either closed or explicitly queued

## Consumer Upgrade Impact

Expected impact: none to small breaking source changes inside non-production
consumer repos if a missed app-local media path is found.

## Current State

`g06.045` is next after `g06.044`.

## Next Task

Execute `g06.045`: media object-key boundary closeout audit.
