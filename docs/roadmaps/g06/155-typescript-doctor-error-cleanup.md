# g06.155 - TypeScript Doctor Error Cleanup

## Why

`g06.154` found that high-severity god-files are cleared, but `effigy doctor`
still fails on two TypeScript structural errors:

- one high attention marker in `ts/src/client/navigation.ts`
- one high comment-ratio finding in `ts/src/client/route-protection.ts`

## Goal

Clear the remaining TypeScript doctor error findings without changing runtime
behavior.

## Scope

In scope:

- remove or reclassify the deprecated marker on `navigateOnCancel(...)`
- trim redundant comments in `ts/src/client/route-protection.ts`
- preserve public exports and behavior
- run targeted tests if available
- run `effigy doctor`

Out of scope:

- changing navigation behavior
- changing route-protection behavior
- cleaning all warning-level doctor findings
- changing consumer apps

## Acceptance Criteria

- `effigy doctor` has no `scan.attention-markers` error finding
- `effigy doctor` has no `scan.comment-ratio` error finding
- no runtime behavior changes are introduced
- roadmap artifact records the final doctor state

## Consumer Upgrade Impact

Expected impact: none.

This should be comment/marker cleanup only.

## Current State

`g06.155` is ready.

## Next Task

Execute `g06.155`: TypeScript doctor error cleanup.
