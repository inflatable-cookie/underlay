# Recipe: Reorderable Collections (Admin)

**Use when**: Items have explicit ordering and admins need drag/drop reorder
with conflict-safe persistence.

**Example prompt**: "Add reorder support for Lessons within Module"

This is now a **mixed recipe**:

- Underlay owns the reorder workflow, endpoint contract, controller logic, and
  conflict handling
- Poodle owns the visible reorder-mode list and surrounding page chrome

## Ownership Boundary

Use Underlay for:

- scope-aware reorder DB functions
- reorder API payloads and conflict responses
- client reorder commands
- `createReorderController()` and related runtime helpers
- toasts, auth-aware load, and refresh wiring

Use Poodle for:

- list shell
- reorder-mode toggle/action placement
- `ReorderableList`
- loading, empty, and error presentation

Start visible implementation from:

- `List And Filter Recipes`
- `Admin Feature Delivery Recipes`
- `Page Shell And Admin Recipes`

## Key Principle

Treat reorder as a first-class workflow:

1. explicit reorder endpoint per scope
2. strict payload validation
3. separate reorder mode from normal browse mode

## Checklist

### Phase 1: DB Reorder Function

- [ ] add scope-aware reorder function
- [ ] only reorder IDs valid for that scope
- [ ] return enough success/conflict metadata to explain the outcome

### Phase 2: API Endpoint

- [ ] add explicit `POST .../reorder` route
- [ ] validate payload shape and duplicate IDs
- [ ] return success DTO or clear conflict error

### Phase 3: Client Command

- [ ] add `reorder*` command
- [ ] keep endpoint and payload encoding centralized in the command layer

### Phase 4: UI Reorder Mode

- [ ] add a dedicated reorder-mode toggle
- [ ] load the full scoped dataset when entering reorder mode
- [ ] use `createReorderController()` with Poodle `ReorderableList`
- [ ] use guarded submit plus conflict recovery when the backend returns reorder conflicts
- [ ] page large reorder sessions with `windowSize` instead of exposing one giant drag surface
- [ ] exit reorder mode on success, cancel, or invalidating filter/scope changes

### Phase 5: Save + Feedback

- [ ] save ordered IDs through the command layer
- [ ] refresh the normal list after success
- [ ] show success/failure toasts

### Phase 6: Guardrails

- [ ] disable reorder when scope is missing or list is too small
- [ ] keep reorder and batch-selection modes mutually exclusive
- [ ] prevent submit while auth/runtime prerequisites are unavailable

## Composition Rules

- keep reorder semantics in Underlay and host code
- keep visible reorder-mode UI Poodle-first
- prefer the shared workflow posture already used in live admin and Dairy lists:
  reorder toggle, `ReorderableList`, guarded submit, and `windowSize={50}` for
  larger sessions
- do not build a new shared Underlay reorder shell around Poodle
- only add Poodle capability if multiple apps prove a missing generic reorder
  interaction, not because one app wants a convenience wrapper

## Reference Implementations

Use these as proof families:

- Dairy module/activity/variant reorder flows
- ACME-style admin list/detail families when reorder is nested under a parent

## Related Recipes

- [Autonomous Admin List](./autonomous-admin-list.md)
- [Nested Entity Management](./nested-entity-management.md)
- [Context-Preserving Navigation](./context-preserving-navigation.md)

## Next Task

If the reordered entities also have soft-delete recovery, layer in
[Trash Lifecycle](./trash-lifecycle.md) rather than mixing irreversible and
recoverable lifecycle behavior into one reorder recipe.
