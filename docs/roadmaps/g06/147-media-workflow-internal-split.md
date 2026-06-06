# g06.147 - Media Workflow Internal Split

## Why

`g06.146` found that `ts/src/patterns/media-workflow.ts` mixes public types,
browse helpers, generic upload workflow, app-level pipeline helpers, and upload
plan normalization in one source file.

The file is a public shared surface. The next step is a mechanical internal
split that keeps the same import path and behavior.

## Goal

Split media workflow into focused internal modules without changing public
exports, upload behavior, duplicate behavior, upload plan defaults, or consumer
imports.

## Scope

In scope:

- keep `ts/src/patterns/media-workflow.ts` as the public front door
- extract exported types into `ts/src/patterns/media-workflow/types.ts`
- extract browse helpers into `ts/src/patterns/media-workflow/browse.ts`
- extract generic upload workflow into `ts/src/patterns/media-workflow/upload.ts`
- extract media pipeline helpers into `ts/src/patterns/media-workflow/pipeline.ts`
- extract upload plan normalization into `ts/src/patterns/media-workflow/plan.ts`
- preserve existing runtime media exports

Out of scope:

- changing public media workflow APIs
- changing upload behavior
- changing consumer apps

## Acceptance Criteria

- public `../patterns/media-workflow` imports continue to compile
- runtime media exports continue to compile
- media upload-flow tests pass
- focused media workflow tests pass if added
- `effigy qa:docs` passes
- roadmap artifact records final module shape and public API impact

## Consumer Upgrade Impact

Expected impact: none.

This should be an internal split. If consumer imports or media workflow
behavior need to change, stop and re-enter planning.

## Current State

`g06.147` is ready.

## Next Task

Execute `g06.147`: media workflow internal split.
