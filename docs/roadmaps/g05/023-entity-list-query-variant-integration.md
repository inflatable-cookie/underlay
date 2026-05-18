# g05.023 — EntityList Query Variant Integration

## Why

Once the API can understand list variants and Poodle owns the selector
primitive, `EntityList` and `EntityListPage` need to orchestrate the behavior.

The selector belongs above `FilterToolbar` because variants are the list's base
query, while filters are temporary refinements on top.

## Goal

Add query-variant support to `EntityList` and `EntityListPage` without turning
Underlay into a second UI primitive kit.

## Scope

Primary Underlay targets:

- `ts/src/templates/EntityList.svelte`
- `ts/src/templates/EntityListPage.svelte`
- `ts/src/templates/EntityInlineListModule.svelte` if shared query support
  falls through automatically
- `ts/src/templates/template.types.ts`
- `docs/usage/templates/entity-list-page.md`
- `docs/usage/templates/entity-list-section.md`
- `docs/contracts/110-admin-template-system.md`

Expected props:

```ts
queryVariants?: QueryVariantConfig[];
defaultVariantId?: string;
capabilitiesLoader?: EntityListCapabilitiesLoader;
```

Expected behavior:

- render Poodle `CardToggleGroup` above `FilterToolbar` when variants exist
- update `query.variant` on selection
- reset `page` to `1` when the variant changes
- keep filters, sort, and pagination in the same query object
- keep URL-controlled query mode working through `query` / `onQueryChange`
- allow the active card to deactivate only when a default fallback is defined

## Toggle Semantics

Do not make "no selected card" globally mean "All".

Rules:

- a product default may be something other than all
- `defaultVariantId` is the fallback when deactivation is allowed
- `all` is an explicit variant when the product needs it
- if no default is configured, selecting an already-active variant should be a
  no-op

Marking Hub expected setup:

- default variant: `pending`
- visible variants: `pending`, `marked`, `void`, `all`

## Boundary

Underlay owns:

- query state
- loader calls
- URL serialization
- variant selector placement above `FilterToolbar`
- mapping `ListCapabilities` into list props

Poodle owns:

- `CardToggleGroup`
- deactivation interaction mechanics
- card visual treatment

Apps own:

- API commands
- endpoint-specific variant meanings
- whether capabilities are static in UI or loaded from API

## Consumer Upgrade Impact

Expected:

- additive `EntityList` and `EntityListPage` props
- no migration for lists that do not use query variants
- controlled query callers must preserve `query.variant` when updating query
  state
- list wrappers can adopt variants incrementally

## Acceptance

- `EntityListPage` supports static `queryVariants`
- `EntityListPage` supports API-loaded capabilities where provided
- `FilterToolbar` remains a filter surface and does not own variants
- selection, reorder, pagination, and refresh behavior still work
- template docs explain variant versus filter semantics
- component tests cover variant selection and page reset

## Next Task

Execute after `g05.021` and `g05.022` are far enough along to fix the final
prop names.
