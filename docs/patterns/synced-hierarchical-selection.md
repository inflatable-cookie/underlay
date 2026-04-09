# Recipe: Synced Hierarchical Selection Forms

**Use when**: Create/edit forms depend on hierarchical context such as
module → section → area and must preserve selections across load and validation
cycles.

**Example prompt**: "Build outcome create form scoped to selected area with fallback selection"

This is now a **mixed recipe**:

- Underlay owns the synchronized selection/runtime behavior
- Poodle owns the visible form layout, field posture, and dependent-control
  presentation

## Ownership Boundary

Use Underlay for:

- query-derived preselection
- `useAuthenticatedData()` loading pattern
- `useSyncedSelection<T>()`
- fallback selection and dependent option synchronization
- back-info and navigation-context helpers

Use Poodle for:

- the visible field stack
- subtitles and section framing
- disabled/loading state presentation
- any inline callouts or guidance text

Start visible composition from:

- `Form Layout And Field Recipes`
- `Page Shell And Admin Recipes`
- `Admin Feature Delivery Recipes`

## Key Principle

Synchronize selection from three sources, in order:

1. route or query preselection
2. loaded data defaults
3. form values after failed submit

## Checklist

### Phase 1: Context Data Resolution

- [ ] resolve hierarchy from route or query params
- [ ] load only the minimum required related entities
- [ ] normalize option arrays for the dependent controls

### Phase 2: Authenticated Load Pattern

- [ ] use `useAuthenticatedData()` to fetch and normalize hierarchy context
- [ ] return a stable fallback shape for loading and type safety

### Phase 3: Synced Selection State

- [ ] use `useSyncedSelection<T>()`
- [ ] initialize from preselected route/query values
- [ ] sync from form values after validation failure
- [ ] derive selected option metadata for subtitle and back context

### Phase 4: Submission and Validation

- [ ] validate required hierarchy IDs in submit handling
- [ ] return field-level errors for missing or invalid hierarchy
- [ ] preserve selected IDs and dependent options on failure

### Phase 5: UX Context

- [ ] build dynamic subtitle or metadata from the selected hierarchy
- [ ] build dynamic back links with navigation helpers when useful
- [ ] hide or disable dependent controls until parent selections are ready

## Composition Rules

- keep dependent field state and selection sync in Underlay or host code
- keep visible field composition Poodle-first
- do not rebuild a second shared Underlay form shell for this behavior

## Reference Implementations

Use Dairy’s learning create/edit forms for outcomes, areas, and sections as the
main proof family.

## Related Recipes

- [Relation Selector with Inline Create](./relation-selector-inline-create.md)
- [Relation Selector with Drill-Down](./relation-selector-drilldown.md)
- [Context-Preserving Navigation](./context-preserving-navigation.md)

## Next Task

If the hierarchy is only needed inside a selector shell rather than across the
whole form, use the relation-selector recipes instead of carrying full synced
selection state into the parent form.
