# 2026-07-17 - g08.023 EntityList generics and split

## Context

`EntityList.svelte` is the flagship list template (three presentations, filters,
batch actions, reorder, pagination). It declared `type T = any` (with an
eslint-disable), so `dataLoader`/`renderItem`/row mapping were type-erased; the
titled and untitled render paths duplicated ~85 lines verbatim; and the
fetch-dedup key was stamped in two independent `JSON.stringify` sites that could
drift.

## Changes

- **Real item generic.** `type T = any` ->
  `generics="T extends object = Record<string, unknown>"`. The `object`
  constraint (rather than `Record<string, unknown>`) is deliberate: consumer and
  wrapper interfaces lack index signatures and are rejected by
  `Record<string, unknown>`, but satisfy `object`. Surfacing the real type
  exposed 11 latent boundary mismatches against the poodle prop types
  (`EditableListItem`, `TableRowAction`, `TableRow`); each is bridged with a
  narrow `as unknown as` cast at the single delegation point.
- **De-duplicated render body.** The reorder + cards/log/table body is now one
  `listBody(logEmptyMessage)` snippet, rendered from both the titled
  (ListContainer) and untitled (EntityListPage) paths. Only the log empty message
  differed, so it is the one parameter. Removes the verbatim ~85-line copy.
- **Single fetch-dedup key.** Both the derived `listQueryKey` and the key stamped
  in `setQuery` now build through one `buildListQueryKey(query)` helper — they
  can no longer diverge in shape.
- **Debounced search + refetching affordance.** New optional `searchDebounceMs`
  prop (default 250ms, `0` disables) debounces free-text search filters;
  select/sort filters still apply immediately. A new `isRefetching` derived
  (loading with prior results on screen) renders an "Updating…" status in the
  toolbar summary, since the full-page loader only fires on the first load.

## Deferred (stop condition)

Splitting the three presentations into separate `.svelte` files. Cards/table/log
already delegate rendering to poodle subcomponents
(`ListGrid`/`DataTable`/`LogList`); a file-level split of EntityList's own wiring
would thread ~20 internal props (batch store, reorder controller, `renderCell`,
`tableColumns`, ...) into each child for no functional gain and real churn. The
god-component's actual defects (type erasure, dup markup, fragile dedup) are
resolved without it. Held under the card's stop condition.

## Consumer sweep

The generic change is `behavioral` in class but backward-compatible in practice:
the default type parameter fills in for consumers that do not specify one, and
Svelte infers `T` from the `dataLoader` item type (always an object). All six
consumers typecheck clean against the new signature: acowtancy/dairy (7740
files), underlay-reference/acme-admin (6402), compli-me/admin (6297),
loophole/composer-admin (2558), contact-patch/cp-admin (6269),
songsprout/greenhouse+bloom+stem.

**Pre-existing break fixed in passing.** songsprout/stem's
`normalizeMediaDetail`/`normalizeMediaSummary` did not map the audit fields the
committed underlay `MediaDetail`/`MediaSummary` types already require
(`updatedBy`, `deletedBy`, and `usageCount` on the summary). This predates
g08.023 but broke the greenhouse/bloom typecheck; added the snake fields and
their mappings so the songsprout apps compile.

## Validation

- `effigy validate`: clean — svelte-check 0 errors (2472 files), guardrails,
  component-test hygiene, poodle prop-name check all pass.
- `bun x vitest run`: 739 unit passed.
- Component suite: 33 passed (was 31; added a `log` presentation test through the
  generic `toLogEntries` mapper, and a search-debounce test proving rapid
  keystrokes collapse to a single refetch).
- Six-consumer svelte-check: all 0 errors (see above).

## Consumer Upgrade Notes

Impact class **behavioral** but transparent. The item generic defaults to
`Record<string, unknown>`, so existing untyped usage is unchanged; typed
`dataLoader`s now flow a real item type through `renderItem`/`renderCell`/
`toLogEntries`. New `searchDebounceMs` prop is optional (default 250ms). No prop
was removed or renamed; the presentation split was deferred, so no staged prop
changes.

## Next

`g08.024` strict-type and dependency hygiene.
