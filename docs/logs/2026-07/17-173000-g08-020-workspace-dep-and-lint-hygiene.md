# 2026-07-17 - g08.020 workspace dependency and lint hygiene

## Context

Version-drift risk and lint residue across the crate set: shared deps declared
per-crate, `image` non-optional despite rendition processing being
feature-gated, clippy warnings, an empty `tasks/` dir.

## Changes

- **Clippy: 0 warnings** (`--all-features`, was 6 - one added by g08.010).
  Real fixes: useless `BlobObjectKey` conversion, field-assignment-outside-
  initializer in media storage config, needless `Ok(..?)` in
  media-postgres. Targeted `#[allow(clippy::too_many_arguments)]` on three
  genuinely arg-rich fns (2 migration-core pipeline stages + the
  `verify_second_factor_throttled` API).
- **`[workspace.lints.clippy] all = warn`** baseline added and wired
  (`[lints] workspace = true`) into 36 crates. Resolves correctly for
  consumers building underlay crates as path deps (verified).
- **Deps hoisted:** `sha1`/`sha2`/`url`/`rand`/`hex` into
  `[workspace.dependencies]`; per-crate `thiserror`/`base64` literals ->
  `workspace = true` (10 crate Cargo files). `reqwest` kept per-crate (four
  distinct feature sets: form/blocking/plain/optional); `underlay-*` path deps
  left (path deps don't drift).
- **`image` optional** behind `renditions`: `image` dep is `optional`, the
  feature pulls it, and `pub mod image` + the `From<image::ImageError>` impl
  are `#[cfg(feature = "renditions")]`. The 3 apps using
  `underlay_media::image` already enable `renditions`/`full`; the 3 non-users
  drop the codec from default builds.
- Removed empty `underlay-jobs/src/tasks/`.

## Deferred

- **reqwest routing** through `underlay-http-client`: the security goal
  (timeouts + SSRF `external()`) was met in g08.009 - every outbound client
  already has timeouts. The wrapper is async-only while the direct users need
  blocking (`underlay-devtools`) and form (`underlay-auth-oauth`); routing is
  an API expansion, not hygiene.

## Validation

- `cargo check --workspace --all-features`: clean.
- `cargo clippy --workspace --all-features`: 0 warnings.
- `cargo test --workspace --all-features`: green (75 suites).
- `underlay-media` builds/tests both with and without `renditions`.
- acowtancy consumer `cargo check` clean (confirms the workspace-lints wiring
  is safe for path-dep consumers).

## Consumer Upgrade Notes

Impact class **none**. All changes are workspace-internal. `image` becoming
feature-gated is transparent - every `underlay_media::image` user already
enables `renditions`/`full`.

## Next

Lane C's remaining card is `g08.019` postgres adapter integration tests, which
needs a live Postgres service (its stop condition ties to the missing CI story
in `g08.025`). Otherwise Lane D `g08.021` (SSR-global state guard).
