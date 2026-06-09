# Admin Section Agent Protocol

Status: active

Use this protocol when an agent is asked to build or extend a normal admin
resource family in an Underlay-based app.

The goal is cohesive admin sections, not isolated route files that happen to
compile.

## Prompt Shape

Use this shape when asking an agent to build a section:

```text
Use /underlay-build admin-section.
Build the admin section for <resource family>.
Follow nearby app examples first, then Underlay template docs.
Deliver list, detail, create/edit, action menu, navigation context, counters,
and validation as one cohesive route family.
Do not hand-roll page headers, detail sections, list cards, or delete actions
when an app-local wrapper or Underlay template pattern exists.
```

If the task is intentionally narrower, name the exception:

```text
Use /underlay-build admin-detail only.
Patch the existing <resource> detail page. Do not build list/form routes.
```

## Required Reads

Before coding:

1. Read this protocol.
2. Read the matching template docs:
   - list: `entity-list-page.md`
   - detail: `entity-detail-page.md`
   - form: `entity-form-page.md`
   - API: `../../contracts/115-admin-resource-api-shapes.md`
3. Inspect two nearby app examples with the same shape.
4. Inspect existing app-local wrappers:
   - `src/lib/cards/*`
   - `src/lib/lists/*`
   - `src/lib/menus/*`
   - `src/lib/forms/*`

Do not start from primitives. Start from the app's existing admin route family
and the Underlay templates.

## Delivery Order

Build in this order:

1. API/client shape
2. list wrapper and card
3. detail route
4. form route/body
5. actions menu and soft-delete flow
6. navigation context and back behavior
7. counters and child tabs
8. focused validation

This avoids the common failure mode where a list compiles but the detail page
is an unrelated hand-rolled composition.

## List Rules

Normal browse/manage lists use:

- app-local `src/lib/lists/<Resource>List.svelte`
- app-local `src/lib/cards/<Resource>ListCard.svelte` for card mode
- `EntityListPage` in the wrapper
- `EntityListCard` in the card
- `toPagedListResult(...)` for page-shaped API responses

Root routes should thin-mount the app-local list wrapper.

Detail tabs that are real child collections should usually reuse the same
wrapper with parent scope props and lower `headerLevel`.

Use `EntityList` only for genuinely narrower inline/embed utility lists.

## Detail Rules

Normal detail pages use `EntityDetailPage`.

The page owns:

- header
- back link
- breadcrumbs
- meta bar
- live/draft status
- top-level tabs
- header action menu
- loading and not-found posture

The body uses `EntityDetail` plus simple detail items/modules. Do not rebuild a
detail page from raw `PageHeader`, `MetaBar`, `Tabs`, `Card`,
`DetailSection`, and `DetailItem` unless the route is an explicit template
exception.

Default detail posture:

- `section` is the plural resource family.
- `title` is the shortest useful record label.
- `subtitle` carries longer descriptive text.
- ID metadata uses `EntityMetaItem`/meta config and Poodle `Code`.
- Slugs and IDs in detail sections use `Code` with plain inline styling, not a
large padded code block.
- Use only `LIVE`/`DRAFT` lifecycle badges unless the domain has a separate
meaningful state.
- Header actions use the same app-local `*ActionsMenu` on detail and edit
routes.
- Soft delete lives in the header actions menu, not the main form action row.
- Child tab counters come from the detail DTO or a deliberate lightweight count
fetch.

Detail pages should not:

- add nested `PageHeader` inside the active tab
- split simple metadata into many titled blocks
- create separate `Configuration`/`Timestamps` headings by default
- use `EntityList` for a real browse/manage tab
- drop navigation context when linking into child records

## Form Rules

Create/edit pages use `EntityFormPage` as the shell and app-owned form bodies
inside `src/lib/forms/*` when the form is reused or non-trivial.

Forms are not declarative templates. Use Poodle fields directly inside the form
body.

Edit routes should mount the same app-local actions menu in `headerActions` as
the detail route for that item.

Do not put soft-delete buttons in the main submit action area.

## Navigation Rules

Preserve context.

For links from a tab to a child detail page, the browser back button and the
page back link should return to the same parent tab.

Rules:

- tab navigation updates `?tab=<key>`
- tab switching replaces history
- record navigation from a tab preserves source context with that tab URL
- back labels use the target resource label, not `Back to ...`, unless the app
  pattern explicitly differs

## Responsive UI Rules

Use the current shared layout posture:

- detail sections max out at two columns
- list-card grids may expand to three columns
- detail fields collapse by container width, not global viewport guesses
- tab strips may collapse responsively instead of wrapping
- list-card `TimeAgo` footers inherit typography and do not include an
  `Updated` label
- code values in detail fields use plain inline code styling unless the value
  is a genuine block

Do not add media-query overrides for Poodle component dimensions when a size or
density prop can be selected in code.

## Validation

Run the smallest repo-owned validation that covers the touched app:

- use `effigy tasks` to discover the surface if needed
- prefer `effigy check` for Svelte app changes
- use API/backend checks only when backend/client code changed
- report unrelated blockers explicitly

## Completion Checklist

Before closing:

- list page uses app-local wrapper over `EntityListPage`
- card uses `EntityListCard`
- detail page uses `EntityDetailPage`
- detail tabs are configured with stable tab IDs and counts when available
- real child list tabs use `EntityListPage` wrapper, not raw inline lists
- edit page uses `EntityFormPage`
- detail/edit share the app-local actions menu
- soft delete is only in header actions
- navigation context returns to the correct tab
- Svelte check or equivalent targeted validation was run
