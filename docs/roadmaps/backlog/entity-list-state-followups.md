# Backlog: createEntityListState Follow-Ups

**Status**: Backlog  
**Priority**: Low  
**Estimated Effort**: 2-3 hours  
**Source**: dairy wrapper rollout (2026-08 consumer template convergence)

---

## Problem Statement

The dairy rollout migrated 37 wrappers onto `createEntityListState`
(`ts/src/patterns/entity-list-state.svelte.ts`) cleanly, but four shapes did
not fit the factory and remain hand-rolled in consumers:

1. **Content-hash reload keys** — `reloadKey` unconditionally appends
   `:${refreshVersion}`; keys like `bundle-topics:{bundleId}:{ids…}` (a
   content hash with no refresh component) are not expressible.
   (dairy: BundleTopicsList, BundleModulesList)
2. **Custom sourceContext fallbacks** — non-list-type/detail-href/
   conditional-label fallbacks need hand-rolled deriveds today.
   (dairy: ExamDocumentsList, BundleModulesList, OutcomesListPage,
   ActivitiesList, PreSeenActivitiesList, SyllabusUpdatesList)
3. **Pure-helper dialect** — `filterValue` reads component state, so pure
   helpers in `*-list/filters.ts` operating on an explicit query arg can't use
   it. (dairy: audios-list, videos-list, summaries-list filter modules)
4. **Non-thunk `pageSize`** — passing a `pageSize` prop by shorthand yields a
   `state_referenced_locally` warning per file; a thunked option would
   eliminate the warning class entirely.

---

## Proposed Solution

1. Accept a raw `reloadKey?: () => string` override (skips the
   `:${refreshVersion}` suffix).
2. Accept `sourceContextFallback?: () => NavigationContext | undefined` used
   when `sourceContext` is not provided.
3. Export a pure `filterValueFromQuery(query, field, options)` alongside the
   factory; implement the factory's `filterValue` on top of it.
4. Accept `pageSize?: number | (() => number)`.

Adopt each in dairy's remaining hand-rolled spots as the proof consumer.

---

## Promotion Criteria

- Promote when the next consumer wrapper rollout hits the same shapes, or when
  the factory is next touched for other reasons.
