# g08.020 - Workspace Dependency And Lint Hygiene

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Reduce version-drift risk and clear lint residue across 35 crates. Shared deps
(`thiserror`, `base64`, `sha1/sha2`, `url`, `reqwest`, `rand` at an odd
`0.10.1` pin, `hex`, `testcontainers`) are declared per-crate instead of
`workspace = true`; only `underlay-aws` and `underlay-http-client` made it into
`[workspace.dependencies]` as path deps while the other 33 use raw `path`. The
`image` codec dep in `underlay-media` is non-optional even though rendition
processing is feature-gated. Clippy shows 5 default-lint warnings.

## Evidence

- `Cargo.toml` `[workspace.dependencies]` (partial hoisting)
- `underlay-media/Cargo.toml` (`image` non-optional)
- clippy: `underlay-migration-core/src/pipeline/orchestrator/stages.rs:188`,
  `underlay-media/src/storage/config.rs:29`,
  `underlay-media-postgres/src/usage_ops.rs:168`, `underlay-blob`

## Governing References

- [010 Foundation primitives and envelopes](../../contracts/010-foundation-primitives-and-envelopes.md)
- [122 Rust public API inventory](../../contracts/122-rust-public-api-inventory.md)

## Planned Changes

- [x] Hoisted the clean single-version shared deps (`sha1`, `sha2`, `url`,
  `rand`, `hex`) into `[workspace.dependencies]` and converted per-crate
  `thiserror`/`base64` literals to `workspace = true` (10 crate Cargo files).
  `reqwest` left per-crate - four crates need different feature sets
  (form/blocking/optional); `underlay-*` path-dep hoisting deferred (path deps
  don't drift).
- [x] `image` is now `optional` and pulled only by the `renditions` feature;
  `pub mod image` and the `From<image::ImageError>` impl are gated behind it.
  Zero consumer churn - the 3 apps using `underlay_media::image` already enable
  `renditions`/`full`; the 3 non-users drop the `image` codec from default
  builds.
- [~] **Deferred.** The security objective (default timeouts + SSRF-guarded
  `external()`) was delivered in `g08.009`; every outbound client already has
  timeouts. `underlay-http-client` is async-only, while the direct users need
  blocking (`underlay-devtools`) and form (`underlay-auth-oauth`) - routing them
  through the wrapper is an API expansion, not hygiene, so it is not done here.
- [x] Cleared all clippy warnings (6, including one added by `g08.010`): real
  fixes for the useless-conversion, field-assignment, and needless-`Ok`/`?`
  cases; targeted `#[allow(clippy::too_many_arguments)]` on three
  genuinely arg-rich internal/API fns. Added a `[workspace.lints.clippy]`
  baseline (`all = warn`) and wired `[lints] workspace = true` into 36 crates.
- [x] Removed the empty `rust/crates/underlay-jobs/src/tasks/` directory.

## Consumer Upgrade Impact

Impact class: `none` (internal hygiene) unless the `image` feature-gate changes
a consumer's default build; note it if so.

## Validation

- [ ] `cargo clippy --workspace` clean; `cargo check --workspace`
- [ ] `effigy validate`

## Stop Conditions

None.

## Completion Notes

Completed 2026-07-17.
- **Clippy: 0 warnings** workspace-wide (`--all-features`). Baseline
  `[workspace.lints.clippy] all = warn` added and wired to 36 crates; it
  resolves correctly for consumers building underlay crates as path deps
  (verified: acowtancy `cargo check` clean, no workspace-lint resolution
  error).
- **Deps hoisted:** `sha1`/`sha2`/`url`/`rand`/`hex` into
  `[workspace.dependencies]`; `thiserror`/`base64` per-crate literals converted
  to `workspace = true`. `reqwest` and `underlay-*` path deps intentionally
  left (feature-variance / no-drift).
- **`image` optional** behind `renditions`; default builds of the 3 non-image
  consumers no longer pull the codec, and the 3 image users already enable the
  feature (zero churn).
- Empty `underlay-jobs/src/tasks/` removed.

Validated: `cargo check --workspace --all-features` clean; `cargo clippy
--workspace --all-features` 0 warnings; `cargo test --workspace --all-features`
green (75 suites); `underlay-media` builds/tests both with and without
`renditions`; acowtancy consumer `cargo check` clean.

## Consumer Rollout

Impact class **none**. All changes are workspace-internal; the
`[lints] workspace = true` wiring resolves against underlay's workspace even
when a consumer builds an underlay crate as a path dep (verified). `image`
becoming feature-gated is transparent - every consumer using
`underlay_media::image` already enables `renditions`/`full`.

## Next Task

Lane C complete. `g08.021` (Lane D) SSR-global state guard.
