# 068 - AiRoutingAdmin Reassessment Wave

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 067

## Overview

`g01.067` confirmed that `SpaFormShell` still earns retained public Underlay
ownership because it owns shared SPA submit/result/navigation workflow across a
broad create/edit route family.

The strongest next public shell to challenge is `AiRoutingAdmin`.

Unlike `SpaFormShell`, this surface no longer shows a live consumer-app caller
family. The current residue is concentrated in the retained shell itself plus
Underlay docs/examples. That makes it the next honest reassessment target:
either it still proves a durable public operational-shell contract, or it has
collapsed far enough that it should retire or internalize instead of remaining
public by inertia.

## Research Basis

- current shared shell:
  - `ts/src/patterns/AiRoutingAdmin.svelte`
- related controller/runtime helpers:
  - `ts/src/patterns/ai-routing-ops.svelte.ts`
- active guide/examples:
  - `docs/guides/176-ai-runtime-routing.md`
  - `docs/guides/code/176-ai-runtime-routing/ai-routing-admin-page.svelte`
  - `docs/guides/code/176-ai-runtime-routing/ai-routing-admin-embedded.svelte`

## Decision Focus

- Determine whether `AiRoutingAdmin` still earns public Underlay ownership as a
  reusable operational shell
- or whether it has collapsed into:
  - guide/example-only composition
  - internal helper/controller usage
  - direct Poodle plus local ops wiring

## Consumer Upgrade Impact

- Do not add new public `AiRoutingAdmin` consumers while this wave is in
  progress.

## Planned Batches

## Batch 68.1 - Caller And Contract Matrix

- [x] Sweep the live `AiRoutingAdmin` caller family across Underlay and the
      active app repos.
- [x] Compare the retained shell contract against direct Poodle composition and
      the lower-level `ai-routing-ops` controller layer.
- [x] Decide whether the public shell still earns export status or should move
      toward retirement/internalization.

### Batch 68.1 Findings

The live residue scan shows no consumer-app callers across `acme-admin`,
`cp-admin`, or `dairy`.

The remaining surface is concentrated in:

- the public shell itself
- Underlay guide prose
- two guide/example files

The retained contract is not exposing unique generic capability anymore. The
shell is already mostly assembly over:

- Poodle `PageHeader`
- Poodle `Card`
- Poodle `Callout`
- Poodle `PageLoading`
- Poodle `DataTable`
- the retained `createAiRoutingOpsController` controller layer

That means the shared reusable value is now mostly in the lower-level
controller and data contracts, not in the page-shaped public shell wrapper.

## Current Judgment

`AiRoutingAdmin` no longer earns a public Underlay export.

The next honest move is retirement or internalization, with the guide/example
surface moving to direct Poodle composition over `createAiRoutingOpsController`
rather than keeping the page-shaped wrapper public by inertia.

## Next Task

Execute `g01.069` Batch `69.1` by writing the strict caller and contract matrix
for `RelationSelector` across the live app and guide surface, then decide
whether it still earns a public Underlay export or should start collapsing into
lower-level helpers plus direct Poodle composition.
