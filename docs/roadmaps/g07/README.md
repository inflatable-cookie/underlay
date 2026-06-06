# g07 - TS Runtime And Workflow Boundary Hardening

Status: complete
Owner: repo maintainers
Started: 2026-06-06
Completed: 2026-06-06

## Current Generation

`g07` owns the TypeScript runtime, workflow pattern, template, and support-layer
boundary hardening that follows the Rust reference-grade reset.

The goal is not another broad UI rewrite. The goal is to make the retained TS
surface easier for consuming apps to understand, import, test, and extend.

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

## Non-Goals

- rewriting the admin template system from scratch
- moving Poodle-owned primitives back into Underlay
- changing Rust crate boundaries
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
- `g07` is complete.

## Next Task

No active `g07` task remains. Keep `runtime/data` retained unless a future
bounded card proves unknown-caller risk is acceptable and the aggregate can be
retired deliberately.
