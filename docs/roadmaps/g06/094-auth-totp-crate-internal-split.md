# g06.094 - Auth TOTP Crate Internal Split

## Why

`g06.093` found that `underlay-auth-totp/src/lib.rs` mixes public TOTP models,
config, error mapping, setup, provisioning, verification, replay protection,
backup-code behavior, private helpers, and tests in one security-sensitive
auth file.

The next split should reduce reasoning load without changing crate-root APIs or
auth semantics.

## Goal

Split the auth TOTP crate into focused internal modules while preserving all
public exports and verification behavior.

## Scope

In scope:

- keep `lib.rs` as the small crate front door
- move algorithm and config types into focused modules
- move setup/result/input public types into a focused types module
- move `TotpError` and auth-error mapping into a focused error module
- move service construction, metadata, setup, and second-factor orchestration
  into a focused service module
- move TOTP decoding, numeric normalization, verification, and HOTP/TOTP math
  into a focused TOTP module
- move provisioning URI and QR SVG behavior into a focused provisioning module
- move backup-code generation, normalization, hashing, and constant-time
  comparison into a focused backup-code module
- preserve private test access to `totp_code` or expose it only as
  `pub(crate)`
- preserve all current tests

Out of scope:

- changing auth TOTP public APIs
- changing secret generation semantics
- changing provisioning URI format
- changing verification or replay behavior
- changing backup-code behavior
- changing consumer apps

## Acceptance Criteria

- `lib.rs` becomes a small crate front door
- responsibility groups live in focused modules
- crate-root public exports remain stable
- TOTP tests and doc-tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal split. If public exports, method names, or auth semantics
must change, stop and re-enter planning.

## Current State

`g06.094` is complete.

Artifact:

- [094 artifact](./094-auth-totp-crate-internal-split-artifact.md)

## Next Task

Execute `g06.095`: devtools migration report modularity audit.
