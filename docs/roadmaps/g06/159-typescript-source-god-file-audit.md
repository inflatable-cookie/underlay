# g06.159 - TypeScript Source God-File Audit

## Why

`g06.158` cleared the comment-ratio warning family. `effigy doctor` now passes
with only one warning family left: `scan.god-files`.

The remaining findings mix source files and test files. The source files are
the meaningful reference-grade risk because they may hide multiple
responsibilities behind broad public surfaces.

## Goal

Audit the remaining TypeScript source god-files and decide which ones need
bounded splits before the TypeScript structural lane can close.

## Scope

In scope:

- inspect current `scan.god-files` detail report
- classify source findings by responsibility boundaries
- decide which source files need split cards
- defer or justify test-only large-file warnings
- record final doctor state

Out of scope:

- splitting files in this decision batch
- changing public APIs
- changing consumer apps
- Rust cleanup

## Acceptance Criteria

- [x] every source god-file warning is classified
- [x] every test god-file warning is classified or deferred with rationale
- [x] required splits are represented as bounded follow-up cards
- [x] `effigy doctor` state is recorded

## Consumer Upgrade Impact

Expected impact: none.

This is an audit batch. Any later source split must classify impact separately.

## Evidence

- `effigy doctor` passes with `ok:15`, `warn:1`, `err:0`.
- Remaining warning family: `scan.god-files`.
- Source findings:
  - `ts/src/patterns/RelationSelector/context.svelte.ts`
  - `ts/src/client/http.ts`
  - `ts/src/patterns/pagination.svelte.ts`
  - `ts/src/patterns/RelationSelector/drilldown-context.svelte.ts`
  - `ts/src/patterns/storage.ts`
- Test findings:
  - `ts/tests/nightfire/utils.test.ts`
  - `ts/tests/client/sveltekit.test.ts`
  - `ts/tests/patterns/forms.test.ts`
  - `ts/tests/patterns/i18n.test.ts`
  - `ts/tests/nightfire/summary-transform.test.ts`
  - `ts/tests/server/csp.test.ts`
  - `ts/tests/patterns/slugify.test.ts`
  - `ts/tests/client/http/auth.test.ts`
  - `ts/tests/client/useAuth.test.ts`

## Classification

Source findings:

- RelationSelector context pair:
  - `context.svelte.ts` mixes public context type, selected-item resolution,
    popover/modal actions, search/suggestion loading, drill-down integration,
    and Svelte context wiring.
  - `drilldown-context.svelte.ts` mixes drill-down state construction,
    context/filter derivation, async search/suggestion loading, navigation, and
    action export wiring.
  - Decision: split first as one coordinated RelationSelector source batch.
- HTTP client:
  - `http.ts` mixes public HTTP/auth types, memory token store, header helpers,
    retry/timeout raw transport, auth-refresh coordination, and method facade.
  - Decision: split after RelationSelector; keep public `client/http` exports
    stable.
- Storage:
  - `storage.ts` mixes public storage types, availability probing, envelope
    parsing/serialization, wrapper methods, reactive store behavior, and public
    factory exports.
  - Decision: split after HTTP; preserve `storage`, `createPersistedStore`, and
    `createSessionStore`.
- Pagination:
  - `pagination.svelte.ts` mixes server cursor pagination and client local
    pagination controllers.
  - Decision: split after storage unless a nearby consumer concern makes it
    urgent.

Test findings:

- Defer test-only large-file splits for now. They are less risky than source
  responsibility clusters and should be split only when paired with a related
  source split or when a test file blocks readability.

## Follow-Up Cards

- `g06.160`: RelationSelector source god-file split
- later: HTTP client source split
- later: storage source split
- later: pagination source split

## Current State

`g06.159` is complete.

## Next Task

Execute `g06.160`: RelationSelector source god-file split.
