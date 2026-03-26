# 021 - Wasteful Endpoint Calls Sweep

This sweep identifies pages and components that make unnecessary, redundant, or overly broad API requests. It covers several related anti-patterns that compound into severe performance degradation when left unchecked.

## Problem this sweep targets

Admin and consumer views accrue wasteful API calls through a small number of recurring anti-patterns. On a moderately complex detail page these can stack into **20+ simultaneous requests on page load** — the majority serving data the user never sees.

### Anti-pattern categories

| # | Pattern | Example | Impact |
|---|---------|---------|--------|
| **A** | All tabs mount simultaneously | tab containers keep inactive panels mounted with full children | Every tab's data fetching runs on page load |
| **B** | Global dataset fetch in constrained context | Component fetches ALL modules/pathways when it already has a fixed `moduleId` prop | Transfers kilobytes of unused data |
| **C** | Duplicate identical requests | Two sibling tabs each call `getPathwaysAdmin()` | Same payload fetched N times in parallel |
| **D** | Dead or vestigial endpoint calls | Calling a `syllabus` endpoint for a page structure that was removed | Wasted request with no consumer |
| **E** | Exhaustive pagination in page load | Looping through all pages of a paginated endpoint (`limit=200`, repeat until done) | Unbounded request count, grows with data |
| **F** | N+1 client-side fan-out | Fetch a list, then make one request per item | Request count scales with data volume |
| **G** | Eager filter/selector data fetch | Filter dropdowns or RelationSelector backing data loaded in the data fetch regardless of visibility | Fetches kilobytes of options the user may never see |
| **H** | Missing supplementary data on list DTO | List endpoint returns only IDs; frontend makes N+1 calls or global fetches to resolve labels/counts | Request count scales with data volume; global lookups waste bandwidth |
| **I** | One-size-fits-all DTO | Single endpoint/DTO serves list cards, filter dropdowns, and detail views with the same shape | Filter dropdowns transfer counts/joins they don't need; detail views may still lack fields; naming is ambiguous |
| **J** | Unguarded queryKey refetch in tab-mounted components | `$effect` calls `refetch()` on every URL change without comparing previous value | Tab-mounted list components re-fetch on every tab switch or sibling filter change |
| **K** | Redundant manual `tryFetch` alongside global auto-fetch | Page has explicit `getToken` option AND manual `$effect` calling `tryFetch`, but `configureAuth()` already provides global auto-fetch | Every page fetches its data twice on load — the global auto-fetch and the manual `$effect` race each other |

---

## Scope

```bash
export API_REPO="/path/to/myapp-api"
export CLIENT_REPO="/path/to/myapp-client"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
export UI_LIB="/path/to/myapp-ui"   # shared component library
```

Acowtancy mapping: `farmyard`, `cattle-grid`, `dairy`, `cream`, `underlay`.

---

## Step 1 — Tab container lazy-mount audit (Pattern A)

### 1.1 Identify tab container implementations

```bash
rg -n "Tabs|TabPanel|activeValue|mountedTabs" "$UI_LIB/src"
```

Review each tab container component:

- Does it render children of inactive tabs? (bits-ui default: yes — sets `hidden` attribute)
- Does it conditionally mount/unmount tab content?
- Does it support lazy-mount (render on first activation, keep mounted thereafter)?

### 1.2 Search for tab-based layouts in consuming apps

```bash
rg -n "DetailPageShell|Tabs|activeValue|mountedTabs" "$ADMIN_REPO/src" "$WEB_REPO/src" --type svelte
```

For each page with tabs, verify:

- Is tab content only mounted when the tab is active?
- Or does all tab content mount simultaneously on page load?

### 1.3 Fix pattern

Tab containers must lazy-mount content. The recommended implementation:

```svelte
<script>
  // Track which tabs have been visited so content mounts lazily
  // but stays mounted to preserve state
  let mountedTabs = $state<Set<string>>(new Set());

  $effect(() => {
    if (activeTab) {
      mountedTabs = new Set([...mountedTabs, activeTab]);
    }
  });
</script>

{#each tabs as tab (tab.value)}
  {#if mountedTabs.has(tab.value) && activeTab === tab.value}
    {@render tabContent(tab.value)}
  {/if}
{/each}
```

Key properties:

- Content only mounts when the tab is first activated
- Once mounted, content stays in the DOM (preserves scroll position, selection state, loaded data)
- Inactive tab panels are not mounted on initial load

### Pass criteria

- Inactive tab content is never mounted on initial page load
- Only the default/active tab's content fetches data on mount
- Switching to a previously visited tab does not re-mount or re-fetch

---

## Step 2 — Global dataset fetches in constrained contexts (Pattern B)

### 2.1 Find dual-mode components

Many list components are designed to work as both a root page (`variant="page"`) and an embedded tab (`variant="tab"`). Search for these:

```bash
rg -n 'variant.*=.*"page"|variant.*=.*"tab"|ListVariant|isConstrained' "$ADMIN_REPO/src/lib"
```

### 2.2 Audit data fetch logic per mode

For each dual-mode component, read its `useAuthenticatedData` callback and check:

- Does it fetch global datasets (all pathways, all modules, all schedules) regardless of variant?
- When a constraining prop like `moduleId` is provided, does it still fetch the full list?
- Are filter dropdowns hidden in tab mode but their backing data still fetched?

Common waste pattern:

