# 022 - API Endpoint Naming Convention Sweep

This sweep verifies that API route naming and projection strategy stays consistent across Underlay-based apps.

## Problem This Sweep Targets

Naming drift causes recurring contract regressions:

- projection in path names (`-for-list`, `-for-filter`)
- mechanics in path names (`/paginated`, `with-counts`)
- inconsistent command naming and frontend endpoint assumptions
- alias buildup with no sunset

## Policy Baseline (Platform Standard)

See [073-api-profiles-and-query-contract.md](../guides/073-api-profiles-and-query-contract.md).

### Canonical routes

- List route: `GET /v1/{scope}/{domain}/{resource}`
- Detail route: `GET /v1/{scope}/{domain}/{resource}/{id}`

### Profile projections

- List profiles: `profile=list|filter`
- Detail enrichment profile: `profile=details` (default detail without profile is base record)

### Disallowed naming

- `/paginated`, `/cursor`, `/offset` path suffix conventions
- route tokens like `with-counts`, `with-joins`, `flat`
- projection path suffixes `-for-list`, `-for-filter` (post-migration target)

### Alias policy

- Prefer no aliases in active dev mode.
- If temporary aliases exist, track owner + removal date in roadmap.

## Scope

Run across API + client + consuming frontends.

```bash
export API_REPO="/path/to/myapp-api"
export CLIENT_REPO="/path/to/myapp-client"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
```

Acowtancy mapping: `farmyard`, `cattle-grid`, `dairy`, `cream`.

---

## Step 1 - Inventory all public route paths

```bash
rg -n 'path\\s*=\\s*"/v1/' "$API_REPO/crates/api/src/routes" --type rust
```

Build a route table including:

- method
- path
- handler
- domain/resource
- endpoint role (`list`, `detail`, `mutation`, `action`)

Pass criteria:

- each resource follows canonical list/detail path shapes
- non-CRUD operations are explicit action paths

---

## Step 2 - Detect disallowed naming tokens in API routes

```bash
rg -n 'path\\s*=\\s*"/v1/.+(paginated|with-counts|with-joins|flat|-for-list|-for-filter)' "$API_REPO/crates/api/src/routes" --type rust
```

Pass criteria:

- no route paths contain disallowed naming tokens

---

## Step 3 - Verify profile contract support

```bash
rg -n 'profile|ListProfile|DetailProfile|profile=' "$API_REPO/crates/api/src" --type rust
```

For each list/detail endpoint family, verify:

- list route supports documented list profiles (`list`, `filter` where applicable)
- detail route supports `profile=details` where detail badges/enrichments exist
- profile values are typed/enumerated (not free-form include strings)

Pass criteria:

- profile behavior is explicit and documented per endpoint family

---

## Step 4 - Cross-check client command naming and paths

```bash
rg -n '"/v1/' "$CLIENT_REPO/src/commands" --type ts
rg -n 'profile|for-list|for-filter|paginated|with-counts' "$CLIENT_REPO/src/commands" --type ts
```

Pass criteria:

- client commands target canonical resource routes
- commands pass typed `profile` params where projection differs
- no command targets disallowed route naming

---

## Step 5 - Cross-check frontend callsites

```bash
rg -n 'profile\\s*:\\s*\"(list|filter|details)\"|for-list|for-filter|paginated|with-counts' "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- list views use list endpoints with explicit list profile where required
- lazy selectors use `profile=filter`
- detail pages with badge counts use `profile=details`
- no frontend callsites depend on deprecated route names

---

## Step 6 - Verify alias cleanup state

```bash
rg -n 'deprecated|alias|replacement_endpoint|deprecated_endpoint' "$API_REPO/crates/api/src/routes"
```

For each alias found, record:

- replacement route
- active consumers
- removal plan/date

Pass criteria:

- zero aliases, or all aliases have explicit removal commitments

---

## Step 7 - Verification commands

```bash
cd "$CLIENT_REPO" && bun check && bun lint
cd "$ADMIN_REPO" && bun check && bun lint
cd "$WEB_REPO" && bun check && bun lint
cd "$API_REPO" && cargo check -p api --all-features
```

---

## Correction Playbook

When violations are found, apply this sequence:

1. Define canonical resource routes and profile map per domain.
2. Implement/normalize API profile handling on canonical routes.
3. Migrate client commands to canonical routes with typed profile params.
4. Migrate frontend callsites to explicit profile usage.
5. Remove aliases and deprecated path variants.
6. Re-run this sweep and record closure.

---

## Severity Rubric

- `high`: mixed naming/projection models causing active consumer confusion or break risk
- `medium`: deprecated naming still present with clear migration path
- `low`: isolated inconsistency with low immediate risk
- `note`: documentation or follow-up cleanup

---

## Findings Template

```md
### [SEVERITY] Endpoint naming/profile drift - <domain/resource>

- **API location:** `crates/api/src/routes/...`
- **Client location:** `src/commands/...`
- **Frontend location:** `src/routes/...` or `src/lib/...`
- **Check step:** Step X
- **Observed mismatch:**
- **Expected canonical/profile pattern:**
- **Impact:**
- **Fix plan:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Endpoint naming sweep summary

- Routes reviewed: N
- Naming violations found: N
- Profile contract gaps found: N
- Aliases still active: N
- Violations fixed during sweep: N
```

---

## Related Docs

- [073-api-profiles-and-query-contract.md](../guides/073-api-profiles-and-query-contract.md)
- [016-api-versioning-and-backward-compat-sweep.md](./016-api-versioning-and-backward-compat-sweep.md)
- [019-pagination-contract-consistency-sweep.md](./019-pagination-contract-consistency-sweep.md)
- [021-wasteful-endpoint-calls-sweep.md](./021-wasteful-endpoint-calls-sweep.md)
