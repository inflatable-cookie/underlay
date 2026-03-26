# 004 - Tab Count Badges Sweep

This sweep ensures tab badge counts are computed at the backend detail-query layer, not by eager-fetching full relation lists in the frontend.

## Problem this sweep targets

A common drift pattern:

1. Detail page loads context item (for example, module/video/user)
2. Page also fetches one or more full relation lists immediately
3. Tab counts are computed from `list.length`

Result:

- unnecessary network IO
- slower time-to-interactive on detail pages
- duplicate count logic across frontend files
- count semantics can drift from backend filtering rules

## Target architecture

For each detail page with tab badges:

1. **DB detail query** returns context row + count subqueries
2. **Domain row/model** includes typed `*_count` fields
3. **API DTO** exposes these counts for the detail response
4. **TS client types/commands** carry the count fields
5. **Frontend tabs** use `entity.someCount`, not relation array lengths
6. Relation lists are loaded lazily when tab content opens

---

## Acowtancy reference patterns

### Good examples (use as template)

- `farmyard/crates/db/src/content/documents/queries/detail.rs`
  - single detail query includes `activity_count` and `exam_edition_count` via SQL subqueries
- `farmyard/crates/api/src/dto/content.rs`
  - `DocumentItemDto` exposes `activity_count` and `exam_edition_count`
- `dairy/src/routes/(app)/content/documents/[documentId]/+page.svelte`
  - badge uses `document.examEditionCount`
  - heavy relation tabs are lazy-loaded only when active

- `farmyard/crates/db/src/users/queries/users.rs`
  - `get_user_with_session_count` includes `active_session_count` and `activity_count`
- `farmyard/crates/api/src/dto/admin/users.rs`
  - `UserDetailResponse` exposes both count fields
- `dairy/src/routes/(app)/users/[userId]/+page.svelte`
  - badges use `user.activeSessionCount` and `user.activityCount`

### Smell signature example

- `dairy/src/routes/(app)/learning/modules/[moduleId]/+page.svelte`
  - initial `Promise.all` fetches multiple relation collections
  - badge counts are derived from relation lengths (`sections`, `bundles`, `preSeenReleases`, etc.)

This is exactly the pattern this sweep is meant to identify and correct.

---

## Prerequisites

```bash
export API_REPO="/path/to/myapp-api"
export CLIENT_REPO="/path/to/myapp-client"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
```

---

## Step 1 - Inventory tab badges

Find all tab triggers with counts:

