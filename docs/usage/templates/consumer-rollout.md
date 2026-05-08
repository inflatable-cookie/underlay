# Template Consumer Rollout Snapshot

Status: historical snapshot
Updated: 2026-05-08

This page is no longer the live rollout authority.

Use it as frozen rollout evidence only. The active template-system delivery
state lives in `docs/roadmaps/g03/` and the contract/assessment state lives in
`docs/roadmaps/g04/`.

## Rollout Order

| # | Consumer | Complexity | Status | Blockers |
|---|----------|-----------|--------|----------|
| 1 | acme-admin (reference) | Low | **Complete** | — |
| 2 | Dairy | High | Blocked | Template gaps (see below) |
| 3 | cp-admin | Medium | Ready | — |
| 4 | compli-me | Medium | Ready | — |
| 5 | songsprout | Medium | Ready | — |
| 6 | loophole/composer | Unknown | Not evaluated | — |

## Per-App Scope

### 1. acme-admin (Reference)

**Status:** Complete (g03.006–009)

| Page | Template | Lines Before | Lines After |
|------|----------|-------------|------------|
| `/projects` | EntityListPage | 506 | 158 |
| `/projects/[id]` | EntityDetailPage + EntityList | 800 | 412 |

**Learnings:**
- Custom batch dialog support was required for "update status"
- Detail pages with tabs are ~400 lines even with templates (target was 80)
- URL sync pattern is reusable across all consumers

### 2. Dairy

**Status:** Blocked on template enhancements

**First migration target:** `/learning/areas` (AreasList, 535 lines)

**Blockers:**
1. **Cascading filter reset** — pathway → module → section filters need child reset on parent change
2. **Client-side filtering** — AreasList loads all data then filters client-side; EntityList assumes server-side
3. **Batch navigation links** — copy/move are navigation links, not dialogs or handlers
4. **Conditional reorder** — reorder only available in section tab with no search and >1 items
5. **Stitched data** — single loader returns `{ sections, areas }`, not a flat array
6. **Custom empty states** — different messages for "no results" vs "no data"
7. **Tab variant** — component works as both page and embedded tab

**Resolution:** Complete g03.010a (template enhancements) before Dairy migration.

### 3. cp-admin

**Status:** Ready for migration

**Suggested first pages:**
- `/users` → EntityListPage (users list with search/filter)
- `/users/[userId]` → EntityDetailPage (user detail)
- `/book/chapters` → EntityListPage (chapters list)
- `/book/chapters/[id]` → EntityDetailPage (chapter detail)
- `/book/subjects` → EntityListPage (subjects list)

**Complexity:** Medium. Similar patterns to acme-admin. No cascading filters or stitched data.

### 4. compli-me

**Status:** Ready for migration

**Suggested first pages:**
- `/compliments/businesses` → EntityListPage
- `/compliments/businesses/[businessId]` → EntityDetailPage
- `/compliments/people` → EntityListPage
- `/compliments/people/[personId]` → EntityDetailPage
- `/compliments/messages` → EntityListPage

**Complexity:** Medium. Standard CRUD patterns. Trash page (`/compliments/trash`) may need custom handling for soft-deleted items.

### 5. songsprout

**Status:** Ready for migration

**Suggested first pages:**
- `greenhouse/catalogue` → EntityListPage (artists/releases)
- `greenhouse/catalogue/artists/[artistId]` → EntityDetailPage
- `bloom/releases` → EntityListPage
- `bloom/tasks` → EntityListPage
- `bloom/tracks` → EntityListPage

**Complexity:** Medium. Mixed admin/public consumers. Greenhouse is admin-focused, bloom is mixed. Start with greenhouse admin pages.

### 6. loophole/composer

**Status:** Not evaluated

**Action:** Audit admin pages and classify as list/detail/form shapes.

## Rollback Criteria

Migrate a page **back** to hand-rolled composition if:

1. **Template requires >80% of original line count** — templates should provide meaningful reduction
2. **Custom behavior doesn't fit declarative API** — and enhancing the template would break other consumers
3. **Performance regression >20%** — e.g., double-loading data, extra renders
4. **Developer ergonomics degrade** — team prefers explicit composition over declarative config
5. **Feature requires template API change that affects stability** — don't rush API changes for edge cases

## Migration Process

For each page:

1. **Audit** — classify as list/detail/form, note special features
2. **Check blockers** — compare against known gaps (see Dairy blockers)
3. **Migrate** — rewrite using templates, preserve all behavior
4. **Validate** — test all interactions: filters, sort, batch, reorder, navigation
5. **Measure** — compare line count before/after
6. **Document** — add to this plan with actual results

## Dependencies

- **Before Dairy:** Complete template enhancements for cascading filters, client-side filtering, batch nav links
- **Before cp-admin/compli-me/songsprout:** No blockers. Can start immediately after acme-admin proof.
- **Before loophole:** Complete audit of admin page inventory.

## Success Metrics

- **Line count reduction:** Target 50%+ reduction per page
- **Consistency:** All admin list/detail pages use same patterns
- **Developer velocity:** New admin pages take <30 minutes to scaffold
- **Bug rate:** No increase in admin page bugs post-migration

## Next Steps

1. Complete g03.010a: template enhancements for Dairy blockers
2. Migrate cp-admin `/users` and `/users/[userId]` as next proof
3. Parallel: migrate compli-me `/compliments/businesses`
4. Evaluate loophole/composer admin inventory
