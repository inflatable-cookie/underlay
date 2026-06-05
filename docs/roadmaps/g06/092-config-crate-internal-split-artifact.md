# g06.092 Artifact - Config Crate Internal Split

## Summary

`underlay-config/src/lib.rs` is now a small crate front door with stable
crate-root exports. The single-file config crate was split into focused
modules.

New module shape:

- `constants.rs`: default config constants
- `discovery.rs`: config directory discovery
- `error.rs`: `ConfigError`
- `merge.rs`: recursive TOML merge, namespace selection, and dotted override
  helpers
- `stack.rs`: `ConfigStack`, file loading, and stack execution
- `tests.rs`: existing unit tests and test fixtures

## Public API Impact

None expected.

The crate-root constants, `discover_config_dir`, `ConfigStack`, and
`ConfigError` remain exported from `underlay_config`.

## Validation

- `cargo test -p underlay-config --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 42 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 11 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

The config crate no longer appears in the god-file report. The next largest
Rust production warning is `rust/crates/underlay-auth-totp/src/lib.rs`.

## Next Target Evidence

Queue `g06.093` as an auth TOTP crate modularity audit before splitting
`underlay-auth-totp/src/lib.rs`. Auth TOTP is security-sensitive, so the next
batch should classify secret handling, provisioning URI construction, code
verification, backup-code behavior, and tests before moving code.
