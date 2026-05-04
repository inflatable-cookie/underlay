# g03

`g03` is the active Underlay roadmap generation.

## Current State

`g03.006` is active (migration complete, pending validation). `g03.001`–`g03.005` and `g03.007`–`g03.008` are complete. `g02` is closed.

`g03` begins from the posture confirmed in `g02.007`: Poodle's package
consolidation is complete and the consumer normalization line is closed. The
live work is now a template-system overhaul that replaces hand-rolled admin page
composition with reusable higher-order Svelte templates.

## Active Lane

`g03.006` is the live execution lane.

Its job is to migrate acme-admin `/projects/+page.svelte` to `EntityListPage`
as the first proof that the template system works in practice.

## Completed Work

- `g03.001`: Generation rollover and posture restatement
- `g03.002`: Docs reorganization (`docs/usage/` created, template docs skeleton)
- `g03.003`: TS structure refactor (`ts/src/templates/` created, package export added)
- `g03.004`: `EntityList` section component (Level 2)
- `g03.005`: `EntityListPage` page shell (Level 1)
- `g03.007`: `EntityDetail` section component (Level 2)
- `g03.008`: `EntityDetailPage` page shell (Level 1)
- `g03.009`: acme-admin detail page migration proof (800 → 412 lines)
- `g03.010`: Dairy complex validation — 7 gaps identified
- `g03.011`: Underlay template skill created
- `g03.012`: Consumer rollout plan documented
- `g03.013`: `EntityForm` and `EntityFormPage` implemented

## Active Lane

`g03.014` is the live execution lane.

Its job is to migrate acme-admin project create/edit pages to `EntityFormPage` as proof.
