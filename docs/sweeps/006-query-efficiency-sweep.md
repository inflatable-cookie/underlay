# 006 - Query Efficiency Sweep

This sweep finds avoidable network and database IO across Underlay-style API + client + frontend stacks.

It focuses on real-world inefficiencies that creep into detail and list pages:

- frontend fan-out request chains
- eager loading of relation lists that are not needed initially
- backend N+1 style data loading
- list endpoints returning more data than needed

## Scope

```bash
export API_REPO="/path/to/myapp-api"
export CLIENT_REPO="/path/to/myapp-client"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
```

Acowtancy mapping: `farmyard`, `cattle-grid`, `dairy`, `cream`.

---

## Step 1 - Identify frontend fan-out hotspots

### 1.1 Parallel fetch clusters in page load code

```bash
rg -n "Promise\.all\(\[|useAuthenticatedData\(" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
```

For each hit, capture:

- route path
- number of requests on initial load
- which requests are required for first paint
- which are only needed for non-default tabs/secondary panels

### 1.2 Same-entity repeated fetches

```bash
rg -n "map\(async|Promise\.all\(|get.*Admin\(" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
```

Flag patterns like:

- fetch list of IDs, then fetch each ID individually
- fetch parent, then repeatedly fetch related entities one-by-one

Pass criteria:

- initial page load only fetches data needed for first paint
- heavy relation data loads lazily when user opens relevant tab/section

---

## Step 2 - Check list endpoint usage discipline

### 2.1 Large unpaginated list calls

```bash
rg -n "limit:\s*(500|1000)|offset:\s*0|/paginated|PaginatedResponse" "$CLIENT_REPO/src/commands" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
```

Flag when pages routinely use large fixed limits or non-paginated endpoints for operational views.

### 2.2 List response over-fetching

```bash
rg -n "SELECT\s+\*|query_as::<_, .*Row>\(" "$API_REPO/crates/db/src"
```

Manual review targets:

- list queries selecting large blob/rich fields unnecessarily
- detail-level fields included in list payloads

Pass criteria:

- list endpoints are paginated where volume can grow
- list DTOs are intentionally thinner than detail DTOs

---

## Step 3 - Backend query efficiency review

### 3.1 N+1 query smell in Rust code

```bash
rg -n "for .* in .*\{[\s\S]{0,220}await" "$API_REPO/crates" -g "*.rs"
rg -n "map\(.*async|join_all|try_join_all" "$API_REPO/crates" -g "*.rs"
```

Review each candidate for:

- one query per item in a loop where a set-based query is possible
- repeated existence checks that can be batched

### 3.2 Count and relation metadata strategy

```bash
rg -n "\(SELECT COUNT\(\*\)" "$API_REPO/crates/db/src"
```

Guidance:

- detail pages should usually use count subqueries on the detail row
- list pages should avoid expensive per-row correlated counts unless measured and justified

Pass criteria:

- no accidental N+1 in hot endpoints
- count strategy matches endpoint shape (detail vs list)

---

## Step 4 - Endpoint shape vs frontend usage

### 4.1 Full-list fetch used only for count/badge

```bash
rg -n "\.length|reduce\(|count=\{" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
```

Cross-check with [004-tab-count-badges-sweep.md](./004-tab-count-badges-sweep.md).

### 4.2 Endpoint payload mismatch

```bash
rg -n "get.*Admin\(|list.*Admin\(" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
```

Flag when frontend calls a heavy detail/list endpoint but uses only tiny subset of fields.

Pass criteria:

- chosen endpoint shapes match view needs
- no expensive endpoint calls used only for one small datum

---

## Step 5 - Runtime network verification

For representative high-traffic pages:

1. open page with network panel enabled
2. measure request count + payload sizes for initial load
3. interact with secondary tabs and compare incremental requests

Capture baseline metrics:

- requests on initial load
- total transferred KB on initial load
- number of follow-up requests triggered by one tab switch

Pass criteria:

- initial request budget is intentional and stable
- secondary data loads are incremental, not preloaded blindly

---

## Step 6 - Correction playbook

When inefficiency is found, apply in this order:

1. **Frontend orchestration**
   - defer non-critical calls
   - lazy-load non-default tabs
2. **Client command/API selection**
   - prefer paginated/list-minimal endpoints for browsing views
3. **Backend query shape**
   - replace per-item queries with set-based queries
   - split list/detail DTOs where needed
4. **Count strategy**
   - move badge/summary counts into detail/list contract where appropriate
5. **Verification**
   - re-check request count and total transfer after changes

---

## Severity rubric

- `high`: clear N+1 or multi-request fan-out causing significant latency/load
- `medium`: avoidable over-fetching with moderate user or infra cost
- `low`: minor inefficiency with limited operational impact
- `note`: optimization opportunity, low urgency

---

## Findings template

```md
### [SEVERITY] Query efficiency issue - <page/endpoint>

- **Location:** `src/...` and/or `crates/db/src/...`
- **Current behavior:**
- **Observed cost:** (requests, payload, query count)
- **Expected pattern:**
- **Recommended change:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Query efficiency sweep summary

- Pages audited: N
- High issues: N
- Medium issues: N
- Low issues: N

## Efficiency impact estimate

- Initial-load requests reduced by: X
- Approx. transferred data reduced by: Y KB
- Backend query count reduction (key endpoints): Z
```

---

## Related docs

- [004-tab-count-badges-sweep.md](./004-tab-count-badges-sweep.md)
- [093-pagination.md](../guides/093-pagination.md)
- [080-typescript-client.md](../guides/080-typescript-client.md)
- [070-api-handlers.md](../guides/070-api-handlers.md)
