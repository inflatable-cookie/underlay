# g06.166 - Consumer Surface Compatibility Sweep

## Why

The TypeScript source structure is now clean enough to test against the real
extension boundary: the known consumer apps.

Before more reference-grade architecture work lands, the six consumer roots
should be checked against the current Underlay public facades.

## Goal

Prove current Underlay compatibility across the known consumer family and record
any import, build, or template-surface drift.

## Scope

In scope:

- inspect the six known consumer roots listed in `AGENTS.md`
- check Underlay imports against documented public surfaces
- run targeted consumer validation where each workspace exposes a reasonable
  local task
- classify any required consumer updates as bounded follow-up work
- record whether current g06 source splits were breaking in practice

Out of scope:

- broad consumer feature work
- production deployment
- new Underlay public APIs unless a compatibility blocker proves one is needed
- unrelated app cleanup

## Acceptance Criteria

- every known consumer root is inspected
- any stale Underlay import path is listed
- validation commands and failures are recorded
- required consumer updates are classified by app
- Underlay-side follow-up is separated from app-side rollout work

## Consumer Upgrade Impact

Expected impact: audit only.

If drift is found, follow-up cards may include consumer updates.

## Current State

`g06.166` is complete.

Consumer sweep result:

- `underlay-reference`: root health exposed an Underlay Svelte type regression
  in `RelationSelector/drilldown-context.svelte.ts`; fixed, and
  `acme-admin/check` now passes.
- `contact-patch`: root health passed.
- `compli-me`: root health passed.
- `acowtancy`: root health fails on existing list-query contract drift in
  `cattle-grid/src/commands/learning/modules.ts`; focused `froyo/health` and
  `cattle-grid/health` pass.
- `songsprout`: root health passed.
- `loophole/composer`: root health passed; transient Bun cache install warnings
  were reported, but the health command completed successfully.

Underlay-side fixes landed in this batch:

- imported the missing `DrillDownBreadcrumb` type in the RelationSelector
  drill-down context module
- added explicit compatibility exports for `.`, `./client`, `./runtime`, and
  `./nightfire`
- kept bare `./client`, `./runtime`, and `./nightfire` narrow so Node tests do
  not pull SvelteKit virtual modules, Svelte runes, or `.svelte` components
  accidentally
- added component-test aliases for SvelteKit `$app/*` virtual modules
- added a package compatibility test for retained bare subpaths

Consumer drift:

- Acowtancy Cattle Grid still has page-vs-cursor payload drift in
  `tests/learning-modules-admin-cache.test.ts`.
- Acowtancy root health still flags offset-style list-query parameters in
  `cattle-grid/src/commands/learning/modules.ts`.

## Next Task

Execute `g06.167`: Acowtancy list-query consumer drift repair.
