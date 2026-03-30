# Recipe: Relation Selector with Inline Create

**Use when**: A form needs an app-local selector shell to pick related
entities and optionally create missing ones without leaving context.

**Example prompt**: "Add relation selector for Audio with inline create"

---

## Key Principle

Treat relation picking as a reusable helper-driven pattern:
1. **Search/suggest/select** existing entities
2. **Remember recent selections**
3. **Inline create** when no suitable relation exists

---

## Checklist

### Phase 1: Relation Data Model

- [ ] Define option type (`id`, `label`, optional `description`)
- [ ] Define mapper to `SelectableRelation`
- [ ] Add local or remote search/suggestion functions

### Phase 2: Base Selector Wiring

- [ ] Use your local selector shell with hidden form field binding
- [ ] Provide `initialSelection` and `selectionHistory` store
- [ ] Handle `onchange` -> update selected relation ID

### Phase 3: Inline Create Form

- [ ] Enable `allowCreate`
- [ ] Provide `createForm` snippet
- [ ] On success: append created item to options and auto-select it
- [ ] Handle create errors and duplicate detection callbacks

### Phase 4: Parent Form Integration

- [ ] Compose selectors for each relation type needed
- [ ] Keep selected IDs in parent form state
- [ ] Serialize selected IDs into form submit payload

### Phase 5: UX Guardrails

- [ ] Disable selector while options are loading
- [ ] Show clear placeholder and empty-state copy
- [ ] Preserve prior selected value across validation failures

---

## References in Acowtancy

- `dairy/src/lib/forms/learning/activity-form/audio-material-selector.svelte`
- `dairy/src/lib/forms/learning/activity-form/video-material-selector.svelte`
- `dairy/src/lib/forms/learning/activity-form/document-material-selector.svelte`
- `dairy/src/lib/forms/learning/activity-form/inline-create-bundle.svelte.ts`
