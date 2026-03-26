---
title: Poodle field cluster review handoff
status: active
owner: Platform
updated: 2026-03-23
tags: [coordination, handoff]
---

## Objective

Produce a narrow Poodle-side decision and implementation brief for the Underlay field cluster so only domain-neutral field behavior moves into Poodle and everything else stays out.

## Scope

- Review `FieldHint`, `FormError`, and `FieldSet` from Underlay against the existing Poodle `Field` contract and decide whether each surface should become a `Field` feature, a standalone grouped-field contract, or remain outside Poodle.
- If a surface belongs in Poodle, define the smallest contract shape needed for the Svelte implementation without widening into unrelated form frameworks.
- Do not widen the batch into auth flows, list/detail shells, form-tab orchestration, page shells, or generic migration mapping work that is already covered by existing Poodle surfaces.

## Inputs

- [/Users/betterthanclay/Dev/projects/underlay/docs/roadmaps/g01/042-poodle-adoption-and-underlay-ui-contraction.md](/Users/betterthanclay/Dev/projects/underlay/docs/roadmaps/g01/042-poodle-adoption-and-underlay-ui-contraction.md)
- [/Users/betterthanclay/Dev/projects/underlay/contracts/ui/poodle-adoption-underlay-surface-groups.json](/Users/betterthanclay/Dev/projects/underlay/contracts/ui/poodle-adoption-underlay-surface-groups.json)
- [/Users/betterthanclay/Dev/projects/underlay/ts/src/components/FieldHint.svelte](/Users/betterthanclay/Dev/projects/underlay/ts/src/components/FieldHint.svelte)
- [/Users/betterthanclay/Dev/projects/underlay/ts/src/components/FormError.svelte](/Users/betterthanclay/Dev/projects/underlay/ts/src/components/FormError.svelte)
- [/Users/betterthanclay/Dev/projects/underlay/ts/src/components/FieldSet.svelte](/Users/betterthanclay/Dev/projects/underlay/ts/src/components/FieldSet.svelte)
- [/Users/betterthanclay/Dev/projects/underlay/ts/src/components/FieldSetGrid.svelte](/Users/betterthanclay/Dev/projects/underlay/ts/src/components/FieldSetGrid.svelte)
- [/Users/betterthanclay/Dev/projects/poodle/packages/svelte/primitives/src/Field.svelte](/Users/betterthanclay/Dev/projects/poodle/packages/svelte/primitives/src/Field.svelte)
- [/Users/betterthanclay/Dev/projects/poodle/packages/svelte/primitives/src/Grid.svelte](/Users/betterthanclay/Dev/projects/poodle/packages/svelte/primitives/src/Grid.svelte)
- [/Users/betterthanclay/Dev/projects/poodle/docs/guides/svelte-developer-guide.md](/Users/betterthanclay/Dev/projects/poodle/docs/guides/svelte-developer-guide.md)

## Constraints

- Follow the repo instructions in `AGENTS.md`.
- Keep edits aligned with the active Northstar vision, roadmap, and log flow.
- Do not widen scope beyond the listed tasks.
- Respect the current Underlay decision that missing behavior should be added in Poodle only when the surface passes the Poodle eligibility rubric.
- Treat `FieldHint` and `FormError` as likely `Field` capabilities first, not automatic standalone Poodle exports.
- Treat `FieldSetGrid` as layout composition over existing Poodle primitives unless a stronger grouped-field contract clearly requires otherwise.

## Deliverables

- [/Users/betterthanclay/Dev/projects/poodle/docs/logs/2026-03/23-153503-poodle-field-cluster-review.md](/Users/betterthanclay/Dev/projects/poodle/docs/logs/2026-03/23-153503-poodle-field-cluster-review.md)
- [/Users/betterthanclay/Dev/projects/poodle/docs/architecture/001-poodle-system-shape.md](/Users/betterthanclay/Dev/projects/poodle/docs/architecture/001-poodle-system-shape.md)
- [/Users/betterthanclay/Dev/projects/poodle/docs/guides/svelte-developer-guide.md](/Users/betterthanclay/Dev/projects/poodle/docs/guides/svelte-developer-guide.md)
- [/Users/betterthanclay/Dev/projects/underlay/contracts/ui/poodle-adoption-underlay-surface-groups.json](/Users/betterthanclay/Dev/projects/underlay/contracts/ui/poodle-adoption-underlay-surface-groups.json)

## Acceptance Criteria

- The Poodle-side output makes an explicit per-surface decision for `FieldHint`, `FormError`, and `FieldSet`: `belongs_in_poodle`, `fold_into_existing_field`, or `stay_outside_poodle`.
- Any Poodle addition is justified as domain-neutral, cross-app, and contract-shaped rather than as a convenience wrapper for Underlay history.
- The brief states explicitly that `FieldHint` and `FormError` were reviewed as `Field` extensions first.
- The brief states explicitly whether `FieldSet` is a real grouped-field contract or whether grouped layout should stay as composition over Poodle layout primitives.
- The Underlay inventory is updated only if the Poodle review changes the current classification for the three field surfaces.

## Notes

- Current context: this handoff advances Underlay roadmap `g01.042` in [/Users/betterthanclay/Dev/projects/underlay/docs/roadmaps/g01/042-poodle-adoption-and-underlay-ui-contraction.md](/Users/betterthanclay/Dev/projects/underlay/docs/roadmaps/g01/042-poodle-adoption-and-underlay-ui-contraction.md), after the Underlay Svelte export inventory was reduced from a broad “maybe Poodle” list to a narrow field-only review batch.
- Decisions: the user explicitly wants careful filtering so only truly non-domain-specific surfaces move into Poodle; `Badge` and `TextButton` were already removed from the real Poodle-work queue because Poodle `Pill` and `Button variant="ghost"` already cover them.
- Watch-outs: Underlay `FormTabsProvider` and `FormTabsSection` are form-orchestration helpers, not good first Poodle candidates; detail/list-card surfaces mostly collapse to composition over existing Poodle `Card`, `ListCard`, `DetailRow`, `DetailSection`, and layout primitives.
- Next move: in the Poodle thread, compare the Underlay field surfaces directly against Poodle `Field` and decide whether the missing behavior should become `Field` slots/props, a grouped-field contract, or remain external composition.

## Completion Protocol

1. Update the Poodle log and any touched architecture or guide docs with the outcome of this batch.
2. If the review changes Underlay’s migration posture, update [/Users/betterthanclay/Dev/projects/underlay/contracts/ui/poodle-adoption-underlay-surface-groups.json](/Users/betterthanclay/Dev/projects/underlay/contracts/ui/poodle-adoption-underlay-surface-groups.json) and leave the next task aligned with roadmap `g01.042`.
3. Record unresolved risks explicitly, especially any ambiguity about whether grouped-field behavior is a real design-system contract or just layout composition.
4. Leave one clear next task for the following thread.
