# Backlog: Collection Helper Extractions (Reorder Scope, Load-All-Pages)

**Status**: Backlog  
**Priority**: Medium  
**Estimated Effort**: 4-6 hours  
**Source**: contact-patch normalization lane (2026-08 consumer template convergence)

---

## Problem Statement

During the 2026-08 consumer normalization, contact-patch's `ChaptersList` /
`book-reorder.ts` was audited against contract 117's retained reorder
controllers (`createLocalReorderSession`, `createLoadedReorderSession` from
`@inflatable-cookie/underlay/runtime/data`). The controllers were not a genuine
fit, and three concrete gaps were identified:

1. **`EntityList` duplicates `createLoadedReorderSession` internally** — the
   loaded-strategy implementation inside `EntityList.svelte` (~lines 895-941)
   re-implements the session lifecycle instead of composing the retained
   controller. Two copies of the same logic can drift.
2. **No retained load-all-pages fetch helper** — consumers each carry their
   own `loadAllReorderItems` fetch loop (contact-patch
   `src/lib/lists/book-reorder.ts`; dairy has equivalents). The reorder
   sessions take `loadItems` as input but nothing retained owns the paged
   fan-out.
3. **No retained canonical-reorder-scope helper** — consumers hand-roll
   "unfiltered query + single `eq` filter" detection from `QueryParams` to
   decide whether reorder is allowed and which canonical scope (e.g. subject)
   it belongs to (`getCanonicalChapterReorderSubjectId`,
   `isUnfilteredReorderQuery`).

---

## Proposed Solution

1. Refactor `EntityList`'s loaded reorder strategy to compose
   `createLoadedReorderSession` (delete the duplicated state machine).
2. Add a retained `loadAllPages(fetchPage)` helper under
   `runtime/data` or `patterns/` that walks a paged endpoint to exhaustion
   with a sane cap.
3. Add a retained `resolveReorderScope(query, { filterField })` helper that
   returns the canonical scope id when the query is reorder-eligible
   (no search, no sort, exactly one `eq` filter on the scoping field), and
   `null` otherwise.

Each extraction must be adopted in contact-patch (`book-reorder.ts`) and dairy
as the proof consumers.

---

## Promotion Criteria

- Promote when the next consumer asks for subject/scope-gated reorder, or when
  `EntityList` reorder internals are next touched for other reasons.
