# Recipe: Autonomous Admin List (Paginated + Filtered + Batch)

**Use when**: You need a self-contained admin list surface that can run as a
page or embedded tab with pagination, filtering, and selection-driven actions.

**Example prompt**: "Build an autonomous list for Resources with filters and batch delete"

This is now a **mixed recipe**:

- Underlay owns the pagination/auth/runtime/controller guidance
- Poodle owns the visible list/filter/bulk-action composition

## Ownership Boundary

Use Underlay for:

- paginated API and command contracts
- auth-aware data loading
- pagination controller and selection/runtime helpers
- navigation context and toasts
- testing expectations

Use Poodle for:

- `ListContainer`
- `FilterToolbar`
- `DataTable` or `Grid`
- `BulkActionBar`
- empty/loading/error presentation

Start the visible layer from:

- `List And Filter Recipes`
- `Admin Feature Delivery Recipes`
- `Page Shell And Admin Recipes`

## Key Principle

The list surface should own its own data lifecycle and controls, but it should
not own a second app-specific UI kit.

That means:

1. fetch and paginate in one place
2. keep filter/query state explicit
3. keep selection/batch behavior explicit
4. compose the visible shell from Poodle directly

## Checklist

### Phase 1: API + Client Contract

- [ ] paginated list endpoint
- [ ] filter/sort query support
- [ ] client command with pagination and filter args

### Phase 2: Component Skeleton

- [ ] props for context filters and reuse mode
- [ ] `ListContainer` as the outer shell
- [ ] `FilterToolbar` for filter posture
- [ ] `DataTable` or `Grid` for list rendering
- [ ] pagination controls at the list boundary

### Phase 3: Data Loading Pattern

- [ ] use `createPaginationController()`
- [ ] use `useAuthenticatedData()` for auth-gated load and retries
- [ ] centralize query mapping in one helper

### Phase 4: Filters and Query Mapping

- [ ] local filter state
- [ ] one mapping function from local state to command query
- [ ] reset or refresh pagination on filter changes
- [ ] persist pagination state only when it materially improves navigation
- [ ] use current Poodle filter input/select events rather than legacy `onchange`
      handlers or raw DOM controls in shared filter shells

### Phase 5: Batch Selection and Actions

- [ ] use batch-selection helpers
- [ ] register destructive or bulk actions explicitly
- [ ] render `BulkActionBar` only when selection mode is active
- [ ] treat selection mode as mutually exclusive with reorder mode; entering one
      should clear or exit the other instead of trying to blend both states
- [ ] keep destructive copy and permission rules host-owned

### Phase 6: Row/Card Actions + Navigation Context

- [ ] build `sourceContext` from the current route
- [ ] use `gotoWithContext()` for create/edit/detail navigation
- [ ] keep row actions in local card/menu components

### Phase 7: UX States

- [ ] loading state
- [ ] failure state
- [ ] empty state
- [ ] success/failure toasts for list actions
- [ ] local retry affordance for recoverable load failures when the surface can
      refetch without navigation

## Composition Rules

- keep visible list chrome Poodle-first
- keep pagination, auth, and selection runtime in Underlay or host code
- prefer `FilterToolbar` with `summaryText="Filters"` and a small ghost
  `Refresh` action in the actions slot for broad admin browse surfaces
- prefer `PageLoading` for loading branches, `Callout tone="danger"` plus a
  local retry action for recoverable failures, and `EmptyState` for real
  no-results posture
- prefer an explicit selection-mode toggle in the page header and hide normal
  create/trash row affordances while that mode is active
- do not recreate a reusable Underlay list shell when `ListContainer`,
  `FilterToolbar`, `BulkActionBar`, `DataTable`, and `Grid` already express
  the visible contract

## Reference Implementations

Good proof families:

- Dairy list surfaces
- ACME admin paginated list/detail flows

Use them as implementation references after following the Poodle recipe layer.

## Related Recipes

- [CRUD Admin Interface](./crud-admin-interface.md)
- [Nested Entity Management](./nested-entity-management.md)
- [Reorderable Collections](./reorderable-collections.md)
- [Trash Lifecycle](./trash-lifecycle.md)
- [Context-Preserving Navigation](./context-preserving-navigation.md)

## Next Task

If the list also needs explicit reorder mode or trash-specific lifecycle
behavior, move to [Reorderable Collections](./reorderable-collections.md) or
[Trash Lifecycle](./trash-lifecycle.md) instead of overloading the base list
recipe.
