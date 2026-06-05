# g06.007 - Devtools Bundle Store Boundary Isolation

## Why

The Rust audit split migration bundle behavior into media shard, remote
registry, and local store modules, and added `MigrationBundleRef`.

That improved local reasoning, but devtools is still a public tooling crate.
The next step is to freeze which bundle/store APIs are intended for apps and
which are internal tooling details.

## Goal

Keep `underlay-devtools` as a tooling-only boundary, prove bundle reference
parsing is the public construction seam, and prevent local/remote store details
from leaking into runtime app contracts.

## Scope

In scope:

- audit `underlay-devtools` public exports against `120` and `122`
- classify migration bundle refs, run options, local store helpers, and remote
  registry helpers
- keep bundle/store modules private unless they are deliberate public tooling
  APIs
- run targeted devtools tests and Rust checks

Out of scope:

- changing migration bundle wire format
- changing OCI registry behavior
- moving devtools into runtime app crates
- broad migration-core redesign

## Contract References

- `070`: Nightfire and migration systems
- `120`: tooling, testing, and contract artifacts
- `023`: release and compatibility rollout
- `122`: Rust public API inventory

## Consumer Upgrade Impact

Impact classification: `additive` / `internal`.

Any public devtools API removal is `deprecation` or `breaking` and needs
consumer proof before landing.

## Acceptance Criteria

- devtools public exports are inventoried
- `MigrationBundleRef` is the preferred ref construction boundary
- local store and remote registry internals are classified
- targeted devtools validation passes

## Public Surface Inventory

Stable tooling surface:

- DB env and connection helpers:
  - `require_env`
  - `connect`
  - `migrate`
  - `migrate_with`
  - `reset_schemas`
  - `migrate_from_env`
  - `migrate_from_env_with`
  - `reset_from_env`
- migration sync:
  - `sync_migrations`
  - `SyncMigrationsError`
  - `SyncMigrationsReport`
- migration bundle operations:
  - `migration_bundle_build`
  - `migration_bundle_publish`
  - `migration_bundle_pull`
  - `migration_run`
  - `Bundle*Options`
  - `Bundle*Report`
  - `MigrationBundleError`
  - `MigrationBundleRef`
- seed bundle operations:
  - `seed_bundle_build`
  - `seed_bundle_publish`
  - `seed_bundle_pull`
  - `SeedBundle*Options`
  - `SeedBundle*Report`
- migration report load, format, build, and write helpers

Private implementation modules:

- `migration_bundle/local_store.rs`
- `migration_bundle/remote_registry.rs`
- `migration_bundle/media_shards.rs`
- `migration_report.rs` module path
- `seed_bundle.rs` module path
- `sync_migrations.rs` module path

Classification:

- `underlay-devtools` is a tooling crate, not a runtime app contract.
- `MigrationBundleRef` is the preferred digest-pinned bundle-ref construction
  boundary for migration run paths.
- `BundleRunOptions::from_bundle_ref` lets new callers stay on the typed ref
  boundary.
- `BundlePublishOptions` and `BundlePullOptions` keep raw `oci_ref` because
  publish/pull intentionally support tag refs and digest refs.
- local store and remote registry behavior remain internal routing details
  behind publish/pull/run.

## Code Changes

- Added `BundleRunOptions::from_bundle_ref`.
- Added a focused test for typed bundle-ref run options.
- Extended `120` with the bundle/store boundary rule.

## Validation

- `cargo test -p underlay-devtools --all-features migration_bundle_ref`
- `cargo test -p underlay-devtools --all-features bundle_run_options_accept_typed_bundle_ref`
- `cargo test -p underlay-devtools --all-features migration_bundle_publish_and_pull_round_trip_from_local_store`
- `cargo clippy -p underlay-devtools --all-features --all-targets -- -D warnings`
- `git diff --check`

## Current State

`g06.007` is complete.

## Next Task

Execute `g06.008`: six-consumer compatibility proof and release-note closeout.