```typescript
// BAD: fetches ALL pathways and ALL modules even when moduleId is known
const [pathways, modules] = await Promise.all([
  getPathwaysAdmin(fetch, token),
  getModulesAdmin(fetch, token)
]);
let targetModules = modules;
if (propModuleId) {
  targetModules = modules.filter(m => m.moduleId === propModuleId);
}
```

### 2.3 Fix pattern

Two complementary fixes — use both for best results:

**1. Constrained data fetch:** When in tab mode, skip global fetches and use scoped endpoints:

```typescript
// GOOD: only fetch what's needed for the constrained context
if (propModuleId) {
  // Tab mode: fetch only the scoped data
  const syllabus = await getModuleSyllabusAdmin(propModuleId, fetch, token);
  return { sections: extractSections(syllabus) };
}

// Page mode: fetch core list data (modules needed to enumerate syllabi)
const modules = await getModulesAdmin(fetch, token);
// ... page-mode logic that needs module list for enumeration
```

**2. Lazy-load filter dropdowns:** Remove filter dropdown data from the main data fetch entirely. Use the Underlay Select `loadItems` / `loadGroups` props so dropdown options are only fetched when the user opens the dropdown (see Step 8 for full details).

### 2.4 Downstream check: filter UI visibility

Verify that filter bars are actually hidden in tab mode. If the filter UI is hidden but the backing data is still fetched, the fetch is pure waste.

```bash
rg -n "isConstrained|variant.*tab|showFilters" "$ADMIN_REPO/src/lib/lists"
```

### Pass criteria

- Tab-mode components only fetch data relevant to their constrained context
- Global filter data (all pathways, all modules, etc.) is only fetched when filter UI is visible
- Filter dropdown data is fetched lazily via `loadItems`/`loadGroups`, not eagerly in the data loader
- No redundant `.filter()` or `.find()` on a full dataset when a scoped endpoint exists

---

## Step 3 — Duplicate identical requests across siblings (Pattern C)

### 3.1 Identify shared data needs

When multiple tab panels or sibling components call the same endpoint:

```bash
rg -n "getPathwaysAdmin|getModulesAdmin|getModuleSyllabus" "$ADMIN_REPO/src/lib/lists" --type svelte
```

Count how many distinct components call each global endpoint. If the same endpoint is called by N sibling tabs, it will fire N times concurrently when those tabs mount.

### 3.2 Fix pattern

Options in priority order:

1. **Eliminate the fetch** — if the component is in constrained/tab mode and doesn't need the global data, remove the call (preferred; see Step 2)
2. **Hoist shared data** — fetch once in the parent page and pass down as props
3. **Request deduplication layer** — implement a short-lived cache keyed on `(endpoint, params)` that coalesces concurrent identical requests

### Pass criteria

- No endpoint is called more than once for the same parameters during a single page load
- Sibling components share data rather than independently fetching the same dataset

---

## Step 4 — Dead or vestigial endpoint calls (Pattern D)

### 4.1 Search for endpoints called from removed features

```bash
rg -n "getModuleSyllabus|getSyllabus" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

For each hit, verify:

- Is the endpoint's data actually consumed and displayed?
- Or is it a leftover from a previous page design that has since been refactored?

### 4.2 Backend endpoint usage audit

List all admin endpoints:

```bash
rg -n '\.get\(|\.post\(|\.put\(|\.delete\(' "$API_REPO/crates/api/src/routes/admin" --type rust | grep -oP '/v1/admin/[^"]+' | sort -u
```

Cross-reference with client commands:

```bash
rg -n '/v1/admin/' "$CLIENT_REPO/src/commands" | grep -oP '/v1/admin/[^"]+' | sort -u
```

Then cross-reference with frontend usage:

```bash
rg -n "learningCommands\.\w+|contentCommands\.\w+|examCommands\.\w+" "$ADMIN_REPO/src" "$WEB_REPO/src" | grep -oP '\.\w+\(' | sort | uniq -c | sort -rn
```

Flag any client commands that are not called from any frontend, or any endpoints that only serve dead code paths.

### Pass criteria

- Every API call made from a page serves data that is rendered or used for logic
- No vestigial calls from previous page designs

---

## Step 5 — Exhaustive pagination on page load (Pattern E)

### 5.1 Find paginated-exhaust patterns

```bash
rg -n "listAll|fetchAll|while.*hasMore|nextCursor|offset.*\+=|page.*\+\+" "$ADMIN_REPO/src" "$CLIENT_REPO/src"
```

Also check for helper utilities:

```bash
rg -n "async function listAll|async function fetchAll|exhaust" "$ADMIN_REPO/src/lib/utils" "$CLIENT_REPO/src/utils"
```

### 5.2 Assess whether exhaustive fetch is justified

For each hit:

- Is the full dataset actually needed, or only a subset?
- Does the dataset grow over time? (If so, the request count is unbounded)
- Is this running on page load or only on explicit user action?

### 5.3 Fix pattern

- If only a count or summary is needed, add it to the parent endpoint
- If only a filtered subset is needed, use server-side filtering with a reasonable page size
- If the full dataset is truly needed (rare), fetch lazily on tab activation — never on page load
- Add a `limit` guard to prevent runaway pagination

### Pass criteria

- No unbounded paginated-exhaust calls on page load
- Exhaustive fetches are only used when genuinely necessary and are deferred to user action

---

## Step 6 — Client-side N+1 fan-out (Pattern F)

### 6.1 Find per-item fetch loops

```bash
rg -n "for.*of.*\{[\s\S]{0,300}await|\.map\(async|Promise\.all\(.*\.map\(" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

