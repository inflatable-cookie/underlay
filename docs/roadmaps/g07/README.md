# g07 - TS Runtime And Workflow Boundary Hardening

Status: active
Owner: repo maintainers
Started: 2026-06-06

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
- [ ] remove or rehome compatibility residue only after caller proof
- [ ] reduce duplicated workflow orchestration where the same lifecycle appears
  in multiple controllers
- [ ] keep templates as higher-order page/workflow composition, not primitive UI
- [ ] keep consumer import changes deliberate, classified, and validated

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
- `g07.011`: TS boundary hardening upgrade-note and closeout checkpoint

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
- `g07.009` is next as the TS public-surface test and guardrail reinforcement.

## Next Task

Execute `g07.009`: TS public-surface test and guardrail reinforcement.
