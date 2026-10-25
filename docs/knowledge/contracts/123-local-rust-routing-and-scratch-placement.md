# Contract: Local Rust Routing and Scratch Placement

Status: active
Owner: repo maintainers
Depends on: `022-testing-posture-and-shared-harnesses.md`, `120-tooling-testing-and-contract-artifacts.md`

## Purpose

Durable rules for two local-host facts about this checkout:

- which command runs Rust for ordinary local work
- where agent and proof scratch lives

Operator ruling: Tom, 2026-10-25, relayed by Oxide Supervisor. It supersedes the
earlier postponed exceptions and the pre-formal-v0.15 retention of `:cargo`
escape tasks. It does not retroactively qualify earlier recorded results and it
does not authorise replays of them.

This contract covers this repository. Consumer apps keep their own manifests and
their own commands; nothing here asks a consumer to install, adopt or rewrite
anything.

## Sources of Truth

- [`effigy.toml`](../../../effigy.toml) — the effective local route catalogue
- [`rust/scripts/check-coverage.sh`](../../../rust/scripts/check-coverage.sh) —
  external tool routed through `oxide exec`
- [`rust/scripts/test-check-coverage.sh`](../../../rust/scripts/test-check-coverage.sh)
  — fake-tool proof of that route
- [`scripts/test-consumer-conformance-fixtures.sh`](../../../scripts/test-consumer-conformance-fixtures.sh)
  and [`scripts/auth-webauthn-manual-dossier.sh`](../../../scripts/auth-webauthn-manual-dossier.sh)
  — proof scratch placement
- [`AGENTS.md`](../../../AGENTS.md), [`rust/AGENTS.md`](../../../rust/AGENTS.md)

## Rust routing

Ordinary local Rust work runs through Oxide.

- Modeled commands: `oxide check|test|clippy|build|run|fmt`. Effigy selectors
  `rust:build`, `rust:check`, `rust:clippy`, `rust:test`, `fmt:rust` and
  `fmt:rust:check` spell Oxide directly.
- A tool that runs Cargo itself, or needs its own target directory or
  `CARGO_TARGET_DIR`, runs `oxide exec -- <tool> …`. The inner `cargo` word
  belongs to that tool. Tarpaulin is the current case.
- Oxide owns build storage, so it refuses `--target-dir` and the build-dir
  config keys.
- The `oxide init` block in `AGENTS.md` is managed; do not restate it.

No selector keeps a plain-Cargo `:cargo` sibling. Their removal is the point:
they read as supported local routes and quietly wrote build output into the
checkout's `target/`.

Escape posture:

- The only escapes are operator decisions, recorded on the task that uses them.
- `EFFIGY_OXIDE_ESCAPE=cargo` applies to a routed surface that honours it, such
  as Effigy's own oxide-selector routes. Underlay's catalog routes spell `oxide`
  directly, so they do not read that variable.
- Running Cargo by hand outside Effigy is the manual escape.
- No custom escape wrapper, no silent fallback, no automatic retry on Cargo. A
  missing or refusing Oxide fails the run loudly.

## Remaining Cargo words by class

The current ordinary local Rust route inventory contains no unexplained Cargo
invocation. Cargo still appears where it is the subject or the host:

- hosted CI: `.github/workflows/` and CI examples in guides run Cargo, because
  hosted runners have no Oxide.
- generic consumer recipes: guide snippets that `cd apps/api` and run a consumer
  app's own workspace. These are consumer examples, not this checkout's routes.
- Cargo as subject or metadata: `cargo-tarpaulin` and other `cargo-*` binary
  names, dependency and crate names, `[package]`/`[workspace]` manifest schema,
  `cargo:` build-script directives, release notes and changelog entries about
  Cargo, and workspace-shape checks that read `Cargo.toml`.
- product and fixture behavior: scripts, samples and fixtures whose behavior is
  the thing being proved, plus container/VM guests that build inside their own
  target directory.
- historical records: release, upgrade and sweep records that quote what a past
  run executed. They are evidence, not live instructions.

These are classified, not pending migrations. Do not mechanically rewrite them.

## Scratch placement

Agent and proof scratch belongs on the Builds volume, never in a home
directory, a drive root, or the checkout.

- Place scratch in a unique directory below
  `/Volumes/Builds/scratch/underlay/`, for example
  `mktemp -d /Volumes/Builds/scratch/underlay/<label>.XXXXXXXX`.
- If the volume is unavailable, refuse and report it. Do not fall back to
  `$TMPDIR`, `$HOME`, a drive root or the checkout, and do not create a
  directory or link that pretends the volume is mounted.
- Delete only scratch your own run created. Preserve unknown state.
- Evidence records key presence and outcomes, never raw secret values.
- A generic `HOME` override is not a scratch strategy.
- Product-managed temp files are not agent scratch. Runtime algorithms and guest
  `/tmp` files stay as they are; this rule governs the agent/proof surface, not
  shipped behavior.
- The retired `~/ur.*` fixtures do not come back.
- Small historical scratch was archived to
  `/Volumes/Builds/home-scratch-archive/2026-10-25/`. That is a location fact
  only: it grants no cleanup, replay, adoption or relocation authority.

## Proof

- `effigy check:coverage-route` proves the coverage wrapper's Oxide dispatch,
  argument pass-through, stdout/stderr and exit preservation, threshold
  handling, and the missing-`cargo-tarpaulin` refusal, with fake executables and
  no real coverage run.
- `effigy check:consumer-conformance-fixtures` exercises the scratch placement
  end to end when the Builds volume is mounted.
- `scripts/auth-webauthn-manual-dossier.sh` needs live consumer apps, so it is
  covered here by syntax and placement review only; running it is a real
  qualification run, not a routine check.

## Related

- [`022-testing-posture-and-shared-harnesses.md`](022-testing-posture-and-shared-harnesses.md)
  owns testing posture and the coverage threshold.
- [`120-tooling-testing-and-contract-artifacts.md`](120-tooling-testing-and-contract-artifacts.md)
  owns shared tooling boundaries.
- [`023-release-and-compatibility-rollout.md`](023-release-and-compatibility-rollout.md)
  owns consumer-visible rollout.
