# g06.090 Artifact - Auth WebAuthn Service Internal Split

## Summary

`underlay-auth-webauthn/src/service.rs` is now a small service front door that
owns `WebAuthnService` construction. The former mixed implementation was split
into responsibility modules under `src/service/`.

New module shape:

- `service.rs`: front door, `WebAuthnService`, and config-based construction
- `service/core.rs`: registration, authentication, and discoverable ceremony
  methods
- `service/encoding.rs`: passkey and feature-gated ceremony-state
  serialization
- `service/storage.rs`: stored-passkey conversion, credential ids, counters,
  transports, sync metadata, lookup, and authentication updates
- `service/http.rs`: HTTP request/response adapter wrappers

## Public API Impact

None expected.

The `WebAuthnService` type, crate-root exports, existing method names,
feature-gated state serialization methods, credential behavior, and HTTP
adapter methods were preserved.

## Validation

- `cargo test -p underlay-auth-webauthn --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 43 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 11 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

The WebAuthn service no longer appears in the god-file report. The next largest
Rust god-file warning is `rust/crates/underlay-config/src/lib.rs`.

## Next Target Evidence

Queue `g06.091` as a config crate modularity audit before splitting
`underlay-config/src/lib.rs`. Config is a foundational API surface, so the next
batch should classify public exports and consumer-visible construction helpers
before moving code.
