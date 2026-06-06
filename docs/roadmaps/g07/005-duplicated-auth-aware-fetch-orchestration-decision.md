# g07.005 - Duplicated Auth-Aware Fetch Orchestration Decision

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.004` narrowed the pattern root and left lower list, reorder, selection,
and pagination helpers under `runtime/data`.

Several retained helpers now sit near the same workflow boundary: authenticated
data loading, list controllers, pagination controllers, and template-owned list
or form pages. This card decides whether auth-aware fetch orchestration should
stay split across those helpers or move toward one clearer runtime contract.

## Goals

- [x] inventory auth-aware fetch orchestration in `runtime/auth`,
  `runtime/data`, client query helpers, and templates
- [x] identify duplicated lifecycle behavior, especially loading/error/retry
  state and auth readiness handling
- [x] decide whether to consolidate, document the split, or queue a bounded
  implementation card
- [x] classify any consumer impact before changing public imports or behavior

## Non-Goals

- broad data-fetching rewrite
- changing API transport contracts
- replacing app-owned domain fetch functions
- changing Rust APIs
- consumer migration without same-card proof

## Execution Plan

- [x] inspect `useAuthenticatedData`, list controllers, pagination helpers, and
  template data-loading seams
- [x] compare the observed behavior to contracts `090`, `100`, and `110`
- [x] scan active docs and consumers for direct usage
- [x] write a decision artifact with one retained posture and bounded follow-on
  work if needed

## Acceptance Criteria

- [x] duplicated auth-aware fetch behavior is either accepted, consolidated, or
  queued with a clear boundary
- [x] each affected helper keeps one explicit owner
- [x] consumer-visible impact is classified under contract `023`
- [x] no behavior change lands without targeted validation evidence

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`
- targeted source and consumer scans

## Consumer Upgrade Impact

None.

The public helper names, import paths, and option shapes are unchanged.
Auth-token lookup, 401 refresh retry, and auth-ready gating were consolidated
behind internal code only.

## Next Task

Move to `g07.006`: list, pagination, reorder, and template seam audit.
