# g06.149 - Forms Internal Split

## Why

`g06.148` found that `ts/src/patterns/forms.ts` mixes public types, draft
persistence, Svelte store state, enhance submit handling, and helper functions
in one public source file.

The file can be split internally while keeping public imports stable.

## Goal

Split forms into focused internal modules without changing public forms exports,
runtime forms exports, form state behavior, draft persistence behavior, or
action-result normalization.

## Scope

In scope:

- keep `ts/src/patterns/forms.ts` as the public front door
- extract public form types to `ts/src/patterns/forms/types.ts`
- extract draft persistence helpers to `ts/src/patterns/forms/draft.ts`
- extract `createFormState(...)` to `ts/src/patterns/forms/state.ts`
- extract helper functions to `ts/src/patterns/forms/helpers.ts`
- update `forms-action-result.ts` to import `FieldErrors` from the type module
- preserve `ts/src/runtime/forms.ts`

Out of scope:

- changing public forms APIs
- changing form behavior
- changing consumer apps
- changing SvelteKit action result semantics

## Acceptance Criteria

- public `../patterns/forms` imports continue to compile
- runtime forms exports continue to compile
- forms tests pass
- forms action-result tests pass
- `effigy check:types` passes
- `effigy qa:docs` passes
- roadmap artifact records final module shape and public API impact

## Consumer Upgrade Impact

Expected impact: none.

This should be an internal split. If consumer imports or behavior need to
change, stop and re-enter planning.

## Current State

`g06.149` is ready.

## Next Task

Execute `g06.149`: forms internal split.
