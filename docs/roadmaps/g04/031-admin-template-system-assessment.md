# 031 - Admin Template System Assessment

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.030` repaired the bounded shared-pattern drift enough for the next
assessment wave to proceed honestly.

The next system family in the contract order is the admin template system,
anchored by `110`.

## Goals

- assess the live admin template implementation against `110`
- separate true contract failures from expected rollout residue or older
  consumer migration evidence
- identify the smallest honest repair set for the retained template boundary
- leave explicit findings and a bounded next lane instead of broad template
  churn

## Non-Goals

- redesigning consumer app pages in the same batch
- re-opening the shared-pattern lane without a new contract failure
- skipping ahead to the tooling assessment before the template boundary is
  clear

## Inputs

- [docs/contracts/110-admin-template-system.md](/Users/tom/Dev/projects/underlay/docs/contracts/110-admin-template-system.md)
- `ts/src/templates/**`
- current template docs and retained rollout evidence

## Exit Criteria

- the live admin template implementation is reviewed against `110`
- the real findings are documented in severity order
- the next repair step is expressed as one bounded roadmap lane or a small
  repair set
- the later tooling assessment can proceed without ambiguity about the
  retained template boundary

## Findings

### 1. The primary template docs still teach stale status and invalid usage patterns

Severity: high

The live code is active and already used in real consumer rollouts, but the
primary docs still present the system as draft or “in development”, and some
examples are not valid Svelte template usage at all. The overview and list/detail
pages still show JSX-like inline component expressions such as
`renderItem={(item) => <ProjectCard {item} />}` and tab content like
`content: <EntityDetail ... />`, which do not match the actual snippet-based
Svelte contract.

Evidence:

- [docs/usage/templates/template-api-reference.md](/Users/tom/Dev/projects/underlay/docs/usage/templates/template-api-reference.md:1)
- [docs/usage/templates/entity-list-page.md](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-list-page.md:1)
- [docs/usage/templates/entity-detail-page.md](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-detail-page.md:1)
- [docs/usage/templates/000-template-system-overview.md](/Users/tom/Dev/projects/underlay/docs/usage/templates/000-template-system-overview.md:1)
- [docs/usage/templates/consumer-rollout.md](/Users/tom/Dev/projects/underlay/docs/usage/templates/consumer-rollout.md:1)

Impact:

- the primary docs are not trustworthy as the public teaching surface
- callers can copy examples that do not match the real Svelte/snippet API
- the contract’s “active retained system” story is undermined by its own docs

### 2. The stable template config surface is still mostly docs-only and duplicated inside components

Severity: medium

The contract already suspected this, and the code confirms it. `EntityListPage`
and `EntityList` each carry private copies of `FilterConfig`,
`BatchActionConfig`, and `ReorderConfig`, while `EntityDetailPage` and
`EntityDetail` each define their own local config types as internal
interfaces. Only `EntityListCard` currently exports a proper dedicated type
surface.

Evidence:

- [ts/src/templates/EntityListPage.svelte](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityListPage.svelte:18)
- [ts/src/templates/EntityList.svelte](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityList.svelte:45)
- [ts/src/templates/EntityDetailPage.svelte](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityDetailPage.svelte:17)
- [ts/src/templates/EntityDetail.svelte](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityDetail.svelte:4)
- [ts/src/templates/index.ts](/Users/tom/Dev/projects/underlay/ts/src/templates/index.ts:1)
- [ts/src/templates/entity-list-card.types.ts](/Users/tom/Dev/projects/underlay/ts/src/templates/entity-list-card.types.ts:1)

Impact:

- the public template API is partly “whatever the docs say” instead of
  importable first-class types
- page-shell and section components can drift apart even when they are meant to
  describe the same config surface
- consumer type discovery is worse than the contract implies

### 3. The implementation is otherwise broadly aligned with the three-level template model

Severity: low

The actual code still fits the retained template boundary. `EntityListPage`
and `EntityDetailPage` act as page shells over `EntityList` and
`EntityDetail`, `EntityFormPage` stays a page wrapper rather than a declarative
form engine, and `EntityListCard` remains the only dedicated Level 2.5 helper.
`EntityList` does carry most of the real complexity, but that is more an
authority/export problem than a proof that the template split failed.

Evidence:

- [ts/src/templates/index.ts](/Users/tom/Dev/projects/underlay/ts/src/templates/index.ts:1)
- [ts/src/templates/EntityListPage.svelte](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityListPage.svelte:1)
- [ts/src/templates/EntityList.svelte](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityList.svelte:1)
- [ts/src/templates/EntityDetailPage.svelte](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityDetailPage.svelte:1)
- [ts/src/templates/EntityDetail.svelte](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityDetail.svelte:1)
- [ts/src/templates/EntityFormPage.svelte](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityFormPage.svelte:1)

Impact:

- the next lane should be a bounded docs/type-surface repair, not a broad
  template rewrite

## Assessment Result

The next real lane is a bounded template authority repair:

- make the primary template docs honest, current, and snippet-correct
- promote the main config interfaces into a clearer exported type surface
  instead of leaving them duplicated inside component internals
- leave the actual page-shell/section architecture in place unless the later
  tooling assessment finds a stronger reason to revisit it

## Next Task

Execute `g04.032`: repair the template docs and public type authority.
