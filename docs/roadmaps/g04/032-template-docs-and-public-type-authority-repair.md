# 032 - Template Docs And Public Type Authority Repair

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.031` assessed the admin template system against `110`.

The next repair is bounded:

- the primary template docs still teach stale status and invalid usage syntax
- the shared template config surface is still mostly private duplicated
  interfaces instead of importable public types

## Goals

- make the primary template docs current and snippet-correct
- promote the key template config interfaces into a dedicated exported public
  type surface where that boundary is genuinely shared
- align the contract/docs/front doors so they describe the active template
  system honestly

## Non-Goals

- redesigning the template architecture in the same batch
- re-opening consumer rollout work in `g03`
- skipping ahead to tooling assessment before the template surface is honest

## Inputs

- [docs/roadmaps/g04/031-admin-template-system-assessment.md](/Users/tom/Dev/projects/underlay/docs/roadmaps/g04/031-admin-template-system-assessment.md)
- [docs/contracts/110-admin-template-system.md](/Users/tom/Dev/projects/underlay/docs/contracts/110-admin-template-system.md)
- `ts/src/templates/**`
- `docs/usage/templates/**`

## Exit Criteria

- primary template docs no longer teach stale status or invalid usage patterns
- the main shared template config types are exported from a clearer public
  surface instead of duplicated private interfaces
- `110` and the related front doors point at the real current lane
- the tooling assessment can proceed without template-surface authority drift

## Changes

- added a dedicated exported template type surface in
  [ts/src/templates/template.types.ts](/Users/tom/Dev/projects/underlay/ts/src/templates/template.types.ts:1)
  and re-exported it from
  [ts/src/templates/index.ts](/Users/tom/Dev/projects/underlay/ts/src/templates/index.ts:1)
- aligned the live components to that public type surface:
  [EntityListPage.svelte](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityListPage.svelte:1),
  [EntityList.svelte](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityList.svelte:1),
  [EntityDetailPage.svelte](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityDetailPage.svelte:1),
  [EntityDetail.svelte](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityDetail.svelte:1),
  and [EntityAttributeList.svelte](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityAttributeList.svelte:1)
- rewrote the primary usage docs so they describe the active template system
  honestly and use snippet-based examples instead of JSX-like pseudo-Svelte:
  [000-template-system-overview.md](/Users/tom/Dev/projects/underlay/docs/usage/templates/000-template-system-overview.md:1),
  [entity-list-page.md](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-list-page.md:1),
  [entity-list-section.md](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-list-section.md:1),
  [entity-detail-page.md](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-detail-page.md:1),
  [entity-detail-section.md](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-detail-section.md:1),
  [template-api-reference.md](/Users/tom/Dev/projects/underlay/docs/usage/templates/template-api-reference.md:1)
- converted [consumer-rollout.md](/Users/tom/Dev/projects/underlay/docs/usage/templates/consumer-rollout.md:1)
  into an explicit historical snapshot and updated
  [110-admin-template-system.md](/Users/tom/Dev/projects/underlay/docs/contracts/110-admin-template-system.md:1)
  so its next-task pointer matches the live lane

## Result

The template authority repair is now honest and bounded:

- the template docs no longer present the system as draft or in-development
- the main shared config interfaces are importable public types instead of
  docs-only private duplicates
- the template contract and docs now point at the real current surface

## Next Task

Execute `g04.033`: assess tooling, testing, and contract artifacts against
`120`.
