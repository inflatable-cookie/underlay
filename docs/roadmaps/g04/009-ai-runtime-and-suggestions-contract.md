# 009 - AI Runtime And Suggestions Contract

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.008` settles the structured-content and migration layer. The next
dependency is the shared AI surface: provider/runtime seams, routing helpers,
and generic suggestion infrastructure.

## Goals

- define the shared AI runtime and suggestions contract
- separate reusable provider/runtime and generic suggestion mechanics from
  app-local prompting, policy, and feature UX
- prepare the later TS runtime, pattern, and template contracts on top of a
  clear AI boundary

## Non-Goals

- implementation repair beyond light authority alignment needed to write the
  contract
- app-specific prompts, model policy, or agent workflows
- frontend template work owned by `g03`

## Inputs

- `rust/crates/underlay-ai-runtime/**`
- `rust/crates/underlay-suggestions/**`
- `ts/src/ai/**`
- `ts/src/suggestions/**`

## Outputs

- [`docs/contracts/080-ai-runtime-and-suggestions.md`](/Users/tom/Dev/projects/underlay/docs/contracts/080-ai-runtime-and-suggestions.md)
- refreshed contract and roadmap front doors so `g04` now points at the TS
  runtime lane

## Outcome

The AI runtime and suggestions contract now exists.

It settles:

- the provider-agnostic Rust LLM runtime boundary
- the OpenAI-compatible transport client and opt-in resilience middleware
- deterministic route selection and fallback-chain behavior
- the generic server-side suggestion param and query-building contract
- the thin TS suggestion helper layer and its relationship to relation-selector
  callers

It also records the main drift to assess later, especially the missing
`ts/src/ai/**` family, the duplicated TS suggestion-helper authority, and the
question of whether `underlay-suggestions` is broad enough to count as a true
retained shared system.

## Next Task

Execute `g04.010`: write `090-ts-runtime-and-client-orchestration.md`.