### 6.2 Assess each hit

- Is the loop making one API call per item in a list?
- Could a batch/list endpoint serve the same data in one call?
- Does the number of iterations scale with data volume?

There are two distinct sub-patterns to watch for:

**Sub-pattern F1: Per-item API call to get a count or summary**

The list endpoint doesn't include a count field, so the frontend calls another endpoint per row just to read `.total`:

```typescript
// BAD: one request per edition to get document count
const editions = await listEditions(fetch, token, query);
const countEntries = await Promise.all(
  editions.data.map(async (edition) => {
    const docs = await listDocumentsAdmin(fetch, token, {
      limit: 1, offset: 0,
      exam_edition_id: edition.examEditionId
    });
    return [edition.examEditionId, docs.total] as const;
  })
);
```

**Fix:** Add the count as a subquery on the list endpoint's SQL (see Step 9).

**Sub-pattern F2: Per-item API call to resolve detail data**

The list endpoint returns only an ID for a related entity, so the frontend fetches the full entity per item:

```typescript
// BAD: one request per document to get title/kind
const uniqueDocumentIds = [...new Set(items.map(i => i.documentId))];
const details = await Promise.all(
  uniqueDocumentIds.map(async (id) => {
    const doc = await getDocumentForAdmin(id, fetch, token);
    return [id, { title: doc.title, kind: doc.kind }] as const;
  })
);
```

**Fix:** JOIN the related table in the list endpoint query and include the needed fields on the DTO (see Step 9).

### 6.3 Broader audit: global fetches that are N+1 in disguise (Pattern F + H)

Sometimes the N+1 isn't a loop per item but a single global fetch used to build a lookup map. While this avoids N requests, it still transfers an entire dataset just to resolve labels for a handful of IDs:

```typescript
// BAD: fetch ALL schedules just to map schedule_id → label for ~10 editions
const [editions, allSchedules] = await Promise.all([
  listEditionsAdmin(fetch, token, query),
  listAllExamSchedulesAdmin(fetch, token)  // paginated exhaust, all schedules ever
]);
const labelById = new Map(allSchedules.map(s => [s.id, getDisplayLabel(s)]));
```

This combines Pattern E (paginated exhaust) with Pattern H (missing supplementary data). The fix is the same: include the label data on the list endpoint response.

### 6.4 Known instances

| Component | List endpoint | N+1 / global lookup endpoint | Per-item field needed | Fix |
|-----------|--------------|------------------------------|----------------------|-----|
| `ExamEditionsList` | `listExamEditionsAdmin` | `listExamDocumentsAdmin` × N editions | `documentCount` | Add subquery to editions list SQL |
| `ExamEditionsList` | `listExamEditionsAdmin` | `listAllExamSchedulesAdmin` (paginated exhaust) | schedule label (dates/on-demand) | JOIN `exams.exam_schedule` in editions list SQL |
| `ExamDocumentsInlineList` | `listExamDocumentsAdmin` | `getDocumentForAdmin` × N documents | `documentTitle`, `documentKind` | JOIN `content.document` in exam documents list SQL |
| `SectionsList` (page mode) | `getModulesAdmin` | `getModuleSyllabusAdmin` × N modules | sections list | Consider a bulk sections endpoint |
| `AreasList` (page mode) | `getModulesAdmin` | `getModuleSyllabusAdmin` × N modules | sections list | Consider a bulk sections endpoint |
| `OutcomesList` (page mode) | `getModulesAdmin` | `getModuleSyllabusAdmin` × N modules | outcomes/sections | Consider a bulk outcomes endpoint |
| `ActivityDetailPage` | QA target IDs | `getQaItemForAdmin` × N targets | QA item title/status | Consider batch QA item endpoint |

### 6.5 Fix pattern

1. **Add count/summary fields to the list DTO** so the frontend doesn't need per-item requests (see Step 9)
2. **JOIN related tables** in the list query and include label fields on the DTO
3. **Create a batch endpoint** if per-item data cannot be summarised as a count
4. **Defer to user action** — only fetch per-item data when a row is expanded or clicked

### Pass criteria

- No loops making one API call per list item on page load
- No paginated-exhaust global fetches used purely for ID → label lookups
- Per-item data is either included in the list response or fetched lazily on interaction

---

## Step 7 — Runtime network verification

For representative pages (detail views with tabs, list views with filters):

1. Open page with Network panel filtered to XHR/Fetch
2. Record requests on initial page load
3. Click through each tab and record incremental requests
4. Verify no duplicate calls, no global fetches in constrained contexts

### Checklist per page

- [ ] Initial load: only requests needed for first paint (detail endpoint + active tab data)
- [ ] Each tab click: at most one scoped data fetch for that tab's content
- [ ] No `modules`, `pathways`, `schedules`, or similar global endpoints on a detail page load
- [ ] No paginated-exhaust patterns on load
- [ ] No per-item fan-out on load (no `Promise.all(items.map(async ...))` in data loaders)
- [ ] No global dataset fetches used purely for ID → label lookups (Pattern H)
- [ ] Filter dropdown data is not fetched until the dropdown is opened
- [ ] RelationSelector backing data in tabs is deferred by lazy-mount
- [ ] Tab-mounted list components do not re-fetch when switching to a different tab (Pattern J)
- [ ] Each page's `useAuthenticatedData` fires exactly one fetch on load, not two (Pattern K)

