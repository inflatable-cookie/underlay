# 024 - Nightfire Value Model And TS Authority Repair

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.023` assessed the Nightfire and migration systems against `070`.

Migration core is broadly aligned. The real shared drift is concentrated in the
Nightfire value boundary:

- the durable value model still allows weak hybrid shapes in both Rust and TS
- the TS editor/runtime still carries a second local schema authority beside
  strategy loading

## Goals

- tighten the shared Nightfire value boundary so the one-of `block` or `blocks`
  contract is enforced or normalized explicitly
- strengthen the retained TS Nightfire type surface so it reflects the durable
  block envelope more honestly
- decide and document the allowed role of local schema fallback beside strategy
  loading

## Non-Goals

- redesigning app-local block inventories or editorial workflows
- rewriting migration-core while it is already aligned enough
- skipping ahead to AI/runtime before the content boundary is repaired

## Inputs

- [docs/roadmaps/g04/023-nightfire-and-migration-assessment.md](/Users/tom/Dev/projects/underlay/docs/roadmaps/g04/023-nightfire-and-migration-assessment.md)
- [docs/contracts/070-nightfire-and-migration-systems.md](/Users/tom/Dev/projects/underlay/docs/contracts/070-nightfire-and-migration-systems.md)
- `rust/crates/underlay-nightfire/**`
- `ts/src/nightfire/**`

## Exit Criteria

- the shared Nightfire value-model boundary is truthful in docs and code
- TS Nightfire types no longer imply less structure than the shared protocol
- the TS strategy-vs-local-schema authority split is narrowed or documented
  honestly
- the next AI/suggestions assessment can treat Nightfire as a stable lower
  dependency

## Changes

- tightened the TS Nightfire value surface into a strict durable
  `NightfireValue` plus editor-local `NightfireDraftValue`
- normalized weak block objects through a shared `coerceNightfireBlock()`
  helper so block envelopes keep `type`, `version`, `hash`, and `data`
- changed the editor-side save boundary so `prepareNightfireForSave()` now
  produces strict durable value or `null`, not a weak draft shape
- added Rust-side `InvalidValueShape` validation so values with both `block`
  and `blocks`, or with neither, are rejected explicitly
- repaired `070` so it states plainly that fetched strategy data is primary
  authority and local schema registration is only compatibility fallback

## Result

The Nightfire boundary is now materially closer to the contract:

- the durable Rust validator rejects hybrid/empty value shapes
- TS no longer treats every editor draft as if it were already a valid durable
  Nightfire value
- the strategy-vs-fallback authority split is explicit instead of implied lore

## Next Task

Execute `g04.025`: assess the AI runtime and suggestions systems against `080`.
