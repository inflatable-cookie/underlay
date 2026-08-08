# g06.143 Artifact - TypeScript Structural Backlog Triage

## Summary

After `g06.142`, Effigy doctor structural findings are TypeScript-only.

Current doctor backlog:

- `scan.god-files`: 18 findings, 5 high-severity errors
- `scan.attention-markers`: 5 findings, 1 high-severity error
- `scan.comment-ratio`: 7 findings, 1 high-severity error

No Rust files remain in these reports.

## God-File Findings

High-severity files:

- `ts/tests/client/http-refactored.test.ts`: 616 code lines
- `ts/src/patterns/media-workflow.ts`: 524 code lines
- `ts/src/patterns/forms.ts`: 522 code lines
- `ts/src/templates/template.types.ts`: 519 code lines
- `ts/tests/patterns/optimistic.test.ts`: 462 code lines

Warning-level files:

- nightfire, client, forms, i18n, CSP, slugify, storage, RelationSelector, and
  pagination test/source files

## Marker And Comment Findings

Attention-marker findings are all TypeScript:

- `ts/src/client/navigation.ts`
- `ts/src/patterns/blob-upload.ts`
- `ts/src/patterns/media-types/requests.ts`
- `ts/src/patterns/storage.ts`
- `ts/tests/patterns/slugify.test.ts`

Comment-ratio findings are all TypeScript:

- `ts/src/client/route-protection.ts`
- `ts/src/utils/sequence.ts`
- `ts/src/patterns/navigation.ts`
- `ts/src/patterns/RelationSelector/drilldown-types.ts`
- `ts/src/server/csp.ts`
- `ts/src/patterns/local-search.ts`
- `ts/src/client/navigation.ts`

## Classification

The highest-severity source files are public shared surfaces:

- `media-workflow.ts` is re-exported through `runtime/media`
- `forms.ts` is re-exported through `runtime/forms`
- `template.types.ts` is consumed broadly by shared Svelte templates

Those need behavior-boundary audits before any split.

The largest high-severity file is test-only:

- `ts/tests/client/http-refactored.test.ts`

It exercises the public `client/http` surface and already has clear internal
test families. Splitting it first reduces backlog with no consumer API impact
and keeps the public HTTP client protected before source-level work.

## Decision

Queue `g06.144` as an HTTP client tests modularity audit.

Suggested next audit focus:

- basic request behavior
- auth token and refresh behavior
- retry behavior
- timeout behavior
- error handling behavior
- token store behavior
- test helper boundaries already provided by `ts/tests/utils/http-mocks`

## Baseline Validation

Passed:

- `bun x vitest run ts/tests/client/http-refactored.test.ts`
  - 1 test file passed
  - 38 tests passed
  - 1 test skipped

## Consumer Impact

Expected impact: none.

The next target is test-only. If follow-up work requires changing
`@inflatable-cookie/underlay/client/http`, stop and classify consumer app updates
before implementation.
