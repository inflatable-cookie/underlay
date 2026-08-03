# Admin Resource Checklist

The canonical shape for every admin resource in an Underlay consumer. This
is the checklist version of the pattern proven in acowtancy's Modules admin
and now shared through Underlay. If a resource deviates from this page, it
is wrong — fix the resource, not the checklist.

Related: `docs/contracts/115-admin-resource-api-shapes.md`,
`docs/contracts/116-canonical-collection-routes-and-query-profiles.md`,
`docs/architecture/070-consumer-drift-prevention.md`,
`docs/guides/192-config-model.md`.

## 1. List contract

**Route:** one route per resource.

```
GET /v1/admin/{domain}/{entities}?profile=list&page&limit&filter[x]=…&search=…
→ { data, total, has_more }
```

**Server:**
- Envelope: `underlay_http::PageList<T>` — never a hand-rolled copy.
- Query parsing: `Query<ListQueryParams>` (page/limit/sort/filters/variant/
  search in one extractor).
- SQL: `LIMIT/OFFSET` + `COUNT`, deterministic default ORDER BY with an
  `id` tiebreak, filter/sort columns behind a `FieldMapping` allowlist.
- Offset pagination on all browse lists. No cursors (cursor machinery is
  runtime/scale lane only — guide 093).
- No unbounded `SELECT`s. Whole-set responses are only acceptable for
  catalogs that are small by design, and must still carry a server-side
  cap (`PageList::from_bounded` or an explicit LIMIT).

**Client:**
- `EntityListPage` with a `dataLoader`; query state via
  `createPageListQueryState({ mode: "url" })` (URL-driven) or `"local"`
  for embedded lists.
- Subsets are filters or `queryVariants` on the same endpoint — never
  separate endpoints, never client-side filtering of a big fetch.

## 2. Detail contract

**Route:** `GET /v1/admin/{domain}/{entities}/{id}?profile=details` →
`SingleResponse { data }`.

- The detail payload carries the entity **plus tab badge counts** — no
  separate count queries.
- Freshness: version ETag derived from `updated_at` (+ child counts);
  writes go `getWithEtag` → `PUT` with `If-Match` → 412 →
  reload-and-reapply UX.
- One fetch per detail view. If a page calls the same endpoint twice
  (load fn + loader, or loader + useAuthenticatedData), one of them goes.

## 3. Tab contract

- Detail-page child collections are **ordinary paged lists** against their
  own endpoints (`/v1/admin/{domain}/{children}?filter[parent_id]=…` or a
  scoped route), each with its own `page`/`limit`.
- Tabs lazy-load on activation; child mutations refetch the parent detail
  (badge counts).
- Never return multiple unbounded child collections inside the detail
  response. Never fan out `Promise.all(claims.map(perItemFetch))` — embed
  with `?include=` or batch server-side (`ANY($1)`).
- Enrichment of list rows (roles, labels, parent names) is a JOIN or one
  batched fetch — never a per-row query in a loop.

## 4. Reorder contract

- Scoped batch reorder: UI loads the full scope, drags locally, explicit
  Save → `POST …/{parent}/{children}/reorder { ids }`.
- Server: `underlay_db::reorder_scoped` — one transaction, `FOR UPDATE`
  lock, single `UPDATE … FROM unnest … WITH ORDINALITY` rewrite.
- Set drift → 409 `{ added_ids, removed_ids }` → client merges via
  `applyReorderConflict` and retries.
- Never a per-row UPDATE loop (non-transactional, corrupts weights on
  partial failure).
- One activity/audit row per reorder, not one per item.

## 5. Sessions & auth

- Session lifecycle only through `underlay_auth_session::SessionService`.
- Refresh re-checks account status and re-issues roles (built into the
  crate — do not reimplement).

## 6. Frontend hygiene

- `{@html}` only behind `sanitizeHtml`/`sanitizeSvgHtml` — never a regex
  blacklist.
- Redirect targets only through `resolveRedirectTo`.
- No tokens in localStorage; no whole-table fetches to render tabs.

## 7. New resource checklist (copy into the PR description)

- [ ] List route uses `PageList` + `ListQueryParams`, allowlisted columns
- [ ] Detail route returns entity + badge counts, version ETag
- [ ] Tabs are lazy paged lists; no `include=…` fan-outs, no row loops
- [ ] Reorder uses `reorder_scoped`, 409 contract, one audit row
- [ ] No unbounded query anywhere (whole-set catalogs carry a server cap)
- [ ] `effigy qa:security` passes
