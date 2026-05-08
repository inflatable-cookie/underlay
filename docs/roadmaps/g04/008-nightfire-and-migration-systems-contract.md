# 008 - Nightfire And Migration Systems Contract

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.007` settles the operator and async infrastructure layer. The next
dependency is the structured-content and migration surface: Nightfire, editor
runtime, validation, and migration-core.

## Goals

- define the shared structured-content and migration contract
- separate generic content-system behavior from app-local schemas and blocks
- prepare the later AI, runtime, pattern, and template contracts on top of a
  clear content boundary

## Non-Goals

- implementation repair beyond light authority alignment needed to write the
  contract
- app-specific block registries or editorial workflows
- TS runtime orchestration work

## Inputs

- `rust/crates/underlay-nightfire/**`
- `rust/crates/underlay-migration-core/**`
- `ts/src/nightfire/**`

## Outputs

- [`docs/contracts/070-nightfire-and-migration-systems.md`](/Users/tom/Dev/projects/underlay/docs/contracts/070-nightfire-and-migration-systems.md)
- refreshed contract and roadmap front doors so `g04` now points at the
  AI/suggestions lane

## Outcome

The Nightfire and migration contract now exists.

It settles:

- the durable Nightfire value, block, schema, strategy, and validation model
- the retained TS editor, renderer, validator, and strategy-loading shell
- the ownership line between shared content protocol and app-local editorial
  workflow
- the migration-core pipeline, stage artifacts, decision governance, bundle,
  replay, verification, and recovery model

It also records the main drift to assess later, especially the weak TS
`NightfireValue` type, the looser TS runtime split versus the Rust protocol,
the still-messy authority stack around `050`, and the question of whether some
Nightfire workflow helpers still deserve retained shared ownership.

## Next Task

Execute `g04.009`: write `080-ai-runtime-and-suggestions.md`.
