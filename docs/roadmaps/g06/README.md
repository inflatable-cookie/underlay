# g06

`g06` is the Rust platform-contract transition generation.

## Current State

`g05` completed the shared page, workflow-template, dev-flow contract, media
capability, and query-variant convergence lanes. Extending that generation
would blur the next problem.

The live problem is now a structural Rust migration:

- narrow Underlay from a broad shared crate collection into a small platform
  contract
- replace stringly extension points with typed safe boundaries
- make security-sensitive construction paths hard to bypass
- split large adapter/tooling modules by stable responsibilities
- prove each breaking or additive shared change against the current consumer
  family

This is a new sequencing baseline, so it starts in `g06`.

## Governing References

- [`docs/architecture/product-guardrails.md`](../../architecture/product-guardrails.md)
- [`docs/contracts/001-working-rules.md`](../../contracts/001-working-rules.md)
- [`docs/contracts/020-http-transport-and-server-boundary.md`](../../contracts/020-http-transport-and-server-boundary.md)
- [`docs/contracts/023-release-and-compatibility-rollout.md`](../../contracts/023-release-and-compatibility-rollout.md)
- [`docs/contracts/030-auth-and-session-systems.md`](../../contracts/030-auth-and-session-systems.md)
- [`docs/contracts/040-storage-blob-and-media-systems.md`](../../contracts/040-storage-blob-and-media-systems.md)
- [`docs/contracts/024-new-app-bootstrap-and-bring-up.md`](../../contracts/024-new-app-bootstrap-and-bring-up.md)
- [`docs/contracts/021-database-migration-and-schema-workflow.md`](../../contracts/021-database-migration-and-schema-workflow.md)
- [`docs/roadmaps/g05/009-rust-runtime-contract-audit-and-next-contract-set.md`](../g05/009-rust-runtime-contract-audit-and-next-contract-set.md)
- [`docs/roadmaps/g05/014-release-and-compatibility-rollout-contract.md`](../g05/014-release-and-compatibility-rollout-contract.md)

## Active Lane

- `g06.001` is complete as the Rust platform contract transition plan and first
  inventory gate.
- `g06.002` is complete as the first typed safety primitive layer.
- `g06.003` is complete as the auth/session contract reset and
  refresh-rotation rollout proof.
- `g06.004` is complete as the HTTP safe-builder consolidation and consumer
  cookie cleanup lane.
- `g06.005` is complete as the DB identifier and schema boundary normalization
  lane.
- `g06.006` is complete as the media repository contract and adapter split
  completion lane.
- `g06.007` is complete as the devtools bundle/store boundary isolation lane.
- `g06.008` is complete as the six-consumer compatibility proof and
  release-note closeout lane.
- `g06.009` is complete as the Effigy doctor structural backlog triage lane.
- `g06.010` is complete as the first Rust god-file split repair batch.
- `g06.011` is complete as the second Rust structural split repair batch.
- `g06.012` is complete as the high-severity Rust structural backlog triage
  lane.
- `g06.013` is complete as the security-adjacent Rust adapter split batch.
- `g06.014` is complete as the Rust platform transition validation and
  release-readiness closeout.
- `g06.015` is complete as the Rust platform transition release-note handoff.
- `g06.016` is complete as the Rust platform hardening backlog batch.
- `g06.017` is complete as the Rust quality re-audit and fresh-start
  assessment.
- `g06.018` is next as the third Rust structural cleanup batch.

## Planned Runway

- `g06.001`: Rust platform contract transition and public API inventory
- `g06.002`: typed safety primitives and construction-boundary migration
- `g06.003`: auth/session contract reset and refresh-rotation rollout
- `g06.004`: HTTP safe-builder consolidation and consumer cookie cleanup
- `g06.005`: DB identifier and schema boundary normalization
- `g06.006`: media repository contract and adapter split completion
- `g06.007`: devtools bundle/store boundary isolation
- `g06.008`: six-consumer compatibility proof and release-note closeout
- `g06.009`: Effigy doctor structural backlog triage for remaining Rust
  god-files and stale markers
- `g06.010`: first Rust god-file split repair batch
- `g06.011`: second Rust structural split repair batch for remaining critical
  files
- `g06.012`: high-severity Rust structural backlog triage and closeout decision
- `g06.013`: security-adjacent Rust adapter split batch
- `g06.014`: Rust platform transition validation and release-readiness closeout
- `g06.015`: Rust platform transition release-note handoff
- `g06.016`: Rust platform hardening backlog batch
- `g06.017`: Rust quality re-audit and fresh-start assessment
- `g06.018`: Rust structural cleanup batch three

## Consumer Family

Consumer proof for this generation uses the current Underlay family:

- `underlay-reference`
- `contact-patch`
- `compli-me`
- `acowtancy`
- `songsprout`
- `loophole/composer`

Each consumer-affecting milestone must classify impact as `additive`,
`deprecation`, or `breaking` under `023`.

## Current Queue

- `g06.001` is complete.
- `g06.002` is complete.
- `g06.003` is complete.
- `g06.004` is complete.
- `g06.005` is complete.
- `g06.006` is complete.
- `g06.007` is complete.
- `g06.008` is complete.
- `g06.009` is complete.
- `g06.010` is complete.
- `g06.011` is complete.
- `g06.012` is complete.
- `g06.013` is complete.
- `g06.014` is complete.
- `g06.015` is complete.
- `g06.016` is complete.
- `g06.017` is complete.
- `g06.018` is next.

## Batch Cards

If `g06` enters strict execution posture, keep its cards under
`g06/batch-cards/`.

## Next Task

Execute `g06.018`: Rust structural cleanup batch three.
