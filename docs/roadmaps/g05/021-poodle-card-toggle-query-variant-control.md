# g05.021 — Poodle Card Toggle Query Variant Control

## Why

Query variants need a compact card-based selector that can sit above an
`EntityList` filter toolbar.

This is generic UI behavior, so Poodle should own it:

- normal button toggle behavior belongs in `ToggleGroup`
- card-shaped toggle behavior belongs beside `CardRadioGroup`
- Underlay should not create a list-specific card-toggle primitive

The correct Poodle name is `CardToggleGroup`, matching `CardRadioGroup`.

## Goal

Add deactivation-capable toggle behavior in Poodle, then expose a reusable
`CardToggleGroup` for card-based single selection.

## Scope

Primary Poodle targets:

- extend `ToggleGroup` with explicit deactivation support
- add `CardToggleGroup`
- align naming, docs, specimens, and public exports with `CardRadioGroup`
- preserve `CardRadioGroup` as the radio semantics component

Expected behavior:

- `ToggleGroup` can optionally allow the active value to be cleared
- `CardToggleGroup` can optionally allow the active card to be cleared
- deactivation is opt-in, not the default behavior
- `CardToggleGroup` supports rich card labels, descriptions, counts, icon or
  leading content, active state, disabled state, and accessible keyboard use

## Boundary

Poodle owns:

- component name: `CardToggleGroup`
- card-toggle visual treatment
- deactivation mechanics
- ARIA and keyboard behavior
- docs, specimens, and public export

Underlay owns:

- mapping list query variants into `CardToggleGroup` options
- deciding whether clearing the active card returns to a default query variant
- URL/query state
- API query semantics

## Consumer Upgrade Impact

Expected:

- no required consumer migration for existing `ToggleGroup` or
  `CardRadioGroup` callers
- new opt-in props are additive
- Underlay can depend on `CardToggleGroup` once the Poodle release is available

## Acceptance

- Poodle `ToggleGroup` supports controlled and uncontrolled deactivation
- Poodle exports `CardToggleGroup`
- Poodle docs distinguish:
  - `CardRadioGroup` for required radio selection
  - `CardToggleGroup` for optional/toggleable card selection
- specimens include active, inactive, disabled, and deactivation examples
- Underlay can consume the new component without local wrapper UI

## Next Task

Complete.

Landed in Poodle:

- `ToggleGroup` now supports opt-in single-select deactivation
- `CardToggleGroup` is exported from `@inflatable-cookie/poodle-svelte`
- `CardToggleGroup` has a component contract, preview metadata, and specimens
- Poodle parity metadata includes the new public component

Poodle commit:

- `efe381eb` Add card toggle group component