### Capture baseline metrics

```md
| Page                | Initial requests | Bytes transferred | Tab-switch requests |
|---------------------|-----------------|-------------------|---------------------|
| Module detail       |                 |                   |                     |
| Question detail     |                 |                   |                     |
| Bundle detail       |                 |                   |                     |
| ...                 |                 |                   |                     |
```

---

## Step 8 — Lazy-load filter dropdowns and selector backing data (Pattern G)

Filter dropdowns and RelationSelector components commonly fetch their options eagerly in the parent's data loader. When those dropdowns are hidden (constrained/tab mode) or never opened, the fetches are pure waste. This step extends Pattern B's fix into a repeatable component-level solution.

### 8.1 Audit filter dropdown data sources

For each list component, trace where filter dropdown items come from:

```bash
rg -n "items=\{|groups=\{|modules=\{" "$ADMIN_REPO/src/lib/lists" --type svelte
```

For each hit, answer:

- Does the data come from the main `useAuthenticatedData` fetch?
- Is it a global dataset (all pathways, all modules, all schedules)?
- Is the dropdown hidden in constrained/tab mode?

Common waste pattern:

```typescript
// BAD: filter dropdown data fetched eagerly in data loader
const pageData = useAuthenticatedData(async (fetch, token) => {
  const [pathways, modules, sections] = await Promise.all([
    getPathwaysAdmin(fetch, token),    // only for dropdown
    getModulesAdmin(fetch, token),     // only for dropdown
    getSectionsForModule(moduleId, fetch, token)  // actual list data
  ]);
  return { pathways, modules, sections };
});

// Template: dropdowns hidden in tab mode, but data still fetched
{#if !isConstrained}
  <Select items={pathwayItems} ... />
  <Select items={moduleItems} ... />
{/if}
```

### 8.2 Fix pattern: Select `loadItems` / `loadGroups`

The Underlay `Select` component supports lazy loading via `loadItems` and `loadGroups` props. Options are fetched on first dropdown open, cached for the session, and invalidated via `loadKey` for cascading filters.

**Before (eager):**

```svelte
<!-- Parent data fetch includes pathways + modules -->
<Select items={pathwayItems} ... />
<Select items={moduleItems} ... />
```

**After (lazy):**

```svelte
<script>
  // Remove pathways/modules from useAuthenticatedData entirely

  async function loadPathwayItems() {
    const token = auth.getToken();
    if (!token) return [];
    const pathways = await getPathwaysAdmin(fetch, token);
    return [
      { value: "All", label: "All pathways" },
      ...pathways.map(p => ({ value: p.pathwayId, label: p.name }))
    ];
  }

  async function loadModuleItems() {
    const token = auth.getToken();
    if (!token) return [];
    const modules = await getModulesAdmin(fetch, token);
    const filtered = selectedPathwayId === "All"
      ? modules
      : modules.filter(m => m.pathwayId === selectedPathwayId);
    return [
      { value: "All", label: "All modules" },
      ...filtered.map(m => ({ value: m.moduleId, label: m.code }))
    ];
  }
</script>

<Select loadItems={loadPathwayItems} placeholder="All pathways" ... />
<Select
  loadItems={loadModuleItems}
  loadKey={selectedPathwayId}
  placeholder="All modules"
  ...
/>
```

Key `Select` lazy-load props:

| Prop | Type | Purpose |
|------|------|---------|
| `loadItems` | `() => Promise<SelectItem[]>` | Async function called on first open; replaces `items` prop |
| `loadGroups` | `() => Promise<SelectGroup[]>` | Async function for grouped options; replaces `groups` prop |
| `valueLabel` | `string` | Label to display for current value before items load |
| `loadKey` | `string` | When this value changes, cached items are invalidated and re-fetched on next open (cascading filters) |

Behaviour:

- Items are fetched on **first open**, not on mount
- Loading and error states render inside the dropdown viewport
- If the dropdown is hidden (`{#if !isConstrained}`) or never opened, **no fetch occurs**
- Cached results persist until `loadKey` changes or the component unmounts

### 8.3 Cascading filter invalidation

When a parent filter changes (e.g. pathway changes, modules need to refresh), pass the parent value as `loadKey`:

```svelte
<!-- Module dropdown refreshes when pathway selection changes -->
<Select loadItems={loadModuleItems} loadKey={selectedPathwayId} ... />
```

On `loadKey` change:

1. Cached items are discarded
2. Load state resets to `idle`
3. Next open triggers a fresh `loadItems` call with the new context

For the child filter value, reset it to "All" when the parent changes rather than trying to validate the old selection against an eagerly-loaded list:

```typescript
let lastPathwayId = $state("All");
$effect(() => {
  if (selectedPathwayId !== lastPathwayId) {
    lastPathwayId = selectedPathwayId;
    if (selectedModuleId !== "All") {
      selectedModuleId = "All";
    }
  }
});
```

### 8.4 RelationSelector and drilldown backing data

RelationSelector drilldowns use a **different loading strategy** from filter dropdowns and generally do not need `loadItems`-style lazy loading. However, they are susceptible to Pattern B waste.

**How drilldowns work:**

- Each drilldown level calls a `suggestions` function when entered, receiving prior selections as context
- Most implementations use `createLocalDrillDownSearchFns` which filters **pre-loaded in-memory data**
- The backing data is typically fetched eagerly via `useAuthenticatedData` when the containing component mounts

