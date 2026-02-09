# Recipe: Reorderable Collections (Admin)

**Use when**: Items have explicit ordering and admins need drag/drop reorder with conflict-safe persistence.

**Example prompt**: "Add reorder support for Lessons within Module"

---

## Key Principle

Treat reorder as a first-class workflow:
1. **Explicit reorder endpoint** per scope
2. **Strict payload validation** (duplicates/invalid IDs)
3. **UI reorder mode** separate from normal list mode

---

## Checklist

### Phase 1: DB Reorder Function

- [ ] Add scope-aware reorder function (`reorder_X_in_Y`)
- [ ] Only reorder IDs valid for that scope
- [ ] Return `reordered_count` and conflict metadata when needed

**References**:
- `farmyard/crates/db/src/learning/activities/reorder/*.rs`
- `farmyard/crates/db/src/exams/reorder.rs`

### Phase 2: API Endpoint

- [ ] Add `POST /.../reorder` route
- [ ] Validate payload shape and duplicate IDs
- [ ] Map DB result to `ReorderSuccessDto`
- [ ] Return conflict error code when optimistic assumptions fail

### Phase 3: Client Command

- [ ] Add `reorder*` command taking ordered IDs (or sectioned items)
- [ ] Keep endpoint encoding centralized in command layer

### Phase 4: UI Reorder Mode

- [ ] Add dedicated `reorderMode` toggle button
- [ ] Load full scoped dataset when entering reorder mode
- [ ] Use `createReorderController()` + `ReorderableList`
- [ ] Exit reorder mode on success/cancel/filter changes

### Phase 5: Save + Feedback

- [ ] Save ordered IDs via command
- [ ] Refresh normal paginated list after success
- [ ] Show toast for success/failure

### Phase 6: Guardrails

- [ ] Disable reorder when scope missing or list too small
- [ ] Prevent reorder submit while auth token missing
- [ ] Keep reorder and batch selection mutually exclusive

---

## References in Acowtancy

- `dairy/src/lib/lists/ModulesList.svelte`
- `dairy/src/lib/lists/ActivitiesList.svelte`
- `dairy/src/lib/views/VariantsTabContent.svelte`
- `cattle-grid/src/commands/learning/reorder.ts`
- `farmyard/crates/api/src/routes/admin/exams/reorder.rs`
