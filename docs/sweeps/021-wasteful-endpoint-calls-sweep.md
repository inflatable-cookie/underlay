# 021 - Wasteful Endpoint Calls Sweep

This sweep identifies pages and components that make unnecessary, redundant, or overly broad API requests. It covers several related anti-patterns that compound into severe performance degradation when left unchecked.

## Problem this sweep targets

Admin and consumer views accrue wasteful API calls through a small number of recurring anti-patterns. On a moderately complex detail page these can stack into **20+ simultaneous requests on page load** — the majority serving data the user never sees.

### Anti-pattern categories

| # | Pattern | Example | Impact |
|---|---------|---------|--------|
| **A** | All tabs mount simultaneously | bits-ui `TabsContent` renders hidden panels with full children | Every tab's data fetching runs on page load |
| **B** | Global dataset fetch in constrained context | Component fetches ALL modules/pathways when it already has a fixed `moduleId` prop | Transfers kilobytes of unused data |
| **C** | Duplicate identical requests | Two sibling tabs each call `getPathwaysAdmin()` | Same payload fetched N times in parallel |
| **D** | Dead or vestigial endpoint calls | Calling a `syllabus` endpoint for a page structure that was removed | Wasted request with no consumer |
| **E** | Exhaustive pagination in page load | Looping through all pages of a paginated endpoint (`limit=200`, repeat until done) | Unbounded request count, grows with data |
| **F** | N+1 client-side fan-out | Fetch a list, then make one request per item | Request count scales with data volume |

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
rg -n "TabsContent|TabPanel|TabsRoot" "$UI_LIB/src"
```

Review each tab container component:

- Does it render children of inactive tabs? (bits-ui default: yes — sets `hidden` attribute)
- Does it conditionally mount/unmount tab content?
- Does it support lazy-mount (render on first activation, keep mounted thereafter)?

### 1.2 Search for tab-based layouts in consuming apps

```bash
rg -n "DetailPageShell|TabsRoot|TabsContent" "$ADMIN_REPO/src" "$WEB_REPO/src" --type svelte
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
  <TabsContent value={tab.value}>
    {#if mountedTabs.has(tab.value)}
      {@render tabContent(tab.value)}
    {/if}
  </TabsContent>
{/each}
```

Key properties:

- Content only mounts when the tab is first activated
- Once mounted, content stays in the DOM (preserves scroll position, selection state, loaded data)
- Empty `TabsContent` wrapper still renders for bits-ui panel structure

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

When in constrained/tab mode, skip the global fetches entirely:

```typescript
// GOOD: only fetch what's needed for the constrained context
if (propModuleId) {
  // Tab mode: skip filter data, fetch only the scoped list
  const sections = await getSectionsAdmin(propModuleId, fetch, token);
  return { pathways: [], modules: [], sections };
}

// Page mode: fetch filter data for dropdowns
const [pathways, modules] = await Promise.all([
  getPathwaysAdmin(fetch, token),
  getModulesAdmin(fetch, token)
]);
// ... full page-mode logic
```

### 2.4 Downstream check: filter UI visibility

Verify that filter bars are actually hidden in tab mode. If the filter UI is hidden but the backing data is still fetched, the fetch is pure waste.

```bash
rg -n "isConstrained|variant.*tab|showFilters" "$ADMIN_REPO/src/lib/lists"
```

### Pass criteria

- Tab-mode components only fetch data relevant to their constrained context
- Global filter data (all pathways, all modules, etc.) is only fetched when filter UI is visible
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

Common pattern:

```typescript
// BAD: one request per edition to check for documents
const editions = await listEditions(moduleId, fetch, token);
for (const edition of editions) {
  const docs = await listDocuments(edition.editionId, fetch, token);
  edition.documentCount = docs.length;
}

// GOOD: include count on the list endpoint response
const editions = await listEditions(moduleId, fetch, token);
// editions[].documentCount already present
```

### 6.3 Fix pattern

1. **Add count/summary fields to the list DTO** so the frontend doesn't need per-item requests
2. **Create a batch endpoint** if per-item data cannot be summarised as a count
3. **Defer to user action** — only fetch per-item data when a row is expanded or clicked

### Pass criteria

- No loops making one API call per list item on page load
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
- [ ] No per-item fan-out on load

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

## Severity rubric

- `critical`: All tabs mount simultaneously causing cascade of fetches on every detail page load
- `high`: Global dataset fetch on constrained page / exhaustive pagination on load / N+1 fan-out
- `medium`: Duplicate identical requests across siblings / unnecessary filter data fetch
- `low`: Vestigial endpoint call with minimal payload
- `note`: Optimization opportunity with limited current impact

---

## Findings template

```md
### [SEVERITY] Wasteful endpoint call - <component/page>

- **Pattern:** A / B / C / D / E / F
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