**Where the waste occurs:**

The containing component (e.g. an outcomes selector in a form) fetches all backing data on mount even if the user never opens the selector. On a form page this is usually acceptable — the selector is the primary purpose of the page. But watch for:

1. **Selectors in tab panels** — if a RelationSelector is inside a tab that may never be activated, the backing data fetch is waste. The lazy-mount tab fix (Pattern A) already prevents this.

2. **Selectors in collapsible/optional form sections** — if a form section with a RelationSelector is collapsed by default, consider deferring the backing data load until the section is expanded.

3. **Heavy backing data** — if a selector loads thousands of items into memory for client-side filtering, consider switching from `createLocalDrillDownSearchFns` to a remote search function that queries the API with the user's search term and drill-down context.

**Drilldown vs Select lazy loading — when to use which:**

| Component | Data depends on prior selection? | Use `loadItems`? | Use drilldown search? |
|-----------|----------------------------------|-------------------|-----------------------|
| Filter dropdown (Select) | No, or only on parent filter | Yes | No |
| Cascading filter (Select) | Yes, on parent filter value | Yes + `loadKey` | No |
| RelationSelector drill-down | Yes, on prior level selections | No | Yes — local or remote |
| RelationSelector final level | Yes, on full drill-down path | No | Yes — `suggestions` fn |

### 8.5 Audit checklist

For each list component and form:

```bash
# Find filter data included in data loaders
rg -n "pathways|modules|schedules" "$ADMIN_REPO/src/lib/lists" --type svelte -A3 | grep -E "useAuthenticatedData|Promise\.all"

# Find RelationSelectors with eager data loading
rg -n "RelationSelector" "$ADMIN_REPO/src" --type svelte -l | xargs rg -n "useAuthenticatedData"
```

### Pass criteria

- Filter dropdown data is **not** included in `useAuthenticatedData` callbacks
- Dropdowns use `loadItems` / `loadGroups` and only fetch on first open
- Cascading filters use `loadKey` for cache invalidation
- RelationSelector backing data in tab panels is protected by lazy-mount (Pattern A)
- RelationSelector components with very large datasets use remote search instead of client-side filtering

---

## Step 9 — Missing supplementary data on list endpoints (Pattern H)

This is the **backend counterpart** to Patterns F and E. When a list endpoint returns only raw IDs for related entities — with no JOINed labels, no subquery counts — the frontend is forced into N+1 fan-out or global-fetch-and-lookup patterns to display basic information like names, dates, and counts.

### 9.1 Why this pattern is critical to prevent

This is the single most wasteful pattern in the codebase. A list of 30 editions with per-item document count lookups produces **30 additional API requests**. Combined with global schedule/module lookups, a single list page can produce **32+ requests** instead of 1. This scales with data volume and compounds with pagination — every page turn re-triggers the fan-out.

**This pattern is never acceptable in production.** List endpoints must return all the data needed to render their list cards.

### 9.2 Audit list endpoints

For every admin list endpoint in the API, verify:

1. **Does the frontend fetch additional data per row?** Check the frontend `useAuthenticatedData` callback for `Promise.all(response.data.map(...))` patterns.
2. **Does the frontend fetch a global dataset to build a lookup map?** Check for parallel fetches of ALL modules, ALL schedules, ALL pathways alongside the list query.
3. **Does the list card component make its own API call?** Check card components for `useAuthenticatedData` or `$effect` with fetch logic.

```bash
# Find per-item fan-out in data loaders
rg -n "response\.data\.map\(async|\.data\.map\(async" "$ADMIN_REPO/src"

# Find global lookup fetches alongside list queries
rg -n "Promise\.all\(\[" "$ADMIN_REPO/src/lib/lists" --type svelte -A10 | grep -E "getModulesAdmin|getPathwaysAdmin|listAll"

# Find card components that fetch their own data
rg -n "useAuthenticatedData" "$ADMIN_REPO/src/lib/cards" --type svelte
```

### 9.3 Fix pattern: enrich the list endpoint

The fix is always the same: **move the data to the SQL query**.

**Counts:** Add a correlated subquery:

```sql
-- Add document count to exam editions list
SELECT
    e.id,
    e.module_id,
    m.code AS module_code,
    -- ... existing columns ...
    COALESCE((
        SELECT COUNT(*)
        FROM exams.exam_document ed
        WHERE ed.exam_edition_id = e.id
          AND ed.deleted_at IS NULL
    ), 0) AS document_count
FROM exams.exam_edition e
LEFT JOIN learning.module m ON m.id = e.module_id
```

**Labels from related tables:** Add a JOIN and select the needed columns:

```sql
-- Add schedule info to exam editions list
SELECT
    e.id,
    es.start_date AS schedule_start_date,
    es.end_date AS schedule_end_date,
    es.is_on_demand AS schedule_is_on_demand,
    -- ... existing columns ...
FROM exams.exam_edition e
LEFT JOIN exams.exam_schedule es ON es.id = e.exam_schedule_id
```

**Update the DTO:** Add the new fields to the Rust response struct and the TypeScript type:

```rust
// farmyard: ExamEditionDetailRow
pub struct ExamEditionDetailRow {
    // ... existing fields ...
    pub document_count: i64,
    pub schedule_start_date: Option<NaiveDate>,
    pub schedule_end_date: Option<NaiveDate>,
    pub schedule_is_on_demand: bool,
}
```

