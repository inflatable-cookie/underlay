# 014 - Implementation Assessment Sequencing

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.001` through `g04.013` complete the first whole-repo contract-coverage
wave.

The next job is not blind repair. It is to turn the new contract spine into a
bounded implementation-assessment queue, ordered so the lowest shared layers
are tested against their goals before higher workflow and template systems.

## Goals

- close the contract-coverage wave honestly
- compile the implementation-vs-contract assessment chain
- confirm the first bounded assessment and repair lane after coverage
- update the program docs so `g04` reads as an assessment generation, not a
  still-open contract-writing pass

## Non-Goals

- executing broad implementation repair in the same batch
- reopening completed contract files except for light authority alignment
- replacing the still-active `g03` template rollout thread

## Inputs

- [`docs/contracts/contract-index.md`](/Users/tom/Dev/projects/underlay/docs/contracts/contract-index.md)
- [`docs/roadmaps/g04/001-underlay-contract-coverage-and-assessment-program.md`](/Users/tom/Dev/projects/underlay/docs/roadmaps/g04/001-underlay-contract-coverage-and-assessment-program.md)
- the completed `010` through `120` contract set

## Exit Criteria

- the coverage wave is explicitly marked complete
- the assessment order and first bounded lane are documented in roadmap
  authority surfaces
- `g04` front doors point at the first implementation-assessment milestone

## Assessment Sequence

The first assessment wave should run in the same lower-to-higher order already
compiled in the contract index, but now as explicit roadmap lanes.

Planned order:

1. `g04.015` foundation and transport
2. auth
3. storage and media
4. jobs and operator systems
5. Nightfire and migration
6. AI and suggestions
7. TS runtime and client orchestration
8. shared patterns and workflow shells
9. admin template system
10. tooling testing and contract artifacts

Sequencing rule:

- do not promote a higher assessment lane while a lower one still has unclear
  contract failure or unresolved shared-boundary ambiguity

First bounded lane:

- start with foundation and transport because `Uuid`, envelopes,
  validation-normalization, query syntax, pagination, cookie handling, and the
  TS HTTP client are inherited by nearly every other system family
- the most obvious first checks are already named by the contracts:
  - `underlay-validation` Axum integration leaking non-canonical validation
    wire shape
  - TS transport ownership drift across `client/types.ts`,
    `client/query.ts`, and related helpers

## Next Task

Execute `g04.015`: assess the live foundation and transport implementation
against `010` and `020`.
