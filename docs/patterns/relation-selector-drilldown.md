# Recipe: Relation Selector with Drill-Down

**Use when**: An app-local selector shell targets items from a deep hierarchy
and a flat option list is impractical.

**Example prompt**: "Add drill-down navigation for Outcome selection scoped to Module → Section"

This is now a **mixed recipe**:

- Underlay owns the drill-down state model, search context, and hierarchy
  helper contracts
- Poodle owns the visible drill-down shell, search field, breadcrumbs, list
  rows, and any surrounding dialog/drawer/frame

## Ownership Boundary

Use Underlay for:

- drill-down level definitions and context typing
- search and suggestions contracts
- state transitions between hierarchy levels
- filter injection into the final relation search
- shared relation runtime helpers

Use Poodle for:

- breadcrumbs and back affordances
- search field and filters
- list-row presentation
- shell framing (dialog, drawer, panel, embedded section)

Start visible implementation from:

- `Form Layout And Field Recipes`
- `Dialog And Detail Recipes`
- `Page Shell And Admin Recipes`

## Key Principle

Layer a Finder-style or step-down navigation flow **on top of** an app-local
selector shell:

1. drill-down levels narrow scope
2. the final level hands off to normal relation search/selection
3. drill-down state feeds the final filter context automatically

## Checklist

### Phase 1: Derive Hierarchy Data

- [ ] identify hierarchy levels
- [ ] decide which levels are drill-down steps versus ordinary filters
- [ ] derive unique items and optional counts for each level

### Phase 2: Build Level Search Functions

- [ ] create search/suggest functions for each level
- [ ] use shared drill-down helpers for local data when possible
- [ ] write custom async functions for remote data when needed

### Phase 3: Build the Visible Drill-Down Shell

- [ ] choose the shell shape (embedded, drawer, dialog, side panel)
- [ ] render breadcrumbs/back trail
- [ ] render level search and results using Poodle primitives
- [ ] keep the result-row visual layer app-local over Poodle

### Phase 4: Final-Level Handoff

- [ ] inject prior selections into the final active filter set
- [ ] keep existing final selector behavior unchanged outside the hierarchy
- [ ] return the selected item to the parent form in one explicit place

### Phase 5: UX Guardrails

- [ ] preserve breadcrumbs and prior selections while searching
- [ ] make cancel/back behavior explicit
- [ ] keep hierarchy state stable across validation errors where relevant

## Composition Rules

- keep the hierarchy runtime contract in Underlay
- keep the visible shell app-local and Poodle-first
- do not reintroduce a public Underlay selector UI surface
- only add a Poodle guide if multiple apps converge on the same generic
  drill-down shell composition

## Reference Implementations

Use Dairy’s deeper learning relation-selector flows as the reference proof
family.

## Related Recipes

- [Relation Selector with Inline Create](./relation-selector-inline-create.md)
- [Synced Hierarchical Selection](./synced-hierarchical-selection.md)

## Next Task

If the hierarchy also drives broader form preselection and dependent field
state, pair this with
[Synced Hierarchical Selection](./synced-hierarchical-selection.md) rather than
trying to overload the selector recipe alone.
