# g10.015 - Database Migration Contract Assessment

Date: 2026-08-26
Card: `g10.015`
Contract: `021-database-migration-and-schema-workflow.md`
Verdict: `drifting`

## Scope And Method

Read-only assessment of:

- Underlay DB, devtools, migration, state-layout, and typed SQL identifier
  surfaces named by contract `021`
- the root Effigy surface and affected API package in each consumer workspace
- current structural migration and dev-overlay trees
- task definitions, reset/migrate implementations, active root/package docs,
  and state manifests

No database, state stack, consumer file, or production source was mutated. Task
evidence came from `effigy --json tasks` and deterministic file inspection.

## Shared Implementation Evidence

| Boundary | Evidence | Verdict |
| --- | --- | --- |
| Structural migrations | Normal apps embed their package migration root with `sqlx::migrate!`; Acowtancy embeds `state/schema` through its advanced Farmyard DB boundary | match |
| Dev overlays | `underlay_db::run_sql_dir` sorts SQL files and returns read/SQL failures; all six roots keep dev SQL separate from structural SQL | match |
| Destructive reset | `underlay_devtools::reset_schemas` routes schema names through `underlay_db::drop_schemas`; `SqlIdentifier` validates and quotes before `DROP SCHEMA` | match |
| Dynamic identifiers | `QualifiedTableName`, `SqlIdentifier`, `TypedExistsCheck`, typed audit/security-alert tables, media/reorder helpers, and `TestDb` own the dynamic identifier seams named by `021` | match |
| Advanced state | `underlay-migration-core` and devtools remain optional support; app-owned transforms and state hooks stay outside the normal baseline | match |

## Fleet Matrix

Counts are current SQL files in the structural/dev roots.

| Consumer | Profile | Layout and naming | Root state | Package task front door | Replay and overlay behavior | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| Underlay Reference | baseline | `apps/acme-api/migrations` (8), `migrations_dev` (4); all ordered semantic names | no state manifest | `db:drop`, `db:migrate`, `db:reset` | reset drops, migrates, and fails if dev seeds fail | drifted |
| Contact Patch | baseline | `apps/cp-api/migrations` (12), `migrations_dev` (4); all ordered semantic names | no state manifest | `db:drop`, `db:migrate`, `db:reset` | reset drops, migrates, and fails if dev seeds fail | drifted |
| Compli Me | baseline | `apps/api/migrations` (9), `migrations_dev` (2); all ordered semantic names | no state manifest | `db:drop`, `db:migrate`, `db:reset` | reset drops, migrates, and fails if dev seeds fail | drifted |
| Songsprout | baseline | `apps/nursery/migrations` (25), `migrations_dev` (1); all ordered semantic names | no state manifest | `db:drop`, `db:migrate`, `db:reset` | migration logs dev-seed failure and still returns success | drifted |
| Composer | baseline | `apps/composer-api/migrations` (15), `migrations_dev` (1); all ordered semantic names | no state manifest | `db:migrate`, `db:reset` | reset applies structural migrations only; runtime seed failure is non-fatal | drifted |
| Acowtancy | advanced | `state/schema` (14), `state/dev-seeds` (1); all ordered semantic names | local/UAT/production stacks declared | `migration:*` plus explicit `state:*` seams | `migration:reset` and `state:reset` own full replay, but root `state apply local` only migrates schema and installs the bundle artifact; its hook does not call the existing bundle/dev-seed DB apply path | advanced, drifted |

No root or package `package.json` recreates the retired DB aliases. The drift is
in child Effigy tasks, active docs, and replay/state wiring.

## Clause Matrix

