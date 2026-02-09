# Recipe: Synced Hierarchical Selection Forms

**Use when**: Create/edit forms depend on hierarchical context (module -> section -> area) and must preserve selections across load and validation cycles.

**Example prompt**: "Build outcome create form scoped to selected area with fallback selection"

---

## Key Principle

Synchronize selection from three sources, in order:
1. **Route/query preselection**
2. **Loaded data defaults**
3. **Form values after failed submits**

---

## Checklist

### Phase 1: Context Data Resolution

- [ ] Resolve hierarchy from query params (`moduleId`, `sectionId`, `areaId`)
- [ ] Load minimum required related entities
- [ ] Produce normalized option arrays for form controls

### Phase 2: Authenticated Load Pattern

- [ ] Use `useAuthenticatedData()` to fetch and normalize context
- [ ] Return fallback default shape for loading and type safety

### Phase 3: Synced Selection State

- [ ] Use `useSyncedSelection<T>()`
- [ ] Initialize from preselected query-derived value
- [ ] Sync from returned `formValues` after validation failure
- [ ] Derive selected option metadata for subtitle/back context

### Phase 4: Submission and Validation

- [ ] Validate required hierarchical IDs in submit handler
- [ ] Return field-level errors for missing/invalid hierarchy
- [ ] Keep selected IDs and dependent option lists stable on failure

### Phase 5: UX Context

- [ ] Build dynamic subtitle from selected hierarchy
- [ ] Build dynamic back links with `computeBackInfo()`
- [ ] Hide dependent controls until parents are resolved

---

## References in Acowtancy

- `dairy/src/routes/(app)/learning/outcomes/new/+page.svelte`
- `dairy/src/routes/(app)/learning/areas/new/+page.svelte`
- `dairy/src/routes/(app)/learning/sections/[sectionId]/edit/+page.svelte`
