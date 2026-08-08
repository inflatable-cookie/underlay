# 029 - Shared Patterns And Workflow Shells Assessment

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.028` repaired the bounded runtime/client drift enough for the next
assessment wave to proceed honestly.

The next system family in the contract order is shared patterns and workflow
shells, anchored by `100`.

## Goals

- assess the live shared patterns and workflow shells implementation against
  `100`
- separate true contract failures from expected runtime-barrel compatibility
  residue
- identify the smallest honest repair set for the retained pattern boundary
- leave explicit findings and a bounded next lane instead of broad workflow
  churn

## Non-Goals

- redesigning consumer app workflows in the same batch
- re-opening the runtime/client lane without a new contract failure
- skipping ahead to template assessment before the pattern boundary is clear

## Inputs

- [docs/contracts/100-shared-patterns-and-workflow-shells.md](/Users/tom/Dev/projects/underlay/docs/contracts/100-shared-patterns-and-workflow-shells.md)
- `ts/src/patterns/**`

## Exit Criteria

- the live patterns/workflow implementation is reviewed against `100`
- the real findings are documented in severity order
- the next repair step is expressed as one bounded roadmap lane or a small
  repair set
- the later template/tooling assessments can proceed without ambiguity about
  the retained pattern boundary

## Findings

### 1. `createReorderController()` never rebases after a successful submit

Severity: high

The shared reorder controller keeps `original` frozen to the initial item
order. `submit()` calls the caller-provided save function but never promotes
the committed `pending` order into the controller baseline. That means
`isDirty` stays true after a successful save and `reset()` still jumps back to
the pre-submit order.

Evidence:

- [ts/src/patterns/reorder-controller.svelte.ts](/Users/tom/Dev/projects/underlay/ts/src/patterns/reorder-controller.svelte.ts:65)
- [docs/contracts/100-shared-patterns-and-workflow-shells.md](/Users/tom/Dev/projects/underlay/docs/contracts/100-shared-patterns-and-workflow-shells.md:160)
- [docs/patterns/reorderable-collections.md](/Users/tom/Dev/projects/underlay/docs/patterns/reorderable-collections.md:21)

Impact:

- long-lived reorder sessions do not become clean after commit
- the retained reorder controller does not fully satisfy the shared
  batch-commit workflow contract it claims to own
- current template callers mask this by exiting reorder mode immediately after
  submit, but the reusable controller itself is still wrong

### 2. The supporting docs still over-teach `@inflatable-cookie/underlay/patterns` as a broad public entrypoint

Severity: medium

The code matches the contract’s “small root barrel, broader runtime subpaths”
story, but several guides still teach imports that do not match the real
retained surface. The sharpest examples are the selection-history guide still
teaching suggestion helpers from `@inflatable-cookie/underlay/patterns`, and the
navigation guide still showing `PageHeader` from `@inflatable-cookie/underlay/patterns`
even though visible page-shell composition now belongs in Poodle.

Evidence:

- [ts/src/patterns/index.ts](/Users/tom/Dev/projects/underlay/ts/src/patterns/index.ts:1)
- [ts/src/runtime/data.ts](/Users/tom/Dev/projects/underlay/ts/src/runtime/data.ts:1)
- [ts/src/runtime/relations.ts](/Users/tom/Dev/projects/underlay/ts/src/runtime/relations.ts:1)
- [docs/guides/092-selection-suggestions.md](/Users/tom/Dev/projects/underlay/docs/guides/092-selection-suggestions.md:47)
- [docs/guides/095-navigation-context.md](/Users/tom/Dev/projects/underlay/docs/guides/095-navigation-context.md:63)

Impact:

- the retained pattern boundary is harder for consumers to learn correctly
- the guides still imply a wider root `patterns` surface than the contract now
  allows

### 3. The rest of the retained pattern layer is broadly aligned

Severity: low

Outside the reorder baseline bug and the stale guidance, the live pattern
families look materially aligned with `100`. The auth-aware loading helpers,
relation-selector context layer, local search adapters, media upload flow, SPA
form shells, and optimistic primitives all fit the retained workflow-shell
story more than the old UI-wrapper story.

Evidence:

- [ts/src/patterns/auth.ts](/Users/tom/Dev/projects/underlay/ts/src/patterns/auth.ts:1)
- [ts/src/patterns/authenticated-data.svelte.ts](/Users/tom/Dev/projects/underlay/ts/src/patterns/authenticated-data.svelte.ts:1)
- [ts/src/patterns/RelationSelector/context.svelte.ts](/Users/tom/Dev/projects/underlay/ts/src/patterns/RelationSelector/context.svelte.ts:1)
- [ts/src/patterns/local-search.ts](/Users/tom/Dev/projects/underlay/ts/src/patterns/local-search.ts:1)
- [ts/src/patterns/drilldown-search.ts](/Users/tom/Dev/projects/underlay/ts/src/patterns/drilldown-search.ts:1)
- [ts/src/patterns/media-upload-flow.svelte.ts](/Users/tom/Dev/projects/underlay/ts/src/patterns/media-upload-flow.svelte.ts:1)
- [ts/src/patterns/SpaFormShell.svelte](/Users/tom/Dev/projects/underlay/ts/src/patterns/SpaFormShell.svelte:1)
- [ts/src/patterns/FormShell.svelte](/Users/tom/Dev/projects/underlay/ts/src/patterns/FormShell.svelte:1)

Impact:

- the next lane should be a bounded repair, not a broad shared-pattern rewrite

## Assessment Result

The next real lane is a bounded shared-pattern repair:

- fix `createReorderController()` so successful submit rebases the controller
  baseline and clears dirty state honestly
- align the supporting guides with the retained root-barrel and runtime-subpath
  boundary instead of teaching the older broad `patterns` import story
- keep the rest of the pattern layer stable unless the later template
  assessment finds a stronger cross-boundary problem

## Next Task

Execute `g04.030`: repair the reorder baseline semantics and the stale
pattern-surface guidance.
