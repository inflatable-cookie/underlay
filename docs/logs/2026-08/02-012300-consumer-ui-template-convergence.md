# Consumer UI template convergence — admin data-layer + template rollout

Date: 2026-08-02
Scope: all six consumers (acme, cp, compli, farmyard, nursery, composer) + underlay `ts/`
Governing refs: `docs/architecture/070-consumer-drift-prevention.md`,
`docs/guides/191-admin-resource-checklist.md`,
`docs/usage/templates/000-template-system-overview.md`

## What shipped

Post-g08 consumer-convergence phase, TS/admin side. Continues the security +
session convergence recorded in sweep 021 and the 2026-07 logs
(`underlay-auth-session` adopted in all six).

### Admin data-layer

- Canonical admin list envelope: `underlay_http::PageList` +
  `ListQueryParams` server-side; `createPageListQueryState`
  (`ts/src/patterns/page-list-query.svelte.ts`) client-side — adopted across
  94 consumer files.
- `CachedListEndpoint` for admin list caching; N+1 and claims-waterfall
  fixes on the API side.

### UI phase (nine items)

1. Form-field layer: `EntityRelationField`, `EntityFormActions`, field-error
   context (`useFormFieldErrors` / `setFormFieldErrors`) — `ts/src/templates/`.
2. Composer form migration: 9 pages onto the form-field layer + `EntityFormPage`.
3. Admin chrome: `AdminNavList` + `AdminUserMenu` templates, adopted in 4 repos.
4. Actions-menu recipe: `EntityActionsMenu` everywhere; shared `MediaActionsMenu`
   re-skinned onto it; 16 dairy ListCards onto `EntityListCard`.
5. Trash pages: 3 consumer trash routes onto `EntityTrashPage`; 6 dead
   consumer-local reorder-conflict copies deleted — canonical
   `recoverReorderConflict` lives in `ts/src/patterns/reorder-conflict.ts`
   (with `UnderlayHttpError` envelope support).
6. `EntityMediaField` — media picker form-field chrome.
7. System lists: greenhouse query-state fix; a system-list factory was
   evaluated and deliberately rejected (thin `EntityListPage` configs win).
8. `UsersListPage` + `UserForm` templates, adopted in 4 repos, with extension
   hooks (`extraRowActions`, `onCustomRowAction`, `searchFilterId`,
   `showSortFilter`, `reloadKey`) so app-specific columns/actions stay
   app-owned.
9. Greenhouse raw tables → `EntityListPage`.

### Fixes found in the rollout

- `EntityFormPage` field-error feedback was clobbered by the prop-sync
  effect — fixed in underlay (`117088f7`), all consumers inherit.

## Verification

- `effigy qa:security` (16/16 conformance checks) green in all six consumer
  repos.
- underlay `effigy validate` green: 755 unit + 34 component tests.

## Rollout gotchas (for the next wave)

- Consumer `node_modules/@inflatable-cookie/underlay` can be stale → `bun install`
  before judging type errors.
- acowtancy uses git submodules: commit inside dairy, then bump the parent.
- Baselines, not regressions: composer svelte-check has 19 pre-existing
  errors; dairy has 64 pre-existing warnings.

## Residual nits (found in the 2026-08-02 runtime spot-checks)

- acme `list_users`/`list_projects` still parse local query structs instead
  of canonical `ListQueryParams`, so the raw `?search=` param is silently
  ignored. Not user-facing: template search filters serialize as
  `filter[field][like]` and those are mapped (`filter_only("query", …)` on
  users). Conformance nit, not a bug.
- cp is the mirror image: endpoints take flat typed params (`search=`,
  `is_live=`) and silently ignore `filter[field][op]`. Also not user-facing:
  cp-admin list wrappers (ChaptersList etc.) translate template `QueryParams`
  filters onto the flat params in their `dataLoader`s. Both shapes work; the
  canonical one-extractor `ListQueryParams` shape is adopted in neither.
- effigy-bundle workspace stacks: named-volume ownership rots to `root:root`
  (cargo registry, node_modules/.vite) — needs an upstream volume-init fix;
  workaround is a one-off `chown -R dev:dialout`. (cp stack did not trigger
  it; acme did.)
- Runtime spot-checks 2026-08-02: acme + cp stacks verified — PageList
  envelopes, pagination/filters, `profile=details` + ETag/304, trash
  soft-delete/restore/purge (acme media, cp chapters), all converted
  template modules transform in vite. Client-side behavior (nav expansion,
  dialogs) not verifiable without a browser.
- songsprout spot-check found four genuine defects (unrelated to the
  template conversion except where noted) — all four fixed 2026-08-02:
  1. `nursery/migrations/202605131430__baseline_media.sql` referenced
     nonexistent `auth.users` — fresh dev DBs could not migrate. Fixed by
     pointing the media audit-column FKs at `artists.artist(id)` (the values
     written there are artist ids). Note: sqlx checksum-validates applied
     migrations, so any dev DB that already applied the old file needs a
     `db:reset` (the migration was never applicable to a real environment —
     it could not have run anywhere without a manual workaround).
  2. `handlers/admin/billing.rs` used `a.name` (should be
     `a.display_name`) — `/v1/admin/billing/subscriptions` 500'd; the
     converted billing table could not load data. Fixed; verified live (all
     sort/search variants 200).
  3. **SSR contract break:** `useAuthenticatedData` resolved auth handlers
     at setup, which throws without a global `getToken`; greenhouse
     deliberately calls `configureAuth` client-side only (process-global
     config would leak tokens across SSR requests), so every
     EntityListPage-backed route 500'd on hard load. Fixed underlay-side:
     handlers now resolve lazily on the fetch path; SSR renders the loading
     shell, hydration fetches, and a missing `getToken` surfaces as the
     hook's error state. Latent same-pattern risk remains in
     `createListController` and `createServerPagination` (setup-time
     resolution) — no converted page uses them; fix if an SSR consumer
     adopts them.
  4. `RateLimitStore` defaulted to `auth.rate_limit_counters` but nursery's
     migration created `accounts.rate_limit_counters` — HTTP rate limiting
     silently fail-open. Fixed with `with_table("accounts.rate_limit_counters")`,
     matching the auth crate's limiter.
  Also: nursery has no dev seeds and no first-admin bootstrap path
  (`/v1/admin/staff/register` requires an existing super_admin) — the
  spot-check admin had to be manufactured by SQL.

## Consumer Upgrade Notes

- Impact class: `additive` (new template/pattern exports; consumer-side
  deletions were of duplicated local code)
- Affected consumers: all six admin apps
- Required actions:
  - `bun install` to pick up current `@inflatable-cookie/underlay`
  - new admin resources follow `docs/guides/191-admin-resource-checklist.md`
    and the template overview; no action needed for already-migrated pages
- Validation:
  - `effigy qa:security` (per consumer repo)
  - `effigy validate` (underlay)
- Deprecation/removal date: n/a
- Reference docs:
  - `docs/usage/templates/000-template-system-overview.md`
  - `docs/guides/191-admin-resource-checklist.md`
  - `docs/sweeps/021-consumer-security-convergence.md`
