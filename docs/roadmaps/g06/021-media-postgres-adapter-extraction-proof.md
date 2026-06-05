# g06.021 - Media Postgres Adapter Extraction Proof

## Why

`g06.020` narrowed the `underlay-media` root surface, but the crate still owns
both app-facing media contracts and concrete Postgres adapter code.

The reference-grade target is a clean contract crate with explicit adapter
crates. The first extraction should be small enough to prove with
`underlay-reference` before repeating the pattern.

## Goal

Extract the media Postgres adapter boundary without hiding it behind root
compatibility exports.

## Scope

In scope:

- inventory `underlay_media::postgres` usage across the six consumers
- define the intended adapter crate or adapter module boundary
- select the smallest implementation step that moves Postgres code out of the
  contract-facing root
- update `underlay-reference` first
- update any other affected consumer in the same batch
- classify release impact under `023`

Out of scope:

- extracting every `underlay-media` helper family
- moving blob storage or rendition generation
- broad app-local media schema rewrites
- publishing or release execution

## Contract References

- `001`: working rules
- `023`: release and compatibility rollout
- `040`: storage blob and media systems
- `122`: Rust public API inventory
- `020-reference-grade-underlay-architecture`: target architecture

## Acceptance Criteria

- consumer usage of `underlay_media::postgres` is recorded
- extraction shape is explicit and justified
- `underlay-reference` compiles against the new adapter boundary
- affected consumers are updated or explicitly unaffected
- targeted Underlay and consumer validation passes or failures are classified
- no root compatibility export is added unless this card records why

## Consumer Upgrade Impact

Impact: likely breaking.

The current generation allows controlled breaking changes because the six known
consumers are not production deployments.

## Current State

`g06.021` is complete.

Artifact:

- `021-media-postgres-adapter-extraction-proof-artifact.md`

## Next Task

Execute `g06.022`: Postgres runtime adapter isolation batch.
