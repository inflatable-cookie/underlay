# g06.111 - Devtools Migration-Bundle Remote Registry Modularity Audit

## Why

After `g06.110`, the largest remaining Rust warning-level production file in
the god-file report is
`underlay-devtools/src/migration_bundle/remote_registry.rs`.

Remote registry behavior is shared tooling infrastructure. It should be split
from responsibility and behavior evidence, not file size alone.

## Goal

Classify the migration-bundle remote-registry surface and decide the safest
next structural batch.

## Scope

In scope:

- inspect `underlay-devtools/src/migration_bundle/remote_registry.rs` by
  responsibility family
- identify reference parsing, HTTP client behavior, OCI manifest/blob
  push/pull behavior, auth/header handling, error mapping, and test boundaries
- identify public or crate-private helper behavior that must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader registry behavior checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing devtools public APIs
- changing OCI registry request semantics
- changing local-store behavior
- changing consumer apps

## Acceptance Criteria

- remote-registry responsibilities are grouped by stable behavior family
- helper and test boundaries are recorded
- behavior impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a tooling structure audit. If the audit finds public exports or
registry behavior that must change, stop and re-enter planning.

## Current State

`g06.111` is complete.

Artifact:

- [111 artifact](./111-devtools-migration-bundle-remote-registry-modularity-audit-artifact.md)

## Next Task

Execute `g06.112`: devtools migration-bundle remote registry internal split.
