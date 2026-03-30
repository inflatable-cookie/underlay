# 078 - Feedback Runtime Reassessment

Status: Complete
Owner: Platform
Created: 2026-03-30
Depends on: 077

## Overview

The retained-runtime follow-on needed one first real boundary decision:

- Poodle now owns `ToastHost`
- Underlay still owns `createToastStore()`, `useToasts()`, and the toast context
  key

This batch reassesses whether that remaining feedback helper surface still
belongs in Underlay or whether it should move into Poodle or another package.

## Findings

The live contract is runtime orchestration, not design-system rendering:

- `createToastStore()` creates app-owned toast state
- `UNDERLAY_TOASTS_CONTEXT_KEY` carries that store through Svelte context
- `useToasts()` is a tiny runtime hook over that context
- `pushSuccessToast()` / `pushErrorToast()` are small convenience helpers over
  the same app-owned store

Poodle already owns the rendered host surface through `ToastHost`, but it does
not own app-level store shape, context wiring, or generic app runtime
orchestration. That is the right split.

## Decision

Keep the feedback helper family in Underlay, but treat it explicitly as
retained runtime surface under `@decodelabs/underlay/runtime/feedback`.

Do not move it into Poodle now because:

- it is not a visual primitive or composite
- it is app-runtime glue over Svelte context and store state
- moving it into Poodle would blur the current design-system boundary again

## Delivery

- recorded the retained decision here
- updated active guide examples to import feedback helpers from
  `@decodelabs/underlay/runtime/feedback`
- removed stale guide language that implied a nonexistent `showToast()` API or
  suggested `patterns` still owned toast helpers

## Consumer Upgrade Impact

- toast rendering stays on Poodle `ToastHost`
- toast store/context helpers stay on
  `@decodelabs/underlay/runtime/feedback`
- root `@decodelabs/underlay/runtime` imports continue to work, but feedback
  examples should prefer the narrower feedback subpath

## Next Task

The strongest next retained-runtime challenge is the auth/browser seam:
reassess whether pieces of `runtime/auth` and `runtime/browser` should stay in
Underlay, move to `client`, or split into a smaller standalone runtime package.
