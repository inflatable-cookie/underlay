# g08.023 - EntityList Generics And Split

Status: done
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Fix the type-erased flagship template. `EntityList.svelte` is a 1,649-line god
component whose top declares `type T = any` (with an eslint-disable), so
`dataLoader`, `renderItem`, and row mapping are all untyped, and it single-handedly
wires filters, batch actions, reorder, three presentations (cards/table/log),
dialogs, and pagination. `EntityDetailPage` already uses a real `$$Generic`, so
the pattern is inconsistent. The titled/untitled render paths also duplicate
~100 lines verbatim.

## Evidence

- `ts/src/templates/EntityList.svelte:2-3` (`type T = any`)
- duplicated markup `EntityList.svelte:1249-1522`
- fragile fetch-dedup `EntityList.svelte:355-426,621-634`
- correct comparison `EntityDetailPage.svelte:133` (`$$Generic`)

## Governing References

- [110 Admin template system](../../contracts/110-admin-template-system.md)
- [116 Canonical collection routes and query profiles](../../contracts/116-canonical-collection-routes-and-query-profiles.md)

## Planned Changes

- [x] Use the Svelte 5 `generics` attribute for a real item type instead of
  `type T = any`. Now `generics="T extends object = Record<string, unknown>"`;
  the `object` constraint (not `Record<string, unknown>`) is deliberate so
  consumer/wrapper interfaces without index signatures still satisfy it.
- [x] Extract the shared titled/untitled snippet. The ~85-line reorder +
  cards/log/table body was duplicated verbatim across the titled (ListContainer)
  and untitled (EntityListPage) paths; it is now a single `listBody` snippet
  parameterised only by the log empty-message. **Deferred:** splitting the three
  presentations into separate `.svelte` files. Cards/table/log already delegate
  rendering to poodle subcomponents (`ListGrid`/`DataTable`/`LogList`); a further
  file split would thread ~20 internal props (batch, reorder controller,
  `renderCell`, `tableColumns`, ...) into each child for no functional gain and
  real churn — held under the stop condition.
- [x] Replace the fetch-dedup key duplication with a single source of truth. The
  derived `listQueryKey` and the key stamped in `setQuery` now both build through
  one `buildListQueryKey(query)` helper, so they cannot drift out of shape.
- [x] Add debounce to the search input (new optional `searchDebounceMs` prop,
  default 250ms; select/sort filters still apply immediately); add a refetching
  affordance (`isRefetching` -> "Updating…" in the toolbar summary) for non-empty
  lists whose full-page loader no longer fires on refetch.

## Consumer Upgrade Impact

Impact class: `behavioral` if the component's generic signature changes; manage
via `023`.

## Validation

- [x] component tests covering the three presentations: `cards`
  (EntityListCardHarness), `table` (Controlled-filter / summary / query-variant
  harnesses), and `log` (new EntityListLogHarness driving the generic
  `toLogEntries` mapper). Added a debounce test proving rapid keystrokes collapse
  to one refetch. Component suite 31 -> 33.
- [x] `bun x vitest run` — 739 unit passed; component suite 33 passed.
- [x] `effigy validate` — clean (svelte-check 0 errors, guardrails, component
  hygiene, poodle prop-name check all pass).
- [x] Six-consumer typecheck: acowtancy/dairy (7740), underlay-reference/
  acme-admin (6402), compli-me/admin (6297), loophole/composer-admin (2558),
  contact-patch/cp-admin (6269), songsprout/greenhouse+bloom+stem — all 0 errors.
  Fixed a pre-existing songsprout/stem media mapper break surfaced during the
  sweep (see log).

## Stop Conditions

Stop if the split changes the public prop surface in ways consumers depend on;
stage the prop changes under the sunset process.

## Next Task

`g08.024` strict-type and dependency hygiene.
