# 003 - TS Structure Refactor

Status: complete
Owner: repo maintainers
Updated: 2026-05-04

## Context

The template components need to be a first-class package surface, not buried in
`ts/src/patterns/` alongside auth workflows and RelationSelector. The current
`patterns/` directory is cluttered and sends the wrong signal about where the
important composition code lives.

## Goals

- create `ts/src/templates/` as a first-class surface
- add `"./templates"` export to `package.json`
- shrink `ts/src/patterns/` to: auth workflows, RelationSelector, and runtime
  helpers
- establish clear file naming for template components

## Non-Goals

- rewriting auth workflow components
- changing the public API of existing patterns exports
- touching runtime/, client/, nightfire/, or utils/

## Execution Plan

### Batch 3.1 - Directory Creation

- [x] create `ts/src/templates/`
- [x] create `ts/src/templates/index.ts` (barrel export)

### Batch 3.2 - Package Export

- [x] add `"./templates"` export to `package.json`
- [x] verify export resolves correctly

### Batch 3.3 - Patterns Cleanup

- [x] audit `ts/src/patterns/` contents
- [x] confirm what stays: auth-workflows/, RelationSelector/, FormShell.svelte,
      SpaFormShell.svelte, and runtime helper files
- [x] confirm what moves or is deprecated: any remaining generic composition
      helpers that belong in templates

## File Plan

```
ts/src/
  templates/
    EntityListPage.svelte      # Level 1: full page shell
    EntityDetailPage.svelte    # Level 1: full page shell
    EntityFormPage.svelte      # Level 1: full page shell (later)
    EntityList.svelte          # Level 2: reusable section
    EntityDetail.svelte        # Level 2: reusable section
    EntityForm.svelte          # Level 2: reusable section (later)
    index.ts
  patterns/
    auth-workflows/            # retained
    RelationSelector/          # retained
    FormShell.svelte           # retained
    SpaFormShell.svelte        # retained
    ...runtime helpers...      # retained
```

## Exit Criteria

- `ts/src/templates/` exists with `index.ts`
- `package.json` exports `"./templates"`
- `ts/src/patterns/` is audited and cleaned
- import path `@inflatable-cookie/underlay/templates` resolves

## Next Task

Execute `g03.004`: build `EntityList` (Level 2 section) — the reusable list
section that powers both `EntityListPage` and detail tabs.
