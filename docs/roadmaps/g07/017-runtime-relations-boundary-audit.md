# g07.017 - Runtime Relations Boundary Audit

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Context

After `runtime/data` gained focused collection, reorder, and selection subpaths,
the next TS boundary question was whether `runtime/relations` needed the same
additive split.

## Goals

- [x] inspect the `runtime/relations` public surface
- [x] inspect relation selector, local search, and drill-down implementation
  ownership
- [x] inspect six-consumer import usage
- [x] decide whether to add focused subpaths or keep one coherent public path

## Decision

Do not split `runtime/relations`.

`runtime/relations` remains the retained public home for relation selector
workflow contracts:

- relation selector context helpers
- relation selector types
- drill-down selector types
- local search adapters
- local drill-down search adapters

## Evidence

- The local search helpers return `RelationSearchFn` and
  `RelationSuggestionsFn`, so they are direct adapters for the exported selector
  contracts.
- The drill-down search helpers return `DrillDownSearchFn` and
  `DrillDownSuggestionsFn`, so they are direct adapters for the exported
  drill-down contracts.
- Consumer use is concentrated in `acowtancy/dairy`, where forms, transform
  flows, app-local selector shells, local adapters, and relation types are used
  together.
- `underlay-reference/acme-admin` uses the same path for relation selector
  types in form code.
- No consumer evidence shows a low-churn, high-clarity split comparable to the
  `runtime/data` collection/reorder/selection split.

## Consumer Upgrade Impact

None.

No code change is required. Consumers should continue importing relation
selector contracts and local adapters from
`@inflatable-cookie/underlay/runtime/relations`.

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy validate`

## Next Task

No active `g07` task remains. Keep `runtime/relations` whole unless a future
consumer migration creates clear evidence for a narrower public path.
