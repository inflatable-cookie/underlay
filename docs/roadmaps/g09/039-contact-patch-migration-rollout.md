# g09.039 - Contact Patch Migration Rollout

Status: complete
Owner: Contact Patch maintainers
Contracts: `021-database-migration-and-schema-workflow.md`,
`022-testing-posture-and-shared-harnesses.md`
Found by: `g09.035`, `g09.036`
Depends on: `g09.038`

## Purpose

Roll the proven baseline migration workflow through Contact Patch without
disturbing its already-strong API and front test posture.

## Promotion Gate

- [x] `g09.038` is complete and its baseline state/task shape is recorded
- [x] Contact Patch `main` is current at `3c85a5e57ce29af448c338f7fd29ad9e45d72ac8`
- [x] a disposable local PostgreSQL target is available and positively identified

Readiness checked 2026-08-26: system `contact-patch-dev`, container
`contact-patch-dev-postgres-1`, database `contact_patch`, user `postgres`, host
binding `127.0.0.1:24532`, volume `contact-patch-dev-postgres-data`, no shared
services, and the database accepts connections. Re-prove every fact before the
first mutation.

## Scope

- add the root local state stack
- rename `apps/cp-api` DB selectors to package-owned `migration:*` selectors
- remove retired `db:*` selectors and active docs references without aliases
- preserve structural/dev-overlay separation and fail-closed replay
- keep the existing API managed DB proof and front routing-test gates intact
- do not promote the admin suite merely to make every package strong

## Acceptance

- [x] root state plan/apply routes through the API-owned migration lifecycle
- [x] from-empty reset/replay applies structural migrations and the dev overlay
- [x] overlay failure fails the owning task and root state apply
- [x] no active task or guide advertises a retired `db:*` selector
- [x] existing Contact Patch API and front strong gates retain their current
  teardown and failure semantics

## Validation

- `effigy tasks`
- `effigy state plan`
- `effigy state apply local --yes` against the disposable database
- routed `cp-api/migration:*` reset/replay proof
- `effigy cp-api/health`
- `effigy cp-api/qa`
- `effigy health`
- `effigy validate`
- `effigy qa`
- retired-selector/docs search

## Stop Conditions

Stop before mutation if the database is not disposable. Stop if the task rename
would weaken the managed test DB teardown or introduce a root-owned schema
workflow.

## Consumer Upgrade Impact

- Impact class: breaking local task-interface cutover
- Affected consumer: Contact Patch
- Required action: replace `db:*` calls with the routed API `migration:*`
  selectors and use root state plan/apply
- Compatibility window: none

## Completion Evidence

Completed 2026-08-26 through
[Contact Patch PR4](https://github.com/contact-patch/contact-patch/pull/4).
The reviewed worker head was
`557c4dc2bc728711ca8d49a3a75b410ec34dfb99`; the squash merge commit is
`8d5b6f4c463eb4bcdef4e2c60fb16d4cc878c8df`.

Post-merge verification on current Contact Patch `main` confirmed the four
package-owned `migration:*` selectors, the reset -> structure -> dev-overlay
state plan, API health, docs QA, Northstar QA, and a clean diff. Destructive
from-empty, replay, and forced-overlay-failure evidence remains in the target
execution log at
`docs/logs/2026-08/26-210145-g09-039-contact-patch-migration-rollout.md`.

## Next Task

This lane is closed. `g09.040`–`g09.043` continue independently; keep
`g09.044` blocked until all four remaining proofs merge.
