# 030 - Reorder Baseline And Pattern Surface Guidance Repair

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.029` assessed the retained shared patterns and workflow shells against
`100`.

The next repair is bounded:

- `createReorderController()` does not rebase its baseline after successful
  submit
- several guides still teach a broader root `@decodelabs/underlay/patterns`
  surface than the contract now allows

## Goals

- fix the shared reorder controller so successful submit clears dirty state and
  makes `reset()` return to the last committed order
- add focused validation for the repaired reorder behavior
- align the sharpest stale guides to the real retained root-barrel and runtime
  subpath boundary

## Non-Goals

- redesigning the wider reorder UX or template integration in the same batch
- broad runtime or pattern barrel reorganization
- skipping ahead to the template assessment before the retained pattern
  boundary is honest again

## Inputs

- [docs/roadmaps/g04/029-shared-patterns-and-workflow-shells-assessment.md](/Users/tom/Dev/projects/underlay/docs/roadmaps/g04/029-shared-patterns-and-workflow-shells-assessment.md)
- [docs/contracts/100-shared-patterns-and-workflow-shells.md](/Users/tom/Dev/projects/underlay/docs/contracts/100-shared-patterns-and-workflow-shells.md)
- `ts/src/patterns/reorder-controller.svelte.ts`
- selected guide pages under `docs/guides/`

## Exit Criteria

- successful reorder submit rebases the shared controller baseline
- focused tests cover the repaired reorder lifecycle
- the sharpest stale guides no longer teach the wrong `patterns` surface
- the next template assessment can treat the shared pattern boundary as stable

## Changes

- repaired [ts/src/patterns/reorder-controller.svelte.ts](/Users/tom/Dev/projects/underlay/ts/src/patterns/reorder-controller.svelte.ts:1)
  so successful submit now rebases the controller baseline instead of leaving
  `original` frozen at the pre-submit order
- tightened the focused test in
  [ts/tests/patterns/reorder-controller.test.ts](/Users/tom/Dev/projects/underlay/ts/tests/patterns/reorder-controller.test.ts:1)
  to prove that successful submit clears dirty state and makes `reset()`
  return to the committed order while failed submit leaves the old baseline in
  place
- aligned the sharpest stale guide examples with the retained public surface:
  [docs/guides/092-selection-suggestions.md](/Users/tom/Dev/projects/underlay/docs/guides/092-selection-suggestions.md:1)
  now teaches `runtime/data` for selection history and
  `client/suggestions` for request-shape helpers, and
  [docs/guides/095-navigation-context.md](/Users/tom/Dev/projects/underlay/docs/guides/095-navigation-context.md:1)
  now shows `PageHeader` from Poodle instead of the retired Underlay visual
  shell path

## Result

The shared-pattern repair is now honest and bounded:

- reorder controllers become clean after a successful commit
- the focused test proves the repaired baseline lifecycle directly
- the sharpest guide drift no longer teaches a broader root
  `@decodelabs/underlay/patterns` surface than the contract allows

## Next Task

Execute `g04.031`: assess the admin template system against `110`.
