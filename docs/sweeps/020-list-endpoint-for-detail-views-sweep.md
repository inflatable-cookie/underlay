# 020 - List Endpoint for Detail Views Sweep

This sweep identifies admin detail pages that fetch data via list endpoints (with client-side or server-side filtering) instead of using dedicated single-item detail endpoints.

## Problem this sweep targets

A common drift pattern:

1. A list endpoint exists for an entity (e.g. `/v1/admin/learning/modules-with-counts`)
2. A detail page needs data from that shape (e.g. module counts for tab badges)
3. Rather than building a dedicated detail endpoint, the page calls the list endpoint with a filter and pulls `rows[0]`

Result:

- semantically incorrect API usage (list endpoint for single-item retrieval)
- unnecessary query overhead (even with server-side filtering, list queries may lack index optimisation for single-row lookup)
- fragile client code (relies on array indexing, filter correctness, and non-empty results)
- confusing intent in the codebase (readers expect list endpoints to serve list views)

## Target architecture

Every admin CRUD entity should have **both**:

1. **List endpoint** — returns all items (with optional filtering, sorting, pagination)
2. **Detail endpoint** — returns a single item by ID or key, with all data the detail view needs

### When a detail view needs supplementary data

If a detail page needs fields not present on the base detail response (e.g. tab counts, related entity counts), the correct approaches in priority order are:

1. **Add the fields to the existing detail endpoint** — preferred when the extra data is cheap (count subqueries, joined columns)
2. **Create a dedicated "detail-with-extras" endpoint** — when the extra data is expensive and only needed in specific contexts
3. **Fetch supplementary data as a second request** — acceptable for lazy-loaded tab content

### Never acceptable

- Calling a list endpoint with a filter to retrieve a single item for a detail view
- Fetching all items of a type and using `.find()` / `[0]` to locate one
- Fetching all items of a related type just to get data for one parent entity

---

## Scope

```bash
export API_REPO="/path/to/myapp-api"
export CLIENT_REPO="/path/to/myapp-client"
export ADMIN_REPO="/path/to/myapp-admin"
```

Acowtancy mapping: `farmyard`, `cattle-grid`, `dairy`.

---

## Step 1 — Find detail pages calling list commands

### 1.1 List-then-filter in useAuthenticatedData

Search for list-fetch patterns inside detail page data loaders:

```bash
rg -n "getAll|\.find\(|\.filter\(|\[0\]|\rows\[0\]" "$ADMIN_REPO/src/routes/**/\[*\]/*page.svelte"
```

Flag any hits where:

- a function returning an array is called inside a `[paramId]` route's data loader
- the result is filtered/indexed to extract a single item

### 1.2 List commands used in detail contexts

```bash
rg -n "list.*Admin|getAll|get.*WithCounts\b" "$ADMIN_REPO/src/routes/**/\[*\]/*page.svelte"
```

For each hit, verify whether the command is:

- fetching a list for a tab/relation (acceptable — lazy loaded child data)
- fetching a list to extract the parent entity's own data (anti-pattern)

### 1.3 Client commands returning arrays used in detail pages

```bash
rg -n "Promise<.*\[\]>" "$CLIENT_REPO/src/commands/**/*.ts"
```

Cross-reference with detail page imports to find list commands used in single-item contexts.

---

## Step 2 — Verify detail endpoints exist

### 2.1 Check API routes for get-by-id handlers

For every entity type, confirm that a `GET /v1/admin/{domain}/{entity}/{id}` route exists:

```bash
rg -n 'path = "/v1/admin/.*\{' "$API_REPO/crates/api/src/routes/admin" --type rust
```

Group by entity and flag any that only have list endpoints but no detail endpoint.

### 2.2 Check detail responses include necessary counts

For each detail endpoint, compare the response DTO fields against what the frontend detail page actually uses:

```bash
# Find all admin detail DTOs
rg -n "pub struct.*Dto" "$API_REPO/crates/api/src/dto" --type rust

# Compare with frontend type usage
rg -n "module\.\w+Count|entity\.\w+Count" "$ADMIN_REPO/src/routes/**/\[*\]"
```

Flag cases where the detail page needs count fields that the detail DTO doesn't include.

---

## Step 3 — Fix identified issues

For each anti-pattern found:

### 3.1 Backend — add missing data to detail endpoint

If the detail page needs count fields, add count subqueries to the existing detail SQL query:

```sql
-- Before: detail query without counts
SELECT m.id, m.slug, m.title, ...
FROM learning.module m
WHERE m.id = $1

-- After: detail query with counts
SELECT m.id, m.slug, m.title, ...,
  COALESCE((SELECT COUNT(*) FROM learning.section s
    WHERE s.module_id = m.id AND s.deleted_at IS NULL), 0) AS section_count
FROM learning.module m
WHERE m.id = $1
```

### 3.2 Backend — add row type, domain model fields, and DTO fields

Thread new count fields through the full stack:

1. DB row struct — add `#[sqlx(default)]` count fields
2. Domain model — add count fields
3. API DTO — add serialized count fields + `From` impl
4. OpenAPI schema — update `ToSchema` derive

### 3.3 Client — add fields to TypeScript types

```typescript
// Extend existing interface
export interface LearningModule {
  // ... existing fields ...
  sectionCount: number;  // new
  areaCount: number;     // new
}
```

### 3.4 Frontend — replace list-fetch with detail-fetch

```typescript
// Before (anti-pattern)
const counts = await listCommand(fetch, token, { filter: id })
  .then(rows => rows[0] ?? null);

// After (correct)
const module = await getModuleAdmin(moduleId, fetch, token);
// counts are now on the module object directly
```

---

## Step 4 — Verify

After fixes:

1. `bun check` in the admin frontend passes
2. `cargo clippy` in the API passes
3. Detail pages load with correct counts
4. No list endpoints are called from `[paramId]` routes for parent entity data
5. Network tab shows single-item requests, not filtered list requests

---

## Acowtancy-specific findings

### Fixed in this sweep

**Module detail page** (`dairy/src/routes/(app)/learning/modules/[moduleId]/+page.svelte`):
- Was calling `getModulesAdminWithCounts()` (list endpoint) with a `module_id` filter
- Fix: add section/area/update/bundle/preseen counts to the existing single-module admin detail endpoint

**Bundle activity detail page** (`dairy/src/routes/(app)/learning/activities/bundle/[activityId]/+page.svelte`):
- Was calling `getBundles()` to fetch all bundles, then `.find()` to locate one
- Was looping through all bundles to fetch their topics
- Fix: use `getBundle(domainId)` for the single bundle, `getBundleTopics(domainId)` for its topics only
