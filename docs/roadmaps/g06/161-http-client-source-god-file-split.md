# g06.161 - HTTP Client Source God-File Split

## Why

`g06.160` cleared the RelationSelector source god-file warnings. The next
source warning is `ts/src/client/http.ts`, which mixes public types, token
storage, header helpers, raw transport, retry/timeout logic, auth refresh, and
method facade construction.

## Goal

Split the HTTP client internals into smaller modules while preserving the public
`@decodelabs/underlay/client/http` API.

## Scope

In scope:

- split public HTTP/auth types from implementation helpers
- split `MemoryTokenStore` into a focused token-store module
- split header/retry/raw-transport helpers if that reduces the client factory
- preserve existing exports and import paths
- run HTTP client focused tests
- run `effigy doctor`

Out of scope:

- changing retry, timeout, refresh, or error semantics
- changing public APIs
- consumer-app changes
- splitting storage or pagination

## Acceptance Criteria

- `ts/src/client/http.ts` no longer reports as a source god-file, or retained
  size is justified by artifact evidence
- public imports remain stable
- focused HTTP client tests pass
- `effigy doctor` state is recorded

## Consumer Upgrade Impact

Expected impact: none.

This should be an internal split with stable exports.

## Current State

`g06.161` is ready.

## Next Task

Execute `g06.161`: HTTP client source god-file split.
