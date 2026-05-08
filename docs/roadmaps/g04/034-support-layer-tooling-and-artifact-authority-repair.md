# 034 - Support Layer Tooling And Artifact Authority Repair

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.033` assessed the tooling/testing/contract-artifact layer against `120`.

The next repair is bounded:

- the TS guardrails and rule-pack surface is described as reusable shared
  tooling, but is not currently exported or package-safe
- the UI machine-readable artifacts are still labeled active authority even
  though the live repo checks barely consume them directly

## Goals

- make the TS guardrails/rule-pack boundary honest as either a real exported
  shared surface or an explicitly repo-local tool surface
- clarify the authority posture of the UI JSON artifacts and restore or remove
  active-authority claims accordingly
- align `120` and the support-layer front doors to the real retained posture

## Non-Goals

- broad refactors of Rust test harnesses or migration-devtools packaging in the
  same batch
- redesigning the lower protocol/runtime contracts
- claiming `g04` closeout before this final support-layer repair is explicit

## Inputs

- [docs/roadmaps/g04/033-tooling-testing-and-contract-artifacts-assessment.md](/Users/tom/Dev/projects/underlay/docs/roadmaps/g04/033-tooling-testing-and-contract-artifacts-assessment.md)
- [docs/contracts/120-tooling-testing-and-contract-artifacts.md](/Users/tom/Dev/projects/underlay/docs/contracts/120-tooling-testing-and-contract-artifacts.md)
- `ts/src/tools/**`
- `contracts/ui/**`
- `package.json`

## Exit Criteria

- the TS guardrails/rule-pack surface has an honest retained-boundary story
- the UI JSON artifacts have a truthful authority posture and, if kept active,
  a visible enforcement or consumption path
- `120` and the related front doors no longer over-claim the support-layer
  boundary
- the repo can see whether `g04` is ready for closeout sequencing after this
  repair

## Outcome

- `package.json` now exports the retained TS guardrails entrypoints and the
  shipped rule-pack templates
- `ts/src/tools/guardrails-config.ts` now supports template-string loading for
  `bannedPatterns` as well as `moduleScopeChecks`, and it can resolve the
  exported package templates without depending only on source-checkout paths
- [docs/contracts/120-tooling-testing-and-contract-artifacts.md](/Users/tom/Dev/projects/underlay/docs/contracts/120-tooling-testing-and-contract-artifacts.md)
  now treats the UI JSON artifacts as preserved compatibility evidence rather
  than as live automated authority
- the support-layer boundary is now honest enough to move `g04` into closeout
  sequencing instead of another repair lane

## Next Task

Execute `g04.035`: compile `g04` closeout sequencing and decide whether the
generation should close or carry an explicit residual queue.
