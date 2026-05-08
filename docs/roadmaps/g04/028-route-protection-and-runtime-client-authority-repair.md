# 028 - Route Protection And Runtime Client Authority Repair

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.027` assessed the retained TS runtime and client orchestration layer
against `090`.

The next repair is bounded:

- `client/route-protection.ts` does not honor the exact-match contract for
  plain public paths
- `runtime/data` and `client/types` still need a more honest authority story

## Goals

- fix route-protection matching so non-wildcard public paths are exact matches
- add focused validation for that behavior
- tighten the `090` wording around `runtime/data` and `client/types` so it
  matches the real retained surface

## Non-Goals

- broad runtime barrel breakup in the same batch
- redesigning app route layouts or permission systems
- skipping ahead to shared-pattern assessment before the runtime/client repair
  is explicit

## Inputs

- [docs/roadmaps/g04/027-ts-runtime-and-client-orchestration-assessment.md](/Users/tom/Dev/projects/underlay/docs/roadmaps/g04/027-ts-runtime-and-client-orchestration-assessment.md)
- [docs/contracts/090-ts-runtime-and-client-orchestration.md](/Users/tom/Dev/projects/underlay/docs/contracts/090-ts-runtime-and-client-orchestration.md)
- `ts/src/client/route-protection.ts`
- `ts/src/runtime/data.ts`
- `ts/src/client/types.ts`

## Exit Criteria

- route-protection behavior matches the shared contract
- tests cover exact-match and wildcard public-path behavior
- `090` describes the retained runtime/client authority more honestly
- the next shared-pattern assessment can treat the runtime/client layer as
  stable lower context

## Changes

- fixed `client/route-protection.ts` so plain public paths are exact matches
  and only `/*` entries act as prefix matches
- tightened the focused route-protection test to cover the accidental widening
  cases like `/login-help` and `/health/check`
- updated `090` so it describes `runtime/data` as the broad compatibility
  barrel it really is and names `client/types.ts` as a pragmatic holding area
  rather than a tiny pure helper slice

## Result

The runtime/client repair is now honest and bounded:

- route-protection behavior matches the documented contract
- the tests cover the specific mismatch that triggered the lane
- the TS runtime/client contract no longer overstates how cleanly sliced
  `runtime/data` and `client/types.ts` are

## Next Task

Execute `g04.029`: assess the shared patterns and workflow shells against
`100`.
