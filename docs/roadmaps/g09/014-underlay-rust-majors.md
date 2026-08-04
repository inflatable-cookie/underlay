# g09.014 - Underlay Rust Majors (auth-crypto cluster + sqlx 0.9)

Status: complete
Completed: 2026-08-03
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

- [x] Bump `rust-version` to 1.94 (toolchain is 1.96) and upgrade `sqlx` to 0.9.
- [x] Auth-crypto cluster: `jsonwebtoken` 11, `ed25519-dalek` 3 + ecosystem,
  RustCrypto 0.11/0.9 wave in `underlay-auth*` — fix API breaks.
- [x] `tera` 2, `toml` 1.1, `validator` 0.21, `utoipa` 5.5 as compatible.
- [x] `cargo update` for minors/patches (incl. `spin` yank fix).
- [x] `cargo test --workspace`, clippy, `effigy validate` green.

## Consumer Upgrade Impact

Impact class: `breaking` for underlay's Rust API where majors change public
types (auth-crypto). Consumers ride path deps; follow-on is `g09.015`.

## Validation

- [x] `effigy validate` green; cargo-deny advisories still clean

## Completion Notes

Completed 2026-08-03 (7aeee9c3). All scoped majors landed: rust-version 1.94, sqlx 0.9 (SqlSafeStr audit — every dynamic query wraps validated identifiers via AssertSqlSafe), jsonwebtoken 11, ed25519-dalek 3 (direct stack; jsonwebtoken 11's own dalek-2 deps remain as upstream duplicates until it releases dalek-3), RustCrypto 0.11/0.9 wave, tera 2, toml 1.1 (with an upstream whole-document from_str bug worked around via from_str::<Value>), validator 0.21, utoipa 5.5, minors + spin yank cleared. cargo test, clippy -D warnings, effigy validate all green. Consumer-facing API notes recorded in the card for g09.015 (sqlx 0.9 required, toml 1.1, validator 0.21, tera 2; no SecretCipher/KeyPair/JwtService signature changes). schemars turned out not to be in the tree; sha1 0.10 (aws-config), digest 0.10 (argon2/blake2), rand 0.8/0.9 (phf/webauthn-rs) remain as upstream-forced duplicates.

## Next Task

`g09.015` consumer Rust follow-on.
