# 022 - API Endpoint Naming Convention Sweep

This sweep verifies that API endpoint naming stays consistent, predictable, and consumer-oriented across Underlay-based projects.

It is designed as a repeatable policy check for existing projects and as a rollout checklist for new projects.

## Problem this sweep targets

Naming drift creates long-term API entropy:

- route names leak implementation details (`/paginated`, `with-counts`)
- equivalent endpoint families use mixed conventions across domains
- client commands become inconsistent and harder to discover
- frontend teams guess endpoint purpose from naming and get it wrong
- legacy aliases accumulate and are never removed

## Policy baseline (platform standard)

These rules apply to all Underlay consuming apps unless an explicit project ADR overrides them.

### 1) Resource and detail routes

- List route: `GET /v1/{scope}/{domain}/{resource}` or project-approved variant
- Detail route: `GET /v1/{scope}/{domain}/{resource}/{id}`
- Create route: `POST /v1/{scope}/{domain}/{resource}`
- Update route: `PUT /v1/{scope}/{domain}/{resource}/{id}`

### 2) Consumer-projection list routes

When a list serves a specific UI projection (cards/table rows), use:

- `GET /v1/{scope}/{domain}/{resource}-for-list`

Optional lightweight selector projection:

- `GET /v1/{scope}/{domain}/{resource}-for-filter`

### 3) Naming anti-patterns (not allowed)

- Route suffixes describing mechanics: `/paginated`, `/cursor`, `/offset`
- Route names describing internal SQL shape: `with-counts`, `with-joins`, `flat`
- Verb-heavy CRUD naming in path segments where HTTP method already encodes action

### 4) Action routes

For non-CRUD actions, use explicit action subpaths, for example:

- `POST /.../{id}/soft-delete`
- `POST /.../batch-soft-delete`
- `POST /.../reorder`
- `POST /.../validate-field`

### 5) Alias policy

- Prefer no long-lived aliases in active development projects.
- If aliases exist temporarily, they must have:
  - explicit deprecation logs
  - an owner and removal date
  - tracking in roadmap/issues

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
rg -n 'path\s*=\s*"/v1/' "$API_REPO/crates/api/src/routes" --type rust
```

Build a table with:

- method
- path
- handler
- entity/domain
- endpoint purpose (`list`, `detail`, `mutation`, `action`)

Pass criteria:

- each endpoint family is classifiable by a small, stable set of patterns
- no unexplained one-off naming shapes

---

## Step 2 - Detect policy violations directly in API routes

```bash
rg -n 'path\s*=\s*"/v1/.+(paginated|with-counts|with-joins|flat|cursor|offset)' "$API_REPO/crates/api/src/routes" --type rust
```

Pass criteria:

- no route paths contain mechanical/implementation suffixes
- projection endpoints use `-for-list` / `-for-filter` where applicable

---

## Step 3 - Cross-check client command naming and paths

```bash
rg -n '"/v1/' "$CLIENT_REPO/src/commands" --type ts
rg -n 'list.*ForList|for-list|for-filter|paginated|with-counts' "$CLIENT_REPO/src/commands" --type ts
```

Pass criteria:

- command names match endpoint intent (`listXForListAdmin`, `getXAdmin`, etc.)
- no client commands target banned route naming patterns
- no duplicate command surface for equivalent list purpose

---

## Step 4 - Cross-check frontend callsites

```bash
rg -n 'list.*ForList|get.*WithCounts|paginated|for-list|for-filter' "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- list pages/tabs use canonical list commands
- no frontend code depends on deprecated route names
- feature behavior does not require route aliases

---

## Step 5 - Verify alias cleanup state

```bash
rg -n 'deprecated|alias|replacement_endpoint|deprecated_endpoint' "$API_REPO/crates/api/src/routes"
```

For each alias found, record:

- replacement route
- consumers still using alias
- removal plan/date

Pass criteria:

- either zero aliases, or all aliases have explicit removal commitments

---

## Step 6 - Enforce domain-level consistency

For each domain (for example `learning`, `content`, `media`, `exams`), compare sibling resources:

- are list routes named with the same pattern?
- do equivalent operations use the same action suffixes?
- are detail routes uniformly id-based?

Pass criteria:

- naming convention is consistent both across the app and within each domain

---

## Step 7 - Verification commands

```bash
cd "$CLIENT_REPO" && bun check && bun lint
cd "$ADMIN_REPO" && bun check && bun lint
cd "$WEB_REPO" && bun check && bun lint
cd "$API_REPO" && cargo check -p api --all-features
```

Use project-appropriate crate/package names where they differ.

---

## Recurring sweep cadence (recommended)

Run this sweep:

- before each release
- monthly for active projects
- after major API/domain refactors

Optional CI gate:

- add a lightweight lint check that fails on banned path tokens (`paginated`, `with-counts`) in route annotations and command paths

---

## Correction playbook

When violations are found, apply this sequence:

1. Define target canonical route names per domain.
2. Rename API routes first and keep temporary aliases only if operationally required.
3. Rename/update client commands to canonical names.
4. Update frontend callsites to canonical commands.
5. Remove aliases and stale commands once consumers are migrated.
6. Re-run this sweep and record closure.

---

## Severity rubric

- `high`: multiple naming schemes actively used for same endpoint purpose, causing active consumer confusion or break risk
- `medium`: legacy naming still present but migration path exists
- `low`: isolated naming inconsistency with no immediate break risk
- `note`: documentation or cleanup follow-up

---

## Findings template

```md
### [SEVERITY] Endpoint naming drift - <domain/resource>

- **API location:** `crates/api/src/routes/...`
- **Client location:** `src/commands/...`
- **Frontend location:** `src/routes/...` or `src/lib/...`
- **Check step:** Step X
- **Observed naming mismatch:**
- **Expected canonical pattern:**
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
- Violations found: N
- Domains with mixed conventions: N
- Aliases still active: N
- Violations fixed during sweep: N
```

---

## Related docs

- [016-api-versioning-and-backward-compat-sweep.md](./016-api-versioning-and-backward-compat-sweep.md)
- [019-pagination-contract-consistency-sweep.md](./019-pagination-contract-consistency-sweep.md)
- [020-list-endpoint-for-detail-views-sweep.md](./020-list-endpoint-for-detail-views-sweep.md)
- [021-wasteful-endpoint-calls-sweep.md](./021-wasteful-endpoint-calls-sweep.md)
