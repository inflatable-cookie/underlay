# g06.093 Artifact - Auth TOTP Crate Modularity Audit

## Summary

`underlay-auth-totp/src/lib.rs` is a single-file security-sensitive auth crate.
It combines public models, config builders, error mapping, TOTP setup,
provisioning URI and QR generation, TOTP verification, replay protection,
backup-code generation and verification, second-factor routing, private
encoding helpers, private HOTP/TOTP math, and tests.

The crate is app-agnostic and intentionally leaves storage, encryption, and
credential association to consuming apps.

The current surface groups:

- `TotpAlgorithm` and config builder methods
- public setup/result/input enums and structs
- `TotpError` and auth-error mapping
- `TotpService` construction, config access, metadata, and setup
- secret generation and base32 decoding
- provisioning URI construction and QR SVG generation
- TOTP verification with skew and replay protection
- backup-code generation, normalization, hashing, and constant-time comparison
- second-factor routing between TOTP and backup-code input
- private numeric-code, URL-escape, TOTP math, and backup-code helpers
- tests that exercise public service behavior and private `totp_code`

## Behavior Evidence

The focused crate validation covers these stable contracts:

- RFC6238 SHA1 vectors pass for eight-digit codes
- verification accepts codes within configured skew
- replay protection rejects reused counters
- provisioning URIs contain expected otpauth fields
- QR SVG generation succeeds
- backup codes generate, hash, verify, and reject wrong input
- second-factor verification accepts backup-code input
- setup includes a secret, otpauth URI, QR SVG, backup codes, and hashes

## Decision

Queue `g06.094` as an auth TOTP crate internal split.

The split should preserve:

- all crate-root public types and service methods
- current default config values
- current secret length and base32 behavior
- current provisioning URI format
- current QR SVG generation behavior
- current verification, skew, and replay semantics
- current backup-code normalization and hash behavior
- current error mapping to `AuthError`
- private test access to `totp_code` through the crate test module, or an
  equivalent `pub(crate)` helper surface

Suggested module shape:

- `lib.rs`: crate docs, module declarations, public re-exports, and test module
- `algorithm.rs`: `TotpAlgorithm`
- `config.rs`: `TotpConfig`
- `types.rs`: setup/result/input public structs and enums
- `error.rs`: `TotpError` and auth-error mapping
- `service.rs`: `TotpService` construction, metadata, setup, and second-factor
  orchestration
- `totp.rs`: secret decode, numeric normalization, verification, and
  HOTP/TOTP math
- `provisioning.rs`: URL escaping, provisioning URI, and QR SVG behavior
- `backup_codes.rs`: backup-code generation, normalization, hashing, and
  constant-time comparison

## Public API Impact

Expected impact: none.

This should be an internal split. If preserving crate-root exports or auth
semantics forces a public API change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-auth-totp --all-features`

Next code batch validation:

- `cargo test -p underlay-auth-totp --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
