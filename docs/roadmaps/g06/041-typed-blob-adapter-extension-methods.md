# g06.041 - Typed Blob Adapter Extension Methods

## Why

`g06.040` decided not to break the `BlobAdapter` trait. Generated object-key
paths now have typed keys, but callers still convert them back to `&str` before
adapter calls.

An additive extension trait can make the typed path ergonomic without breaking
raw database-loaded key usage.

## Goal

Add typed blob adapter extension methods for `BlobObjectKey` and migrate the
generated-key call sites that can use them cleanly.

## Scope

In scope:

- add a public extension trait in `underlay-blob`
- delegate typed methods to the existing raw `BlobAdapter` methods
- add focused extension-method tests
- migrate generated upload/rendition call sites where practical
- keep database-loaded object-key strings on raw adapter methods

Out of scope:

- changing `BlobAdapter` trait signatures
- changing database column types
- requiring all stored object keys to parse during DTO/listing paths
- S3/local behavior changes
- TypeScript/Svelte work
- release execution or publishing

## Acceptance Criteria

- typed extension methods exist and are documented
- raw adapter methods remain stable
- generated-key call sites no longer need `as_str()` where the extension method
  fits
- consumer impact is additive and validated

## Consumer Upgrade Impact

Expected impact: additive.

Consumer code can opt into typed extension methods. Existing raw adapter calls
remain valid.

## Current State

`g06.041` is next after `g06.040`.

## Next Task

Execute `g06.041`: typed blob adapter extension methods.