```typescript
// cattle-grid: ExamEdition type
export interface ExamEdition {
  // ... existing fields ...
  documentCount: number;
  scheduleStartDate?: string | null;
  scheduleEndDate?: string | null;
  scheduleIsOnDemand: boolean;
}
```

**Remove frontend workarounds:** Once the list endpoint returns the enriched data:

1. Delete the `Promise.all(response.data.map(...))` fan-out
2. Delete the global lookup fetch (e.g. `listAllExamSchedulesAdmin`)
3. Delete the lookup map construction
4. Read the supplementary data directly from the list response

### 9.4 Checklist: what every list endpoint should include

| Data need | Wrong approach | Correct approach |
|-----------|---------------|-----------------|
| Count of child entities | Client fetches children with `limit: 1` per item | Correlated subquery `COUNT(*)` in list SQL |
| Related entity label | Client fetches ALL related entities globally | `LEFT JOIN` in list SQL, select label columns |
| Related entity status | Client fetches entity per item | `LEFT JOIN` in list SQL, select status column |
| Computed display value | Client computes from fetched global data | Compute in SQL or return raw fields for client formatting |

### 9.5 When secondary queries are justified

N+1 patterns are sometimes acceptable on **detail pages** (single entity) for **optional/expandable sections**:

- Detail page inline lists (e.g. documents within an edition detail) — these fetch on mount of the detail page which loads one entity, not N
- Lazy-mounted tab content that loads on activation — deferred, not N+1
- User-initiated expand/drill-down — fetched on interaction

They are **never acceptable** on:

- List views — every row triggers a fetch
- Data loaders that run on page load — no user interaction to justify
- Paginated lists — re-triggers on every page turn

### Pass criteria

- Every list endpoint returns all data needed to render its list cards without supplementary fetches
- No `Promise.all(items.map(async ...))` patterns in list component data loaders
- No global dataset fetches (`listAll*`, `getModulesAdmin`, etc.) used purely for ID → label resolution in list views
- Related entity labels and child counts are available directly on the list response DTO

---

## Step 10 — Profile contract and projection hygiene (Pattern I)

A single route/DTO shape serving list cards, filter dropdowns, and detail views without explicit profile selection creates ambiguity and waste. The fix is canonical resource routes with typed projection profiles.

### 10.1 Canonical contract

| Context | Route pattern | Profile | Expected payload shape |
|---------|---------------|---------|------------------------|
| List cards/tables | `GET /v1/admin/{domain}/{resource}` | `profile=list` | Display fields, labels, counts, status needed by the list |
| Filter dropdowns | `GET /v1/admin/{domain}/{resource}` | `profile=filter` | Minimal selector shape (id + label + tiny metadata only) |
| Detail with badges | `GET /v1/admin/{domain}/{resource}/{id}` | `profile=details` | Base entity + detail enrichments (including tab badge counts) |

### 10.2 Audit existing routes and endpoints

```bash
# Deprecated projection path naming
rg -n '"/v1/admin/.+(-for-list|-for-filter|paginated|with-counts)' "$API_REPO/crates/api/src/routes" --type rust

# Profile support in API/client/frontend
rg -n 'profile|ListProfile|DetailProfile' "$API_REPO/crates/api/src" "$CLIENT_REPO/src" "$ADMIN_REPO/src" "$WEB_REPO/src"

# Filter dropdowns that may still consume list profile data
rg -n 'loadItems|loadGroups|getModulesAdmin|getPathwaysAdmin|list.*ForList' "$ADMIN_REPO/src/lib" --type svelte
```

For each hit, classify:

1. Filter selector should use `profile=filter`
2. List view should use `profile=list`
3. Detail with tab badges should use `profile=details`

### 10.3 Fix pattern

**Phase 1 — Backend (Farmyard):**

- Keep canonical resource route paths.
- Add typed profile enum/query parsing and route to profile-specific SQL projection.
- Ensure detail `profile=details` includes all badge count subqueries required by the detail tabs.

**Phase 2 — Client (Cattle Grid):**

- Keep canonical resource command names.
- Add typed `profile` parameter to commands.
- Reuse shared pagination/query helpers for list profiles.

**Phase 3 — Frontend (Dairy/Cream):**

- List components call canonical list route with `profile=list`.
- Filter dropdowns lazy-load with `profile=filter`.
- Detail pages with badge tabs use detail fetch with `profile=details`.
- Remove supplementary count-only calls and route-name-specific wrappers.

### Pass criteria

- Canonical resource paths are used (no `-for-list`, `-for-filter`, `/paginated`, `with-counts`)
- List/filter/detail contexts use explicit profile selection
- Filter payloads are minimal and no longer receive list-count fields
- Detail badge counts come from main detail response (`profile=details`)
- No single unprofiled endpoint payload is forced to satisfy incompatible consumer contexts

---

## Step 11 — Unguarded queryKey refetch in tab-mounted components (Pattern J)

When a list component is embedded inside a `DetailPageShell` tab, it mounts on first tab visit and **stays mounted** thereafter (lazy-mount keeps tabs alive). If that component's URL-change `$effect` calls `refetch()` without comparing the previous value, every subsequent URL change — including tab switches and sibling tab filter changes — triggers a spurious API call.

### 11.1 Why this happens

