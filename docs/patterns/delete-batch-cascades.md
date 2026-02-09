# Recipe: Delete Batch Cascades (Learning-style)

**Use when**: Deleting a parent entity must soft-delete multiple dependent entities atomically and track them under one batch ID.

**Example prompt**: "Implement cascade delete batch for Sections"

---

## Key Principle

For cascade deletes, store a **single batch ID** across all affected rows so restore/purge can be done as one operation.

---

## Checklist

### Phase 1: Batch Model

- [ ] Add batch ID generation (`new_delete_batch_id` style)
- [ ] Ensure all cascade updates write same `delete_batch_id`
- [ ] Keep operation transactional

### Phase 2: Cascade DB Function

- [ ] Load parent state and guard `not found` / `already deleted`
- [ ] Update parent + children in order within one transaction
- [ ] Return `SoftDeleteResult` with batch ID

### Phase 3: Batch Query Surface

- [ ] Add aggregated `list_delete_batches` query with counts by type
- [ ] Include primary entity type/id/name and deleted timestamp

### Phase 4: Restore + Purge Endpoints

- [ ] Restore by batch ID endpoint
- [ ] Purge by batch ID endpoint
- [ ] Enforce auth/role guardrails

### Phase 5: Admin UI

- [ ] List batches with cascade count summary
- [ ] Support restore + purge actions
- [ ] Show type-specific icons/labels

---

## References in Acowtancy

- `farmyard/crates/db/src/learning/soft_delete/simple/cascade/section.rs`
- `farmyard/crates/api/src/routes/admin/learning/delete_batches/queries.rs`
- `farmyard/crates/api/src/routes/admin/learning/delete_batches/mutations.rs`
- `dairy/src/routes/(app)/learning/trash/+page.svelte`
