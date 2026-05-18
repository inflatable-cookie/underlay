# g05.024 — Marking Hub Query Variant Proof And Rollout

Status: complete.

Proof status:

- Farmyard commit `7a52099` adds `variant` to
  `GET /v1/admin/marking/queue`.
- Cattle Grid commit `1291080` passes marking queue variant params.
- Dairy commit `df737db3` replaces the Marking Queue stat-card filter strip
  with `EntityListPage` query variants.

## Why

The Marking Hub in Acowtancy is the best proof for query variants because its
default list should not be "all".

The useful named views are:

- pending marking
- marked
- void
- all

Trying to model those as repeated manual filter selections is weak UX and
wrong API semantics.

## Goal

Prove the full query-variant contract through one real API/UI stack, then
roll the pattern into other list surfaces that have named base queries.

## Scope

Primary proof target:

- Acowtancy Marking Hub submitted answers list

Expected stack:

- Farmyard API understands `variant`
- Cattle-grid client preserves `variant`
- Dairy list wrapper uses `EntityListPage` query variants
- Poodle `CardToggleGroup` renders the selector through Underlay

Candidate variants:

- `pending`
- `marked`
- `void`
- `all`

Expected default:

- `pending`

## Proof Requirements

API:

- accepted variants are typed and validated
- unknown variants return a clear request error
- default variant is documented
- filters and sort apply after the variant baseline
- `profile=list-config` can return the available variants and filters when
  implemented

Client:

- command params include `variant`
- URL query state round-trips `variant`
- generated or handwritten types do not collapse variants into loose filters

UI:

- variant cards sit above `FilterToolbar`
- changing variant resets pagination
- filters remain visible as refinements
- clearing an active card falls back to `pending`, not implicit all
- `all` is visible as a real variant card

## Rollout Inventory

After the Marking Hub proof, inspect all six consumer apps for named base-list
queries.

Current consumer family:

- `underlay-reference`
- `contact-patch`
- `compli-me`
- `acowtancy`
- `songsprout`
- `loophole/composer`

Classify each candidate list as:

- no variant needed
- static UI variants enough
- API-published variants needed
- app-local exception

Initial inventory:

| App | Candidate | Classification | Notes |
|---|---|---|---|
| `acowtancy` | Marking Queue | API-published variants needed | Proof landed with `pending`, `marked`, `void`, and `all`; API owns baseline statuses and existing status filter layers on top. |
| `acowtancy` | Other Dairy browse lists | No variant needed for now | Most are ordinary entity lists with temporary filters, not named base queries. |
| `underlay-reference` | Jobs list | Static UI variants enough later | Jobs already have status stats; a future platform-system pass can expose `active`, `failed`, `succeeded`, and `all` as variants. |
| `underlay-reference` | Task lists | Static UI variants complete | Reference task lists now use `open`, `completed`, and `all`; API applies the variant baseline before filters. |
| `contact-patch` | Admin browse lists | No variant needed for now | No clear named base-list query surfaced beyond separate trash routes. |
| `compli-me` | Messages list | No variant needed for now | Visibility and publishability are ordinary filters, not baseline variants. |
| `songsprout` | Jobs list | Static UI variants enough later | Same platform-system candidate as `underlay-reference`. |
| `songsprout` | Programs list | App-local exception for now | Dashboard-style stats are not yet a server-owned list variant contract. |
| `loophole/composer` | Moderation queue | API-published variants needed later | Current page is still hand-rolled and defaults to pending; it is the closest follow-on to Marking Hub. |
| `loophole/composer` | Scan history | Static UI variants enough later | Status filters could become variants if product wants processing/completed/failed as named views. |
| `loophole/composer` | Jobs list | Static UI variants enough later | Same platform-system candidate as the other system jobs lists. |

## Consumer Upgrade Impact

Expected:

- Acowtancy gets a better Marking Hub default view
- other apps gain a consistent route and API pattern for named list views
- list URLs may now include `variant=...`
- API clients and OpenAPI declarations may gain variant enums per endpoint

## Acceptance

- Marking Hub defaults to pending answers — complete
- marked, void, and all variants work as first-class server queries — complete
- manual filters still layer on top — complete
- docs include the proof as the reference example — complete in this roadmap
- rollout inventory identifies follow-on candidates across the six apps —
  initial pass complete

## Next Task

Promote a follow-on batch for Composer moderation queue or the shared
platform-system jobs list variants. Do not migrate ordinary status filters
without a named baseline-query product reason.