```bash
rg -n "count:\\s*|items=\\{.*count|TabItem" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Create a worksheet row per detail page:

- page path
- tab value
- current badge expression
- entity type (module, video, user, etc.)

---

## Step 2 - Identify count-source anti-patterns

### 2.1 Count from relation lengths

```bash
rg -n "\.length" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
rg -n "const .*Count|\$derived\(" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
```

Flag when tab count is derived from:

- `items.length`
- nested reductions over loaded collections
- "loaded ? list.length : undefined" where list is fetched only for count

### 2.2 Eager relation fetches in initial detail load

```bash
rg -n "Promise\.all\(\[" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
rg -n "list.*\(|get.*(By|For).*(Admin|Detail)\(" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
```

For each detail page, flag if initial load fetches relation lists mainly to support badges.

---

## Step 3 - Verify backend count contract exists

For each flagged page, trace the detail endpoint and verify all required count fields exist end-to-end.

### 3.1 DB query includes count subqueries

```bash
rg -n "get_.*(detail|admin|by_id)|SELECT" "$API_REPO/crates/db/src"
rg -n "\(SELECT COUNT\(\*\)" "$API_REPO/crates/db/src"
```

Pass criteria:

- detail query for the context item includes one count expression per badge
- count filters match tab data semantics (`deleted_at IS NULL`, domain predicates, etc.)
- count columns are `COALESCE(..., 0)` where needed

### 3.2 DTO exposes count fields explicitly

```bash
rg -n "pub struct .*Dto|_count" "$API_REPO/crates/api/src/dto"
```

Pass criteria:

- detail DTO includes every badge count needed by the page
- naming is stable and predictable (`*_count` in API payload)

### 3.3 API route returns the count-bearing DTO

```bash
rg -n "path = \"/v1/admin/.+\{.+Id\}\"|SingleResponse" "$API_REPO/crates/api/src/routes"
```

Pass criteria:

- detail route uses the DTO that includes tab badge counts
- no extra endpoint calls required just to obtain count metadata

---

## Step 4 - Verify client and frontend consume count contract

### 4.1 TS client type includes count fields

```bash
rg -n "interface .*Detail|type .*Detail|Count" "$CLIENT_REPO/src"
```

Pass criteria:

- count fields are present in client detail type
- no frontends infer counts from arrays when typed count exists

### 4.2 Tabs use count fields from detail entity

```bash
rg -n "count:\\s*|items=\\{.*count|TabItem" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
```

Pass criteria:

- badges bind to detail fields (for example `entity.activityCount`)
- counts do not depend on relation lists loaded for tab content

### 4.3 Relation lists lazy-load by active tab

```bash
rg -n "activeTab|load.*\(|if \(activeTab ===" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
```

Pass criteria:

- relation tab content fetches occur only when tab becomes active (or are explicitly justified)
- opening detail page does not immediately fan out to every tab's data endpoint

---

## Step 5 - Network IO verification

For each audited page, validate behavior in browser network tools:

1. Load detail page on default tab
2. Confirm only context/detail request(s) fire initially
3. Confirm relation tab requests fire only after tab activation

Optional scripted check approach:

- add Playwright/Cypress route interception and assert no relation-list requests before tab click

---

## Step 6 - Correction playbook

When a page fails this sweep, apply this sequence.

### 6.1 Backend: add count-bearing detail query

Use single-row detail query with count subqueries:

```sql
SELECT
  e.id,
  e.title,
  COALESCE((SELECT COUNT(*) FROM child_a c WHERE c.parent_id = e.id AND c.deleted_at IS NULL), 0) AS child_a_count,
  COALESCE((SELECT COUNT(*) FROM child_b c WHERE c.parent_id = e.id AND c.deleted_at IS NULL), 0) AS child_b_count
FROM domain.entity e
WHERE e.id = $1
  AND e.deleted_at IS NULL;
```

Guidance:

- for detail pages (single row), correlated count subqueries are usually acceptable
- for list pages, avoid expensive per-row subquery explosion unless measured and accepted

### 6.2 API: add custom detail DTO for tab counts

- create/extend detail DTO with explicit count fields
- map from domain row to DTO
- keep naming stable (`*_count` in payload, camelCase in TS clients)

### 6.3 Frontend: remove count-from-list behavior

- replace `list.length` badge sources with detail count fields
- keep relation list fetches for tab content only
- lazy-load relation tab content on active tab switch

### 6.4 Tests

- backend: integration test asserting count correctness for known fixtures
- frontend: interaction test asserting no eager relation fetch before tab activation

---

## Severity rubric

- `high`: page eagerly fetches multiple heavy relation lists primarily for badge counts
- `medium`: one or two unnecessary relation fetches, measurable but limited impact
- `low`: minor count-source inconsistency with low IO impact
- `note`: already optimized but lacks tests/documentation

---

## Findings template

```md
### [SEVERITY] Tab badge count source drift - <entity/page>

- **Location:** `src/routes/.../+page.svelte`
- **Current count source:**
- **Current extra IO:**
- **Expected source:** detail DTO count fields
- **Backend changes needed:** query / DTO / route / client type
- **Frontend changes needed:** tab badge binding / lazy load behavior
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Tab count sweep summary

- Pages audited: N
- Optimized (detail-count based): N
- Needs backend count contract: N
- Needs frontend lazy-load cleanup: N

## Performance recommendation

- Immediate fixes:
- Next wave:
```

---

## Related docs

- [002-underlay-reuse-sweep.md](./002-underlay-reuse-sweep.md)
- [003-frontend-consistency-sweep.md](./003-frontend-consistency-sweep.md)
- [080-typescript-client.md](../guides/080-typescript-client.md)
- [090-ui-kit.md](../guides/090-ui-kit.md)
- [097-autonomous-list-components.md](../guides/097-autonomous-list-components.md)
- [070-api-handlers.md](../guides/070-api-handlers.md)
