# g07 - Runtime, Workflow, And Doctor Warning Boundary Hardening

Status: complete
Owner: repo maintainers
Started: 2026-06-06
Completed: 2026-06-06

## Current Generation

`g07` owns the TypeScript runtime, workflow pattern, template, support-layer,
residual Rust public-policy, and doctor-warning cleanup that follows the Rust
reference-grade reset.

The goal is not another broad UI rewrite. The goal is to make the retained
shared surface easier for consuming apps to understand, import, test, and
extend.

## Governing Authority

- [`090-ts-runtime-and-client-orchestration`](../../contracts/090-ts-runtime-and-client-orchestration.md)
- [`100-shared-patterns-and-workflow-shells`](../../contracts/100-shared-patterns-and-workflow-shells.md)
- [`110-admin-template-system`](../../contracts/110-admin-template-system.md)
- [`111-consumer-template-adoption-and-exception-policy`](../../contracts/111-consumer-template-adoption-and-exception-policy.md)
- [`120-tooling-testing-and-contract-artifacts`](../../contracts/120-tooling-testing-and-contract-artifacts.md)
- [`023-release-and-compatibility-rollout`](../../contracts/023-release-and-compatibility-rollout.md)
- [`020-reference-grade-underlay-architecture`](../../architecture/020-reference-grade-underlay-architecture.md)

## Goals

- [x] classify the retained `runtime/*`, `patterns/*`, `templates/*`,
  `testing/*`, and `tools/*` TypeScript surfaces by real ownership
- [x] remove or rehome compatibility residue only after caller proof
- [x] reduce duplicated workflow orchestration where the same lifecycle appears
  in multiple controllers
- [x] keep templates as higher-order page/workflow composition, not primitive UI
- [x] keep consumer import changes deliberate, classified, and validated
- [x] retire residual direct Rust config construction where builders/accessors
  make extension safer
- [x] clear Effigy doctor structural warnings without changing public package or
  crate exports

## Non-Goals

- rewriting the admin template system from scratch
- moving Poodle-owned primitives back into Underlay
- changing package or crate ownership boundaries
- adding compatibility aliases without a dated retirement plan
- sweeping all consumer apps without a specific affected surface

## Planned Runway

- `g07.001`: TS runtime and workflow ownership inventory
- `g07.002`: runtime subpath public surface audit
- `g07.003`: runtime import guidance cleanup
- `g07.004`: pattern helper ownership diet
- `g07.005`: duplicated auth-aware fetch orchestration decision
- `g07.006`: list, pagination, reorder, and template seam audit
- `g07.007`: relation selector boundary audit
- `g07.008`: TS testing and guardrail support gap inventory
- `g07.009`: TS public-surface test and guardrail reinforcement
- `g07.010`: consumer import compatibility sweep
- `g07.011`: stale components config cleanup
- `g07.012`: TS boundary hardening upgrade-note and closeout checkpoint
- `g07.013`: suggestion helper compatibility export retirement
- `g07.014`: remaining TS compatibility export audit
- `g07.015`: runtime data subpath split
- `g07.016`: runtime data focused consumer migration
- `g07.017`: runtime relations boundary audit
- `g07.018`: runtime media subpath split
- `g07.019`: runtime media focused consumer migration
- `g07.020`: client types subpath split
- `g07.021`: client types focused consumer migration
- `g07.022`: client query pagination boundary audit
- `g07.023`: client HTTP retry timeout audit
- `g07.024`: migration-core pipeline and integrity policy field retirement
- `g07.025`: devtools bundle/seed option constructor and accessor audit
- `g07.026`: migration-core governance/OCI/manifest policy model audit
- `g07.027`: residual Rust public config closeout and compatibility proof
- `g07.028`: doctor warning triage and Rust structural cleanup
- `g07.029`: TypeScript auth test god-file split batch
- `g07.030`: slugify test god-file split batch
- `g07.031`: forms test god-file split batch
- `g07.032`: i18n test god-file split batch
- `g07.033`: SvelteKit test god-file split batch
- `g07.034`: CSP test god-file split batch
- `g07.035`: Nightfire utils test god-file split batch
- `g07.036`: Nightfire summary-transform test split
- `g07.037`: OAuth Rust file cleanup and doctor warning closeout

## Consumer Family

Consumer proof uses the current Underlay family:

- `underlay-reference`
- `contact-patch`
- `compli-me`
- `acowtancy`
- `songsprout`
- `loophole/composer`

Each consumer-affecting milestone must classify impact as `additive`,
`deprecation`, or `breaking` under `023`.

## Current Queue

- `g07.001` is complete as the TS runtime and workflow ownership inventory.
- `g07.002` is complete as the runtime subpath public surface audit.
- `g07.003` is complete as the runtime import guidance cleanup.
- `g07.004` is complete as the pattern helper ownership diet.
- `g07.005` is complete as the duplicated auth-aware fetch orchestration decision.
- `g07.006` is complete as the list, pagination, reorder, and template seam audit.
- `g07.007` is complete as the relation selector boundary audit.
- `g07.008` is complete as the TS testing and guardrail support gap inventory.
- `g07.009` is complete as the TS public-surface test and guardrail reinforcement.
- `g07.010` is complete as the consumer import compatibility sweep.
- `g07.011` is complete as the stale components config cleanup.
- `g07.012` is complete as the TS boundary hardening upgrade-note and closeout
  checkpoint.
- `g07.013` is complete as the suggestion helper compatibility export
  retirement.
- `g07.014` is complete as the remaining TS compatibility export audit.
- `g07.015` is complete as the runtime data subpath split.
- `g07.016` is complete as the runtime data focused consumer migration.
- `g07.017` is complete as the runtime relations boundary audit.
- `g07.018` is complete as the runtime media subpath split.
- `g07.019` is complete as the runtime media focused consumer migration.
- `g07.020` is complete as the client types subpath split.
- `g07.021` is complete as the client types focused consumer migration.
- `g07.022` is complete as the client query pagination boundary audit.
- `g07.023` is complete as the client HTTP retry timeout audit.
- `g07.024` is complete as the migration-core policy field retirement batch.
- `g07.025` is complete as the devtools bundle/seed option constructor and
  accessor audit.
- `g07.026` is complete as the migration-core governance/OCI/manifest policy
  model audit.
- `g07.027` is complete as the residual Rust public config closeout and
  compatibility proof.
- `g07.028` is complete as the doctor warning triage and Rust structural
  cleanup batch.
- `g07.029` is complete as the TypeScript auth test god-file split batch.
- `g07.030` is complete as the slugify test god-file split batch.
- `g07.031` is complete as the forms test god-file split batch.
- `g07.032` is complete as the i18n test god-file split batch.
- `g07.033` is complete as the SvelteKit test god-file split batch.
- `g07.034` is complete as the CSP test god-file split batch.
- `g07.035` is complete as the Nightfire utils test god-file split batch.
- `g07.036` is complete as the Nightfire summary-transform test split.
- `g07.037` is complete as the OAuth Rust file cleanup and doctor warning
  closeout.
- `g07` is complete.

## Next Task

No active `g07` task remains. The validation-rejection transport-normalization
drift named in `010` and `020` was resolved by `g06.181`.
