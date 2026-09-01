# AGENTS (Underlay Rust)

Refines the root `AGENTS.md`; it still applies. This file covers what is
specific to `rust/`.

## Scope

One Cargo workspace, 37 `underlay-*` crates under `rust/crates/`, all published
together at a single `[workspace.package]` version. Crates are layered: a small
foundation (`underlay-core`, `underlay-config`, `underlay-validation`), then
capability crates (`underlay-http`, `underlay-auth*`, `underlay-media`,
`underlay-jobs`, `underlay-blob`), then Postgres adapters (`*-postgres`) and
tooling (`underlay-devtools`, `underlay-testing`).

## Hard Rules

- A crate names its layer. Adapters depend on their capability crate, never the
  reverse, and no runtime crate depends on `underlay-devtools`.
- Shared dependency versions live in `[workspace.dependencies]` in the root
  `Cargo.toml`. Add a version there, not in a member manifest.
- MSRV is 1.95, declared once at `[workspace.package]`. Do not use a newer
  language or `std` API without an explicit version-policy decision.
- The lint baseline is `[workspace.lints.clippy]`; crates opt in with
  `[lints] workspace = true`. `effigy rust:clippy` denies warnings.
- `Debug` must not render credentials, bearer tokens, or other protected data.
  Write a manual `Debug` that redacts the secret field, as `DbConfig`,
  `JwtConfig`, and `GoogleOAuthConfig` already do.
- Public API changes are consumer changes. Check
  `docs/contracts/122-rust-public-api-inventory.md` for the crate's
  classification before altering a signature, trait, or visibility.

## Validation

```bash
effigy rust:check    # cargo check --workspace --all-features
effigy rust:clippy   # cargo clippy --workspace --all-targets -- -D warnings
effigy rust:test     # cargo test --workspace --all-features
```

`underlay-db`'s Postgres integration target is `#[ignore]`d and needs a Docker
runtime:

```bash
effigy rust:test -- -p underlay-db --test postgres_integration -- --ignored
```

## Source of Truth

- `../AGENTS.md`
- `../docs/contracts/122-rust-public-api-inventory.md`
- `../docs/contracts/023-release-and-compatibility-rollout.md`
- `../docs/architecture/020-rust-api-foundation.md`
- `../docs/contracts/rust-quality-profile.json`
- `../docs/contracts/rust-quality-deviations.json`

<!-- northstar:rust-quality:start -->
## Northstar Rust Quality

Scope: Rust source, Cargo manifests, build files, tests, and directly related
documentation under this directory.

Use Northstar's strict everyday-authoring route for ordinary Rust work. Resolve
the repository-owned profile and deviations under `docs/contracts/`; never
assume a universal MSRV. Re-enter at task start and coherent batch closeout.
Preserve unrelated work. A quality audit, no-slop pass, or audit-and-fix request
is explicit audit intent; never route it through everyday authoring.
<!-- northstar:rust-quality:end -->
