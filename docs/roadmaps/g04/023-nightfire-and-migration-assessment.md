# 023 - Nightfire And Migration Assessment

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.022` repaired the operator boundary drift enough for the next assessment
wave to proceed honestly.

The next system family in the contract order is Nightfire and migration,
anchored by `070`.

## Goals

- assess the live Nightfire and migration implementation against `070`
- separate true contract failures from older layering or packaging residue
- identify the smallest honest repair set for the shared content and migration
  boundary
- leave explicit findings and a bounded next lane instead of broad editor or
  migration churn

## Non-Goals

- redesigning editorial workflows in the same batch
- rewriting Nightfire TS runtime structure before the assessment is explicit
- skipping ahead to AI/runtime before the Nightfire and migration boundary is
  clear

## Inputs

- [docs/contracts/070-nightfire-and-migration-systems.md](/Users/tom/Dev/projects/underlay/docs/contracts/070-nightfire-and-migration-systems.md)
- `rust/crates/underlay-nightfire/**`
- `rust/crates/underlay-migration-core/**`
- `ts/src/nightfire/**`

## Exit Criteria

- the live Nightfire and migration implementation is reviewed against `070`
- the real findings are documented in severity order
- the next repair step is expressed as one bounded roadmap lane or a small
  repair set
- the later AI/runtime assessments can proceed without ambiguity about the
  shared content-system boundary

## Findings

### 1. The Nightfire durable value envelope is weaker than the contract on both Rust and TS sides

Severity: high

The shared contract says a value is either `{ schema, block }` or
`{ schema, blocks }`. The shipped shared types still allow both at once, and
the validators do not close that gap.

Evidence:

- [rust/crates/underlay-nightfire/src/value.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-nightfire/src/value.rs:1)
- [rust/crates/underlay-nightfire/src/validation.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-nightfire/src/validation.rs:1)
- [ts/src/nightfire/types.ts](/Users/tom/Dev/projects/underlay/ts/src/nightfire/types.ts:1)
- [ts/src/nightfire/validator-registry.ts](/Users/tom/Dev/projects/underlay/ts/src/nightfire/validator-registry.ts:1)

Impact:

- malformed hybrid values can survive shared validation instead of being
  rejected or normalized decisively
- the durable protocol is partly “repo lore” rather than enforced shared code

### 2. The TS Nightfire editor still carries a second local schema authority beside strategy loading

Severity: medium

The retained TS shell still mixes server-loaded strategy truth with a local
`SchemaDefinition` registry and fallback defaults for mode/default type. That
is useful for resilience, but it weakens the contract claim that strategy
loading is the real shared authority for field shape.

Evidence:

- [ts/src/nightfire/editor-registry.ts](/Users/tom/Dev/projects/underlay/ts/src/nightfire/editor-registry.ts:1)
- [ts/src/nightfire/editor/schema-resolution.ts](/Users/tom/Dev/projects/underlay/ts/src/nightfire/editor/schema-resolution.ts:1)
- [ts/src/nightfire/editor/strategy-normalisation.ts](/Users/tom/Dev/projects/underlay/ts/src/nightfire/editor/strategy-normalisation.ts:1)
- [ts/src/nightfire/NightfireEditor.svelte](/Users/tom/Dev/projects/underlay/ts/src/nightfire/NightfireEditor.svelte:1)

Impact:

- editor behavior can drift from strategy truth when local schema defs or
  fallbacks disagree with fetched strategy data
- the runtime authority split is broader than `070` currently presents

### 3. Migration core is broadly aligned with the contract and does not need the next repair lane

Severity: low

The shared migration pipeline, stage artifacts, decision memory, replay, drift,
integrity, and verification surfaces are materially present. I did not find a
bounded implementation mismatch comparable to the Nightfire value-model issues.

Evidence:

- [rust/crates/underlay-migration-core/src/lib.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-migration-core/src/lib.rs:1)
- [rust/crates/underlay-migration-core/src/pipeline.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-migration-core/src/pipeline.rs:1)
- [rust/crates/underlay-migration-core/src/plugin.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-migration-core/src/plugin.rs:1)
- [rust/crates/underlay-migration-core/src/context.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-migration-core/src/context.rs:1)
- [rust/crates/underlay-migration-core/src/manifest.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-migration-core/src/manifest.rs:1)

Impact:

- the next bounded repair should focus on Nightfire boundary enforcement, not
  on broad migration-core churn

## Assessment Result

The next real lane is Nightfire-specific, not a broad content/migration
rewrite:

- tighten the shared value-model contract so hybrid `block` plus `blocks`
  shapes cannot drift through shared code
- decide how much local schema fallback the TS shell is still allowed to own
  beside strategy loading
- leave migration core alone unless a later higher-layer assessment finds a
  concrete caller-facing gap

## Next Task

Execute `g04.024`: repair the Nightfire value-model and TS authority drift.
