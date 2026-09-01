# Contract: Database Migration and Schema Workflow

Status: active
Owner: repo maintainers
Depends on: `024-new-app-bootstrap-and-bring-up.md`, `025-rust-app-runtime-assembly-and-router-topology.md`, `070-nightfire-and-migration-systems.md`, `120-tooling-testing-and-contract-artifacts.md`

## Purpose

Define the normal authoring and rollout workflow for database schema changes in
Underlay app workspaces.

This contract covers:

- migration directory layout
- durable schema migrations versus dev-only seed overlays
- migration naming posture
- local authoring and replay workflow
- reset, replay, and verification posture
- when richer migration/state systems are optional rather than required

It does not define the lower migration-core protocol. That stays with `070`.
It does not define product-specific transform semantics or legacy import logic.

## Sources of Truth

Reference consumer evidence:

- `underlay-reference` — `apps/acme-api/effigy.toml`,
  `apps/acme-api/migrations/`, and `apps/acme-api/migrations_dev/`
- `compli-me` — `apps/api/effigy.toml`, `apps/api/migrations/`, and
  `apps/api/migrations_dev/`
- `contact-patch` — `apps/cp-api/effigy.toml`, `apps/cp-api/migrations/`, and
  `apps/cp-api/migrations_dev/`
- `songsprout` — `apps/nursery/effigy.toml`, `apps/nursery/migrations/`, and
  `apps/nursery/migrations_dev/`
- `acowtancy` — `apps/farmyard/effigy.toml` and its advanced state/replay
  surfaces

Supporting shared contracts:

- [`024-new-app-bootstrap-and-bring-up.md`](./024-new-app-bootstrap-and-bring-up.md)
- [`025-rust-app-runtime-assembly-and-router-topology.md`](./025-rust-app-runtime-assembly-and-router-topology.md)
- [`070-nightfire-and-migration-systems.md`](./070-nightfire-and-migration-systems.md)
- [`120-tooling-testing-and-contract-artifacts.md`](./120-tooling-testing-and-contract-artifacts.md)
- [`Migration State Layout And Effigy`](../usage/migration/000-state-layout-and-effigy.md)

If these diverge, the contract plus the clearest modern posture
(`underlay-reference`, `compli-me`, `contact-patch`) win. `farmyard` remains
the richer proof for advanced state and replay work, not the baseline for every
app.

## Contract Goal

Underlay should make schema work predictable.

A normal app should not have to rediscover:

- where structural migrations live
- where dev-only seed overlays live
- how to reset and replay a local database
- when to use package-owned `migration:*` tasks versus richer state tooling
- what proof is expected before merging a schema change

The goal is one boring schema workflow for normal apps, with richer migration
systems layered on only when the product actually needs them.

## Scope Boundary

In scope:

- schema migration file layout
- dev seed overlay layout
- local migration/reset tasks
- migration naming
- forward-only authoring posture
- verification and proof expectations

Out of scope:

- full legacy import pipelines
- migration bundle OCI policy in depth
- app-specific reconciliation logic
- deploy-time production runbooks

## Shared Boundary

### Structural migration rule

Durable schema changes belong in the API package `migrations/` directory.

Rules:

- structural schema migrations are part of the durable app state
- they should be replayable from an empty database
- they should not depend on local dev data being present
- new apps should start with one clear `migrations/` root in the API package

### Dev overlay rule

Dev-only seed and convenience data belongs in a separate dev overlay lane.

Allowed patterns today:

- `migrations_dev/`
- richer state/dev-seed layering through Effigy when the app already needs it

Rules:

- dev overlays must not be mixed into durable structural migrations
- dev overlays may create test users, sample content, or convenience records
- production correctness must not rely on `migrations_dev/` or equivalent

### Naming rule

Migration filenames must be explicit, ordered, and human-scannable.

Observed valid patterns:

- `202601301200__baseline_schemas.sql`
- `202602241700__add_auth_security_alert_events.sql`
- `20260125000000_baseline.sql`

Rules:

- use ordered timestamp prefixes
- include a short semantic suffix after `__` or `_`
- avoid anonymous names like:
  - `update.sql`
  - `fix.sql`
  - `migration_2.sql`
- keep names specific to the schema change or baseline they introduce

### Forward-only rule

Normal app schema work is forward-only.

Rules:

- add new migrations; do not rewrite old committed migrations casually
- repair history with a new migration unless the migration is still strictly
  local and unshared
- baseline migrations are allowed for new apps and explicit reset points, but
  they should remain intentional and named as such

### Root state and migration routing rule

The root workspace owns local state-stack orchestration:

- `effigy state plan`
- `effigy state apply local --yes`

Schema migration execution still belongs to the API package. The API package
exposes its operations through `migration:*` tasks, and the workspace root
routes those tasks through the package catalog. The root must not grow a second
schema workflow around them.

Rules:

- use the root state stack for local database and seed state
- use the routed API-package `migration:*` front door for schema work
- do not reintroduce root or package `db:migrate`, `db:reset`, or `db:drop`
  aliases
- root orchestration must not fork the schema workflow from the API package

### API package task rule

The API package must expose the boring local schema loop under a
`migration:*` namespace. Concrete task names remain package-owned; a package
may provide reset, generate, debug, or test-database variants as its workflow
requires.

