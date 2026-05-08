# 025 - AI Runtime And Suggestions Assessment

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.024` repaired the Nightfire value-model and TS authority drift enough for
the next assessment wave to proceed honestly.

The next system family in the contract order is AI runtime and suggestions,
anchored by `080`.

## Goals

- assess the live AI runtime and suggestions implementation against `080`
- separate true contract failures from narrower naming or packaging residue
- identify the smallest honest repair set for the shared AI and suggestion
  boundary
- leave explicit findings and a bounded next lane instead of broad AI churn

## Non-Goals

- redesigning app-local prompting or suggestion workflows in the same batch
- inventing a larger TS AI runtime if the repo does not actually have one
- skipping ahead to TS runtime/pattern assessment before the AI boundary is
  clear

## Inputs

- [docs/contracts/080-ai-runtime-and-suggestions.md](/Users/tom/Dev/projects/underlay/docs/contracts/080-ai-runtime-and-suggestions.md)
- `rust/crates/underlay-ai-runtime/**`
- `rust/crates/underlay-suggestions/**`
- `ts/src/client/suggestions.ts`
- `ts/src/patterns/selection-history.ts`

## Exit Criteria

- the live AI runtime and suggestions implementation is reviewed against `080`
- the real findings are documented in severity order
- the next repair step is expressed as one bounded roadmap lane or a small
  repair set
- the later TS runtime/pattern assessments can proceed without ambiguity about
  the shared AI boundary

## Findings

### 1. TS suggestion helper authority is still split across two shared surfaces

Severity: medium

The shared suggestion request helpers are still duplicated in two places:
`ts/src/client/suggestions.ts` and `ts/src/patterns/selection-history.ts`.
They currently agree, but the repo still has no single authoritative TS owner
for `SuggestionRequestOptions`, `buildSuggestionParams()`, and
`appendSuggestionParams()`.

Evidence:

- [ts/src/client/suggestions.ts](/Users/tom/Dev/projects/underlay/ts/src/client/suggestions.ts:1)
- [ts/src/patterns/selection-history.ts](/Users/tom/Dev/projects/underlay/ts/src/patterns/selection-history.ts:266)

Impact:

- the shared TS suggestion vocabulary can drift by copy-paste instead of
  through one owned helper surface
- the current split makes the retained `080` boundary harder to explain than it
  needs to be

### 2. The TS AI surface exists, but only as a thin ops-controller barrel

Severity: medium

The repo now has `ts/src/runtime/ai.ts`, but it is not a broader AI runtime.
It only re-exports the AI routing ops controller from patterns. That means the
older “no runtime/ai exists” drift note in `080` is stale, but the broader
contract conclusion is still true: there is no substantial TS AI runtime family
that matches the Rust-side runtime.

Evidence:

- [ts/src/runtime/ai.ts](/Users/tom/Dev/projects/underlay/ts/src/runtime/ai.ts:1)
- [ts/src/patterns/ai-routing-ops.svelte.ts](/Users/tom/Dev/projects/underlay/ts/src/patterns/ai-routing-ops.svelte.ts:1)
- [docs/contracts/080-ai-runtime-and-suggestions.md](/Users/tom/Dev/projects/underlay/docs/contracts/080-ai-runtime-and-suggestions.md:225)

Impact:

- the current contract wording needs a small honesty repair so it describes the
  actual TS AI surface instead of an older absence claim
- this is an authority/documentation issue, not a major implementation break

### 3. Rust AI runtime and server-side suggestion mechanics are broadly aligned

Severity: low

The lower Rust AI/runtime layer and the server-side suggestion vocabulary are
materially present and coherent: provider-agnostic request contracts, route
selection, retry, circuit breaker, fallback chain, query parsing, and
suggestion query helpers all exist as described.

Evidence:

- [rust/crates/underlay-ai-runtime/src/lib.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-ai-runtime/src/lib.rs:1)
- [rust/crates/underlay-ai-runtime/src/chain.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-ai-runtime/src/chain.rs:1)
- [rust/crates/underlay-ai-runtime/src/retry.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-ai-runtime/src/retry.rs:1)
- [rust/crates/underlay-ai-runtime/src/circuit_breaker.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-ai-runtime/src/circuit_breaker.rs:1)
- [rust/crates/underlay-suggestions/src/params.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-suggestions/src/params.rs:1)
- [rust/crates/underlay-suggestions/src/query.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-suggestions/src/query.rs:1)

Impact:

- the next lane should be narrow TS/doc cleanup, not a broad runtime rewrite

## Assessment Result

The next real lane is a narrow authority repair:

- dedupe the TS suggestion helper boundary onto one retained owner
- repair the stale TS AI-surface wording in `080` so it matches the current
  repo
- leave the Rust AI runtime and server-side suggestion crates alone unless a
  later higher-layer assessment finds a caller-facing gap

## Next Task

Execute `g04.026`: repair the TS AI and suggestion authority drift.
