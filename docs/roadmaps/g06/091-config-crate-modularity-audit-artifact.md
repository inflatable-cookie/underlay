# g06.091 Artifact - Config Crate Modularity Audit

## Summary

`underlay-config/src/lib.rs` is a single-file crate that combines the public
config stack API, default constants, config directory discovery, TOML file
reading, value merging, namespace selection, dotted override insertion, error
definitions, and unit tests.

The crate is foundational but narrow. It does not define app-specific config
models; apps own their typed structs. That makes this a good internal split
candidate as long as exports stay stable.

The current surface groups:

- default constants: `DEFAULT_CONFIG_DIR`, `DEFAULT_ENVIRONMENT`,
  `DEFAULT_ENV_VAR`
- config directory discovery: `discover_config_dir`
- loader builder and execution: `ConfigStack`
- public error type: `ConfigError`
- private TOML file read helper
- private recursive value merge helper
- private namespace-over-legacy selection helper
- private dotted override insertion helper
- inline tests and test-only sample config structs

## Behavior Evidence

The focused crate validation covers these stable contracts:

- default, environment, local, and explicit overrides stack in order
- the default environment overlay is `dev`
- missing environment overlays are allowed
- namespaced config overrides legacy root values
- config discovery can find a parent `../config` directory

## Decision

Queue `g06.092` as a config crate internal split.

The split should preserve:

- all crate-root constants
- `discover_config_dir`
- `ConfigStack`
- `ConfigError`
- current TOML stacking order
- current optional overlay behavior
- current namespaced-or-legacy behavior
- current dotted override semantics
- current tests

Suggested module shape:

- `lib.rs`: crate docs, module declarations, and public re-exports
- `constants.rs`: default constants
- `discovery.rs`: config directory discovery
- `error.rs`: `ConfigError`
- `stack.rs`: `ConfigStack` and file loading
- `merge.rs`: recursive merge, namespace selection, dotted override helpers
- `tests.rs`: existing unit tests

## Public API Impact

Expected impact: none.

This should be an internal split. If preserving crate-root exports forces a
behavior or API change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-config --all-features`

Next code batch validation:

- `cargo test -p underlay-config --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
