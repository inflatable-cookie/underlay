# 2026-07-18 - g08 quality audit (post-merge)

## Scope and method

Full post-merge audit of all 32 g08 cards. Method: fresh mechanical validation
(fmt, clippy `-D warnings`, workspace unit tests, `effigy validate`, the 17
Postgres adapter integration tests, main-branch CI), plus independent
code-level verification of every Lane A/B/C card claim against source
(fix + test evidence per card), a fleet-consistency check, and a footgun hunt.

## Mechanical validation (all green)

- `cargo fmt --check`, `cargo clippy --workspace --all-targets -D warnings`
  (including `--features postgres`), `cargo test --workspace` (75 suites).
- `effigy validate`: svelte-check 0 errors (2472 files), 742 TS unit + 33
  component tests, guardrails/hygiene/prop-name checks.
- Main CI green post-merge (fmt, clippy, build, unit, adapter tests vs
  `postgres:16`).

## Claim verification results

- **Lane A 001–007, Lane B 011–014, Lane C 015–018/020: CONFIRMED** — every
  claimed fix exists in source with test evidence. No TODO/FIXME/unimplemented
  markers anywhere in `rust/crates` or `ts/src`.
- **Three Lane A cards had checked boxes overstating in-tree coverage** (all
  code fixes were real; the gaps were tests/scope):
  1. **008**: "distributed backend enforces a shared window across simulated
     instances" had no in-tree test — it was deferred pending a live Postgres,
     which `g08.019` later provisioned but the test was never backfilled.
  2. **009**: "timeout fires" had no test — defaults were set but unexercised.
  3. **010**: constant-time login miss had `dummy_verify` wired but untested,
     and "rate-limit password-reset initiation" was checked while the
     completion notes deferred it.

## Remediations (this audit)

- **008**: added `two_instances_enforce_one_shared_window`
  (`underlay-ratelimit/src/tests/postgres_integration.rs`) — two
  `PostgresBackend` instances over one database enforce a single window;
  verified green against Postgres 16 and added to CI as a feature-gated step.
  Card annotated: the `InMemoryBackend` "gate" is documentation + a
  `single_instance()` semantic constructor, not a hard config gate.
- **009**: added `timeout_fires_on_stalled_server` (stalled local listener →
  timeout error through the constructed client) and
  `default_timeouts_are_bounded` (guards the 10s/30s wiring values).
- **010**: added `unknown_email_miss_costs_a_kdf_pass` — asserts the
  unknown-email miss costs at least ~¼ of a real wrong-password verify
  (an unguarded miss is ~1000× faster, so the margin cannot flake). Unbundled
  the reset-initiation item into an explicit **deferred, unchecked** entry.
- **Fleet lockfile drift**: the `0.8.0` bump never propagated to consumer
  `Cargo.lock`s (all pinned underlay at `0.0.1`; next build would dirty every
  tree). Surgically bumped 6 consumer locks (acme-api, cp-api, compli-me/api,
  farmyard, nursery, composer-api — name-aware for composer's own `0.0.1`
  crates); verified resolution via `cargo metadata`. `spark` has no underlay
  entries.
- **Footguns documented**: the jobs-postgres integration test's
  `DROP SCHEMA platform CASCADE` now carries an explicit throwaway-database
  warning; `UNDERLAY_TEST_DATABASE_URL` docs note that external mode leaves
  `test_*` schemas behind (throwaway databases are the intended target).

## Accepted/noted, no change

- `v0.8.0` tag sits 11 commits before the merge commit (tagged at the g08.028
  proof point; the generation continued). The tagged tree was green; fine as a
  pin. Process note for `g09`: tag at generation close.
- Card 015's "via thiserror" phrasing: `underlay-jobs::JobHandlerError`
  implements `std::error::Error` manually rather than deriving — satisfies the
  actual rule (every public error implements `Error`).
- `underlay-media-postgres` unadopted status: already documented (prior log).

## Verdict

No functional defects found in shipped code. The material gaps were three
overstated validation checkboxes (now backed by real tests), fleet lockfile
drift (fixed), and two documented footguns. g08 stands as complete.
