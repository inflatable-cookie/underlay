---
title: Nightfire Surface Audit
owner: Codex
status: complete
updated: 2026-03-30
---

# Nightfire Surface Audit

## Goal

Audit the retained `@decodelabs/underlay/nightfire` package surface and decide
whether any useful reorganization can be done now for a future standalone
extraction without changing the current public package boundary.

## Outcome

The public `nightfire` package surface stays retained as-is for now.

There is no honest broad extraction batch yet:

- the live caller family is real
- the package surface is coherent
- the remaining work is future package extraction planning, not current public
  boundary reduction

The useful cleanup in this batch was internal ownership cleanup only:

- duplicated tiny editor wrappers were removed where Nightfire can use direct
  Poodle or local markup instead
- top-level Nightfire now better reflects its real public surface

## Judgment

Nightfire is still a retained package, not migration residue.

The public surface in `index.ts` is concentrated around:

- `NightfireEditor`
- `NightfireRenderer`
- `NightfireBlockEditor`
- slash commands
- registries
- strategies
- media context
- validation/serialization helpers

That is a coherent package surface for now, and there is real live usage in:

- retained guides/examples
- `acme-ui` Nightfire block registration points
- app-level strategy configuration

The useful cleanup was internal placement, not API surgery.

The internal cleanup in this line removed duplicated tiny wrappers rather than
preserving them under a new internal folder. Nightfire now relies directly on
Poodle `Select` where the generic primitive already covers the needed behavior,
and the one-use field-error wrapper was inlined into `NightfireEditor`.

## Changes

- removed `NightfireSelect.svelte` in favor of direct Poodle `Select`
- inlined `NightfireFieldError.svelte` into `NightfireEditor.svelte`
- updated internal Nightfire imports accordingly
- updated roadmap front doors and durable inventory

## Next Task

Take the next broad TS audit on the retained `utils` surface, and decide
whether it is already clean as a small standalone helper package boundary or
whether there is any dead or misplaced utility residue left to remove.
