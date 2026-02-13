# 019 - Pagination Contract Consistency Sweep

This sweep verifies that pagination stays contract-consistent across backend routes, TypeScript client commands, and frontend list controllers.

It is designed to catch mixed pagination paradigms (cursor vs page/offset) that often create runtime bugs like missing `total_pages`, incorrect next/prev behavior, or brittle adapter layers.

## Problem this sweep targets

Common drift symptoms:

- backend endpoint returns cursor response but client/frontend expects `pagination.total_pages`
- client commands use local legacy pagination types instead of Underlay pagination contracts
- list pages convert controller params through temporary adapter helpers (`toLegacyPaginationParams`)
- app code loops pages manually when endpoint is cursor-based
- similar list pages use different pagination contracts and query keys

## Scope

Run this across API + client + frontend repos.

```bash
export API_REPO="/path/to/myapp-api"
export CLIENT_REPO="/path/to/myapp-client"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
export UNDERLAY_REPO="/path/to/underlay"
```

Acowtancy mapping: `farmyard`, `cattle-grid`, `dairy`, `cream`, `underlay`.

---

## Step 1 - Inventory paginated endpoints and response DTOs

```bash
rg -n "path = \"/v1/.*/paginated\"|PaginationQuery|PaginationParams|PaginatedResponseDto" "$API_REPO/crates/api/src"
```

Capture per endpoint:

- route path + method
- accepted query params (`limit`, `cursor`, `direction`, `includeTotal`)
- response envelope fields

Pass criteria:

- paginated endpoints expose a single, explicit pagination contract
- route handlers do not silently mix cursor and page-number styles

---

## Step 2 - Verify client command pagination types and helpers

```bash
rg -n "PaginationParams|PaginatedResponse|appendPaginationParams" "$CLIENT_REPO/src/commands" "$CLIENT_REPO/src/types"
rg -n "types/pagination-types|toLegacyPaginationParams|toUnderlayPaginatedResponse|total_pages|pagination\.page" "$CLIENT_REPO/src"
```

Pass criteria:

- client commands use Underlay pagination contracts directly
- command signatures and return types align with endpoint response shape
- no command-level references to legacy page metadata (`total_pages`, `page`, `sort_by`, `sort_order`) unless explicitly grandfathered

---

## Step 3 - Verify frontend list controller callsites

```bash
rg -n "createPaginationController|Pagination controller|persistKey" "$ADMIN_REPO/src" "$WEB_REPO/src"
rg -n "toLegacyPaginationParams|toUnderlayPaginatedResponse|pagination-adapter|total_pages|response\.pagination" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Review each paginated list page:

- does the controller pass `params` directly to the client command?
- does the fetcher return the command response as-is?
- are next/prev controls driven by `nextCursor`/`prevCursor`/`hasMore`?

Pass criteria:

- no adapter shims are required for normal list pages
- similar list pages follow the same pagination wiring pattern

---

## Step 4 - Query key parity across API, client, and frontend

```bash
rg -n "cursor|direction|includeTotal|limit|offset|page|total_pages" "$API_REPO/crates/api/src/routes" "$CLIENT_REPO/src/commands" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Manual review notes:

- `offset` can still be valid for non-cursor endpoints; only flag mismatches on cursor-paginated paths
- `page`-style keys must not appear in cursor endpoint call chains

Pass criteria:

- query keys used by frontend/client exactly match API parser expectations
- no mixed pagination keyset on one endpoint family

---

## Step 5 - Detect brittle page-loop implementations

```bash
rg -n "while \(true\)|page \+= 1|total_pages|hasMore|nextCursor|prevCursor" "$CLIENT_REPO/src" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Classify findings:

- valid: bounded loops over known offset APIs with explicit contract and tests
- violation: loops assuming page counts against cursor endpoints

Pass criteria:

- cursor endpoints use cursor traversal semantics, not page-number assumptions
- helper utilities for "fetch all" use endpoint-appropriate strategy

---

## Step 6 - Verify consistency with Underlay patterns

```bash
rg -n "PaginationParams|PaginatedResponse|appendPaginationParams|createPaginationController" "$UNDERLAY_REPO/docs/guides/093-pagination.md" "$UNDERLAY_REPO/docs/guides/080-typescript-client.md" "$UNDERLAY_REPO/docs/guides/100-frontend-web.md"
```

Pass criteria:

- project implementation aligns with documented Underlay pagination contract
- any deliberate exceptions are documented locally (with reason and exit plan)

---

## Step 7 - Verification commands

Run narrow checks first:

```bash
cd "$CLIENT_REPO" && bun check && bun lint
cd "$ADMIN_REPO" && bun check && bun lint
cd "$WEB_REPO" && bun check && bun lint
```

Optional backend check:

```bash
cd "$API_REPO" && cargo check -p api --all-features
```

---

## Correction playbook

When findings are detected, remediate in this order:

1. confirm backend endpoint contract (query params + response shape)
2. align client command types and pagination helper imports
3. remove frontend adapter shims and direct legacy conversions
4. normalize list fetchers to `createPaginationController` happy path
5. add/refresh tests and run type checks in client + consumers

If a legacy endpoint must remain temporarily:

- isolate compatibility logic to one explicit boundary
- add comments/tests describing sunset criteria
- track migration work in roadmap/issues

---

## Severity rubric

- `high`: runtime break risk (wrong envelope assumptions, broken navigation, crashes)
- `medium`: type/API mismatch forcing adapters or unsafe conversions
- `low`: inconsistency or naming drift without immediate user impact
- `note`: cleanup opportunity

---

## Findings template

```md
### [SEVERITY] Pagination contract drift - <endpoint/list>

- **API location:** `crates/api/src/routes/...`
- **Client location:** `src/commands/...`
- **Frontend location:** `src/lib/lists/...` or `src/routes/...`
- **Check step:** Step X
- **Observed mismatch:**
- **Impact:**
- **Fix plan:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Pagination contract sweep summary

- Endpoint families reviewed: N
- Client command mismatches: N
- Frontend adapter shims found: N
- Runtime-risk findings: N
- Normalized during sweep: N
```

---

## Related docs

- [093-pagination.md](../guides/093-pagination.md)
- [080-typescript-client.md](../guides/080-typescript-client.md)
- [100-frontend-web.md](../guides/100-frontend-web.md)
- [097-autonomous-list-components.md](../guides/097-autonomous-list-components.md)
- [005-api-client-contract-drift-sweep.md](./005-api-client-contract-drift-sweep.md)
- [003-frontend-consistency-sweep.md](./003-frontend-consistency-sweep.md)
