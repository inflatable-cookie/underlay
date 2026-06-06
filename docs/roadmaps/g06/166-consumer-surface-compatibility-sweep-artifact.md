# g06.166 Artifact - Consumer Surface Compatibility Sweep

## Result

The current Underlay source splits were not broadly breaking across the six
known consumers, but the sweep found one Underlay regression and one consumer
drift lane.

## Import Surface

Live source usage is concentrated on the retained public surfaces:

- `@decodelabs/underlay/templates`
- `@decodelabs/underlay/patterns`
- `@decodelabs/underlay/client/*`
- `@decodelabs/underlay/runtime/*`
- `@decodelabs/underlay/nightfire/*`
- `@decodelabs/underlay/styles/*`
- `@decodelabs/underlay/server`

Two legacy bare-subpath uses still exist in Acowtancy tests:

- `@decodelabs/underlay/nightfire`
- `@decodelabs/underlay/runtime`

Underlay now publishes narrow compatibility exports for those retained
bare-subpaths instead of relying on consumer source aliases.

## Validation

Consumer sweep:

- `underlay-reference`: root health initially failed on missing
  `DrillDownBreadcrumb`; `effigy acme-admin/check` passed after the Underlay fix.
- `contact-patch`: `effigy health` passed.
- `compli-me`: `effigy health` passed.
- `acowtancy`: root `effigy health` failed on existing list-query contract
  drift; `effigy froyo/health` and `effigy cattle-grid/health` passed.
- `songsprout`: `effigy health` passed.
- `loophole/composer`: `effigy health` passed.

Underlay validation:

- `effigy check`: passed
- `effigy check:exports`: passed
- `effigy test:components`: passed
- `bun x vitest run ts/tests/package-compatibility.test.ts`: passed
- `bun x vitest run ts/tests/patterns/relation-selector.test.ts ts/tests/nightfire/utils.test.ts`: passed

Targeted consumer probes:

- `underlay-reference`: `effigy acme-admin/check` passed
- `acowtancy/froyo`: `bun x vitest run tests/nightfire-widget-registrations.test.ts` passed
- `acowtancy/cattle-grid`: `bun x vitest run tests/learning-modules-admin-cache.test.ts` failed on payload shape, not import resolution

## Underlay Fixes

- Added the missing `DrillDownBreadcrumb` type import in
  `ts/src/patterns/RelationSelector/drilldown-context.svelte.ts`.
- Added explicit package exports for `.`, `./client`, `./runtime`, and
  `./nightfire`.
- Added narrow compatibility barrels:
  - `ts/src/client/index.ts`
  - `ts/src/runtime/index.ts`
  - `ts/src/nightfire/index.ts`
  - `ts/src/index.ts`
- Added Vitest component aliases for `$app/environment` and `$app/navigation`.
- Added `ts/tests/package-compatibility.test.ts`.

## Consumer Drift

Acowtancy Cattle Grid needs a follow-up:

- remove offset-style list-query parameters from
  `cattle-grid/src/commands/learning/modules.ts`
- align `tests/learning-modules-admin-cache.test.ts` with the cursor-shaped
  response returned by `listModulesForListAdmin`, or change the command back to
  page-shaped output if that is the intended app contract

This is consumer-side drift, not a blocker for Underlay's current source
structure.