`DetailPageShell` lazy-mounts tab content via `{#if isTabMounted(tab.value)}`. Once mounted, the component's Svelte 5 `$effect` hooks remain active even when the tab is hidden (bits-ui applies the `hidden` attribute but keeps the DOM and reactivity intact). Any `$effect` that reads `$page.url.searchParams` (directly or via a `$derived`) will re-evaluate on every URL change.

The bug pattern:

```typescript
// BAD: calls refetch() on every effect run, including mount
const queryKey = $derived(dataSearchParams($page.url.searchParams).toString());
$effect(() => {
    queryKey;
    if ($currentUser) {
      void pageData.refetch();
    }
});
```

Problems:
1. **Double-fetch on mount:** `tryFetch()` handles initial load, but this effect also fires on mount and calls `refetch()`.
2. **Spurious refetch on tab switch:** When the user switches tabs, `$page.url.searchParams` changes (the `tab` param changes). Even though `dataSearchParams` strips `tab`, the effect may still re-run.
3. **Cross-tab param pollution:** If a sibling tab writes filter/sort params to the URL (e.g. PreSeenReleasesList writes `sort`, `status`), this component's `queryKey` picks up those params and triggers a refetch with irrelevant filter state.

### 11.2 Detect

```bash
# Find $effect blocks that call refetch() without a previous-value guard
rg -n "refetch\(\)" "$ADMIN_REPO/src/lib/lists" --type svelte -B5 | grep -E "queryKey|\$effect"

# Find $derived queryKey patterns
rg -n "queryKey.*=.*\$derived" "$ADMIN_REPO/src/lib/lists" --type svelte

# Find $effect blocks reading $page.url.searchParams that call refetch
rg -n "\$page\.url\.searchParams" "$ADMIN_REPO/src/lib/lists" --type svelte -A3 | grep -E "refetch"
```

### 11.3 Fix pattern: `queryKey` option (preferred)

Use the built-in `queryKey` option on `useAuthenticatedData`. It internally tracks the previous key value, sets the initial key after the first successful fetch, and only calls `refetch()` when the key genuinely changes. Combined with global `configureAuth()` providing `getAuthLoading`/`getCurrentUser`, this eliminates all manual `$effect` boilerplate.

**Before (buggy):**

```typescript
const queryKey = $derived(dataSearchParams($page.url.searchParams).toString());
$effect(() => {
    queryKey;
    if ($currentUser) {
      void pageData.refetch();
    }
});
```

**After (using `queryKey` option):**

```typescript
const pageData = useAuthenticatedData(
    async (fetchFn, token) => { /* ... */ },
    {
      defaultValue: { data: [], total: 0 },
      queryKey: () => dataSearchParams($page.url.searchParams).toString()
    }
);
// No $effect needed — auto-fetch and queryKey watching are handled internally
```

Prerequisites:
- `configureAuth()` in the app layout must include `getAuthLoading` and `getCurrentUser` getters (enables auto-fetch without manual `tryFetch` `$effect`)
- The `queryKey` getter must strip UI-only params (use `dataSearchParams()` to remove `tab`, etc.)

Key properties:
- **No double-fetch on mount:** The internal `_previousQueryKey` is `null` until `onSuccess` fires, so the queryKey `$effect` won't fire until after the initial fetch.
- **No spurious refetch on tab switch:** `dataSearchParams` strips the `tab` param, and the internal comparison guard ensures refetch only fires when the remaining params genuinely change.
- **Cross-tab pollution still possible** if sibling tabs write shared URL params — but at least the component only refetches when those params change, not on every tab switch. For full isolation, consider migrating tab-mode components to local state instead of URL params.

#### Manual fallback (when `queryKey` option isn't suitable)

If you need custom refetch logic beyond what `queryKey` provides, use the manual `previousQueryKey` comparison guard pattern. See the git history for `ExamEditionsList.svelte` at `dairy@447371b` for the full before/after.

### 11.4 Known instances

| Component | Tab mode? | Status |
|-----------|-----------|--------|
| `ExamEditionsList` | Yes (module detail) | Fixed — uses `queryKey` option |
| `ExamSchedulesList` | Yes (pathway detail) | Fixed — uses `queryKey` option |
| `MockExamsList` | No (page only) | Fixed — uses `queryKey` option |
| `PreSeenReleasesList` | Yes (module detail) | Fixed — uses `queryKey` option |
| `PathwaysList` | No (page only) | Fixed — uses `queryKey` option |
| `BundlesList` | No (page only) | Fixed — uses `queryKey` option |
| `SectionsList` | Yes (module detail) | Not affected — uses local state, no URL-param refetch |
| `AreasList` | Yes (module detail) | Not affected — uses local state, no URL-param refetch |
| `SyllabusUpdatesList` | Yes (module detail) | Not affected — no URL-param refetch |
| `ActivitiesList` | Yes (various) | Not affected — uses local state, no URL-param refetch |
| `SummariesList` | No (page only) | Minor — `pagination.reset()` on mount, no tab impact |
| `DocumentsList` | No (page only) | Minor — `pagination.reset()` on mount, no tab impact |
| `AudiosList` | No (page only) | Minor — `pagination.reset()` on mount, no tab impact |
| `VideosList` | No (page only) | Minor — `pagination.reset()` on mount, no tab impact |
| QA page (`content/qa`) | No (page only) | Minor — `pagination.reset()` on mount, no tab impact |

### Pass criteria

