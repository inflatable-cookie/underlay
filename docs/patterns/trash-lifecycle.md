# Recipe: Trash Lifecycle (Soft Delete + Restore + Purge)

**Use when**: Admin workflows require reversible deletion followed by optional
permanent purge.

**Example prompt**: "Add trash management for Content entities"

This is now a **mixed recipe**:

- Underlay owns the lifecycle semantics, API/client contract, and runtime
  orchestration
- Poodle owns the visible trash-page composition, action chrome, and confirm UI

## Ownership Boundary

Use Underlay for:

- soft-delete and restore semantics
- trash list/restore/purge DB functions
- API routes and error codes
- client commands
- auth-aware data loading, toasts, and refresh flow

Use Poodle for:

- trash page shell
- list-card or table presentation
- action menus and buttons
- purge confirmation dialog
- empty/loading/error states

Start visible implementation from:

- `List And Filter Recipes`
- `Dialog And Detail Recipes`
- `Admin Feature Delivery Recipes`

## Key Principle

Split deletion into three explicit lifecycle steps:

1. soft delete from primary views
2. restore from trash
3. purge for irreversible removal

## Checklist

### Phase 1: DB Semantics

- [ ] use soft-delete fields (`deleted_at`, optional `delete_batch_id`)
- [ ] exclude deleted rows from normal list queries
- [ ] add trash list, restore, and purge functions

### Phase 2: API Endpoints

- [ ] add trash list endpoint
- [ ] add restore endpoint
- [ ] add purge endpoint
- [ ] return clear error codes for not found or already-restored states

### Phase 3: Client Commands

- [ ] add `listTrash`
- [ ] add `restore`
- [ ] add `purge`
- [ ] keep kind/id encoding centralized in the command layer

### Phase 4: Trash UI

- [ ] build the page with auth-aware loading
- [ ] render trash items with Poodle list or card composition
- [ ] provide `Restore` as the primary recoverable action
- [ ] provide purge confirmation through `AlertDialog`

### Phase 5: UX + Safety

- [ ] explicit irreversible copy for purge
- [ ] quick metadata like deleted time and kind labels
- [ ] success/failure toasts and refetch after actions

## Composition Rules

- keep lifecycle policy in Underlay and host code
- keep trash page composition Poodle-first
- keep destructive wording and permission rules app-owned
- do not rebuild a shared Underlay trash page shell

## Reference Implementations

Use these proof families:

- Dairy content trash and learning trash routes
- corresponding `cattle-grid` trash commands
- `farmyard` trash/listing routes

## Related Recipes

- [CRUD Admin Interface](./crud-admin-interface.md)
- [Autonomous Admin List](./autonomous-admin-list.md)
- [Context-Preserving Navigation](./context-preserving-navigation.md)

## Next Task

If delete operations participate in broader cascade recovery or batch semantics,
layer in [Delete Batch Cascades](./delete-batch-cascades.md) rather than
expanding this recipe into general soft-delete policy.
