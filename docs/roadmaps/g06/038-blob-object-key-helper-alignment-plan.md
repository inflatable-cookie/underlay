# g06.038 - Blob Object Key Helper Alignment Plan

## Why

The DB identifier lane is complete. The next known shared Rust weak spot is the
storage-key/object-key boundary.

`underlay-blob` already exposes `BlobObjectKey`, and upload/download request
constructors can use typed object keys. `underlay-media` still has
string-returning storage key helpers that are stable for consumers but remain a
candidate-type surface.

## Goal

Plan how to align media storage-key generation with `BlobObjectKey` without
causing avoidable churn across the current consumers.

## Scope

In scope:

- inspect `underlay-media` storage key helpers
- inspect `underlay-blob` typed object-key request constructors
- scan six consumers for storage key helper usage
- decide whether the next batch should add typed helpers, migrate consumers, or
  retain the current string helpers with stronger docs
- update contracts with the chosen posture

Out of scope:

- blob adapter trait redesign
- provider-specific S3/local behavior changes
- media repository trait redesign
- TypeScript/Svelte work
- release execution or publishing

## Acceptance Criteria

- current storage-key helper usage is known
- object-key compatibility impact is classified
- next execution batch is concrete if a typed migration is worthwhile
- contracts reflect the current chosen posture

## Consumer Upgrade Impact

Expected impact: none for this planning batch.

The likely follow-up may be additive first: typed key generation helpers while
retaining current string helpers until consumer proof is complete.

## Current State

`g06.038` is complete.

Artifact:

- [038 artifact](./038-blob-object-key-helper-alignment-plan-artifact.md)

## Next Task

Execute `g06.039`: typed media storage key helpers.
