# g06.145 - HTTP Client Tests Internal Split

## Why

`g06.144` found that `ts/tests/client/http-refactored.test.ts` mixes all HTTP
client behavior families in one large test file.

The file is test-only. The next step is a mechanical split that preserves all
current behavior coverage.

## Goal

Split HTTP client tests into focused test files without changing public HTTP
client APIs or behavior.

## Scope

In scope:

- create `ts/tests/client/http/` test files by behavior family
- move basic request tests into `requests.test.ts`
- move auth and refresh tests into `auth.test.ts`
- move retry and timeout tests into `retry-timeout.test.ts`
- move error, debug, and metadata tests into `errors-metadata.test.ts`
- move `MemoryTokenStore` tests into `token-store.test.ts`
- retire `ts/tests/client/http-refactored.test.ts`
- preserve all 38 passing tests and the 1 skipped timeout test

Out of scope:

- changing `client/http`
- changing `client/errors`
- changing consumer apps

## Acceptance Criteria

- split HTTP client test files pass
- behavior coverage remains equivalent
- no public HTTP client behavior changes
- `effigy qa:docs` passes
- roadmap artifact records final test shape and doctor impact

## Consumer Upgrade Impact

Expected impact: none.

This should be a test-only split. If HTTP client behavior needs to change, stop
and re-enter planning.

## Current State

`g06.145` is ready.

## Next Task

Execute `g06.145`: HTTP client tests internal split.
