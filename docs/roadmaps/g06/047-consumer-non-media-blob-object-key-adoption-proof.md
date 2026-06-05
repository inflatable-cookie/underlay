# g06.047 - Consumer Non-Media Blob Object-Key Adoption Proof

## Why

`g06.046` set the non-media blob object-key policy. The audit identified one
live consumer runtime path outside the media library: Farmyard syllabus-notes
PDF output keys.

That path currently accepts optional output object-key strings, generates
default PDF object-key strings, and calls blob storage directly with raw
strings.

## Goal

Move the identified live non-media consumer blob runtime path onto
`BlobObjectKey` without changing persisted values or dragging migration/replay
tooling into the same policy.

## Scope

In scope:

- update Farmyard PDF output-key generation/request handling to parse
  `BlobObjectKey`
- use typed blob request constructors or adapter extension methods where
  practical
- keep API JSON fields as strings
- leave migration/replay tooling raw unless a live runtime path depends on it
- validate Farmyard and any affected parent pointer repo

Out of scope:

- changing blob adapter trait signatures
- changing persisted object-key values
- changing database column types
- media-library paths already closed by `g06.045`
- TypeScript/Svelte work
- release execution or publishing

## Acceptance Criteria

- Farmyard live PDF upload/download blob calls use parsed `BlobObjectKey`
- invalid user-supplied output keys fail before blob storage access
- default PDF keys are generated as validated object keys
- migration/replay tooling remains classified raw tooling
- Farmyard Rust workspace passes

## Consumer Upgrade Impact

Expected impact: breaking source change inside Farmyard only.

The app is not in production, so this can roll directly.

## Current State

`g06.047` is next after `g06.046`.

## Next Task

Execute `g06.047`: consumer non-media blob object-key adoption proof.
