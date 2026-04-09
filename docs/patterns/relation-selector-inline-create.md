# Recipe: Relation Selector with Inline Create

**Use when**: A form needs an app-local selector shell to pick related
entities and optionally create missing ones without leaving context.

**Example prompt**: "Add relation selector for Audio with inline create"

This is now a **mixed recipe**:

- Underlay owns the selector helper/runtime layer, selection history, and
  search/create integration contracts
- Poodle owns the visible field, dialog, drawer, and action composition around
  that helper layer

## Ownership Boundary

Use Underlay for:

- relation option modelling and mapping
- local or remote search function contracts
- selection history and recent selections
- inline-create success/error wiring
- runtime helper layer under `runtime/relations`

Use Poodle for:

- field framing
- inline-create dialog or drawer shell
- search input and result-row presentation
- empty/error/loading states

Start visible implementation from:

- `Form Layout And Field Recipes`
- `Dialog And Detail Recipes`
- `Admin Feature Delivery Recipes`

## Key Principle

Treat relation picking as a helper-driven pattern:

1. search or suggest existing entities
2. remember recent selections when useful
3. allow inline create without losing parent-form context

## Checklist

### Phase 1: Relation Data Model

- [ ] define option type (`id`, `label`, optional `description`)
- [ ] add mapper to the shared relation shape
- [ ] add local or remote search/suggestion functions

### Phase 2: Base Selector Wiring

- [ ] build or reuse an app-local selector shell
- [ ] keep the selected relation ID in parent-form state
- [ ] use selection-history helpers when the flow benefits from recents

### Phase 3: Inline Create Form

- [ ] choose the shell: inline panel, dialog, or drawer
- [ ] provide create-form state and submit wiring
- [ ] append created item to options and auto-select it on success
- [ ] handle create errors and duplicate detection explicitly

### Phase 4: Parent Form Integration

- [ ] compose the selector into the parent form
- [ ] keep selected IDs in form state
- [ ] serialize IDs in the submit payload

### Phase 5: UX Guardrails

- [ ] disable selector while options are loading
- [ ] show placeholder and empty-state copy
- [ ] preserve the selected value across validation failures

## Composition Rules

- keep the visible selector shell app-local and Poodle-first
- keep shared relation search/history/runtime in Underlay
- do not rebuild the old Underlay selector UI wrapper layer
- add to Poodle only if multiple apps prove the same generic selector-shell
  composition, not because one app wants less local code

## Reference Implementations

Use Dairy’s activity-form selector family as the main proof set, especially the
material selectors and inline-create helpers.

## Related Recipes

- [Relation Selector with Drill-Down](./relation-selector-drilldown.md)
- [Synced Hierarchical Selection](./synced-hierarchical-selection.md)
- [CRUD Admin Interface](./crud-admin-interface.md)

## Next Task

If the relation space is too large for flat search or suggestions, move to
[Relation Selector with Drill-Down](./relation-selector-drilldown.md) instead
of stuffing hierarchy navigation into the base inline-create selector.
