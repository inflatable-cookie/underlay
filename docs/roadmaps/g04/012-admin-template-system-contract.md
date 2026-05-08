# 012 - Admin Template System Contract

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.011` settles the shared workflow-shell layer. The next dependency is the
admin template system itself: entity list, detail, form, inline modules, and
the extension model proven through the current consumer rollout.

## Goals

- define the admin template system contract
- separate stable template extension points from rollout-era compatibility glue
- prepare the final tooling and contract-artifact lane on top of a clear
  template boundary

## Non-Goals

- consumer rollout execution work owned by `g03`
- implementation repair beyond light authority alignment needed to write the
  contract
- broad pattern/runtime replanning

## Inputs

- `ts/src/templates/**`
- template docs under `docs/usage/templates/**`
- rollout evidence in current roadmaps and consumer migrations

## Outputs

- [`docs/contracts/110-admin-template-system.md`](/Users/tom/Dev/projects/underlay/docs/contracts/110-admin-template-system.md)
- refreshed contract and roadmap front doors so `g04` now points at the final
  tooling lane

## Outcome

The admin template system contract now exists.

It settles:

- the three-level composition hierarchy
- the list/detail/form template ownership split
- the public section-level reuse boundary
- the stable extension model around loaders, snippets, and declarative config
- the explicit stop point for forms at `EntityFormPage`

It also records the main drift to assess later, especially stale docs status,
rollout docs lag, embedded non-exported config types, the heavy complexity
concentration inside `EntityList`, and the unresolved complex-shape evidence
from Dairy.

## Next Task

Execute `g04.013`: write `120-tooling-testing-and-contract-artifacts.md`.
