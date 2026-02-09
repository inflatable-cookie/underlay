# Recipe: Trash Lifecycle (Soft Delete + Restore + Purge)

**Use when**: Admin workflows require reversible deletion, followed by optional permanent purge.

**Example prompt**: "Add trash management for Content entities"

---

## Key Principle

Split deletion into three explicit lifecycle steps:
1. **Soft delete** from primary views
2. **Restore** from trash
3. **Purge** for irreversible removal

---

## Checklist

### Phase 1: DB Semantics

- [ ] Use soft-delete fields (`deleted_at`, optional `delete_batch_id`)
- [ ] Exclude deleted rows from standard list queries
- [ ] Add list/restore/purge DB functions for trash context

### Phase 2: API Endpoints

- [ ] Add list trash endpoint
- [ ] Add restore endpoint
- [ ] Add purge endpoint
- [ ] Return clear error codes for missing/already-restored entities

### Phase 3: Client Commands

- [ ] Add `listTrash`, `restore`, and `purge` commands
- [ ] Keep kind/id encoding in command layer

### Phase 4: Admin Trash Page

- [ ] Build page with `useAuthenticatedData()`
- [ ] Render trash items as `ListCard` grid
- [ ] Add `Restore` primary action per item
- [ ] Add purge confirm via `AlertDialog`

### Phase 5: UX + Safety

- [ ] Add explicit irreversible copy in purge dialog
- [ ] Provide quick metadata (deleted time, kind labels)
- [ ] Show success/failure toasts and refetch after actions

---

## References in Acowtancy

- `dairy/src/routes/(app)/content/trash/+page.svelte`
- `dairy/src/routes/(app)/learning/trash/+page.svelte`
- `cattle-grid/src/commands/content/trash.ts`
- `farmyard/crates/api/src/routes/admin/content/trash/listing.rs`
