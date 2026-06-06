# g06.168 - Fleet Compatibility Closeout Audit

## Why

`g06.166` proved the current Underlay surface across the six known consumers and
found one Acowtancy drift item. `g06.167` repaired that drift.

The lane needs a final fleet closeout record before moving to the next
reference-grade architecture target.

## Goal

Record the final consumer compatibility state after the Underlay export fixes
and Acowtancy repair.

## Scope

In scope:

- confirm Underlay has no new source validation regression
- confirm Acowtancy remains healthy after the Cattle Grid repair
- record the six-consumer compatibility state from the recent sweep
- identify any remaining warning-only or environmental risk
- recommend the next reference-grade lane

Out of scope:

- new consumer feature work
- additional consumer code edits unless validation reveals a regression
- broad Underlay API redesign

## Acceptance Criteria

- final validation state is recorded
- consumer compatibility status is summarized by root
- remaining risks are classified
- the next lane is named

## Consumer Upgrade Impact

Expected impact: none beyond the already-committed Acowtancy repair.

## Current State

`g06.168` is complete.

Final compatibility state:

- `underlay-reference`: compatible after the Underlay
  `DrillDownBreadcrumb` import fix; `acme-admin/check` passed.
- `contact-patch`: compatible; root health passed in the sweep.
- `compli-me`: compatible; root health passed in the sweep.
- `acowtancy`: compatible after the Cattle Grid list-query repair; root health
  passed.
- `songsprout`: compatible; root health passed in the sweep.
- `loophole/composer`: compatible; root health passed in the sweep.

Underlay state:

- package exports for `.`, `./client`, `./runtime`, and `./nightfire` are now
  explicit
- retained bare subpaths are narrow compatibility surfaces, not broad source
  barrels
- TypeScript source structure is stable after the source god-file splits
- `effigy doctor` has only the accepted 9 warning-only test-size findings

Remaining risks:

- Underlay test god-file warnings remain a navigation cost, not a production
  modularity blocker.
- Acowtancy health reports one existing non-failing Rust warning in
  `farmyard-migration`.
- The fleet uses local file dependencies and source aliases in places; the
  export-map compatibility test now guards the Underlay side, but package-style
  publish simulation is still a useful future release-readiness check.

Recommendation: move next to a Rust runtime/security surface re-audit. The
TypeScript and fleet compatibility lanes are closed enough that the next
reference-grade pressure is server-side behavior, construction boundaries, and
security-sensitive crate APIs.

## Next Task

Execute `g06.169`: Rust runtime security surface re-audit.
