# g06.040 - Blob Adapter Typed Methods Decision

## Why

`g06.039` added typed media storage-key generation and migrated practical upload
paths. The remaining raw object-key boundary is the `BlobAdapter` trait and the
repository/database paths that load stored object keys as strings.

Changing the trait directly would be a broader compatibility wave, so decide
the adapter shape before implementation.

## Goal

Decide whether Underlay should add typed blob adapter convenience methods,
change trait method signatures, or retain raw adapter methods with typed
construction at call sites.

## Scope

In scope:

- audit current `BlobAdapter` object-key method usage
- classify generated keys versus database-loaded keys
- decide between extension methods, additive helper functions, or trait changes
- classify consumer upgrade impact
- update contracts with the chosen posture

Out of scope:

- S3/local behavior changes
- database column type changes
- media repository trait redesign
- TypeScript/Svelte work
- release execution or publishing

## Acceptance Criteria

- adapter-method decision is explicit
- consumer impact is classified
- next implementation batch is concrete if change is warranted
- contracts reflect the chosen typed/raw boundary

## Consumer Upgrade Impact

Expected impact: decision only.

Any implementation follow-up must prove the six-consumer rollout because all
current apps use blob adapter methods directly or indirectly.

## Current State

`g06.040` is complete.

Artifact:

- [040 artifact](./040-blob-adapter-typed-methods-decision-artifact.md)

## Next Task

Execute `g06.041`: typed blob adapter extension methods.
