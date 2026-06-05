# g06.112 - Devtools Migration-Bundle Remote Registry Internal Split

## Why

`g06.111` found that `migration_bundle/remote_registry.rs` mixes remote
reference parsing, registry ping, blob upload, remote publish, remote pull,
manifest handling, digest verification, and error mapping in one production
file.

The next split should reduce reasoning load without changing remote registry
behavior or migration-bundle public APIs.

## Goal

Split the migration-bundle remote registry implementation into focused
internal modules while preserving crate-private entry points and behavior.

## Scope

In scope:

- keep `remote_registry.rs` as the small module front door
- move remote reference parsing into a focused reference module
- move registry ping and blob upload helpers into a focused client module
- move remote publish behavior into a focused publish module
- move remote pull behavior into a focused pull module
- preserve existing Docker-backed ignored test

Out of scope:

- changing devtools public APIs
- changing OCI registry request semantics
- changing digest validation behavior
- changing local-store behavior
- changing consumer apps

## Acceptance Criteria

- `remote_registry.rs` becomes a small module front door
- responsibility groups live in focused modules
- `migration_bundle/run.rs` call sites remain stable
- devtools tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal tooling split. If remote registry behavior or public
migration-bundle APIs must change, stop and re-enter planning.

## Current State

`g06.112` is complete.

Artifact:

- [112 artifact](./112-devtools-migration-bundle-remote-registry-internal-split-artifact.md)

## Next Task

Execute `g06.113`: devtools lib tests modularity audit.
