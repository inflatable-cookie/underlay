# 011 - Shared Patterns And Workflow Shells Contract

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.010` settles the retained TS runtime and client orchestration boundary.
The next dependency is the pattern-owned workflow layer itself: relation
selector, form shells, upload and list controllers, optimistic helpers,
navigation state, and other retained workflow shells.

## Goals

- define the shared patterns and workflow-shell contract
- separate true retained workflow shells from compatibility residue and app UI
- prepare the later admin-template contract on top of a clear patterns boundary

## Non-Goals

- implementation repair beyond light authority alignment needed to write the
  contract
- consumer rollout work owned by `g03`
- app-specific page composition or visual design

## Inputs

- `ts/src/patterns/**`
- dependent contracts for runtime, auth, media, AI, and Nightfire

## Outputs

- [`docs/contracts/100-shared-patterns-and-workflow-shells.md`](/Users/tom/Dev/projects/underlay/docs/contracts/100-shared-patterns-and-workflow-shells.md)
- refreshed contract and roadmap front doors so `g04` now points at the
  template lane

## Outcome

The shared patterns and workflow-shell contract now exists.

It settles:

- the retained SPA form shell boundary
- auth-aware data-loading workflow ownership
- list, pagination, batch-selection, and reorder controller families
- the relation-selector and drill-down workflow system
- media upload and optimistic workflow primitives
- the narrow retained auth-workflow component family

It also records the main drift to assess later, especially the split between
tiny root-barrel exports and the broader implementation surface, duplicate
auth-refresh workflow logic across controllers, the overlap with the template
system, and the question of which leftover helpers still earn pattern
ownership.

## Next Task

Execute `g04.012`: write `110-admin-template-system.md`.