| Contract clause | Fleet evidence | Verdict | Disposition |
| --- | --- | --- | --- |
| Structural migration rule | Five baseline apps use one API-owned `migrations/` root; Acowtancy uses the declared advanced `state/schema` variant | match | none |
| Dev overlay rule | Every consumer separates dev SQL; structural migrators embed only the structural directory | match | none |
| Naming rule | All 96 inspected SQL filenames use ordered timestamp prefixes and semantic suffixes | match | none |
| Forward-only rule | Current trees are additive and ordered; Songsprout also states the policy explicitly. Static inspection cannot prove future author behavior | match, medium confidence | retain contract authority; no repair from current evidence |
| Root state and migration routing | Only Acowtancy declares a root state stack; the five baseline workspaces cannot provide the contracted `state plan` / `state apply local` loop | drift | findings-driven consumer rollout |
| API package task rule | Five baseline APIs retain explicitly retired `db:*` selectors; Acowtancy uses `migration:*` | drift | findings-driven consumer rollout |
| Local workflow rule | Five root/package docs teach `db:*`; several diagnostic strings still name Bun/pnpm DB commands that do not exist | drift | repair task/docs atomically; no compatibility aliases |
| Rich state and replay rule | Acowtancy is correctly isolated as the advanced profile, but its root local-state artifact hook stages the canonical bundle without applying it to the DB | drift | bounded Acowtancy implementation repair |
| Verification rule | Reference, Contact Patch, and Compli Me fail closed on dev-seed errors. Songsprout and Composer do not; Composer reset also omits its intentional overlay | drift | bounded Songsprout/Composer repairs |
| SQL versus helper boundary | Durable changes remain visible SQL inputs; app binaries and shared helpers orchestrate them | match | none |
| Dynamic identifier rule | Shared destructive, existence, audit, security-alert, media, reorder, and test seams validate typed identifiers and bind runtime values | match | none |

## Findings And Candidates

### A. Baseline migration workflow rollout

- Pressure: five normal workspaces contradict the active task/state contract.
- Consequence: the reference fixture and fleet teach a workflow Underlay has
  already retired; new apps cannot copy a contract-conformant baseline.
- Improvement: prove the simple root state stack and `migration:*` package
  surface in Underlay Reference, then roll the same boundary through Contact
  Patch, Compli Me, Songsprout, and Composer.
- Rejected alternative: restore `db:*` in contract `021`. That reverses
  `g10.002` and the chosen Acowtancy proof. Compatibility aliases are also
  explicitly forbidden.
- Risk/cost: medium; selector and docs cutover is broad but source-compatible
  application behavior can be preserved.
- Validation: task inventory, state plan, reset/replay proof in disposable local
  state, workspace health, and retired-selector search.
- Promotion: findings-driven roadmap wave after `g10.016`.
- Confidence: high.

### B. Acowtancy local state application completeness

- Pressure: `state apply local` resets/migrates schema and stages the canonical
  bundle, but `seed-bundle-state-hook.rhai` never invokes Farmyard's existing
  `db apply-seed-bundles-dev` path.
- Consequence: bootstrap can report successful state application while leaving
  canonical bundle rows and dev seeds unapplied.
- Improvement: make the artifact layer hook or a following task layer apply the
  installed bundle and dev overlay once, with reportable failure.
- Rejected alternative: redefine root state apply as staging-only. The root
  README and contracts already define it as applied local state.
- Risk/cost: medium-high; destructive bootstrap/state behavior requires a
  disposable database and idempotent replay proof.
- Validation: state plan plus from-empty local apply, row/invariant proof,
  repeated apply, and existing `migration:reset`/`state:reset` comparison.
- Promotion: bounded Acowtancy roadmap card after `g10.016`.
- Confidence: high from static flow inspection; runtime proof remains for the
  repair card.

### C. Fail-closed dev overlay semantics

- Pressure: Songsprout logs seed errors and succeeds; Composer reset omits the
  overlay while API startup logs seed errors and continues.
- Consequence: local reset/start can appear healthy without the declared dev
  baseline.
- Improvement: make intentional dev-overlay application part of reset/replay
  and propagate failure. Keep the repair app-owned; `run_sql_dir` already fails
  correctly.
- Rejected alternative: weaken the shared helper or silently remove the
  overlays. Both apps retain committed dev SQL as intentional state.
- Risk/cost: low-medium; local-only behavior, with possible fixture assumptions.
- Validation: focused reset/replay against disposable DBs plus health.
- Promotion: bounded Songsprout and Composer repair cards after `g10.016`.
- Confidence: high.

## Architecture Verdict

`drifting`.

The contract boundary is coherent and the normal-versus-advanced distinction
holds. The implementation fleet has not adopted the task/state posture promoted
by `g10.002`, and three replay paths can report success without fully applying
their declared local state.

## Operator Decisions

None required to continue assessment. The findings do not change migration
policy or the API proof bar. Repair grouping and order stay deferred until the
testing assessment closes.

Implementation authorized: no. This assessment changes only Underlay planning
and evidence state.

## Next Route

Execute `g10.016`, the read-only testing posture assessment. Then compile one
findings-driven repair wave across both assessments.
