# g06.039 - Typed Media Storage Key Helpers

## Why

`g06.038` found that `underlay-blob` already has `BlobObjectKey`, while
`underlay-media::storage` still only returns strings.

The current string helpers are stable and widely used, but upload paths can move
to typed object-key generation without changing the blob adapter trait yet.

## Goal

Add typed media storage-key helpers and migrate the current upload/rendition
generation call sites that can use them without broader adapter churn.

## Scope

In scope:

- add an explicit `underlay-media` feature for object-key helpers
- add `BlobObjectKey`-returning storage helper methods/functions
- keep existing string-returning helpers
- add focused storage helper tests
- migrate consumer upload initiation paths to `UploadRequest::from_object_key`
  where practical
- classify any consumer call sites intentionally retained on strings

Out of scope:

- changing `BlobAdapter` trait method signatures
- changing database object-key column types
- changing media repository domain row field types
- S3/local adapter behavior changes
- TypeScript/Svelte work
- release execution or publishing

## Acceptance Criteria

- typed helper methods/functions exist and are feature-gated clearly
- string helpers still compile
- generated typed keys match existing string values
- consumer migration impact is proved
- targeted Underlay and consumer Rust checks pass or failures are classified

## Consumer Upgrade Impact

Expected impact: additive first.

Consumer app changes are allowed but should be local to media upload/rendition
generation paths. Existing persisted object-key values must not change.

## Current State

`g06.039` is next after `g06.038`.

## Next Task

Execute `g06.039`: typed media storage key helpers.