- No `$effect` calls `refetch()` without comparing against a previous value
- Tab-mounted list components do not re-fetch when switching to a different tab
- Tab-mounted list components do not re-fetch when a sibling tab changes its filter/sort params
- Initial mount triggers exactly one fetch (via `tryFetch`), not two

---

## Step 12 — Redundant manual `tryFetch` alongside global auto-fetch (Pattern K)

When `configureAuth()` provides `getAuthLoading` and `getCurrentUser` getters, `useAuthenticatedData` creates an internal `$effect` that calls `tryFetch()` automatically when auth becomes ready. If a page **also** passes `getToken` in its options and has a manual `$effect(() => { pageData.tryFetch($authLoading, $currentUser); })`, both effects fire on auth readiness, causing the fetch to run twice.

### 12.1 Why this happens

The auto-fetch feature was added to `useAuthenticatedData` after ~100 pages already used the manual pattern. When the global `configureAuth()` in the app layout was updated with `getAuthLoading`/`getCurrentUser`, all existing pages gained the internal auto-fetch `$effect` — but the old manual `$effect` was not removed. Both effects react to the same auth readiness signals and race each other, resulting in two identical API calls on every page load.

The explicit `getToken` option is also redundant when `configureAuth()` provides `getToken` globally — the `useAuthenticatedData` internals resolve getToken from global config when no option-level override is present.

### 12.2 Detect

```bash
# Find pages with explicit getToken AND manual tryFetch (the double-fetch pattern)
rg -l "getToken.*auth\.getToken" "$ADMIN_REPO/src/routes" | xargs rg -l "tryFetch\(\\\$authLoading"

# Count total instances
rg -c "pageData\.tryFetch\(\\\$authLoading" "$ADMIN_REPO/src/routes"
```

### 12.3 Fix pattern

Remove both the explicit `getToken` option and the manual `tryFetch` `$effect`. The global auto-fetch handles everything.

**Before (double-fetch):**

```typescript
import { auth, authLoading, currentUser } from "$lib/stores/auth";

const pageData = useAuthenticatedData(
    async (fetch, token) => { /* ... */ },
    {
      getToken: () => auth.getToken()
    }
);

$effect(() => {
    pageData.tryFetch($authLoading, $currentUser);
});
```

**After (single fetch):**

```typescript
import { auth } from "$lib/stores/auth";

const pageData = useAuthenticatedData(
    async (fetch, token) => { /* ... */ },
    {}
);
// No $effect needed — auto-fetch is handled internally via configureAuth()
```

Notes:
- Keep `import { auth }` if the page uses `auth.getToken()` elsewhere (e.g. for mutation calls)
- Remove `authLoading` and `currentUser` from the import unless used elsewhere
- If the `useAuthenticatedData` options only contained `getToken`, replace with `{}`
- If the options also contain `defaultValue`, `queryKey`, etc., just remove the `getToken` property

### 12.4 Import cleanup

After removing the manual `$effect`, the `authLoading` and `currentUser` store imports become unused on most pages. Clean up:

```typescript
// Before
import { auth, authLoading, currentUser } from "$lib/stores/auth";

// After (if auth is still needed for mutations)
import { auth } from "$lib/stores/auth";

// After (if auth is not needed at all)
// Remove the import entirely
```

### Pass criteria

- No page has both `getToken` in options AND a manual `tryFetch` `$effect`
- No page has a manual `$effect` calling `tryFetch($authLoading, $currentUser)` when global auto-fetch is active
- Each `useAuthenticatedData` triggers exactly one fetch on page load
- `authLoading` and `currentUser` are not imported unless used for purposes other than the tryFetch `$effect`

---

## Severity rubric

- `critical`: All tabs mount simultaneously causing cascade of fetches on every detail page load / N+1 fan-out on list views (scales with data volume)
- `high`: Global dataset fetch on constrained page / exhaustive pagination on load / missing supplementary data on list DTO requiring global lookups / eager filter data for hidden dropdowns / one-size-fits-all DTO transferring unnecessary data to filter dropdowns / unguarded queryKey refetch causing repeated API calls on every tab switch (Pattern J) / redundant manual tryFetch causing double-fetch on every page load (Pattern K)
- `medium`: Duplicate identical requests across siblings / eager filter data for visible but unopened dropdowns / eager RelationSelector backing data in optional sections
- `low`: Vestigial endpoint call with minimal payload
- `note`: Optimization opportunity with limited current impact

---

## Findings template

```md
### [SEVERITY] Wasteful endpoint call - <component/page>

- **Pattern:** A / B / C / D / E / F / G / H / I / J / K
- **Location:** `src/...`
- **Current behavior:** (what gets called, when, and how many requests)
- **Observed cost:** (request count, payload size, latency impact)
- **Expected behavior:** (what should happen instead)
- **Recommended fix:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Wasteful endpoint calls sweep summary

- Pages/components audited: N
- Critical issues: N
- High issues: N
- Medium issues: N
- Low issues: N

## Impact estimate

- Initial-load requests reduced by: X (per detail page)
- Eliminated global dataset transfers: Y KB
- Deferred tab data fetches: Z endpoints
```

---

## Related docs

- [006-query-efficiency-sweep.md](./006-query-efficiency-sweep.md)
- [004-tab-count-badges-sweep.md](./004-tab-count-badges-sweep.md)
- [020-list-endpoint-for-detail-views-sweep.md](./020-list-endpoint-for-detail-views-sweep.md)
