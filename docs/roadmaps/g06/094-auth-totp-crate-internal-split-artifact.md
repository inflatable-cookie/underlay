# g06.094 Artifact - Auth TOTP Crate Internal Split

## Summary

`underlay-auth-totp/src/lib.rs` is now a small crate front door with stable
crate-root exports. The former single-file TOTP implementation was split into
focused modules.

New module shape:

- `algorithm.rs`: `TotpAlgorithm`
- `config.rs`: `TotpConfig` and builder methods
- `types.rs`: setup/result/input public types
- `error.rs`: `TotpError` and auth-error mapping
- `service.rs`: `TotpService` construction, metadata, setup, and
  second-factor orchestration
- `totp.rs`: secret generation/decoding, numeric-code normalization,
  verification, replay checks, and HOTP/TOTP math
- `provisioning.rs`: provisioning URI and QR SVG behavior
- `backup_codes.rs`: backup-code generation, normalization, hashing, and
  constant-time comparison

## Public API Impact

None expected.

Crate-root public exports, service method names, defaults, provisioning URI
format, verification behavior, replay behavior, backup-code behavior, and error
mapping were preserved. The private `totp_code` helper remains available to the
crate test module as `pub(crate)`.

## Validation

- `cargo test -p underlay-auth-totp --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 41 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 11 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

The auth TOTP crate no longer appears in the god-file report. The next largest
Rust production warning is
`rust/crates/underlay-devtools/src/migration_report.rs`.

## Next Target Evidence

Queue `g06.095` as a devtools migration report modularity audit before
splitting `migration_report.rs`. This is devtools/reporting code, so the next
batch should classify data models, report construction, rendering, and tests
before moving code.
