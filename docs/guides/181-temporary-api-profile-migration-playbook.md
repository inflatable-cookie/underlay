# 181 - Temporary API Profile Migration Playbook

Status: Temporary
Removal Trigger: Remove this file once all active Underlay-consuming apps are fully migrated to the `profile` contract and sweep-clean.

## Purpose

Use this playbook to run a repeatable migration from projection path naming (`*-for-list`, `*-for-filter`) to canonical resource routes with typed `profile` query params.

This is intended for active migration waves across multiple apps (for example Farmyard + Cattle Grid + Dairy + Cream + Cattle Grid consumers).

## Preconditions

- App team agrees to no compatibility shims, or has documented exception scope.
- Current API route inventory is available.
- Current client command inventory is available.
- Frontend list/detail callsites are searchable.

## Step-by-Step Sweep Workflow

1. Inventory endpoints and classify shape
- Find all public routes.
- Mark each as canonical CRUD, projection list path, or action path.
- Record every route using `/paginated`, `with-counts`, `-for-list`, `-for-filter`.

2. Define target canonical route map
- For each resource, keep one list route and one detail route:
  - `GET /v1/{scope}/{domain}/{resource}`
  - `GET /v1/{scope}/{domain}/{resource}/{id}`
- Define supported profiles:
  - list route: `list`, `filter`
  - detail route: `details` (only where enrichments are needed)

3. Implement API profile support
- Add typed profile enums and parser validation.
- Route to profile-specific query functions internally.
- Keep one public path; move projection differences into query/profile handling.
- Add/verify shared list query params (`limit`, `cursor`, `direction`, `includeTotal`, `sort`, resource filters).

4. Migrate client commands
- Replace path-specific list commands with canonical resource commands.
- Add typed `profile` param and shared query param support.
- Remove command wrappers tied to deprecated route names.

5. Migrate frontend callsites
- Lists/tabs: use `profile=list`.
- Lazy selectors/filter dropdowns: use `profile=filter`.
- Detail pages with tab badges: use detail fetch `profile=details`.
- Remove supplementary tab-count fetches.

6. Remove deprecated routes and aliases
- Delete old projection path routes.
- Delete old command wrappers.
- Re-scan for stale route strings.

7. Verify and close
- Run sweeps:
  - naming convention sweep
  - pagination contract sweep
  - wasteful endpoint calls sweep
- Update roadmap checklists (including parent items).

## Findings Template (copy into roadmap)

```md
### [SEVERITY] Profile contract migration gap - <domain/resource>

- **API location:** `...`
- **Client location:** `...`
- **Frontend location:** `...`
- **Observed issue:**
- **Expected contract:**
- **Fix plan:**
- **Owner:**
- **Status:** Open / In progress / Resolved
```

## Exit Criteria Per App

- No routes contain `/paginated`, `with-counts`, `-for-list`, or `-for-filter`
- Canonical list/detail resource paths are in use
- List endpoints implement `profile=list|filter`
- Detail endpoints with badge tabs implement `profile=details`
- Frontend badge counts are sourced from the main detail fetch
- Naming/pagination/wasteful-call sweeps pass with no open high-severity findings

## Related Docs

- [073-api-profiles-and-query-contract.md](./073-api-profiles-and-query-contract.md)
- [019-pagination-contract-consistency-sweep.md](../sweeps/019-pagination-contract-consistency-sweep.md)
- [021-wasteful-endpoint-calls-sweep.md](../sweeps/021-wasteful-endpoint-calls-sweep.md)
- [022-api-endpoint-naming-convention-sweep.md](../sweeps/022-api-endpoint-naming-convention-sweep.md)
