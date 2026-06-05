# g06.092 - Config Crate Internal Split

## Why

`g06.091` found that `underlay-config/src/lib.rs` is a narrow but foundational
single-file crate. It combines public loader APIs, constants, discovery,
errors, TOML merge helpers, namespace handling, dotted overrides, and tests.

The next split should make the crate easier to reason about without changing
consumer imports or config behavior.

## Goal

Split the config crate into focused internal modules while preserving all
crate-root public APIs and config stacking semantics.

## Scope

In scope:

- keep `lib.rs` as the small crate front door
- move default constants into a focused constants module
- move config directory discovery into a focused discovery module
- move `ConfigError` into a focused error module
- move `ConfigStack` and file loading into a focused stack module
- move recursive merge, namespace selection, and dotted override helpers into a
  focused merge module
- move existing unit tests into a focused test module
- preserve crate-root exports and all current assertions

Out of scope:

- changing config public APIs
- changing environment variable semantics
- changing TOML stacking order
- changing namespaced-or-legacy behavior
- changing consumer apps

## Acceptance Criteria

- `lib.rs` becomes a small crate front door
- config responsibilities live in focused modules
- crate-root constants, `discover_config_dir`, `ConfigStack`, and `ConfigError`
  remain available
- config tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal split. If public exports or config semantics must change,
stop and re-enter planning.

## Current State

`g06.092` is complete.

Artifact:

- [092 artifact](./092-config-crate-internal-split-artifact.md)

## Next Task

Execute `g06.093`: auth TOTP crate modularity audit.
