# g05.024 — Marking Hub Query Variant Proof And Rollout

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

## Consumer Upgrade Impact

Expected:

- Acowtancy gets a better Marking Hub default view
- other apps gain a consistent route and API pattern for named list views
- list URLs may now include `variant=...`
- API clients and OpenAPI declarations may gain variant enums per endpoint

## Acceptance

- Marking Hub defaults to pending answers
- marked, void, and all variants work as first-class server queries
- manual filters still layer on top
- docs include the proof as the reference example
- rollout inventory identifies follow-on candidates across the six apps

## Next Task

Execute after `g05.023` lands. Start with Acowtancy and only expand to other
apps after the Marking Hub behavior is proven.
