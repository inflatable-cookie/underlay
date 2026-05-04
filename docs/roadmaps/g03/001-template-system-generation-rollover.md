# 001 - Template System Generation Rollover

Status: complete
Owner: repo maintainers
Updated: 2026-05-04

## Context

`g02.007` completed the Poodle Svelte package consolidation. The broad
consumer-family normalization line is closed. `g02` has served its purpose:
freezing the proof-app posture, running bounded downstream gates, and closing
the package-boundary cleanup.

The live work is no longer consumer normalization. It is building a reusable
template system that replaces the 300–800 line hand-rolled admin pages in the
reference apps with ~50–100 line declarative configurations.

## Goals

- close `g02` cleanly and open `g03` as the template-system generation
- restate the active seams: templates own higher-order composition, Poodle owns
  primitives, Underlay retains runtime/auth/nightfire
- sequence the first bounded template implementation waves

## Non-Goals

- reopening the g02 consumer-normalization line
- changing Poodle component APIs
- broad consumer rollout before templates are proven

## Execution Plan

### Batch 1.1 - Generation Closeout

- [x] mark all g02 roadmaps as complete in `docs/roadmaps/g02/README.md`
- [x] create `docs/roadmaps/g03/README.md`
- [x] refresh roadmap front doors to point at g03

### Batch 1.2 - Posture Restatement

- [x] confirm the three-level composition hierarchy:
  - Level 1: Page shells (`EntityListPage`, `EntityDetailPage`, `EntityFormPage`)
  - Level 2: Sections (`EntityList`, `EntityDetail`, `EntityForm`) — reusable in
    pages, tabs, and dialogs
  - Level 3: Primitives (Poodle: `PageHeader`, `DataTable`, `DetailSection`, etc.)
- [x] confirm naming: `Entity*` not `Admin*`
- [x] confirm Level 2 sections are public patterns, not internal nesting

## Exit Criteria

- g03 is clearly the active generation
- g02 is closed with all roadmaps marked complete
- template-system posture is explicit in the new generation

## Next Task

Execute `g03.002`: reorganize docs from `guides/` into `usage/`, delete
deprecated UI guides, and establish the template documentation front door.
