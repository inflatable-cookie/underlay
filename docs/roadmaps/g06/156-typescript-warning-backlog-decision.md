# g06.156 - TypeScript Warning Backlog Decision

## Why

`g06.155` cleared TypeScript doctor errors. `effigy doctor` now passes, but it
still reports warning-only TypeScript structural backlog:

- attention markers
- comment-heavy files
- large files

The lane needs a deliberate stop-or-continue decision before spending more
cycles on warning-level cleanup.

## Goal

Classify the remaining TypeScript warning backlog and decide which findings
must be cleaned now for reference-grade quality, which can remain intentional,
and which should become bounded follow-up cards.

## Scope

In scope:

- inspect the remaining doctor warning reports
- classify each warning family by risk and extensibility impact
- decide whether any warning should be promoted into executable cleanup
- record the final TypeScript structural state

Out of scope:

- changing runtime behavior
- broad TypeScript package redesign
- Rust structural cleanup
- consumer-app changes

## Acceptance Criteria

- [x] remaining TypeScript warning families are classified
- [x] any required cleanup is represented as bounded follow-up cards
- [x] intentional warning backlog is documented with rationale
- [x] `effigy doctor` state is recorded

## Consumer Upgrade Impact

Expected impact: none.

This is a classification batch. Any later consumer-visible change needs its own
impact section.

## Decision

Continue, but only on warning families that improve the reference-grade shape:

- remove scanner-noise `Note:` markers now
- trim code-adjacent examples and redundant comments in public TypeScript
  helper files
- split source god-files where the file mixes stable responsibilities
- defer large test-file splits unless a nearby source split needs them

## Evidence

- `effigy doctor` passes with warning-only scan findings:
  - `scan.attention-markers`: 4 warnings, 0 errors
  - `scan.comment-ratio`: 6 warnings, 0 errors
  - `scan.god-files`: 14 warnings, 0 errors
- `.effigy/reports/doctor/scan-attention-markers.md`
- `.effigy/reports/doctor/scan-comment-ratio.md`
- `.effigy/reports/doctor/scan-god-files.md`

## Classification

Attention markers:

- `ts/src/patterns/blob-upload.ts`
- `ts/src/patterns/media-types/requests.ts`
- `ts/src/patterns/storage.ts`
- `ts/tests/patterns/slugify.test.ts`

These are not deferred-work risks. They are explanatory `Note:` text that
trips the scanner. Clean them as wording-only changes.

Comment-ratio warnings:

- `ts/src/utils/sequence.ts`
- `ts/src/patterns/navigation.ts`
- `ts/src/patterns/RelationSelector/drilldown-types.ts`
- `ts/src/server/csp.ts`
- `ts/src/patterns/local-search.ts`
- `ts/src/client/navigation.ts`

These are not security issues. The quality issue is that source files carry too
many examples and field-level explanations, making the implementation harder to
scan. Trim source comments and leave usage guidance to docs/tests.

God-file warnings:

- Source files worth further action:
  - `ts/src/patterns/RelationSelector/context.svelte.ts`
  - `ts/src/client/http.ts`
  - `ts/src/patterns/pagination.svelte.ts`
  - `ts/src/patterns/RelationSelector/drilldown-context.svelte.ts`
  - `ts/src/patterns/storage.ts`
- Test files to defer unless paired with a source split:
  - `ts/tests/nightfire/utils.test.ts`
  - `ts/tests/client/sveltekit.test.ts`
  - `ts/tests/patterns/forms.test.ts`
  - `ts/tests/patterns/i18n.test.ts`
  - `ts/tests/nightfire/summary-transform.test.ts`
  - `ts/tests/server/csp.test.ts`
  - `ts/tests/patterns/slugify.test.ts`
  - `ts/tests/client/http/auth.test.ts`
  - `ts/tests/client/useAuth.test.ts`

The source god-files represent the real extensibility backlog. They should be
audited and split by responsibility when the split can preserve public exports.

## Current State

`g06.156` is complete.

## Next Task

Execute `g06.157`: TypeScript attention marker cleanup.
