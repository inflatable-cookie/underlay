# g07.005 Artifact - Duplicated Auth-Aware Fetch Orchestration Decision

## Result

Keep the public helper split.

- `runtime/auth` owns page/detail/form-style authenticated data loading through
  `useAuthenticatedData`.
- `runtime/data` owns list, pagination, selection, reorder, and batch data
  helpers.
- templates own page-shell composition and may call `useAuthenticatedData`
  where they need authenticated template loading.

Do not merge these into one public data controller. They solve different
workflow shapes.

## Internal Consolidation

The duplicated auth-fetch mechanics are now centralized in
`ts/src/patterns/auth-fetch.ts`.

Shared internal behavior:

- resolve instance/global `getToken` and `onRefresh`
- throw the same setup error shape when no token provider exists
- skip initial fetch until auth is ready
- detect 401 errors
- call the configured refresh handler
- retry once with the refreshed token
- normalize unknown errors into `Error`

Controllers still own their state:

- `useAuthenticatedData` keeps `data`, `loading`, `refetching`, query-key
  watching, queued refetch, and one-shot fetch semantics.
- `createListController` keeps list items, filters, local item updates, and
  filter-driven refetch.
- `createPaginationController` keeps cursor state, page size, cursor history,
  and pagination-specific reset behavior.

## Inventory

| Surface | Public path | Role | Decision |
| --- | --- | --- | --- |
| `useAuthenticatedData` | `runtime/auth` | general authenticated page/component data loader | keep public owner |
| `createListController` | `runtime/data` | autonomous list data/filter controller | keep public owner |
| `createPaginationController` | `runtime/data` | authenticated server cursor pagination controller | keep public owner |
| `createClientPagination` | `runtime/data` | local client pagination over already-loaded items | no auth-fetch concern |
| entity templates | `templates` | page/list/detail composition | keep as template callers, not auth-fetch owners |
| client query/http helpers | `client/*` | lower transport/query helpers | keep below workflow state |

## Consumer Evidence

The six-consumer family has broad `useAuthenticatedData` usage and some
template/list-adjacent usage. This confirmed that public contracts should stay
stable.

Roots scanned:

- `underlay-reference`
- `contact-patch`
- `compli-me`
- `acowtancy`
- `songsprout`
- `loophole/composer`

## Consumer Upgrade Impact

None.

No consumer imports or option shapes changed. This is internal consolidation
plus contract/roadmap documentation.

## Follow-on

`g07.006` should inspect the remaining seam between list controllers,
pagination controllers, reorder sessions, and templates. The question is no
longer auth refresh duplication; it is whether the list/data/template workflow
split is still the cleanest public shape.

## Validation Inputs

- source inspection of `useAuthenticatedData`, `createListController`,
  `createPaginationController`, and template data loaders
- six-consumer scan for direct usage
- `effigy check:types`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`

## Next Task

Move to `g07.006`: list, pagination, reorder, and template seam audit.
