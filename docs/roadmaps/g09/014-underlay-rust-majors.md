# g09.014 - Underlay Rust Majors (auth-crypto cluster + sqlx 0.9)

Status: ready
Owner: repo maintainers

## Purpose

Last broad Rust refresh was 2026-04-22. The majors are clustered in the
auth-crypto cluster and most consumer staleness is inherited from underlay
path deps, so underlay goes first. Security posture is clean (zero
RUSTSEC; one yanked `spin`).

## Evidence

- Rust dependency survey 2026-08-03 (agent report; cargo-deny clean)
- `jsonwebtoken` 10.3→11, `ed25519-dalek` 2.2→3.0 (+curve25519/signature
  ecosystem), RustCrypto wave (aes-gcm 0.10→0.11, aes/sha2/hmac/digest),
  `tera` 1.20→2.1, `toml` 0.8→1.1, `schemars` 0.9→1.2 (transitive),
  `sqlx` 0.8.6→0.9.0 (needs rust-version 1.94), minors via `cargo update`,
  `spin` yank

## Planned Changes

- [ ] Bump `rust-version` to 1.94 (toolchain is 1.96) and upgrade `sqlx` to 0.9.
- [ ] Auth-crypto cluster: `jsonwebtoken` 11, `ed25519-dalek` 3 + ecosystem,
  RustCrypto 0.11/0.9 wave in `underlay-auth*` — fix API breaks.
- [ ] `tera` 2, `toml` 1.1, `validator` 0.21, `utoipa` 5.5 as compatible.
- [ ] `cargo update` for minors/patches (incl. `spin` yank fix).
- [ ] `cargo test --workspace`, clippy, `effigy validate` green.

## Consumer Upgrade Impact

Impact class: `breaking` for underlay's Rust API where majors change public
types (auth-crypto). Consumers ride path deps; follow-on is `g09.015`.

## Validation

- [ ] `effigy validate` green; cargo-deny advisories still clean

## Next Task

`g09.015` consumer Rust follow-on.
