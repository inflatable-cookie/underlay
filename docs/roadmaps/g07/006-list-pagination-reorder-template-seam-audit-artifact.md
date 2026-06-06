# g07.006 Artifact - List, Pagination, Reorder, And Template Seam Audit

## Result

Keep `runtime/data` broad for now.

The list/data family is wide, but it has one coherent lower-layer job:
collection workflow mechanics below page/list templates.

No public exports, imports, or runtime behavior changed.

## Classification

| Surface | Owner | Public path | Posture |
| --- | --- | --- | --- |
| `createListController` | runtime data | `runtime/data` | retained lower autonomous-list controller; not a template shell |
| `createPaginationController` | runtime data | `runtime/data` | retained server cursor pagination controller; compatibility for cursor-backed lists |
| `createClientPagination` | runtime data | `runtime/data` | retained local pagination helper for already-loaded collections |
| `useBatchSelection` | runtime data | `runtime/data` | retained selection-state primitive |
| `useBatchActions` | runtime data | `runtime/data` | retained batch action/selection primitive |
| `useSyncedSelection` | runtime data | `runtime/data` | retained form/dropdown synchronization helper |
| `createReorderController` | runtime data | `runtime/data` | retained reorder state controller |
| reorder conflict helpers | runtime data | `runtime/data` | retained conflict parsing/recovery helpers |
| `createLocalReorderSession` | runtime data | `runtime/data` | retained local full-set reorder session |
| `createLoadedReorderSession` | runtime data | `runtime/data` | retained fetch-all reorder session for paged browse contexts |
| `createSelectionModeController` | runtime data | `runtime/data` | retained selection-mode transition helper |
| `buildSelectionTransformState` | runtime data | `runtime/data` | retained selection-derived transform-launch helper |
| `EntityListPage` | templates | `templates` | retained Level 1 list page shell |
| `EntityList` | templates | `templates` | retained Level 2 list engine under page shells and narrow embeds |
| `EntityInlineListModule` | templates | `templates` | retained compact managed child-collection module |
| `EntityListCard` | templates | `templates` | retained card composition helper |

## Boundary Decision

`runtime/data` should remain the public home for lower collection workflow
helpers. Do not split it now.

Reasons:

- consumer code already imports focused runtime/data helpers where needed
- most current list consumers use `templates` for page/list shell composition
- `EntityListPage` and `EntityList` deliberately do not expose cursor-runtime
  pagination as their primary contract
- contracts `115` and `116` keep page-shaped list envelopes separate from lower
  cursor helper compatibility
- splitting `runtime/data` would add churn without removing real ambiguity yet

## Template Boundary

Templates own:

- page/list shell composition
- card/list rendering posture
- declarative batch, filter, reorder, and query props where the page shape fits
- page-shaped list data loader contracts

Runtime data helpers own:

- lower state machines
- auth-aware list/pagination fetch state where not using templates
- selection/batch state
- reorder sessions and conflict recovery
- local/client pagination helpers

Apps own:

- DTOs
- route wording
- query-profile selection
- domain-specific transform targets
- action implementations and permissions

## Consumer Evidence

The six-consumer scan showed:

- broad `EntityListPage`, `EntityListCard`, and `EntityInlineListModule`
  adoption across `underlay-reference`, `contact-patch`, `compli-me`,
  `acowtancy`, `songsprout`, and `loophole/composer`
- focused `runtime/data` use for reorder, synced selection, and client
  pagination where a template shell is not the right abstraction
- no evidence that consumers need an immediate split of `runtime/data`

## Consumer Upgrade Impact

None.

This is an audit and contract cleanup only.

## Follow-on

`g07.007` should audit `runtime/relations`, relation selector context/types,
local search helpers, drill-down helpers, selection history, and
`client/suggestions`.

## Validation Inputs

- inspected `runtime/data`
- inspected list, pagination, reorder, batch, and synced-selection helpers
- inspected template list components and contracts `100`, `110`, `115`, `116`,
  and `117`
- scanned active docs and six-consumer usage
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`
- `effigy check:types`

## Next Task

Move to `g07.007`: relation selector boundary audit.
