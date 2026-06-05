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

- [`underlay-reference/acme-api/effigy.toml`](/Users/tom/Dev/projects/underlay-reference/acme-api/effigy.toml)
- [`underlay-reference/acme-api/migrations/`](/Users/tom/Dev/projects/underlay-reference/acme-api/migrations)
- [`underlay-reference/acme-api/migrations_dev/`](/Users/tom/Dev/projects/underlay-reference/acme-api/migrations_dev)
- [`compli-me/api/effigy.toml`](/Users/tom/Dev/projects/compli-me/api/effigy.toml)
- [`compli-me/api/migrations/`](/Users/tom/Dev/projects/compli-me/api/migrations)
- [`compli-me/api/migrations_dev/`](/Users/tom/Dev/projects/compli-me/api/migrations_dev)
- [`contact-patch/cp-api/effigy.toml`](/Users/tom/Dev/projects/contact-patch/cp-api/effigy.toml)
- [`contact-patch/cp-api/migrations/`](/Users/tom/Dev/projects/contact-patch/cp-api/migrations)
- [`contact-patch/cp-api/migrations_dev/`](/Users/tom/Dev/projects/contact-patch/cp-api/migrations_dev)
- [`songsprout/nursery/effigy.toml`](/Users/tom/Dev/projects/songsprout/nursery/effigy.toml)
- [`songsprout/nursery/migrations/`](/Users/tom/Dev/projects/songsprout/nursery/migrations)
- [`loophole/composer/composer-api/effigy.toml`](/Users/tom/Dev/projects/loophole/composer/composer-api/effigy.toml)
- [`loophole/composer/composer-api/migrations/`](/Users/tom/Dev/projects/loophole/composer/composer-api/migrations)
- [`acowtancy/farmyard/effigy.toml`](/Users/tom/Dev/projects/acowtancy/farmyard/effigy.toml)
- [`docs/usage/migration/000-state-layout-and-effigy.md`](/Users/tom/Dev/projects/underlay/docs/usage/migration/000-state-layout-and-effigy.md)

Supporting shared contracts:

- [`docs/contracts/024-new-app-bootstrap-and-bring-up.md`](/Users/tom/Dev/projects/underlay/docs/contracts/024-new-app-bootstrap-and-bring-up.md)
- [`docs/contracts/025-rust-app-runtime-assembly-and-router-topology.md`](/Users/tom/Dev/projects/underlay/docs/contracts/025-rust-app-runtime-assembly-and-router-topology.md)
- [`docs/contracts/070-nightfire-and-migration-systems.md`](/Users/tom/Dev/projects/underlay/docs/contracts/070-nightfire-and-migration-systems.md)
- [`docs/contracts/120-tooling-testing-and-contract-artifacts.md`](/Users/tom/Dev/projects/underlay/docs/contracts/120-tooling-testing-and-contract-artifacts.md)

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
- when to use plain `db:migrate` versus richer state tooling
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

### Root task rule

The root workspace may expose:

- `effigy db:migrate`
- `effigy db:reset`

But DB ownership still belongs to the API package.

Rules:

- root DB tasks should resolve through child-catalog routing
- the API package owns the actual reset/migrate implementation
- root orchestration must not fork the schema workflow from the API package

### API package task rule

The API package must expose the boring local schema loop.

Expected tasks:

- `db:migrate`
- `db:reset`
- optional `db:drop`
- optional test DB variants when the app has managed DB-backed tests

Rules:

- `db:migrate` should apply structural migrations
- `db:reset` should reset local dev state to the declared baseline plus any
  local dev overlay the app intentionally includes
- test DB reset flows may exist separately from dev DB reset flows

### Local workflow rule

Normal local schema work should follow this loop:

1. author a new migration file
2. run `effigy db:migrate`
3. run `effigy db:reset` when a clean replay is needed
4. run `effigy health` or package validation after the schema change

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

## What Good Looks Like

Good outcomes:

- one obvious `migrations/` root in the API package
- dev-only seed overlays are separate
- `db:migrate` and `db:reset` exist everywhere
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

Use this contract as the baseline for new app schema work, then layer richer
state or legacy-import systems only when the app genuinely needs them.
