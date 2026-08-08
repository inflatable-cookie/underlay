# g07.007 Artifact - Relation Selector Boundary Classification

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Scope

This artifact classifies relation selector, drill-down, local search, selection
history, and suggestion-client public surfaces after `g07.007`.

## Classification

### `runtime/relations`

Status: retained.

Owner: runtime public entrypoint for relation selector workflow mechanics.

Retained surfaces:

- relation selector context and Svelte context helpers
- relation selector types
- drill-down relation selector types
- `createLocalSearchFns()`
- `createLocalDrillDownSearchFns()`

Decision:

- keep `runtime/relations` as one coherent public path
- do not split it during `g07`
- keep it focused on selector runtime contracts and local dataset adapters

Reasoning:

- relation selector context, search contracts, and drill-down contracts form one
  reusable workflow family
- local search helpers adapt app-owned in-memory datasets to those selector
  contracts
- consuming apps already use this path for selector types and helper functions
  without requiring behavior changes

### `client/suggestions`

Status: retained.

Owner: client request-shape and query-param helpers for suggestion routes.

Retained surfaces:

- `SuggestionRequestOptions`
- `formatHintsParam()`
- `parseHintsParam()`
- `buildSuggestionParams()`
- `appendSuggestionParams()`

Decision:

- keep URL/query-param construction under `@inflatable-cookie/underlay/client/suggestions`
- do not teach these helpers through `runtime/relations`

Reasoning:

- these helpers shape browser/API client requests
- they do not own selector state, UI state, or runtime search behavior
- active docs now import them from the client path

### `runtime/data`

Status: retained.

Owner: lower collection workflow helpers, including relation-adjacent local
selection history.

Retained surface:

- `createSelectionHistory()`

Decision:

- keep selection history under `@inflatable-cookie/underlay/runtime/data`
- do not move it into `runtime/relations` during this generation

Reasoning:

- selection history is a local persistence/history helper
- current docs and consumer apps already use the `runtime/data` path
- moving it would create churn without improving the selector contract

### `patterns/selection-history.ts`

Status: retained implementation. The compatibility-only suggestion-param
re-exports were later retired by `g07.013`.

Decision:

- do not document suggestion-param helpers through this path
- use `client/suggestions` for request helpers
- use `runtime/data` for `createSelectionHistory()`

Reasoning:

- the implementation imports suggestion helpers from `client/suggestions`
- callers should prefer `client/suggestions` for request helpers and
  `runtime/data` for `createSelectionHistory()`

### Visible Relation Selector UI

Status: app-owned or Poodle-adjacent, depending on the consumer.

Decision:

- Underlay owns generic selector state, context, types, and local adapters
- apps own visible relation selector composition, DTO mapping, route naming,
  permissions, and wording
- do not add an Underlay visible selector component as part of `g07`

## Consumer Findings

The six-consumer scan found the expected shape:

- Acowtancy/Dairy imports selector types, local search helpers, drill-down
  helpers, and selector context from `runtime/relations`
- Acowtancy/Dairy imports `createSelectionHistory()` from `runtime/data`
- API-client packages use `SuggestionRequestOptions` through
  `client/suggestions` or generated/local client barrels
- no consumer currently needs a migration for this card

## Doc Findings

Active docs now teach:

- `runtime/relations` for relation selector types/context and local search
  adapters
- `runtime/data` for selection history
- `client/suggestions` for suggestion request query-param helpers

One stale example import in `docs/guides/092-selection-suggestions.md` was
corrected from `runtime/relations` to `client/suggestions`.

## Follow-On

`g07.008` should inventory TS testing and guardrail support, including whether
guardrail scanners can enforce preferred public paths for retained runtime,
client, pattern, and template surfaces.
