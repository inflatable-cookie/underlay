# g06.151 - Template Types Internal Split

## Why

`g06.150` found that `ts/src/templates/template.types.ts` is type-only shared
surface but groups many unrelated template type families in one public source
file.

The file can be split internally while keeping public imports stable.

## Goal

Split template types into focused internal modules without changing public
template exports, direct `./template.types` imports, or template type contracts.

## Scope

In scope:

- keep `ts/src/templates/template.types.ts` as the public front door
- extract shared primitives to `ts/src/templates/template-types/primitives.ts`
- extract list/filter/capability types to `template-types/list.ts`
- extract action/dialog/reorder types to `template-types/actions.ts`
- extract detail types to `template-types/detail.ts`
- extract system and error-log types to `template-types/system.ts`
- extract media types to `template-types/media.ts`
- extract entity-list aggregate and adapter types to
  `template-types/entity-list.ts`
- preserve `ts/src/templates/index.ts` public type exports

Out of scope:

- changing public template APIs
- changing template behavior
- changing consumer apps
- changing component implementations beyond type import fallout if needed

## Acceptance Criteria

- public `./template.types` imports continue to compile
- `ts/src/templates/index.ts` exports continue to compile
- `effigy check:types` passes
- component template validation is run if the `$app/navigation` test-environment
  blocker is resolved; otherwise record the blocker
- `effigy qa:docs` passes
- roadmap artifact records final module shape and public API impact

## Consumer Upgrade Impact

Expected impact: none.

This should be an internal split. If consumer imports or behavior need to
change, stop and re-enter planning.

## Current State

`g06.151` is ready.

## Next Task

Execute `g06.151`: template types internal split.
