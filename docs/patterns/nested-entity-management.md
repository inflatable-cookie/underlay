# Recipe: Nested Entity Management

**Use when**: You are managing child entities inside a parent route family.

**Example prompt**: "Build the Variants tab for Modules"

This is now a **mixed recipe**:

- Underlay owns the parent/child contract, navigation/runtime rules, and
  full-stack implementation order
- Poodle owns the visible tabs, list, dialog, detail, and form composition

## Ownership Boundary

Use Underlay for:

- parent-scoped DB and API contracts
- client command structure
- navigation-context handling
- `SpaFormShell` where shared SPA form orchestration is still useful
- testing expectations

Use Poodle for:

- tabs
- page headers and metadata
- list and filter chrome
- inline create or modal edit shell
- fields, actions, and detail presentation

Start visible implementation from:

- `Page Shell And Admin Recipes`
- `Admin Feature Delivery Recipes`
- `Dialog And Detail Recipes`
- `List And Filter Recipes`

## Key Principle

Child entities should usually be managed through:

1. a scoped list endpoint on the parent
2. independent CRUD on the child entity
3. a parent detail route that composes the child surface using Poodle tabs and
   host-owned actions

Keep the visible parent detail shell stable:

- one `PageHeader`
- one `MetaBar` directly beneath it
- one top-level `Tabs` control for child surfaces
- detail-tab content rendered as cards and `DetailSection`s, not a second inner
  page header
- related inline lists like versions or usages rendered with
  `InlineListSection` under that shell
- richer child browse collections like aliases, notices, and variants kept on
  host-owned list-card/grid composition, not revived mini page shells
- compact child collections inside a detail tab should still expose a local
  title, count badge, empty copy, and row-level actions
- richer child collections that need filters, reorder, selection mode, or batch
  actions should live on a dedicated tab or section body instead of being
  squeezed into the overview/details tab

## Checklist

### Phase 1: Backend - Database Layer

**File**: `crates/db/src/{domain}.rs`

- [ ] scoped child list for the parent
- [ ] child lookup by id
- [ ] create child with parent FK
- [ ] update child
- [ ] soft delete child if required
- [ ] scoped uniqueness checks where needed

### Phase 2: Backend - DTOs

**File**: `crates/api/src/dto/{domain}.rs`

- [ ] child DTO
- [ ] create payload with parent reference
- [ ] update payload without parent reassignment unless reassignment is
      explicitly supported

### Phase 3: Backend - Routes

- [ ] scoped list under the parent route
- [ ] child create route
- [ ] child detail route
- [ ] child update route
- [ ] child delete route

Keep route semantics explicit rather than hiding them behind generic nested UI.

### Phase 4: Client Commands

- [ ] parent-scoped list command
- [ ] child CRUD commands
- [ ] any reorder or default/primary child mutation commands

### Phase 5: Parent Detail UI

Compose the visible parent route in Poodle:

- `PageHeader`
- `MetaBar`
- `Tabs`
- `ListContainer` or direct tab content
- `Card` + `DetailSection` for the parent overview tab

Keep tab items and counts host-owned. Do not revive old Underlay tab examples.

### Phase 6: Child Create/Edit/Delete UI

Choose the shell based on workflow:

- route page + `PageHeader` + `Card` for simple single-submit child forms
- route page + `SpaFormShell` for larger intent-driven create/edit flows
- `FormDialog` for compact modal create/edit
- `AlertDialog` for destructive confirm

Keep all visible field and action composition in Poodle.

For route-page child forms:

- keep parent context explicit in the back link and header subtitle
- keep the editable body inside one carded form section
- use `SpaFormShell` only when save-vs-save-close/delete intent handling or
  navigation-context return behavior is part of the route workflow

### Phase 7: Navigation and Return Context

Nested routes are where context drift becomes expensive. Apply:

- `gotoWithContext()`
- `consumeNavigationContext()`
- `navigateOnCancel()`

Use [Context-Preserving Navigation](./context-preserving-navigation.md).

## Composition Rules

- keep parent/child route ownership explicit
- keep visible tabs and forms Poodle-first
- keep navigation/runtime behavior in Underlay or host code
- avoid inventing a new shared nested-entity shell unless a truly generic
  runtime seam emerges

## Reference Implementations

Use the ACME admin parent/child route families and Dairy nested learning flows
as concrete references for route splitting, tab posture, and command shape.

## Related Recipes

- [CRUD Admin Interface](./crud-admin-interface.md)
- [Autonomous Admin List](./autonomous-admin-list.md)
- [Reorderable Collections](./reorderable-collections.md)
- [Context-Preserving Navigation](./context-preserving-navigation.md)

## Next Task

If the child surface starts needing local pagination, filters, selection mode,
or batch actions, switch to
[Autonomous Admin List](./autonomous-admin-list.md) for the list body instead
of growing ad hoc nested list logic.