Rules:

- the package-owned migration task applies structural migrations
- its reset/replay task returns local dev state to the declared baseline plus
  any local dev overlay the app intentionally includes
- test DB reset flows may exist separately from dev DB reset flows
- package task names must not recreate the retired `db:*` aliases

### Local workflow rule

Normal local schema work should follow this loop:

1. author a new migration file
2. run the routed API-package `migration:*` task that applies it
3. run the package-owned reset/replay task when a clean replay is needed
4. use `effigy state plan` and `effigy state apply local --yes` when the
   workspace state stack is part of the local baseline
5. run `effigy health` or package validation after the schema change

Rules:

- schema work should be replay-tested, not just applied incrementally
- reset must remain cheap enough for developers to trust it
- the local loop should not require production-like migration bundle machinery
  unless the app genuinely uses those systems

### Rich state and replay rule

Richer state, replay, capture, and legacy import tooling is optional.

Rules:

- apps like `farmyard` may add:
  - state layers
  - seed bundle generation
  - replay artifacts
  - migration handoff reports
- these are advanced migration/state profiles, not the baseline requirement for
  every app
- new normal apps should start with the simpler schema plus dev-overlay model
  unless there is a clear migration/state reason to do more

### Verification rule

Every schema change should prove at least:

- migrations apply cleanly from the current dev baseline
- reset and replay from empty state still work
- the app still builds or passes its local health baseline

Stronger proof is expected when the change touches:

- auth
- account
- media/storage
- jobs/operator tables
- shared runtime systems

### SQL versus helper boundary

Normal app schema changes should default to explicit SQL migration files.

Rules:

- use shared migration helpers and devtools for execution, replay, reporting,
  and richer state operations
- do not hide ordinary schema changes inside app-local binaries unless the app
  already has a declared advanced migration/state reason
- app-owned migration binaries are allowed for execution surfaces, but the
  durable schema change itself should remain visible as migration input

### Dynamic identifier rule

Runtime SQL values must stay bound parameters. Runtime SQL identifiers must be
validated and quoted through the shared DB identifier boundary.

Rules:

- new shared DB-adjacent APIs should prefer `SqlIdentifier` and
  `QualifiedTableName`
- raw schema, table, and column string helpers are compatibility surface only
  when a roadmap explicitly retains them
- existence-check code must use `TypedExistsCheck` for composite constraints
  and the typed value-exists helpers for simple constraints
- destructive schema helpers must parse schema names as `SqlIdentifier` and
  quote them before SQL execution
- dynamic table names in audit, security alert, media, existence, and dev/test
  helpers must not be interpolated without validation and quoting
- audit and security-alert callers must use the typed table config APIs; the
  raw-string operator wrappers were removed after the six-consumer proof

## Assessment State

`g09.035` assessed the current Underlay implementation and all six consumer
workspaces on 2026-08-26. Full evidence is in
[`g09.035 - Database Migration Contract Assessment`](../logs/2026-08/26-162845-g09-035-database-migration-assessment.md).

Initial verdict: `drifting`.

Confirmed matches:

- all structural and dev-overlay SQL roots are separate and explicitly named
- all 96 inspected SQL filenames are ordered and semantic
- normal schema changes remain visible SQL inputs
- Underlay's destructive, existence, audit, security-alert, media, reorder, and
  test helpers retain typed dynamic identifier boundaries
- Acowtancy remains a justified advanced profile rather than the normal baseline

Confirmed drift:

- the five baseline consumers lack the declared root local state stack and
  retain the retired package-owned `db:*` task namespace
- active consumer docs still teach the retired task loop
- Acowtancy's local state artifact hook installs the canonical bundle without
  invoking its DB apply/dev-seed path
- Songsprout and Composer can continue after dev-overlay failure; Composer's
  reset omits its committed dev overlay

These were implementation gaps, not a contract-policy split. `g09.037`–`g09.044`
repaired the five baseline roots, Acowtancy's advanced state path, fail-closed
overlays, active guidance, and merge-gate reachability.

Repair state: `conforming`. The final six-root matrix and retained residuals are
in
[`g09.044 - Migration And Testing Fleet Closeout`](../logs/2026-08/26-222718-g09-044-migration-testing-fleet-closeout.md).

## What Good Looks Like

Good outcomes:

- one obvious `migrations/` root in the API package
- dev-only seed overlays are separate
- the API package owns a clear `migration:*` front door
- schema work is replay-tested, not only incrementally applied
- advanced replay and bundle systems stay explicit instead of leaking into the
  normal baseline

Bad outcomes:

- structural and dev-only data are mixed together
- old migrations are rewritten casually
- reset and replay posture differs between root and API package
- a simple app is forced into migration-bundle complexity without need
- schema proof depends on tribal knowledge instead of the declared task loop

## Questions This Contract Should Settle

- What is the normal schema workflow for a new Underlay app?
- When is `migrations_dev/` or similar allowed?
- When does an app need advanced state/replay tooling instead of the baseline
  migration loop?
- What proof is required before a schema change is considered safe?

## Next Task

Assess bootstrap, runtime assembly, and access-model contracts through
`g09.045`.
