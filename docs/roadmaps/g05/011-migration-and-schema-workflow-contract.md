# g05.011 — Migration And Schema Workflow Contract

## Why

Migration and schema work is still one of the easiest places for consumer apps
to drift:

- migration naming
- SQL versus helper boundaries
- bundle and replay posture
- verification, rollback, and recovery rules
- proof expectations before rollout

Underlay has strong lower migration systems already. What is missing is the
delivery contract that tells app teams how to use them.

## Goal

Write the shared migration and schema workflow contract for normal Underlay app
development.

## Audit Readout

The repeated baseline across the current API family is clearer than the apps'
different domain histories make it look.

Normal repeated posture:

- structural schema lives in `migrations/`
- dev-only overlay data is separate:
  - `migrations_dev/` in the cleaner app families
- API package owns:
  - `db:migrate`
  - `db:reset`
- root workspace DB tasks route through the API package instead of forking the
  workflow
- config layering is explicit through committed defaults, local override, and
  env

The strongest baseline proofs are:

- `underlay-reference`
- `compli-me`
- `contact-patch`
- `nursery`

`composer` proves the same baseline can use a devtools binary instead of a
package-specific DB binary without changing the workflow shape.

`farmyard` remains the rich-state proof:

- seed bundles
- replay artifacts
- capture and handoff state
- migration/runtime reports

That is real and valuable, but it is not the baseline new-app requirement.

## Scope

Primary targets:

- migration naming and file layout
- forward-only versus repair posture
- raw SQL versus helper boundaries
- local dev workflow
- replay, verification, rollback, and recovery expectations
- bundle/report expectations where those systems are in play
- consumer proof rules before rollout

Likely outputs:

- one new contract
- one or more checklist or artifact updates under `contracts/` or `docs/guides/`

## Contracts Landed In This Lane

### 021 — Database migration and schema workflow

Landed.

Defines:

- durable `migrations/` ownership
- dev overlay separation
- migration naming posture
- forward-only baseline
- `db:migrate` / `db:reset` ownership
- local replay and proof expectations
- when advanced state and replay systems are optional rather than baseline

## Consumer Upgrade Impact

Expected:

- clearer migration authoring rules
- stricter verification posture
- clearer review expectations for schema changes

## Next Task

Use `021` as the source of truth for the next delivery-layer contracts instead
of reopening migration authoring folklore again.
