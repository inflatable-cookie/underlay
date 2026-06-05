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

This is a new sequencing baseline, so it stays in `g06`. The same generation
now moves from platform-contract hardening into the reference-grade reset, with
controlled breaking changes allowed across the six known consumers.

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
- `g06.018` is superseded by `g06.019`.
- `g06.019` is complete as the reference-grade architecture reset inventory.
- `g06.020` is complete as the public Rust surface diet and consumer import
  matrix.
- `g06.021` is complete as the media Postgres adapter extraction proof.
- `g06.022` is complete as the Postgres runtime adapter isolation batch.
- `g06.023` is complete as the jobs Postgres adapter extraction plan.
- `g06.024` is complete as the jobs Postgres adapter extraction execution batch.
- `g06.025` is complete as the six-consumer rollout and compatibility
  retirement proof.
- `g06.026` is complete as the reference-grade docs and upgrade-note closeout.
- `g06.027` is complete as the post-reset Rust quality re-audit.
- `g06.028` is complete as the typed operator table config batch.
- `g06.029` is complete as the consumer typed operator table adoption and
  raw-wrapper deprecation decision batch.
- `g06.030` is complete as the raw operator wrapper removal readiness and
  dynamic-identifier audit batch.
- `g06.031` is complete as the remaining typed DB helper migration plan.
- `g06.032` is complete as the typed `ExistsCheck` execution and rollout batch.
- `g06.033` is complete as the raw existence helper removal decision.
- `g06.034` is complete as the test DB typed schema cleanup batch.
- `g06.035` is complete as the remaining dynamic identifier closeout audit.
- `g06.036` is complete as the Postgres media config typed identifier cleanup.
- `g06.037` is complete as the typed DB identifier lane closeout audit.
- `g06.038` is complete as the blob object key helper alignment plan.
- `g06.039` is complete as the typed media storage key helper batch.
- `g06.040` is complete as the blob adapter typed method decision.
- `g06.041` is complete as the typed blob adapter extension method batch.
- `g06.042` is complete as the stored object-key parse-boundary audit.
- `g06.043` is complete as the typed media domain object-key field rollout.
- `g06.044` is complete as the consumer app-local media object-key
  parse-boundary rollout.
- `g06.045` is complete as the media object-key boundary closeout audit.
- `g06.046` is complete as the non-media blob object-key boundary policy.
- `g06.047` is complete as the consumer non-media blob object-key adoption
  proof.
- `g06.048` is complete as the post-blob-key Rust quality checkpoint.
- `g06.049` is complete as the devtools migration-bundle boundary split.
- `g06.050` is complete as the migration-core public model modularity audit.
- `g06.051` is complete as the migration-core pipeline internal split.
- `g06.052` is complete as the Rust structural backlog checkpoint.
- `g06.053` is complete as the media domain internal split.
- `g06.054` is complete as the media renditions internal split.
- `g06.055` is complete as the jobs public model modularity audit.
- `g06.056` is complete as the jobs types internal split.
- `g06.057` is complete as the DB pagination public model modularity audit.
- `g06.058` is complete as the DB pagination internal split.
- `g06.059` is complete as the HTTP query public model modularity audit.
- `g06.060` is complete as the HTTP query internal split.
- `g06.061` is complete as the HTTP cookies public model modularity audit.
- `g06.062` is complete as the HTTP cookies internal split.
- `g06.063` is complete as the HTTP error logging public model modularity audit.
- `g06.064` is next as the HTTP error logging internal split.

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
- `g06.018`: superseded by `g06.019`
- `g06.019`: reference-grade architecture reset inventory
- `g06.020`: public Rust surface diet and consumer import matrix
- `g06.021`: media Postgres adapter extraction proof
- `g06.022`: Postgres runtime adapter isolation batch
- `g06.023`: jobs Postgres adapter extraction plan
- `g06.024`: jobs Postgres adapter extraction execution
- `g06.025`: six-consumer rollout and compatibility retirement proof
- `g06.026`: reference-grade docs and upgrade-note closeout
- `g06.027`: post-reset Rust quality re-audit
- `g06.028`: typed operator table config
- `g06.029`: consumer typed operator table adoption and raw-wrapper
  deprecation decision
- `g06.030`: raw operator wrapper removal readiness and remaining
  dynamic-identifier audit
- `g06.031`: remaining typed DB helper migration plan
- `g06.032`: typed `ExistsCheck` execution and rollout
- `g06.033`: raw existence helper deprecation decision
- `g06.034`: test DB typed schema cleanup
- `g06.035`: remaining dynamic identifier closeout audit
- `g06.036`: Postgres media config typed identifier cleanup
- `g06.037`: typed DB identifier lane closeout audit
- `g06.038`: blob object key helper alignment plan
- `g06.039`: typed media storage key helpers
- `g06.040`: blob adapter typed method decision
- `g06.041`: typed blob adapter extension methods
- `g06.042`: stored object-key parse-boundary audit
- `g06.043`: typed media domain object-key fields
- `g06.044`: consumer app-local media object-key parse boundaries
- `g06.045`: media object-key boundary closeout audit
- `g06.046`: non-media blob object-key boundary policy
- `g06.047`: consumer non-media blob object-key adoption proof
- `g06.048`: post-blob-key Rust quality checkpoint
- `g06.049`: devtools migration-bundle boundary split
- `g06.050`: migration-core public model modularity audit
- `g06.051`: migration-core pipeline internal split
- `g06.052`: Rust structural backlog checkpoint
- `g06.053`: media domain internal split
- `g06.054`: media renditions internal split
- `g06.055`: jobs public model modularity audit
- `g06.056`: jobs types internal split
- `g06.057`: DB pagination public model modularity audit
- `g06.058`: DB pagination internal split
- `g06.059`: HTTP query public model modularity audit
- `g06.060`: HTTP query internal split
- `g06.061`: HTTP cookies public model modularity audit
- `g06.062`: HTTP cookies internal split
- `g06.063`: HTTP error logging public model modularity audit
- `g06.064`: HTTP error logging internal split

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
- `g06.018` is superseded.
- `g06.019` is complete.
- `g06.020` is complete.
- `g06.021` is complete.
- `g06.022` is complete.
- `g06.023` is complete.
- `g06.024` is complete.
- `g06.025` is complete.
- `g06.026` is complete.
- `g06.027` is complete.
- `g06.028` is complete.
- `g06.029` is complete.
- `g06.030` is complete.
- `g06.031` is complete.
- `g06.032` is complete.
- `g06.033` is complete.
- `g06.034` is complete.
- `g06.035` is complete.
- `g06.036` is complete.
- `g06.037` is complete.
- `g06.038` is complete.
- `g06.039` is complete.
- `g06.040` is complete.
- `g06.041` is complete.
- `g06.042` is complete.
- `g06.043` is complete.
- `g06.044` is complete.
- `g06.045` is complete.
- `g06.046` is complete.
- `g06.047` is complete.
- `g06.048` is complete.
- `g06.049` is complete.
- `g06.050` is complete.
- `g06.051` is complete.
- `g06.052` is complete.
- `g06.053` is complete.
- `g06.054` is complete.
- `g06.055` is complete.
- `g06.056` is complete.
- `g06.057` is complete.
- `g06.058` is complete.
- `g06.059` is complete.
- `g06.060` is complete.
- `g06.061` is complete.
- `g06.062` is complete.
- `g06.063` is complete.
- `g06.064` is next.

## Batch Cards

If `g06` enters strict execution posture, keep its cards under
`g06/batch-cards/`.

## Next Task

Execute `g06.064`: HTTP error logging internal split.
