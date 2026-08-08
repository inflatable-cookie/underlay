# 007 - Poodle Svelte Package Consolidation Migration

Status: complete
Owner: repo maintainers
Updated: 2026-04-11

## Context

Poodle’s Svelte surface was refactored again after the broad consumer-family
rollout line closed. The old split package names:

- `@inflatable-cookie/poodle-svelte-primitives`
- `@inflatable-cookie/poodle-svelte-composites`

have been consolidated behind the new package:

- `@inflatable-cookie/poodle-svelte`

The current fallout is not another pattern-normalization program. It is a
bounded package-consolidation migration across Underlay and the consumer
families that still import the split names directly.

## Goals

- re-anchor Underlay and the live consumer families on the new
  `@inflatable-cookie/poodle-svelte` package surface
- update source imports, package manifests, and local alias wiring where the
  split package names still exist
- validate the touched repos cleanly and leave the untouched consumer families
  explicitly confirmed clean

## Non-Goals

- reopening the broad consumer-family rollout line
- changing the Poodle component API beyond the package boundary shift
- widening into unrelated app cleanup while touching the migration surface
- rewriting archival logs or closed roadmap evidence that still mentions the
  old split packages historically

## Scope

### In scope

- `underlay`
- `underlay-reference`
- `contact-patch`
- `acowtancy`
- `loophole/composer`
- active Underlay docs/guides that teach the live Poodle import surface

### Out of scope

- rewriting historical logs/roadmaps just to erase the old package names
- `compli-me` and `songsprout` unless the residue scan reveals
  live split-package usage
- new shared-surface design work inside Poodle itself

## Execution Plan

### Batch 7.1 - Live Import And Manifest Migration

- [x] replace live source imports from the split package names with
      `@inflatable-cookie/poodle-svelte`
- [x] update package manifests and local alias wiring to the consolidated
      package
- [x] confirm which consumer families are actually touched versus already clean

### Batch 7.2 - Active Guide And Contract Alignment

- [x] update active Underlay guides and active contract surfaces to teach
      `@inflatable-cookie/poodle-svelte`
- [x] leave archival docs, logs, and closed roadmap evidence intact

### Batch 7.3 - Validation And Residue Freeze

- [x] run targeted validation across the repos touched by the migration
- [x] run a final residue scan across Underlay and the six consumer families
- [x] record the migration result and close the lane if no further package
      fallout remains

## Initial Evidence

Initial live source and manifest fallout was concentrated in:

- `underlay`
- `underlay-reference`
- `contact-patch`
- `acowtancy`
- `loophole/composer`

Current scan evidence says these families are already clean for the old split
Poodle package names:

- `compli-me`
- `songsprout`

## Consumer Upgrade Impact

Impact class: `consumer-visible`

This lane changed the shared import posture from the split Poodle package names
to the consolidated `@inflatable-cookie/poodle-svelte` package across Underlay and affected
consumers. Existing callers were migrated onto the surviving unified package
surface, including replacing temporary `SearchField`, `TextArea`, and
`ReorderableList` compatibility usage in live consumer code before closure.

## Exit Criteria

- no live source imports remain to `@inflatable-cookie/poodle-svelte-primitives` or
  `@inflatable-cookie/poodle-svelte-composites` in Underlay or the affected consumer families
- touched package manifests and local alias wiring point at `@inflatable-cookie/poodle-svelte`
- active Underlay guides teach the consolidated package surface
- residue scan explicitly confirms which consumer families were already clean

## Result

Complete. Underlay, `underlay-reference`, `contact-patch`, `acowtancy`, and
`loophole/composer` are migrated to `@inflatable-cookie/poodle-svelte`; `compli-me` and
`songsprout` remain explicitly clean for this package-boundary change; and the
active Underlay guide layer now teaches the consolidated import surface.
