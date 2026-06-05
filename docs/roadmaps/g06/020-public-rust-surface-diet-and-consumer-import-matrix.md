# g06.020 - Public Rust Surface Diet And Consumer Import Matrix

## Why

The reset should start by narrowing public roots, not by renaming crates or
moving large implementation modules blindly. Root export cleanup is the safest
first breaking step because it clarifies ownership while consumer updates are
still mechanical and easy to prove.

## Goal

Build the exact consumer import matrix and execute the first narrow public-root
diet batch.

## Scope

In scope:

- scan the six consumer roots for Rust imports and Cargo dependencies on
  Underlay crates
- classify current root exports in the selected target crate as `keep`,
  `submodule-only`, `adapter-only`, or `retire`
- pick one target crate for the first break, with `underlay-media` preferred
  unless the import matrix points elsewhere
- update Underlay docs and exports for that target
- update `underlay-reference` first
- update any other affected consumer before completion

Out of scope:

- broad crate renames
- multiple unrelated root cleanups
- TypeScript package reset work
- release execution or publishing

## Contract References

- `001`: working rules
- `023`: release and compatibility rollout
- `122`: Rust public API inventory
- `020-reference-grade-underlay-architecture`: target architecture

## Acceptance Criteria

- consumer import matrix exists and names exact affected files
- one public-root diet target is selected with rationale
- affected Underlay exports are narrowed or rehomed
- affected consumers are updated in the same batch
- targeted Underlay and consumer validation passes or failures are classified
- no compatibility shim is added unless the batch records why it is needed

## Consumer Upgrade Impact

Impact: likely breaking.

The intended posture is break-and-update, not long deprecation. The batch must
update affected consumers before completion.

## Current State

`g06.020` is complete.

Artifact:

- `020-public-rust-surface-diet-and-consumer-import-matrix-artifact.md`

## Next Task

Execute `g06.021`: Media Postgres adapter extraction proof.
